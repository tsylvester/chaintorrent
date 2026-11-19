#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::create_chain;
use super::interface::{
    BatchPageLimitParams, BatchPageLimitPayload, BatchPageLimitSuccessReturn,
    ChainMemberIdentifier, ChainSelection, ConsumeChainParams, ConsumeChainPayload,
    CreateChainErrorReturn, CreateChainParams, CreateChainReturn, EndpointParams, EndpointPayload,
    EndpointSuccessReturn, IChainAdapter, IChainConsumer, IChainForms,
};
use super::mock::{
    ChainEndpointOverrides, CreateChainParamsOverrides, build_chain_endpoint,
    build_create_chain_deps, build_create_chain_params, build_create_chain_payload,
};
use crate::admit_chain_composition::provides::{
    AdmitChainCompositionDeps, AdmitChainCompositionErrorReturn, AdmitChainCompositionFn,
    AdmitChainCompositionParams, AdmitChainCompositionPayload, AdmitChainCompositionReturn,
    ChainFormsIdentifierDisagreementOverrides, build_chain_forms_identifier_disagreement,
    mock_admit_chain_composition,
};
use crate::base::provides::{
    BaseChainSettingsOverrides, BaseChainTryNewErrorReturn, build_base_chain_settings,
};
use crate::create_chain_forms::provides::{
    ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsReturn, IChainFormsConsumer, ICreateChainFormsCaseConsumer,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms,
    create_chain_forms_case,
};
use core::cell::Cell;
use encoding::IEncoderAdapter;

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

/// Contract: `create_chain`, the mock selection admitted →
///   `Ok(CreateChainSuccessReturn { output })` whose output is the
///   `K::DECLARATION.identifier` the consumer read, the consumer called
///   once.
/// Arrange: params from `build_create_chain_params` at its defaults, the
///   mock selection; the chain consumer adding one to a `Cell<u32>` it
///   borrows and returning `K::DECLARATION.identifier`; the case runner at
///   its defaults, the mock encoding concrete.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`.
/// Assert:  the case result and the inner result are `Ok`; the output
///   equals `ChainMemberIdentifier::Mock` by `assert_eq!`; the counter
///   equals `1`.
#[test]
fn create_chain_hands_the_mock_concrete_to_the_consumer_exactly_once() {
    #[derive(Clone)]
    struct CountingConsumer<'a> {
        calls: &'a Cell<u32>,
    }

    impl IChainConsumer for CountingConsumer<'_> {
        type Output = ChainMemberIdentifier;

        fn consume_chain<K: IChainAdapter>(
            &self,
            _params: ConsumeChainParams,
            _payload: ConsumeChainPayload<K>,
        ) -> Self::Output {
            self.calls.set(self.calls.get() + 1);
            K::DECLARATION.identifier
        }
    }

    // Arrange
    let calls = Cell::new(0);
    let deps = build_create_chain_forms_case_deps(
        CaseConsumer {
            consumer: FormsConsumer {
                params: build_create_chain_params(Default::default()),
                admit_chain_composition: mock_admit_chain_composition,
                consumer: CountingConsumer { calls: &calls },
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
            .expect("the composition admission admits the mock selection")
            .output,
        ChainMemberIdentifier::Mock
    );
    assert_eq!(calls.get(), 1);
}
// create_chain_hands_the_mock_concrete_to_the_consumer_exactly_once

/// Contract: `create_chain`, the `Base` selection → `Ok` whose output is
///   the endpoint text and the limit of the concrete over the settings
///   the selection carries.
/// Arrange: params whose selection is `ChainSelection::Base` over
///   `build_base_chain_settings` overriding `endpoint` with
///   `build_chain_endpoint` over the text `https://example.test` and
///   `batch_page_limit` with `5`, values that differ from the declared
///   defaults `https://mainnet.base.org` and `64`; the chain consumer
///   returning `ChainReading` over the `text` of `endpoint` and the
///   `limit` of `batch_page_limit`, each success bound by the irrefutable
///   pattern; the case runner at its defaults.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`.
/// Assert:  the case result and the inner result are `Ok`; the output
///   equals `ChainReading { endpoint: "https://example.test".to_string(),
///   limit: 5 }` by `assert_eq!`.
#[test]
fn create_chain_constructs_the_base_concrete_over_the_settings_the_selection_carries() {
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
    let deps = build_create_chain_forms_case_deps(
        CaseConsumer {
            consumer: FormsConsumer {
                params: build_create_chain_params(CreateChainParamsOverrides {
                    selection: Some(ChainSelection::Base {
                        settings: build_base_chain_settings(BaseChainSettingsOverrides {
                            endpoint: Some(build_chain_endpoint(ChainEndpointOverrides {
                                text: Some("https://example.test".to_string()),
                            })),
                            batch_page_limit: Some(5),
                            ..Default::default()
                        }),
                    }),
                }),
                admit_chain_composition: mock_admit_chain_composition,
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
            endpoint: "https://example.test".to_string(),
            limit: 5,
        }
    );
}
// create_chain_constructs_the_base_concrete_over_the_settings_the_selection_carries

/// Contract: `create_chain`, the Base concrete refusing its endpoint text
///   → `Err(CreateChainErrorReturn::Base(BaseChainTryNewErrorReturn::
///   Endpoint(error)))`, the consumer not called.
/// Arrange: params whose selection is `ChainSelection::Base` over
///   `build_base_chain_settings` overriding `endpoint` with
///   `build_chain_endpoint` over the text `ftp://example.test`; the chain
///   consumer adding one to a `Cell<u32>` it borrows and returning `()`;
///   the case runner at its defaults.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`.
/// Assert:  the error matches `CreateChainErrorReturn::Base(
///   BaseChainTryNewErrorReturn::Endpoint(error))` with the error's
///   rendered text containing `invalid URL scheme: ftp`; the counter
///   equals `0`.
#[test]
fn create_chain_returns_a_base_constructor_refusal_whole_and_does_not_call_the_consumer() {
    #[derive(Clone)]
    struct CountingConsumer<'a> {
        calls: &'a Cell<u32>,
    }

    impl IChainConsumer for CountingConsumer<'_> {
        type Output = ();

        fn consume_chain<K: IChainAdapter>(
            &self,
            _params: ConsumeChainParams,
            _payload: ConsumeChainPayload<K>,
        ) -> Self::Output {
            self.calls.set(self.calls.get() + 1);
        }
    }

    // Arrange
    let calls = Cell::new(0);
    let deps = build_create_chain_forms_case_deps(
        CaseConsumer {
            consumer: FormsConsumer {
                params: build_create_chain_params(CreateChainParamsOverrides {
                    selection: Some(ChainSelection::Base {
                        settings: build_base_chain_settings(BaseChainSettingsOverrides {
                            endpoint: Some(build_chain_endpoint(ChainEndpointOverrides {
                                text: Some("ftp://example.test".to_string()),
                            })),
                            ..Default::default()
                        }),
                    }),
                }),
                admit_chain_composition: mock_admit_chain_composition,
                consumer: CountingConsumer { calls: &calls },
            },
        },
        Default::default(),
    );
    let case_params = build_create_chain_forms_case_params(Default::default());
    let case_payload = build_create_chain_forms_case_payload(Default::default());

    // Act
    let result = create_chain_forms_case(&deps, case_params, case_payload);

    // Assert
    let inner = result
        .expect("the encoding level admits the selection")
        .output
        .expect("the forms level admits the mock concrete")
        .output;
    let Err(CreateChainErrorReturn::Base(BaseChainTryNewErrorReturn::Endpoint(error))) = inner
    else {
        panic!("expected a Base endpoint transport refusal");
    };
    assert!(
        error.to_string().contains("invalid URL scheme: ftp"),
        "{error}"
    );
    assert_eq!(calls.get(), 0);
}
// create_chain_returns_a_base_constructor_refusal_whole_and_does_not_call_the_consumer

