#![cfg(feature = "mocks")]

use chain::{
    ADMIT_CHAIN_SELECTION_DEPS, AdmitChainCompositionFn, AdmitChainSelectionParamsOverrides,
    BatchPageLimitParams, BatchPageLimitPayload, BatchPageLimitSuccessReturn,
    CHAIN_FORMS_CONCRETES, ChainDeclaration, ChainFormsIdentifier, ChainMemberIdentifier,
    ChainSelection, ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams,
    ConsumeChainFormsPayload, ConsumeChainParams, ConsumeChainPayload,
    CreateChainFormsCaseParamsOverrides, CreateChainFormsParams, CreateChainFormsParamsOverrides,
    CreateChainFormsReturn, CreateChainParams, CreateChainParamsOverrides, CreateChainReturn,
    CurrentReferenceAtErrorReturn, CurrentReferenceAtParams, CurrentReferenceAtPayload,
    CurrentReferenceAtReturn, DECLARED_CHAIN_MEMBERS, EndpointParams, EndpointPayload,
    EndpointSuccessReturn, IChainAdapter, IChainConsumer, IChainForms, IChainFormsConsumer,
    ICreateChainFormsCaseConsumer, MockIChainAdapterCurrentReferenceAtErrorReturn,
    MockIChainAdapterFailureMode, SettlementTier, admit_chain_selection,
    build_admit_chain_selection_params, build_chain_selection, build_create_chain_deps,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, build_create_chain_params,
    build_create_chain_payload, create_chain, create_chain_forms, create_chain_forms_case,
    mock_admit_chain_composition,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier, IEncoderAdapter,
};
use serde_json::json;

#[derive(Debug, PartialEq, Eq)]
pub struct ChainReading {
    pub endpoint: String,
    pub limit: u64,
}

#[derive(Clone)]
struct FormsConsumer<C> {
    params: CreateChainParams,
    admit_chain_composition: AdmitChainCompositionFn,
    consumer: C,
}

impl<C: IChainConsumer + Clone> IChainFormsConsumer for FormsConsumer<C> {
    type Output = CreateChainReturn<C::Output>;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        create_chain::<F, _>(
            &build_create_chain_deps(
                self.admit_chain_composition,
                self.consumer.clone(),
                Default::default(),
            ),
            self.params.clone(),
            build_create_chain_payload(Default::default()),
        )
    }
}

struct CaseConsumer<C> {
    params: CreateChainFormsParams,
    consumer: FormsConsumer<C>,
}

impl<C: IChainConsumer + Clone> ICreateChainFormsCaseConsumer for CaseConsumer<C> {
    type Output = CreateChainFormsReturn<CreateChainReturn<C::Output>>;

    fn consume_case<E: IEncoderAdapter>(
        &self,
        _params: ConsumeCaseParams,
        payload: ConsumeCasePayload<'_, E>,
    ) -> Self::Output {
        create_chain_forms(
            &build_create_chain_forms_deps(
                self.consumer.clone(),
                payload.encoder,
                Default::default(),
            ),
            self.params,
            build_create_chain_forms_payload(Default::default()),
        )
    }
}

fn declared_abi_encodings() -> Vec<CreateEncodingParams> {
    DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
        .map(|selection| CreateEncodingParams {
            concrete: selection.concrete,
            identifier: selection.identifier,
        })
        .collect()
}

