#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::create_chain;
use super::interface::{
    BatchPageLimitParams, BatchPageLimitPayload, BatchPageLimitSuccessReturn, ChainDeclaration,
    ChainFormsIdentifier, ChainMemberIdentifier, ChainSelection, ConsumeChainParams,
    ConsumeChainPayload, CreateChainParams, CreateChainReturn, CurrentReferenceAtErrorReturn,
    CurrentReferenceAtParams, CurrentReferenceAtPayload, CurrentReferenceAtReturn,
    DECLARED_CHAIN_MEMBERS, EndpointParams, EndpointPayload, EndpointSuccessReturn, IChainAdapter,
    IChainConsumer, IChainForms, MockIChainAdapterCurrentReferenceAtErrorReturn,
    MockIChainAdapterFailureMode, SettlementTier,
};
use super::mock::{
    CreateChainParamsOverrides, build_chain_selection, build_create_chain_deps,
    build_create_chain_params, build_create_chain_payload,
};
use crate::admit_chain_composition::provides::{AdmitChainCompositionFn, admit_chain_composition};
use crate::admit_chain_selection::provides::{
    ADMIT_CHAIN_SELECTION_DEPS, AdmitChainSelectionParamsOverrides, admit_chain_selection,
    build_admit_chain_selection_params,
};
use crate::create_chain_forms::provides::{
    ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsReturn, IChainFormsConsumer, ICreateChainFormsCaseConsumer,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms,
    create_chain_forms_case,
};
use encoding::IEncoderAdapter;
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
            build_create_chain_forms_params(Default::default()),
            build_create_chain_forms_payload(Default::default()),
        )
    }
}

/// Contract: own entry
///   `every_declared_member_is_constructed_under_the_declaration_it_owns`,
///   absorbing the `chain/admit_chain_composition` entries
///   `every_declared_member_is_admitted_against_the_forms_identifier_it_lists`
///   and `the_mock_member_is_admitted_against_the_forms_identifier_it_lists`:
///   `create_chain` over the real admission constructs each member under the
///   declaration it owns.
/// Arrange: the block's body run over each member of
///   `DECLARED_CHAIN_MEMBERS` and the mock member, the params' selection
///   from `build_chain_selection(identifier, Default::default())`; the
///   chain consumer returning `K::DECLARATION`; the real
///   `admit_chain_composition` as the deps' handle, reaching the real
///   `lookup_chain_declaration` through `ADMIT_CHAIN_COMPOSITION_DEPS`;
///   the case runner at its defaults, the mock encoding concrete and the
///   `Mock` forms concrete.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member.
/// Assert:  the result is `Ok`; the declaration's `identifier` equals the
///   identifier of the member the iteration selected and its
///   `forms_identifier` equals `ChainFormsIdentifier::EvmV1`.
/// Boundary: `create_chain_forms_case`, `create_chain_forms`,
///   `create_chain` handed the real `admit_chain_composition`, the real
///   `lookup_chain_declaration`, and the selected concrete's producer.
/// Mocked:  nothing a network serves, the Base concrete's providers being
///   constructed and never called; the encoding concrete is the family's
///   mock.
#[test]
fn every_declared_member_is_constructed_under_the_declaration_it_owns() {
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
        // Arrange
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                consumer: FormsConsumer {
                    params: build_create_chain_params(CreateChainParamsOverrides {
                        selection: Some(build_chain_selection(identifier, Default::default())),
                    }),
                    admit_chain_composition,
                    consumer: DeclaringConsumer,
                },
            },
            Default::default(),
        );
        let case_params = build_create_chain_forms_case_params(Default::default());
        let case_payload = build_create_chain_forms_case_payload(Default::default());

        // Act
        let result = create_chain_forms_case(&deps, case_params, case_payload);

        // Assert
        let declaration = result
            .expect("the encoding level admits the selection")
            .output
            .expect("the forms level admits the mock concrete")
            .output
            .expect("the composition admission admits the member")
            .output;
        assert_eq!(declaration.identifier, identifier);
        assert_eq!(declaration.forms_identifier, ChainFormsIdentifier::EvmV1);
    }
}
// every_declared_member_is_constructed_under_the_declaration_it_owns

/// Contract: own entry
///   `a_value_the_selection_admission_returned_reaches_the_chain_the_factory_constructs`,
///   carrying the `chain/admit_chain_selection` entry
///   `a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings`:
///   the settings the selection carries reach the concrete unchanged.
/// Arrange: `admit_chain_selection` with `&ADMIT_CHAIN_SELECTION_DEPS`
///   over `ChainMemberIdentifier::Base` and a document supplying
///   `endpoint` and `batch_page_limit`, its success's selection the
///   params' selection; the chain consumer returning `ChainReading` over
///   the `text` of `endpoint` and the `limit` of `batch_page_limit`, each
///   success bound by the irrefutable pattern; the real
///   `admit_chain_composition` as the deps' handle; the case runner at
///   its defaults.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`.
/// Assert:  the output equals `ChainReading { endpoint:
///   "http://localhost:8545".to_string(), limit: 128 }` by `assert_eq!`.
/// Boundary: `admit_chain_selection` over `ADMIT_CHAIN_SELECTION_DEPS`,
///   the member's guard, the real declaration lookup, the member's
///   settings admission, `create_chain`, and `BaseChain::try_new`.
/// Mocked:  nothing.
#[test]
fn a_value_the_selection_admission_returned_reaches_the_chain_the_factory_constructs() {
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
    let selection = admit_chain_selection(
        &ADMIT_CHAIN_SELECTION_DEPS,
        build_admit_chain_selection_params(AdmitChainSelectionParamsOverrides {
            identifier: Some(ChainMemberIdentifier::Base),
        }),
        json!({ "endpoint": "http://localhost:8545", "batch_page_limit": "128" }),
    )
    .expect("the document admits")
    .selection;
    let deps = build_create_chain_forms_case_deps(
        CaseConsumer {
            consumer: FormsConsumer {
                params: build_create_chain_params(CreateChainParamsOverrides {
                    selection: Some(selection),
                }),
                admit_chain_composition,
                consumer: ReadingConsumer,
            },
        },
        Default::default(),
    );
    let case_params = build_create_chain_forms_case_params(Default::default());
    let case_payload = build_create_chain_forms_case_payload(Default::default());

    // Act
    let result = create_chain_forms_case(&deps, case_params, case_payload);

    // Assert
    assert_eq!(
        result
            .expect("the encoding level admits the selection")
            .output
            .expect("the forms level admits the mock concrete")
            .output
            .expect("the composition admission admits the base selection")
            .output,
        ChainReading {
            endpoint: "http://localhost:8545".to_string(),
            limit: 128,
        }
    );
}
// a_value_the_selection_admission_returned_reaches_the_chain_the_factory_constructs

/// Contract: own entry
///   `the_failure_mode_a_mock_selection_names_is_the_failure_mode_of_the_chain_constructed`:
///   the failure mode the selection names is the failure mode of the
///   concrete the factory constructs, and the concrete's own error
///   arrives whole.
/// Arrange: the block's body run over `NoFailure` and `TransportRefused`,
///   the params' selection `ChainSelection::Mock { failure_mode }`; the
///   chain consumer returning the result of `current_reference_at` for
///   `SettlementTier::Soft`; the real `admit_chain_composition` as the
///   deps' handle; the case runner at its defaults.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per failure mode.
/// Assert:  under `NoFailure` the result is `Ok` holding a reference
///   whose `ordinal` equals `30`; under `TransportRefused` the result
///   matches `Err(CurrentReferenceAtErrorReturn::Mock(
///   MockIChainAdapterCurrentReferenceAtErrorReturn::TransportRefused))`.
/// Boundary: `create_chain_forms_case`, `create_chain`, and the mock
///   concrete's producer and `current_reference_at`.
/// Mocked:  nothing; the mock concrete is the family's own.
#[test]
fn the_failure_mode_a_mock_selection_names_is_the_failure_mode_of_the_chain_constructed() {
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
        // Arrange
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                consumer: FormsConsumer {
                    params: build_create_chain_params(CreateChainParamsOverrides {
                        selection: Some(ChainSelection::Mock { failure_mode }),
                    }),
                    admit_chain_composition,
                    consumer: ReferencingConsumer,
                },
            },
            Default::default(),
        );
        let case_params = build_create_chain_forms_case_params(Default::default());
        let case_payload = build_create_chain_forms_case_payload(Default::default());

        // Act
        let result = create_chain_forms_case(&deps, case_params, case_payload);

        // Assert
        let reference = result
            .expect("the encoding level admits the selection")
            .output
            .expect("the forms level admits the mock concrete")
            .output
            .expect("the composition admission admits the mock selection")
            .output;
        match failure_mode {
            MockIChainAdapterFailureMode::NoFailure => {
                assert_eq!(
                    reference
                        .expect("the mock concrete answers under no failure")
                        .reference
                        .ordinal(),
                    30
                );
            }
            MockIChainAdapterFailureMode::TransportRefused => {
                let Err(CurrentReferenceAtErrorReturn::Mock(
                    MockIChainAdapterCurrentReferenceAtErrorReturn::TransportRefused,
                )) = reference
                else {
                    panic!("expected the mock transport refusal");
                };
            }
        }
    }
}
// the_failure_mode_a_mock_selection_names_is_the_failure_mode_of_the_chain_constructed

// Enumeration: the own entries
// `every_declared_member_is_constructed_under_the_declaration_it_owns`,
// `a_value_the_selection_admission_returned_reaches_the_chain_the_factory_constructs`,
// and `the_failure_mode_a_mock_selection_names_is_the_failure_mode_of_the_chain_constructed`;
// the absorbed `chain/admit_chain_composition` entries
// `every_declared_member_is_admitted_against_the_forms_identifier_it_lists`
// and `the_mock_member_is_admitted_against_the_forms_identifier_it_lists`,
// proven by the first block; and the carried `chain/admit_chain_selection`
// entry
// `a_value_the_document_supplies_replaces_its_declared_default_in_the_selected_settings`,
// proven by the second block.
