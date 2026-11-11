#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams,
    ConsumeChainFormsPayload, CreateChainFormsCaseParamsOverrides, CreateChainFormsParams,
    CreateChainFormsParamsOverrides, CreateChainFormsReturn, IChainForms, IChainFormsConsumer,
    ICreateChainFormsCaseConsumer, build_create_chain_forms_case_deps,
    build_create_chain_forms_case_params, build_create_chain_forms_case_payload,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms, create_chain_forms_case,
};
use encoding::{
    CanonicalFieldValue, CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, FromFieldErrorReturn,
    ICanonicalField, IEncoderAdapter, build_to_field_params,
};
use proof::MintPurpose;

use super::admit_contexts;
use super::delivery_context_field::provides::DeliveryContextField;
use super::interface::{AdmitContextsErrorReturn, AdmitContextsReturn};
use super::mock::{
    build_admit_contexts_deps, build_admit_contexts_params, build_admit_contexts_payload,
};
use crate::config::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, build_canonical_delivery_context,
};

/// Contract: the own entry
///   `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value`:
///   `admit_contexts` is called, inside the consumer a concrete of
///   `CHAIN_FORMS_CONCRETES` hands, with the canonical context at its builder
///   defaults → `Ok` whose context holds, in each of the eight form fields,
///   the form whose `to_field` returns the like-named canonical value of the
///   context, and whose suite identifier, asset root, parameter-set
///   identifier, and expiry equal the canonical context's.
/// Arrange: the block's `Consumer`, whose `consume_chain_forms` makes the
///   single call to `admit_contexts` over the default context and
///   `MintPurpose::Mint` and asserts on the return in place; the case
///   consumer builds deps with `build_create_chain_forms_deps(consumer,
///   payload.encoder, …)`; per member of `CHAIN_FORMS_CONCRETES`,
///   `build_create_chain_forms_params` overriding `concrete` and
///   `build_create_chain_forms_case_params` overriding `encoding` with the
///   member of `DECLARED_ENCODING_SELECTIONS` the member's declaration
///   requires.
/// Act:    `create_chain_forms_case(&deps, case_params, case_payload)` per
///   member and required encoding.
/// Assert: for each member the case result is `Ok`, the inner result is
///   `Ok`; in place, `to_field` over each of the eight admitted forms
///   returns the like-named canonical value of the context, each form's
///   `KIND` equals the kind of the value it was built from, and the suite
///   identifier, asset root, parameter-set identifier, and expiry equal the
///   context's.
/// Boundary: `create_chain_forms_case`, `create_encoding`, the real ABI
///   concrete, `create_chain_forms`, the concrete's producer,
///   `consume_chain_forms`, `admit_contexts`, and each form's `from_field`
///   and `to_field`.
/// Mocked:  nothing.
#[test]
fn every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value() {
    #[derive(Clone, Copy)]
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
    }

    impl IChainFormsConsumer for ChainFormsConsumer<'_> {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let result: AdmitContextsReturn<F, MintPurpose> = admit_contexts(
                &build_admit_contexts_deps(Default::default()),
                build_admit_contexts_params(Default::default()),
                build_admit_contexts_payload(self.context, MintPurpose::Mint, Default::default()),
            );
            let context = result.expect("the canonical context admits").context;
            assert_eq!(F::ChainIdentifier::KIND, self.context.chain.kind());
            assert_eq!(
                F::ChainIdentifier::to_field(
                    build_to_field_params(Default::default()),
                    &context.chain
                )
                .expect("the chain identifier form yields its field")
                .field,
                self.context.chain
            );
            assert_eq!(F::Identity::KIND, self.context.entitlement_contract.kind());
            assert_eq!(
                F::Identity::to_field(
                    build_to_field_params(Default::default()),
                    &context.entitlement_contract
                )
                .expect("the entitlement contract form yields its field")
                .field,
                self.context.entitlement_contract
            );
            assert_eq!(F::Entitlement::KIND, self.context.source_entitlement.kind());
            assert_eq!(
                F::Entitlement::to_field(
                    build_to_field_params(Default::default()),
                    &context.source_entitlement
                )
                .expect("the source entitlement form yields its field")
                .field,
                self.context.source_entitlement
            );
            assert_eq!(F::Entitlement::KIND, self.context.target_entitlement.kind());
            assert_eq!(
                F::Entitlement::to_field(
                    build_to_field_params(Default::default()),
                    &context.target_entitlement
                )
                .expect("the target entitlement form yields its field")
                .field,
                self.context.target_entitlement
            );
            assert_eq!(F::Interval::KIND, self.context.old_interval.kind());
            assert_eq!(
                F::Interval::to_field(
                    build_to_field_params(Default::default()),
                    &context.old_interval
                )
                .expect("the old interval form yields its field")
                .field,
                self.context.old_interval
            );
            assert_eq!(F::Interval::KIND, self.context.new_interval.kind());
            assert_eq!(
                F::Interval::to_field(
                    build_to_field_params(Default::default()),
                    &context.new_interval
                )
                .expect("the new interval form yields its field")
                .field,
                self.context.new_interval
            );
            assert_eq!(F::Identity::KIND, self.context.seller.kind());
            assert_eq!(
                F::Identity::to_field(build_to_field_params(Default::default()), &context.seller)
                    .expect("the seller form yields its field")
                    .field,
                self.context.seller
            );
            assert_eq!(F::Identity::KIND, self.context.buyer.kind());
            assert_eq!(
                F::Identity::to_field(build_to_field_params(Default::default()), &context.buyer)
                    .expect("the buyer form yields its field")
                    .field,
                self.context.buyer
            );
            assert_eq!(context.suite_identifier, self.context.suite_identifier);
            assert_eq!(context.asset_root, self.context.asset_root);
            assert_eq!(
                context.parameter_set_identifier,
                self.context.parameter_set_identifier
            );
            assert_eq!(context.expiry, self.context.expiry);
        }
    }

    struct CaseConsumer<'a> {
        params: CreateChainFormsParams,
        consumer: ChainFormsConsumer<'a>,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer<'_> {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(self.consumer, payload.encoder, Default::default()),
                self.params,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Arrange
    let context = build_canonical_delivery_context(Default::default());

    // Act / Assert — every member of `CHAIN_FORMS_CONCRETES`, each paired
    // with the member of `DECLARED_ENCODING_SELECTIONS` its declaration
    // requires.
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params: build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                }),
                consumer: ChainFormsConsumer { context: &context },
            },
            Default::default(),
        );
        for encoding in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: encoding.concrete,
                        identifier: encoding.identifier,
                    }),
                });
            let result = create_chain_forms_case(
                &deps,
                case_params,
                build_create_chain_forms_case_payload(Default::default()),
            );
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}
// every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value