/// Contract: `create_chain`, the composition admission refusing →
///   `Err(CreateChainErrorReturn::Composition(error))` carrying the
///   refusal unchanged, nothing constructed and the consumer not called.
/// Arrange: params from `build_create_chain_params` at its defaults; a
///   block-declared handle of the production type `AdmitChainCompositionFn`
///   returning `Err(AdmitChainCompositionErrorReturn::Incompatible {
///   forms })` over `build_chain_forms_identifier_disagreement` at its
///   defaults; the chain consumer adding one to a `Cell<u32>` it borrows
///   and returning `()`; the case runner at its defaults.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`.
/// Assert:  the error matches `CreateChainErrorReturn::Composition(error)`
///   with `error` equal to the refusal the handle returned by
///   `assert_eq!`; the counter equals `0`.
#[test]
fn create_chain_returns_the_composition_refusal_whole_and_constructs_nothing() {
    #[derive(Clone)]
    struct CountingConsumer<'a> {
        calls: &'a Cell<u32>,
    }

    impl IChainConsumer for CountingConsumer<'_> {
        type Output = ();

        fn consume_chain<K: IChainAdapter>(
            &self,
            _params: ConsumeChainParams,
            _payload: ConsumeChainPayload<K>,
        ) -> Self::Output {
            self.calls.set(self.calls.get() + 1);
        }
    }

    fn refuse_composition(
        _deps: &AdmitChainCompositionDeps,
        _params: AdmitChainCompositionParams,
        _payload: AdmitChainCompositionPayload<'_>,
    ) -> AdmitChainCompositionReturn {
        Err(AdmitChainCompositionErrorReturn::Incompatible {
            forms: build_chain_forms_identifier_disagreement(
                ChainFormsIdentifierDisagreementOverrides::default(),
            ),
        })
    }

    // Arrange
    let calls = Cell::new(0);
    let deps = build_create_chain_forms_case_deps(
        CaseConsumer {
            consumer: FormsConsumer {
                params: build_create_chain_params(Default::default()),
                admit_chain_composition: refuse_composition,
                consumer: CountingConsumer { calls: &calls },
            },
        },
        Default::default(),
    );
    let case_params = build_create_chain_forms_case_params(Default::default());
    let case_payload = build_create_chain_forms_case_payload(Default::default());

    // Act
    let result = create_chain_forms_case(&deps, case_params, case_payload);

    // Assert
    let inner = result
        .expect("the encoding level admits the selection")
        .output
        .expect("the forms level admits the mock concrete")
        .output;
    let Err(CreateChainErrorReturn::Composition(error)) = inner else {
        panic!("expected a composition refusal");
    };
    assert_eq!(
        error,
        AdmitChainCompositionErrorReturn::Incompatible {
            forms: build_chain_forms_identifier_disagreement(
                ChainFormsIdentifierDisagreementOverrides::default(),
            ),
        }
    );
    assert_eq!(calls.get(), 0);
}
// create_chain_returns_the_composition_refusal_whole_and_constructs_nothing
