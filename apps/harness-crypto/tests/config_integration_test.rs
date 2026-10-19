#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::env::temp_dir;
use std::ffi::OsString;
use std::fs::{create_dir_all, remove_dir_all, write};
use std::path::PathBuf;
use std::process::id;

use domain::{
    AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::CanonicalFieldValue;
use pairing::{PairingConcrete, PrecompileEncoding, TargetGroupEncodingIdentifier};

use harness_crypto::{
    CanonicalDeliveryContextOverrides, ConfigureDepsOverrides, ConfigurePayloadOverrides,
    ConfigureSuccessReturn, CurveSelectionOverrides, GenerateConcrete, HARNESS_DEFAULTS,
    HarnessSelectionsOverrides, HarnessStatementsOverrides, MsmTermCountConstructorParamsOverrides,
    OutputDestination, SolidityCompilerVersionOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    VectorKind, VectorKindQuantitiesOverrides, admit_generate, admit_output, admit_selections,
    admit_statements, build_canonical_delivery_context, build_configure_deps,
    build_configure_params, build_configure_payload, build_curve_selection,
    build_evm_generate_settings, build_harness_selections, build_harness_statements,
    build_msm_term_count, build_network_profile, build_solidity_compiler_version,
    build_spdx_license_identifier, build_vector_count, build_vector_kind_quantities, layers,
};

/// Contract: the own entry `the_shipped_defaults_admit_the_selections`; the
///   selections the shipped defaults admit equal the declared concretes:
///   every selection table its builder's default, the two curves in document
///   order, the four benchmarked concretes in document order, and the two
///   precompile encodings.
/// Arrange: `deps` naming the real `layers`, `admit_selections`,
///   `admit_statements`, `admit_generate`, and `admit_output`; the payload
///   `build_configure_payload` over `HARNESS_DEFAULTS` with `arguments` the
///   program name alone.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.selections` equals `build_harness_selections`
///   with `curves`, `benchmark_concretes`, and `precompile_encodings` the
///   shipped values, by `assert_eq!`.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn the_shipped_defaults_admit_the_selections() {
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
            arguments: Some(vec![OsString::from("harness-crypto")]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the shipped defaults admit");
    };
    assert_eq!(
        configuration.selections,
        build_harness_selections(HarnessSelectionsOverrides {
            curves: Some(vec![
                build_curve_selection(Default::default()),
                build_curve_selection(CurveSelectionOverrides {
                    pairing: Some(PairingConcrete::Bls12381Arkworks),
                    target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
                }),
            ]),
            benchmark_concretes: Some(vec![
                build_curve_selection(CurveSelectionOverrides {
                    pairing: Some(PairingConcrete::Bn254Arkworks),
                    target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
                }),
                build_curve_selection(CurveSelectionOverrides {
                    pairing: Some(PairingConcrete::Bn254Halo2curves),
                    target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
                }),
                build_curve_selection(CurveSelectionOverrides {
                    pairing: Some(PairingConcrete::Bls12381Arkworks),
                    target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
                }),
                build_curve_selection(CurveSelectionOverrides {
                    pairing: Some(PairingConcrete::Bls12381Halo2curves),
                    target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
                }),
            ]),
            precompile_encodings: Some(vec![
                PrecompileEncoding::Eip196Eip197,
                PrecompileEncoding::Eip2537,
            ]),
            ..Default::default()
        })
    );
    // the_shipped_defaults_admit_the_selections
}

/// Contract: the own entry
///   `the_shipped_defaults_admit_the_network_profile_and_statements`; the
///   network profile and the statements the shipped defaults admit equal
///   the shipped values: the Base Sepolia chain and the zero entitlement
///   contract, both contexts over the shipped identifiers, fields, and
///   expiry, the two purposes, the two identities, and the proof identity.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` the program name alone.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.network` equals
///   `build_network_profile(Default::default())` and
///   `configuration.statements` equals `build_harness_statements` with both
///   contexts the shipped context, `identities` the shipped pair of
///   thirty-two-byte vectors, and `proof_identity` the first identity, by
///   `assert_eq!`.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn the_shipped_defaults_admit_the_network_profile_and_statements() {
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
            arguments: Some(vec![OsString::from("harness-crypto")]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the shipped defaults admit");
    };
    let network = build_network_profile(Default::default());
    let mut identifier = [0x00; 32];
    identifier[31] = 0x01;
    let mut other_identity = [0x00; 32];
    other_identity[31] = 0x02;
    let mut one = [0x00; 32];
    one[24..].copy_from_slice(&1u64.to_be_bytes());
    let mut seller = [0x00; 20];
    seller[19] = 0x01;
    let mut buyer = [0x00; 20];
    buyer[19] = 0x02;
    let context = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some(identifier),
                version: Some(1),
            },
        )),
        chain: Some(network.chain.clone()),
        entitlement_contract: Some(network.entitlement_contract.clone()),
        asset_identity_hash: Some(build_asset_identity_hash(
            AssetIdentityHashConstructorParamsOverrides {
                bytes: Some(identifier),
            },
        )),
        parameter_set_identifier: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some(identifier),
            },
        )),
        source_entitlement: Some(CanonicalFieldValue::Unsigned256(one)),
        target_entitlement: Some(CanonicalFieldValue::Unsigned256(one)),
        old_interval: Some(CanonicalFieldValue::Unsigned64(0)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(1)),
        seller: Some(CanonicalFieldValue::FixedBytes20(seller)),
        buyer: Some(CanonicalFieldValue::FixedBytes20(buyer)),
        expiry: Some(4102444800),
    });
    assert_eq!(configuration.network, network);
    assert_eq!(
        configuration.statements,
        build_harness_statements(HarnessStatementsOverrides {
            mint: Some(context.clone()),
            transfer: Some(context),
            identities: Some(vec![identifier.to_vec(), other_identity.to_vec()]),
            proof_identity: Some(identifier.to_vec()),
            ..Default::default()
        })
    );
    // the_shipped_defaults_admit_the_network_profile_and_statements
}