/// Contract: the own entry
///   `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`:
///   `admit_contexts` is called, inside the consumer a concrete of
///   `CHAIN_FORMS_CONCRETES` hands, with the canonical context at its builder
///   defaults except one form-typed field holding a `CanonicalFieldValue::
///   Text` → `Err(AdmitContextsErrorReturn::ChainIdentifier(
///   FromFieldErrorReturn::WrongKind { expected }))` for the chain identifier
///   and `Err(AdmitContextsErrorReturn::Field { field, error:
///   FromFieldErrorReturn::WrongKind { expected } })` for each other field,
///   `expected` the `KIND` of the form type the field names.
/// Arrange: the block's `Consumer`, whose `consume_chain_forms` makes the
///   single call to `admit_contexts` and asserts the whole error in place;
///   the case consumer and params as in the rebuild block; per member of
///   `CHAIN_FORMS_CONCRETES` and each of the eight fields in turn, the
///   default context with that field alone overriding to
///   `CanonicalFieldValue::Text("not a field of this form".to_string())`.
/// Act:    `create_chain_forms_case(&deps, case_params, case_payload)` per
///   member, field, and required encoding.
/// Assert: for each member and field the case result is `Ok`, the inner
///   result is `Ok`; in place, `result.err()` equals the expected refusal
///   with `expected` the `KIND` of the form type the field names.
/// Boundary: as the rebuild block states.
/// Mocked:  nothing.
#[test]
fn a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete() {
    #[derive(Clone, Copy)]
    enum RefusedField {
        Chain,
        Other(DeliveryContextField),
    }

    #[derive(Clone, Copy)]
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        field: RefusedField,
    }

    impl IChainFormsConsumer for ChainFormsConsumer<'_> {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let result: AdmitContextsReturn<F, MintPurpose> = admit_contexts(
                &build_admit_contexts_deps(Default::default()),
                build_admit_contexts_params(Default::default()),
                build_admit_contexts_payload(self.context, MintPurpose::Mint, Default::default()),
            );
            let expected_kind = match self.field {
                RefusedField::Chain => F::ChainIdentifier::KIND,
                RefusedField::Other(
                    DeliveryContextField::EntitlementContract
                    | DeliveryContextField::Seller
                    | DeliveryContextField::Buyer,
                ) => F::Identity::KIND,
                RefusedField::Other(
                    DeliveryContextField::SourceEntitlement
                    | DeliveryContextField::TargetEntitlement,
                ) => F::Entitlement::KIND,
                RefusedField::Other(
                    DeliveryContextField::OldInterval | DeliveryContextField::NewInterval,
                ) => F::Interval::KIND,
            };
            let expected = match self.field {
                RefusedField::Chain => {
                    AdmitContextsErrorReturn::ChainIdentifier(FromFieldErrorReturn::WrongKind {
                        expected: expected_kind,
                    })
                }
                RefusedField::Other(field) => AdmitContextsErrorReturn::Field {
                    field,
                    error: FromFieldErrorReturn::WrongKind {
                        expected: expected_kind,
                    },
                },
            };
            assert_eq!(result.err(), Some(expected));
        }
    }

    struct CaseConsumer<'a> {
        params: CreateChainFormsParams,
        consumer: ChainFormsConsumer<'a>,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer<'_> {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(self.consumer, payload.encoder, Default::default()),
                self.params,
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Arrange / Act / Assert — every member of `CHAIN_FORMS_CONCRETES` and
    // each of the eight fields in turn.
    for field in [
        RefusedField::Chain,
        RefusedField::Other(DeliveryContextField::EntitlementContract),
        RefusedField::Other(DeliveryContextField::SourceEntitlement),
        RefusedField::Other(DeliveryContextField::TargetEntitlement),
        RefusedField::Other(DeliveryContextField::OldInterval),
        RefusedField::Other(DeliveryContextField::NewInterval),
        RefusedField::Other(DeliveryContextField::Seller),
        RefusedField::Other(DeliveryContextField::Buyer),
    ] {
        let refused = CanonicalFieldValue::Text("not a field of this form".to_string());
        let overrides = match field {
            RefusedField::Chain => CanonicalDeliveryContextOverrides {
                chain: Some(refused),
                ..Default::default()
            },
            RefusedField::Other(DeliveryContextField::EntitlementContract) => {
                CanonicalDeliveryContextOverrides {
                    entitlement_contract: Some(refused),
                    ..Default::default()
                }
            }
            RefusedField::Other(DeliveryContextField::SourceEntitlement) => {
                CanonicalDeliveryContextOverrides {
                    source_entitlement: Some(refused),
                    ..Default::default()
                }
            }
            RefusedField::Other(DeliveryContextField::TargetEntitlement) => {
                CanonicalDeliveryContextOverrides {
                    target_entitlement: Some(refused),
                    ..Default::default()
                }
            }
            RefusedField::Other(DeliveryContextField::OldInterval) => {
                CanonicalDeliveryContextOverrides {
                    old_interval: Some(refused),
                    ..Default::default()
                }
            }
            RefusedField::Other(DeliveryContextField::NewInterval) => {
                CanonicalDeliveryContextOverrides {
                    new_interval: Some(refused),
                    ..Default::default()
                }
            }
            RefusedField::Other(DeliveryContextField::Seller) => {
                CanonicalDeliveryContextOverrides {
                    seller: Some(refused),
                    ..Default::default()
                }
            }
            RefusedField::Other(DeliveryContextField::Buyer) => CanonicalDeliveryContextOverrides {
                buyer: Some(refused),
                ..Default::default()
            },
        };
        let context = build_canonical_delivery_context(overrides);
        for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
            let deps = build_create_chain_forms_case_deps(
                CaseConsumer {
                    params: build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(concrete),
                        ..Default::default()
                    }),
                    consumer: ChainFormsConsumer {
                        context: &context,
                        field,
                    },
                },
                Default::default(),
            );
            for encoding in DECLARED_ENCODING_SELECTIONS {
                let case_params =
                    build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                        encoding: Some(CreateEncodingParams {
                            concrete: encoding.concrete,
                            identifier: encoding.identifier,
                        }),
                    });
                let result = create_chain_forms_case(
                    &deps,
                    case_params,
                    build_create_chain_forms_case_payload(Default::default()),
                );
                assert!(result.is_ok());
                assert!(result.is_ok_and(|success| success.output.is_ok()));
            }
        }
    }
}
// a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete
