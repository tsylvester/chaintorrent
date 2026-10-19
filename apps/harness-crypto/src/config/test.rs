#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::ffi::OsString;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use encoding::CanonicalFieldValue;
use pairing::{PairingConcrete, TargetGroupEncodingIdentifier};
use proof::UndeclaredDeliveryPurposeCode;

use super::admit_generate::provides::{
    AdmitGenerateDeps, AdmitGenerateErrorReturn, AdmitGenerateParams, AdmitGeneratePayload,
    AdmitGenerateReturn, AdmitGenerateSuccessReturnOverrides, build_admit_generate_success_return,
};
use super::admit_output::provides::{
    AdmitOutputDeps, AdmitOutputErrorReturn, AdmitOutputParams, AdmitOutputPayload,
    AdmitOutputReturn, AdmitOutputSuccessReturnOverrides, OutputDestination,
    build_admit_output_success_return,
};
use super::admit_selections::provides::{
    AdmitSelectionsDeps, AdmitSelectionsErrorReturn, AdmitSelectionsParams, AdmitSelectionsPayload,
    AdmitSelectionsReturn, AdmitSelectionsSuccessReturnOverrides, CurveSelectionOverrides,
    HarnessSelectionsOverrides, SelectionField, build_admit_selections_success_return,
    build_curve_selection, build_harness_selections,
};
use super::admit_statements::provides::{
    AdmitStatementsDeps, AdmitStatementsErrorReturn, AdmitStatementsParams, AdmitStatementsPayload,
    AdmitStatementsReturn, AdmitStatementsSuccessReturnOverrides, HarnessStatementsOverrides,
    NetworkProfileOverrides, build_admit_statements_success_return, build_harness_statements,
    build_network_profile,
};
use super::configure;
use super::interface::*;
use super::layers::provides::{
    BenchmarkDocumentOverrides, CurveDocumentOverrides, HarnessConfigDocumentOverrides, LayersDeps,
    LayersErrorReturn, LayersParams, LayersPayload, LayersReturn, LayersSuccessReturnOverrides,
    SelectionDocumentOverrides, build_benchmark_document, build_curve_document,
    build_harness_config_document, build_layers_success_return, build_selection_document,
};
use super::mock::{
    ConfigureDepsOverrides, ConfigurePayloadOverrides, build_configure_deps,
    build_configure_params, build_configure_payload,
};
use crate::generate::provides::{
    EvmGenerateSettingsOverrides, GenerateConcrete, SpdxLicenseIdentifierTryNewErrorReturn,
    VectorKind, VectorKindConfigurationOverrides, build_evm_generate_settings,
    build_vector_kind_configuration,
};