/// Contract: the own entry
///   `the_shipped_defaults_admit_every_declared_generate_kind`; the
///   generate section the shipped defaults admit is the EVM concrete whose
///   settings carry the shipped names, root, compiler version, and license,
///   and whose per-kind configurations carry the shipped kinds and
///   quantities in declaration order.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` the program name alone.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.generate` unpacks as `GenerateConcrete::Evm`;
///   each `generate_kinds` entry's `kind` and `quantities` equal the shipped
///   values in order, each `settings.kinds` entry's `kind` matches and its
///   `names` equals the names at the same position of
///   `build_evm_generate_settings(Default::default()).kinds`;
///   `settings.foundry_root`, `compiler_version`, `constants_library`,
///   `records_library`, `generated_directory`, `curve_directories`, and
///   `license` equal the shipped values.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn the_shipped_defaults_admit_every_declared_generate_kind() {
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
            arguments: Some(vec![OsString::from("harness-crypto")]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the shipped defaults admit");
    };
    let GenerateConcrete::Evm { settings } = configuration.generate else {
        panic!("the shipped concrete is evm");
    };
    let kinds = [
        VectorKind::G1Addition,
        VectorKind::G1Multiplication,
        VectorKind::PairingCheck,
        VectorKind::G2Addition,
        VectorKind::G1MultiScalarMultiplication,
        VectorKind::G2MultiScalarMultiplication,
        VectorKind::DecodeRejection,
        VectorKind::HashToScalar,
        VectorKind::IdentityMapping,
        VectorKind::Possession,
        VectorKind::Challenge,
        VectorKind::MintProof,
        VectorKind::TransferProof,
    ];
    let expected_quantities = |kind: VectorKind| match kind {
        VectorKind::DecodeRejection | VectorKind::IdentityMapping => {
            build_vector_kind_quantities(VectorKindQuantitiesOverrides {
                count: Some(None),
                terms: Some(None),
                message_length: Some(None),
            })
        }
        VectorKind::G1MultiScalarMultiplication | VectorKind::G2MultiScalarMultiplication => {
            build_vector_kind_quantities(VectorKindQuantitiesOverrides {
                count: Some(Some(build_vector_count(
                    VectorCountConstructorParamsOverrides { count: Some(8) },
                ))),
                terms: Some(Some(build_msm_term_count(
                    MsmTermCountConstructorParamsOverrides { count: Some(4) },
                ))),
                message_length: Some(None),
            })
        }
        VectorKind::HashToScalar => build_vector_kind_quantities(VectorKindQuantitiesOverrides {
            count: Some(Some(build_vector_count(
                VectorCountConstructorParamsOverrides { count: Some(8) },
            ))),
            terms: Some(None),
            message_length: Some(Some(32)),
        }),
        _ => build_vector_kind_quantities(VectorKindQuantitiesOverrides {
            count: Some(Some(build_vector_count(
                VectorCountConstructorParamsOverrides { count: Some(8) },
            ))),
            terms: Some(None),
            message_length: Some(None),
        }),
    };
    assert_eq!(configuration.generate_kinds.len(), kinds.len());
    for (entry, kind) in configuration.generate_kinds.iter().zip(kinds) {
        assert_eq!(entry.kind, kind);
        assert_eq!(entry.quantities, expected_quantities(kind));
    }
    let default_settings = build_evm_generate_settings(Default::default());
    assert_eq!(settings.kinds.len(), configuration.generate_kinds.len());
    for (index, entry) in settings.kinds.iter().enumerate() {
        assert_eq!(entry.kind, configuration.generate_kinds[index].kind);
        assert_eq!(entry.names, default_settings.kinds[index].names);
    }
    assert_eq!(settings.foundry_root, PathBuf::from("contracts/evm"));
    assert_eq!(
        settings.compiler_version,
        build_solidity_compiler_version(SolidityCompilerVersionOverrides {
            major: Some(0),
            minor: Some(8),
            patch: Some(37),
        })
    );
    assert_eq!(
        settings.constants_library,
        default_settings.constants_library
    );
    assert_eq!(settings.records_library, default_settings.records_library);
    assert_eq!(
        settings.generated_directory,
        default_settings.generated_directory
    );
    assert_eq!(
        settings.curve_directories,
        default_settings.curve_directories
    );
    assert_eq!(
        settings.license,
        build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
            text: Some("UNLICENSED".to_string()),
        })
    );
    // the_shipped_defaults_admit_every_declared_generate_kind
}