/// Contract: own entry
///   `every_declared_member_is_constructed_under_the_declaration_it_owns`
///   as observed from outside: the public `create_chain` constructs each
///   declared member under the declaration it owns.
/// Arrange: each member of `DECLARED_CHAIN_MEMBERS` and the mock member,
///   the selection from `build_chain_selection`; the chain consumer
///   returning `K::DECLARATION`; `mock_admit_chain_composition` as the
///   deps' handle; the run over each concrete of `CHAIN_FORMS_CONCRETES`
///   with each member of `DECLARED_ENCODING_SELECTIONS` that declares
///   `EncodingIdentifier::EthereumAbiV1`.
/// Act:     `create_chain_forms_case` per member, concrete, and encoding.
/// Assert:  each result is `Ok`, the declaration's `identifier` equals
///   the identifier of the member the iteration selected, and its
///   `forms_identifier` equals `ChainFormsIdentifier::EvmV1`.
/// Boundary: the public `create_chain` over the real forms concrete.
/// Mocked:  nothing; the composition admission is the block's mocked
///   collaborator through `mock_admit_chain_composition`.
#[test]
fn create_chain_hands_each_declared_member_to_the_consumer_under_its_declaration_for_an_outside_caller()
 {
    #[derive(Clone)]
    struct DeclaringConsumer;

    impl IChainConsumer for DeclaringConsumer {
        type Output = ChainDeclaration;

        fn consume_chain<K: IChainAdapter>(
            &self,
            _params: ConsumeChainParams,
            _payload: ConsumeChainPayload<K>,
        ) -> Self::Output {
            K::DECLARATION
        }
    }

    for identifier in DECLARED_CHAIN_MEMBERS
        .iter()
        .copied()
        .chain(core::iter::once(ChainMemberIdentifier::Mock))
    {
        for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
            // Arrange
            let deps = build_create_chain_forms_case_deps(
                CaseConsumer {
                    params: build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    consumer: FormsConsumer {
                        params: build_create_chain_params(CreateChainParamsOverrides {
                            selection: Some(build_chain_selection(identifier, Default::default())),
                        }),
                        admit_chain_composition: mock_admit_chain_composition,
                        consumer: DeclaringConsumer,
                    },
                },
                Default::default(),
            );
            for encoding in declared_abi_encodings() {
                let case_params =
                    build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                        encoding: Some(encoding),
                    });
                let case_payload = build_create_chain_forms_case_payload(Default::default());

                // Act
                let result = create_chain_forms_case(&deps, case_params, case_payload);

                // Assert
                assert!(result.is_ok());
                let declaration = result
                    .ok()
                    .and_then(|success| success.output.ok())
                    .and_then(|success| success.output.ok())
                    .map(|success| success.output);
                assert_eq!(declaration.map(|d| d.identifier), Some(identifier));
                assert_eq!(
                    declaration.map(|d| d.forms_identifier),
                    Some(ChainFormsIdentifier::EvmV1)
                );
            }
        }
    }
}
// create_chain_hands_each_declared_member_to_the_consumer_under_its_declaration_for_an_outside_caller

/// Contract: carried entry
///   `the_declared_settings_are_admitted_through_the_real_constructors`
///   as observed from outside: the public `admit_chain_selection` over an
///   empty document yields the declared defaults, and `create_chain`
///   constructs over them.
/// Arrange: the public `admit_chain_selection` with
///   `&ADMIT_CHAIN_SELECTION_DEPS` over `ChainMemberIdentifier::Base` and
///   `serde_json::json!({})`; the chain consumer returning `ChainReading`
///   over the `text` of `endpoint` and the `limit` of `batch_page_limit`;
///   `mock_admit_chain_composition` as the deps' handle; the run over
///   each concrete of `CHAIN_FORMS_CONCRETES` with each encoding
///   declaring `EthereumAbiV1`.
/// Act:     `create_chain_forms_case` per concrete and encoding.
/// Assert:  the output equals `ChainReading { endpoint:
///   "https://mainnet.base.org".to_string(), limit: 64 }` by
///   `assert_eq!`.
/// Boundary: the public `admit_chain_selection` and `create_chain`.
/// Mocked:  nothing.
#[test]
fn create_chain_constructs_over_the_declared_defaults_the_selection_admission_returns_for_an_outside_caller()
 {
    #[derive(Clone)]
    struct ReadingConsumer;

    impl IChainConsumer for ReadingConsumer {
        type Output = ChainReading;

        fn consume_chain<K: IChainAdapter>(
            &self,
            _params: ConsumeChainParams,
            payload: ConsumeChainPayload<K>,
        ) -> Self::Output {
            let Ok(EndpointSuccessReturn { endpoint }) =
                payload.adapter.endpoint(EndpointParams, EndpointPayload);
            let Ok(BatchPageLimitSuccessReturn { limit }) = payload
                .adapter
                .batch_page_limit(BatchPageLimitParams, BatchPageLimitPayload);
            ChainReading {
                endpoint: endpoint.text,
                limit,
            }
        }
    }

    // Arrange
    let admitted = admit_chain_selection(
        &ADMIT_CHAIN_SELECTION_DEPS,
        build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
            identifier: Some(ChainMemberIdentifier::Base),
        }),
        json!({}),
    );
    assert!(admitted.is_ok());
    let selection = admitted.ok().map(|success| success.selection);
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params: build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                }),
                consumer: FormsConsumer {
                    params: build_create_chain_params(CreateChainParamsOverrides {
                        selection: selection.clone(),
                    }),
                    admit_chain_composition: mock_admit_chain_composition,
                    consumer: ReadingConsumer,
                },
            },
            Default::default(),
        );
        for encoding in declared_abi_encodings() {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(encoding),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            let reading = result
                .ok()
                .and_then(|success| success.output.ok())
                .and_then(|success| success.output.ok())
                .map(|success| success.output);
            assert_eq!(
                reading,
                Some(ChainReading {
                    endpoint: "https://mainnet.base.org".to_string(),
                    limit: 64,
                })
            );
        }
    }
}
// create_chain_constructs_over_the_declared_defaults_the_selection_admission_returns_for_an_outside_caller