/// Contract: a `layers` refusal → `Err(ConfigureErrorReturn::Layers(error))`,
///   the refusal unchanged.
/// Arrange: `layers` returns `Err(LayersErrorReturn::ReadFile { path:
///   PathBuf::from("missing.toml"), error: Error::from(ErrorKind::NotFound) })`,
///   every other collaborator its function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Layers(LayersErrorReturn::
///   ReadFile { path, error })` with `path` `PathBuf::from("missing.toml")`
///   and `error.kind()` `ErrorKind::NotFound`.
#[test]
fn configure_returns_the_layers_refusal_unchanged() {
    // Arrange
    fn layers(
        _deps: &LayersDeps,
        _params: LayersParams,
        _payload: LayersPayload<'_>,
    ) -> LayersReturn {
        Err(LayersErrorReturn::ReadFile {
            path: PathBuf::from("missing.toml"),
            error: Error::from(ErrorKind::NotFound),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::Layers(LayersErrorReturn::ReadFile { path, error })) = result
    else {
        panic!("expected the layers refusal");
    };
    assert_eq!(path, PathBuf::from("missing.toml"));
    assert_eq!(error.kind(), ErrorKind::NotFound);
    // configure_returns_the_layers_refusal_unchanged
}

/// Contract: an `admit_selections` refusal →
///   `Err(ConfigureErrorReturn::Selections(error))`, the refusal unchanged.
/// Arrange: `admit_selections` returns `Err(AdmitSelectionsErrorReturn::
///   UnknownValue { field: SelectionField::KemScope, text: "holder" })`, every
///   other collaborator its function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Selections(error)` with
///   `error` equal to the selections refusal.
#[test]
fn configure_returns_the_selections_refusal_unchanged() {
    // Arrange
    fn admit_selections(
        _deps: &AdmitSelectionsDeps,
        _params: AdmitSelectionsParams,
        _payload: AdmitSelectionsPayload<'_>,
    ) -> AdmitSelectionsReturn {
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KemScope,
            text: "holder".to_string(),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        admit_selections: Some(admit_selections),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_selections_refusal_unchanged
}

/// Contract: an `admit_statements` refusal →
///   `Err(ConfigureErrorReturn::Statements(error))`, the refusal unchanged.
/// Arrange: `admit_statements` returns `Err(AdmitStatementsErrorReturn::
///   MintPurpose(UndeclaredDeliveryPurposeCode { code: 2 }))`, every other
///   collaborator its function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Statements(error)` with
///   `error` equal to the statements refusal.
#[test]
fn configure_returns_the_statements_refusal_unchanged() {
    // Arrange
    fn admit_statements(
        _deps: &AdmitStatementsDeps,
        _params: AdmitStatementsParams,
        _payload: AdmitStatementsPayload<'_>,
    ) -> AdmitStatementsReturn {
        Err(AdmitStatementsErrorReturn::MintPurpose(
            UndeclaredDeliveryPurposeCode { code: 2 },
        ))
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        admit_statements: Some(admit_statements),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_statements_refusal_unchanged
}

/// Contract: an `admit_generate` refusal →
///   `Err(ConfigureErrorReturn::Generate(error))`, the refusal unchanged.
/// Arrange: `admit_generate` returns `Err(AdmitGenerateErrorReturn::License(
///   SpdxLicenseIdentifierTryNewErrorReturn::Empty))`, every other
///   collaborator its function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Generate(error)` with
///   `error` equal to the generate refusal.
#[test]
fn configure_returns_the_generate_refusal_unchanged() {
    // Arrange
    fn admit_generate(
        _deps: &AdmitGenerateDeps,
        _params: AdmitGenerateParams,
        _payload: AdmitGeneratePayload<'_>,
    ) -> AdmitGenerateReturn {
        Err(AdmitGenerateErrorReturn::License(
            SpdxLicenseIdentifierTryNewErrorReturn::Empty,
        ))
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        admit_generate: Some(admit_generate),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_generate_refusal_unchanged
}

/// Contract: an `admit_output` refusal →
///   `Err(ConfigureErrorReturn::Output(error))`, the refusal unchanged.
/// Arrange: `admit_output` returns `Err(AdmitOutputErrorReturn::UnknownKind {
///   index: 0, text: "screen" })`, every other collaborator its function
///   mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Output(error)` with `error`
///   equal to the output refusal.
#[test]
fn configure_returns_the_output_refusal_unchanged() {
    // Arrange
    fn admit_output(
        _deps: &AdmitOutputDeps,
        _params: AdmitOutputParams,
        _payload: AdmitOutputPayload<'_>,
    ) -> AdmitOutputReturn {
        Err(AdmitOutputErrorReturn::UnknownKind {
            index: 0,
            text: "screen".to_string(),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        admit_output: Some(admit_output),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_output_refusal_unchanged
}

/// Contract: a document whose benchmark iteration count is zero →
///   `Err(ConfigureErrorReturn::ZeroBenchmarkIterations)`.
/// Arrange: `layers` returns `build_layers_success_return` whose document's
///   `benchmark` is `build_benchmark_document` over `iterations` `0`, every
///   admission its function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::ZeroBenchmarkIterations`.
#[test]
fn configure_refuses_zero_benchmark_iterations() {
    // Arrange
    fn layers(
        _deps: &LayersDeps,
        _params: LayersParams,
        _payload: LayersPayload<'_>,
    ) -> LayersReturn {
        Ok(build_layers_success_return(LayersSuccessReturnOverrides {
            document: Some(build_harness_config_document(
                HarnessConfigDocumentOverrides {
                    benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                        iterations: Some(0),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            )),
        }))
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Err(ConfigureErrorReturn::ZeroBenchmarkIterations) = result else {
        panic!("expected the zero iteration refusal");
    };
    // configure_refuses_zero_benchmark_iterations
}

/// Contract: every admitted value composes into its own field of the
///   `HarnessConfiguration`.
/// Arrange: `layers` returns a document whose `benchmark` is
///   `build_benchmark_document` over `iterations` `7`; `admit_selections`
///   returns `build_harness_selections` with `curves` `vec![build_curve_
///   selection` over `PairingConcrete::Bls12381Arkworks` and
///   `TargetGroupEncodingIdentifier::Bls12381V1`]; `admit_statements` returns
///   `build_network_profile` with `chain` `CanonicalFieldValue::Unsigned64(11)`
///   and `build_harness_statements` with `identities` `vec![vec![0x02; 32]]`;
///   `admit_generate` returns `GenerateConcrete::Evm { settings }` over
///   `build_evm_generate_settings` with `foundry_root`
///   `PathBuf::from("contracts/other")` and `kinds`
///   `vec![build_vector_kind_configuration` over `kind`
///   `VectorKind::HashToScalar`]; `admit_output` returns `vec![
///   OutputDestination::File { path: PathBuf::from("out.txt") }]`, each
///   success built through its collaborator's success-return builder.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: `configuration` holds each admitted value in its own field.
#[test]
fn configure_composes_each_admitted_value_into_its_own_field() {
    // Arrange
    fn layers(
        _deps: &LayersDeps,
        _params: LayersParams,
        _payload: LayersPayload<'_>,
    ) -> LayersReturn {
        Ok(build_layers_success_return(LayersSuccessReturnOverrides {
            document: Some(build_harness_config_document(
                HarnessConfigDocumentOverrides {
                    benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                        iterations: Some(7),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            )),
        }))
    }
    fn admit_selections(
        _deps: &AdmitSelectionsDeps,
        _params: AdmitSelectionsParams,
        _payload: AdmitSelectionsPayload<'_>,
    ) -> AdmitSelectionsReturn {
        Ok(build_admit_selections_success_return(
            AdmitSelectionsSuccessReturnOverrides {
                selections: Some(build_harness_selections(HarnessSelectionsOverrides {
                    curves: Some(vec![build_curve_selection(CurveSelectionOverrides {
                        pairing: Some(PairingConcrete::Bls12381Arkworks),
                        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
                    })]),
                    ..Default::default()
                })),
            },
        ))
    }
    fn admit_statements(
        _deps: &AdmitStatementsDeps,
        _params: AdmitStatementsParams,
        _payload: AdmitStatementsPayload<'_>,
    ) -> AdmitStatementsReturn {
        Ok(build_admit_statements_success_return(
            AdmitStatementsSuccessReturnOverrides {
                network: Some(build_network_profile(NetworkProfileOverrides {
                    chain: Some(CanonicalFieldValue::Unsigned64(11)),
                    ..Default::default()
                })),
                statements: Some(build_harness_statements(HarnessStatementsOverrides {
                    identities: Some(vec![vec![0x02; 32]]),
                    ..Default::default()
                })),
            },
        ))
    }
    fn admit_generate(
        _deps: &AdmitGenerateDeps,
        _params: AdmitGenerateParams,
        _payload: AdmitGeneratePayload<'_>,
    ) -> AdmitGenerateReturn {
        Ok(build_admit_generate_success_return(
            AdmitGenerateSuccessReturnOverrides {
                concrete: Some(GenerateConcrete::Evm {
                    settings: Box::new(build_evm_generate_settings(EvmGenerateSettingsOverrides {
                        foundry_root: Some(PathBuf::from("contracts/other")),
                        ..Default::default()
                    })),
                }),
                kinds: Some(vec![build_vector_kind_configuration(
                    VectorKindConfigurationOverrides {
                        kind: Some(VectorKind::HashToScalar),
                        ..Default::default()
                    },
                )]),
            },
        ))
    }
    fn admit_output(
        _deps: &AdmitOutputDeps,
        _params: AdmitOutputParams,
        _payload: AdmitOutputPayload<'_>,
    ) -> AdmitOutputReturn {
        Ok(build_admit_output_success_return(
            AdmitOutputSuccessReturnOverrides {
                destinations: Some(vec![OutputDestination::File {
                    path: PathBuf::from("out.txt"),
                }]),
            },
        ))
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

    // Act
    let result = configure(&deps, params, payload);

    // Assert
    let Ok(ConfigureSuccessReturn { configuration }) = result else {
        panic!("expected the composed configuration");
    };
    assert_eq!(configuration.benchmark_iterations.get(), 7);
    assert_eq!(
        configuration.selections.curves,
        vec![build_curve_selection(CurveSelectionOverrides {
            pairing: Some(PairingConcrete::Bls12381Arkworks),
            target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
        })]
    );
    assert_eq!(
        configuration.network.chain,
        CanonicalFieldValue::Unsigned64(11)
    );
    assert_eq!(configuration.statements.identities, vec![vec![0x02; 32]]);
    let GenerateConcrete::Evm { settings } = &configuration.generate else {
        panic!("expected the EVM generate concrete");
    };
    assert_eq!(settings.foundry_root, PathBuf::from("contracts/other"));
    assert_eq!(
        configuration.generate_kinds,
        vec![build_vector_kind_configuration(
            VectorKindConfigurationOverrides {
                kind: Some(VectorKind::HashToScalar),
                ..Default::default()
            },
        )]
    );
    assert_eq!(
        configuration.output,
        vec![OutputDestination::File {
            path: PathBuf::from("out.txt"),
        }]
    );
    // configure_composes_each_admitted_value_into_its_own_field
}

/// Contract: the document's `encoding`, `hash_to_scalar`, and `chain_forms`
///   tables reach `admit_selections` in the slots of their names.
/// Arrange: `layers` returns a document whose `encoding`, `hash_to_scalar`,
///   and `chain_forms` are `build_selection_document` over `concrete`
///   `"marker_encoding"`, `"marker_hash_to_scalar"`, and
///   `"marker_chain_forms"`; `admit_selections` returns `Err(
///   AdmitSelectionsErrorReturn::UnknownValue { field: SelectionField::KemScope,
///   text })`, `text` the payload's three concretes joined by `|`.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Selections(error)` with
///   `error` equal to `AdmitSelectionsErrorReturn::UnknownValue { field:
///   SelectionField::KemScope, text: "marker_encoding|marker_hash_to_scalar|
///   marker_chain_forms" }`.
#[test]
fn configure_hands_each_selection_table_to_the_slot_of_its_name() {
    // Arrange
    fn layers(
        _deps: &LayersDeps,
        _params: LayersParams,
        _payload: LayersPayload<'_>,
    ) -> LayersReturn {
        let concrete = |text: &str| {
            build_selection_document(SelectionDocumentOverrides {
                concrete: Some(text.to_string()),
                ..Default::default()
            })
        };
        Ok(build_layers_success_return(LayersSuccessReturnOverrides {
            document: Some(build_harness_config_document(
                HarnessConfigDocumentOverrides {
                    encoding: Some(concrete("marker_encoding")),
                    hash_to_scalar: Some(concrete("marker_hash_to_scalar")),
                    chain_forms: Some(concrete("marker_chain_forms")),
                    ..Default::default()
                },
            )),
        }))
    }
    fn admit_selections(
        _deps: &AdmitSelectionsDeps,
        _params: AdmitSelectionsParams,
        payload: AdmitSelectionsPayload<'_>,
    ) -> AdmitSelectionsReturn {
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KemScope,
            text: [
                payload.encoding.concrete.as_str(),
                payload.hash_to_scalar.concrete.as_str(),
                payload.chain_forms.concrete.as_str(),
            ]
            .join("|"),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
            text: "marker_encoding|marker_hash_to_scalar|marker_chain_forms".to_string(),
        }
    );
    // configure_hands_each_selection_table_to_the_slot_of_its_name
}

/// Contract: the document's `curves` and its benchmark's `concretes` reach
///   `admit_selections` in their own slots.
/// Arrange: `layers` returns a document whose `curves` hold
///   `build_curve_document` over `pairing` `"marker_curve"` and whose
///   `benchmark.concretes` hold `build_curve_document` over `pairing`
///   `"marker_benchmark"`; `admit_selections` returns `Err(
///   AdmitSelectionsErrorReturn::UnknownValue { field: SelectionField::KemScope,
///   text })`, `text` the first curve's and the first benchmarked concrete's
///   `pairing` joined by `|`.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Selections(error)` with
///   `error` equal to `AdmitSelectionsErrorReturn::UnknownValue { field:
///   SelectionField::KemScope, text: "marker_curve|marker_benchmark" }`.
#[test]
fn configure_hands_the_curves_and_the_benchmarked_concretes_to_their_own_slots() {
    // Arrange
    fn layers(
        _deps: &LayersDeps,
        _params: LayersParams,
        _payload: LayersPayload<'_>,
    ) -> LayersReturn {
        let pairing = |text: &str| {
            build_curve_document(CurveDocumentOverrides {
                pairing: Some(text.to_string()),
                ..Default::default()
            })
        };
        Ok(build_layers_success_return(LayersSuccessReturnOverrides {
            document: Some(build_harness_config_document(
                HarnessConfigDocumentOverrides {
                    curves: Some(vec![pairing("marker_curve")]),
                    benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                        concretes: Some(vec![pairing("marker_benchmark")]),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            )),
        }))
    }
    fn admit_selections(
        _deps: &AdmitSelectionsDeps,
        _params: AdmitSelectionsParams,
        payload: AdmitSelectionsPayload<'_>,
    ) -> AdmitSelectionsReturn {
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KemScope,
            text: [
                payload.curves[0].pairing.as_str(),
                payload.benchmark_concretes[0].pairing.as_str(),
            ]
            .join("|"),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_selections: Some(admit_selections),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
            text: "marker_curve|marker_benchmark".to_string(),
        }
    );
    // configure_hands_the_curves_and_the_benchmarked_concretes_to_their_own_slots
}

/// Contract: `configure` hands the payload's defaults text and arguments to
///   `layers`.
/// Arrange: the payload is `build_configure_payload` over the defaults text
///   `"marker defaults"` with `arguments` `vec![OsString::from(
///   "harness-crypto"), OsString::from("--marker")]`; `layers` returns `Err(
///   LayersErrorReturn::ReadFile { path, error })`, `path` the payload's
///   `defaults` and its second argument joined by `|`, `error`
///   `Error::from(ErrorKind::NotFound)`.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Layers(LayersErrorReturn::
///   ReadFile { path, error })` with `path` `PathBuf::from("marker defaults|
///   --marker")` and `error.kind()` `ErrorKind::NotFound`.
#[test]
fn configure_hands_its_defaults_and_arguments_to_layers() {
    // Arrange
    fn layers(
        _deps: &LayersDeps,
        _params: LayersParams,
        payload: LayersPayload<'_>,
    ) -> LayersReturn {
        Err(LayersErrorReturn::ReadFile {
            path: PathBuf::from(format!(
                "{}|{}",
                payload.defaults,
                payload.arguments[1].to_string_lossy()
            )),
            error: Error::from(ErrorKind::NotFound),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(
        "marker defaults",
        ConfigurePayloadOverrides {
            arguments: Some(vec![
                OsString::from("harness-crypto"),
                OsString::from("--marker"),
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
    assert_eq!(path, PathBuf::from("marker defaults|--marker"));
    assert_eq!(error.kind(), ErrorKind::NotFound);
    // configure_hands_its_defaults_and_arguments_to_layers
}

/// Contract: a selections refusal returns before a statements refusal,
///   selections composed first.
/// Arrange: `admit_selections` returns its `UnknownValue` refusal and
///   `admit_statements` its `MintPurpose` refusal, every other collaborator
///   its function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Selections(error)` with
///   `error` equal to the selections refusal.
#[test]
fn configure_returns_the_selections_refusal_before_the_statements_refusal() {
    // Arrange
    fn admit_selections(
        _deps: &AdmitSelectionsDeps,
        _params: AdmitSelectionsParams,
        _payload: AdmitSelectionsPayload<'_>,
    ) -> AdmitSelectionsReturn {
        Err(AdmitSelectionsErrorReturn::UnknownValue {
            field: SelectionField::KemScope,
            text: "holder".to_string(),
        })
    }
    fn admit_statements(
        _deps: &AdmitStatementsDeps,
        _params: AdmitStatementsParams,
        _payload: AdmitStatementsPayload<'_>,
    ) -> AdmitStatementsReturn {
        Err(AdmitStatementsErrorReturn::MintPurpose(
            UndeclaredDeliveryPurposeCode { code: 2 },
        ))
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        admit_selections: Some(admit_selections),
        admit_statements: Some(admit_statements),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_selections_refusal_before_the_statements_refusal
}

/// Contract: a statements refusal returns before a generate refusal,
///   statements composed first.
/// Arrange: `admit_statements` returns its `MintPurpose` refusal and
///   `admit_generate` its `License` refusal, every other collaborator its
///   function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Statements(error)` with
///   `error` equal to the statements refusal.
#[test]
fn configure_returns_the_statements_refusal_before_the_generate_refusal() {
    // Arrange
    fn admit_statements(
        _deps: &AdmitStatementsDeps,
        _params: AdmitStatementsParams,
        _payload: AdmitStatementsPayload<'_>,
    ) -> AdmitStatementsReturn {
        Err(AdmitStatementsErrorReturn::MintPurpose(
            UndeclaredDeliveryPurposeCode { code: 2 },
        ))
    }
    fn admit_generate(
        _deps: &AdmitGenerateDeps,
        _params: AdmitGenerateParams,
        _payload: AdmitGeneratePayload<'_>,
    ) -> AdmitGenerateReturn {
        Err(AdmitGenerateErrorReturn::License(
            SpdxLicenseIdentifierTryNewErrorReturn::Empty,
        ))
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        admit_statements: Some(admit_statements),
        admit_generate: Some(admit_generate),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_statements_refusal_before_the_generate_refusal
}

/// Contract: a generate refusal returns before an output refusal, generate
///   composed first.
/// Arrange: `admit_generate` returns its `License` refusal and
///   `admit_output` its `UnknownKind` refusal, every other collaborator its
///   function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Generate(error)` with
///   `error` equal to the generate refusal.
#[test]
fn configure_returns_the_generate_refusal_before_the_output_refusal() {
    // Arrange
    fn admit_generate(
        _deps: &AdmitGenerateDeps,
        _params: AdmitGenerateParams,
        _payload: AdmitGeneratePayload<'_>,
    ) -> AdmitGenerateReturn {
        Err(AdmitGenerateErrorReturn::License(
            SpdxLicenseIdentifierTryNewErrorReturn::Empty,
        ))
    }
    fn admit_output(
        _deps: &AdmitOutputDeps,
        _params: AdmitOutputParams,
        _payload: AdmitOutputPayload<'_>,
    ) -> AdmitOutputReturn {
        Err(AdmitOutputErrorReturn::UnknownKind {
            index: 0,
            text: "screen".to_string(),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        admit_generate: Some(admit_generate),
        admit_output: Some(admit_output),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_generate_refusal_before_the_output_refusal
}

/// Contract: an output refusal returns before the zero-iteration refusal,
///   output composed before the iteration count.
/// Arrange: `layers` returns `build_layers_success_return` whose document's
///   `benchmark` is `build_benchmark_document` over `iterations` `0`, and
///   `admit_output` returns its `UnknownKind` refusal, every other admission
///   its function mock.
/// Act:    `configure(&deps, params, payload)`.
/// Assert: the refusal is `ConfigureErrorReturn::Output(error)` with `error`
///   equal to the output refusal.
#[test]
fn configure_returns_the_output_refusal_before_the_zero_iteration_refusal() {
    // Arrange
    fn layers(
        _deps: &LayersDeps,
        _params: LayersParams,
        _payload: LayersPayload<'_>,
    ) -> LayersReturn {
        Ok(build_layers_success_return(LayersSuccessReturnOverrides {
            document: Some(build_harness_config_document(
                HarnessConfigDocumentOverrides {
                    benchmark: Some(build_benchmark_document(BenchmarkDocumentOverrides {
                        iterations: Some(0),
                        ..Default::default()
                    })),
                    ..Default::default()
                },
            )),
        }))
    }
    fn admit_output(
        _deps: &AdmitOutputDeps,
        _params: AdmitOutputParams,
        _payload: AdmitOutputPayload<'_>,
    ) -> AdmitOutputReturn {
        Err(AdmitOutputErrorReturn::UnknownKind {
            index: 0,
            text: "screen".to_string(),
        })
    }
    let deps = build_configure_deps(ConfigureDepsOverrides {
        layers: Some(layers),
        admit_output: Some(admit_output),
        ..Default::default()
    });
    let params = build_configure_params(Default::default());
    let payload = build_configure_payload(HARNESS_DEFAULTS, Default::default());

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
    // configure_returns_the_output_refusal_before_the_zero_iteration_refusal
}