/// Contract: the own entry `the_shipped_defaults_admit_the_benchmark_
///   iterations`; the shipped iteration count admits as the nonzero count.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` the program name alone.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.benchmark_iterations.get()` equals `100`.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn the_shipped_defaults_admit_the_benchmark_iterations() {
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
            arguments: Some(vec![OsString::from("harness-crypto")]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the shipped defaults admit");
    };
    assert_eq!(configuration.benchmark_iterations.get(), 100);
    // the_shipped_defaults_admit_the_benchmark_iterations
}

/// Contract: the own entry `the_shipped_defaults_admit_standard_output`; the
///   shipped output section admits as the one standard-output destination.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` the program name alone.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.output` equals
///   `vec![OutputDestination::StandardOutput]`.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn the_shipped_defaults_admit_standard_output() {
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
            arguments: Some(vec![OsString::from("harness-crypto")]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the shipped defaults admit");
    };
    assert_eq!(
        configuration.output,
        vec![OutputDestination::StandardOutput]
    );
    // the_shipped_defaults_admit_standard_output
}

/// Contract: the own entry
///   `a_command_line_assignment_replaces_a_shipped_default`; a `--set`
///   assignment overlays the shipped value, the rest of the defaults
///   admitted unchanged.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--set"`, and
///   `benchmark.iterations=5`.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.benchmark_iterations.get()` equals `5` and
///   `configuration.output` equals
///   `vec![OutputDestination::StandardOutput]`.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn a_command_line_assignment_replaces_a_shipped_default() {
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
                OsString::from("benchmark.iterations=5"),
            ]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the layered defaults admit");
    };
    assert_eq!(configuration.benchmark_iterations.get(), 5);
    assert_eq!(
        configuration.output,
        vec![OutputDestination::StandardOutput]
    );
    // a_command_line_assignment_replaces_a_shipped_default
}

/// Contract: the own entry
///   `a_command_line_assignment_replaces_the_output_destinations`; a
///   `--set` assignment replaces the shipped destinations with the listed
///   kinds in order.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--set"`, and
///   `output.destinations=[{kind="standard_output"},{kind="file",
///   path="harness-output.txt"}]`.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.output` equals
///   `vec![OutputDestination::StandardOutput, OutputDestination::File {
///   path: PathBuf::from("harness-output.txt") }]`.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn a_command_line_assignment_replaces_the_output_destinations() {
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
                OsString::from(
                    "output.destinations=[{kind=\"standard_output\"},{kind=\"file\",path=\"harness-output.txt\"}]",
                ),
            ]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the layered defaults admit");
    };
    assert_eq!(
        configuration.output,
        vec![
            OutputDestination::StandardOutput,
            OutputDestination::File {
                path: PathBuf::from("harness-output.txt"),
            },
        ]
    );
    // a_command_line_assignment_replaces_the_output_destinations
}

/// Contract: the own entry `a_configuration_file_replaces_a_shipped_default`;
///   a `--config` file overlays the shipped value, the rest of the defaults
///   admitted unchanged.
/// Arrange: `deps` naming the real functions; the payload over
///   `HARNESS_DEFAULTS` with `arguments` `"harness-crypto"`, `"--config"`,
///   and a path to `override.toml` in a directory under
///   `std::env::temp_dir()` removed and made afresh, the file holding
///   `[benchmark]` and `iterations = 7` on their own lines.
/// Act:     `harness_crypto::configure(&deps, params, payload)`.
/// Assert:  `configuration.benchmark_iterations.get()` equals `7` and
///   `configuration.output` equals
///   `vec![OutputDestination::StandardOutput]`.
/// Boundary: the public call `harness_crypto::configure` over the real chain
///   `layers`, `admit_selections`, `admit_statements`, `admit_generate`, and
///   `admit_output`.
/// Mocked:  no collaborator is mocked; the filesystem a `--config` path
///   names is the outer edge.
#[test]
fn a_configuration_file_replaces_a_shipped_default() {
    // Arrange
    let directory = temp_dir().join(format!(
        "chaintorrent-configure-{}-a_configuration_file_replaces_a_shipped_default",
        id()
    ));
    let _ = remove_dir_all(&directory);
    create_dir_all(&directory).expect("the test directory is created");
    let path = directory.join("override.toml");
    write(&path, "[benchmark]\niterations = 7\n").expect("the override file is written");
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
                path.into_os_string(),
            ]),
        },
    );

    // Act
    let result = harness_crypto::configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("the layered defaults admit");
    };
    assert_eq!(configuration.benchmark_iterations.get(), 7);
    assert_eq!(
        configuration.output,
        vec![OutputDestination::StandardOutput]
    );
    // a_configuration_file_replaces_a_shipped_default
}