/// Contract: own entry
///   `a_value_the_selection_admission_returned_reaches_the_chain_the_factory_constructs`
///   as observed from outside: the settings the public
///   `admit_chain_selection` returns reach the concrete unchanged.
/// Arrange: the public `admit_chain_selection` with
///   `&ADMIT_CHAIN_SELECTION_DEPS` over `ChainMemberIdentifier::Base` and
///   `serde_json::json!` supplying `endpoint` `http://localhost:8545` and
///   `batch_page_limit` `128`; the same `ChainReading` consumer;
///   `mock_admit_chain_composition` as the deps' handle; the run over
///   each concrete of `CHAIN_FORMS_CONCRETES` with each encoding
///   declaring `EthereumAbiV1`.
/// Act:     `create_chain_forms_case` per concrete and encoding.
/// Assert:  the output equals `ChainReading { endpoint:
///   "http://localhost:8545".to_string(), limit: 128 }` by `assert_eq!`.
/// Boundary: the public `admit_chain_selection` and `create_chain`.
/// Mocked:  nothing.
#[test]
fn create_chain_constructs_over_the_value_the_selection_admission_returns_for_an_outside_caller() {
    #[derive(Clone)]
    struct ReadingConsumer;

    impl IChainConsumer for ReadingConsumer {
        type Output = ChainReading;

        fn consume_chain<K: IChainAdapter>(
            &self,
            _params: ConsumeChainParams,
            payload: ConsumeChainPayload<K>,
        ) -> Self::Output {
            let Ok(EndpointSuccessReturn { endpoint }) =
                payload.adapter.endpoint(EndpointParams, EndpointPayload);
            let Ok(BatchPageLimitSuccessReturn { limit }) = payload
                .adapter
                .batch_page_limit(BatchPageLimitParams, BatchPageLimitPayload);
            ChainReading {
                endpoint: endpoint.text,
                limit,
            }
        }
    }

    // Arrange
    let admitted = admit_chain_selection(
        &ADMIT_CHAIN_SELECTION_DEPS,
        build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
            identifier: Some(ChainMemberIdentifier::Base),
        }),
        json!({ "endpoint": "http://localhost:8545", "batch_page_limit": "128" }),
    );
    assert!(admitted.is_ok());
    let selection = admitted.ok().map(|success| success.selection);
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params: build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                }),
                consumer: FormsConsumer {
                    params: build_create_chain_params(CreateChainParamsOverrides {
                        selection: selection.clone(),
                    }),
                    admit_chain_composition: mock_admit_chain_composition,
                    consumer: ReadingConsumer,
                },
            },
            Default::default(),
        );
        for encoding in declared_abi_encodings() {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(encoding),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            let reading = result
                .ok()
                .and_then(|success| success.output.ok())
                .and_then(|success| success.output.ok())
                .map(|success| success.output);
            assert_eq!(
                reading,
                Some(ChainReading {
                    endpoint: "http://localhost:8545".to_string(),
                    limit: 128,
                })
            );
        }
    }
}
// create_chain_constructs_over_the_value_the_selection_admission_returns_for_an_outside_caller

/// Contract: own entry
///   `the_failure_mode_a_mock_selection_names_is_the_failure_mode_of_the_chain_constructed`
///   as observed from outside: the public `create_chain` runs the mock
///   concrete under the failure mode the selection names.
/// Arrange: `ChainSelection::Mock { failure_mode }` for `NoFailure` and
///   `TransportRefused`; the chain consumer returning the result of
///   `current_reference_at` for `SettlementTier::Soft`;
///   `mock_admit_chain_composition` as the deps' handle; the run over
///   each concrete of `CHAIN_FORMS_CONCRETES` with each encoding
///   declaring `EthereumAbiV1`.
/// Act:     `create_chain_forms_case` per mode, concrete, and encoding.
/// Assert:  under `NoFailure` an `Ok` whose reference's `ordinal` equals
///   `30`; under `TransportRefused` the result matches
///   `Err(CurrentReferenceAtErrorReturn::Mock(
///   MockIChainAdapterCurrentReferenceAtErrorReturn::TransportRefused))`.
/// Boundary: the public `create_chain`.
/// Mocked:  nothing, because the mock member is the family's own mock
///   concrete.
#[test]
fn create_chain_runs_the_mock_concrete_under_the_failure_mode_the_selection_names_for_an_outside_caller()
 {
    #[derive(Clone)]
    struct ReferencingConsumer;

    impl IChainConsumer for ReferencingConsumer {
        type Output = CurrentReferenceAtReturn;

        fn consume_chain<K: IChainAdapter>(
            &self,
            _params: ConsumeChainParams,
            payload: ConsumeChainPayload<K>,
        ) -> Self::Output {
            payload.adapter.current_reference_at(
                CurrentReferenceAtParams {
                    tier: SettlementTier::Soft,
                },
                CurrentReferenceAtPayload,
            )
        }
    }

    for failure_mode in [
        MockIChainAdapterFailureMode::NoFailure,
        MockIChainAdapterFailureMode::TransportRefused,
    ] {
        for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
            // Arrange
            let deps = build_create_chain_forms_case_deps(
                CaseConsumer {
                    params: build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    consumer: FormsConsumer {
                        params: build_create_chain_params(CreateChainParamsOverrides {
                            selection: Some(ChainSelection::Mock { failure_mode }),
                        }),
                        admit_chain_composition: mock_admit_chain_composition,
                        consumer: ReferencingConsumer,
                    },
                },
                Default::default(),
            );
            for encoding in declared_abi_encodings() {
                let case_params =
                    build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                        encoding: Some(encoding),
                    });
                let case_payload = build_create_chain_forms_case_payload(Default::default());

                // Act
                let result = create_chain_forms_case(&deps, case_params, case_payload);

                // Assert
                assert!(result.is_ok());
                let reference = result
                    .ok()
                    .and_then(|success| success.output.ok())
                    .and_then(|success| success.output.ok())
                    .map(|success| success.output);
                match failure_mode {
                    MockIChainAdapterFailureMode::NoFailure => {
                        assert_eq!(
                            reference
                                .and_then(|r| r.ok())
                                .map(|s| s.reference.ordinal()),
                            Some(30)
                        );
                    }
                    MockIChainAdapterFailureMode::TransportRefused => {
                        assert!(matches!(
                            reference,
                            Some(Err(CurrentReferenceAtErrorReturn::Mock(
                                MockIChainAdapterCurrentReferenceAtErrorReturn::TransportRefused
                            )))
                        ));
                    }
                }
            }
        }
    }
}
// create_chain_runs_the_mock_concrete_under_the_failure_mode_the_selection_names_for_an_outside_caller

// Public-surface enumeration: the own entries
//   `every_declared_member_is_constructed_under_the_declaration_it_owns`,
//   proven by `create_chain_hands_each_declared_member_to_the_consumer_under_its_declaration_for_an_outside_caller`,
//   `a_value_the_selection_admission_returned_reaches_the_chain_the_factory_constructs`,
//   proven by `create_chain_constructs_over_the_value_the_selection_admission_returns_for_an_outside_caller`,
//   and `the_failure_mode_a_mock_selection_names_is_the_failure_mode_of_the_chain_constructed`,
//   proven by `create_chain_runs_the_mock_concrete_under_the_failure_mode_the_selection_names_for_an_outside_caller`;
//   and the carried `chain/admit_chain_selection` entries
//   `the_declared_settings_are_admitted_through_the_real_constructors`,
//   proven by `create_chain_constructs_over_the_declared_defaults_the_selection_admission_returns_for_an_outside_caller`,
//   and `a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings`,
//   carried inside the entry the third block proves.
