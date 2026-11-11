#![allow(clippy::expect_used)]

use chain::{
    ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsReturn, IChainForms, IChainFormsConsumer, ICreateChainFormsCaseConsumer,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms,
    create_chain_forms_case,
};
use encoding::{CanonicalFieldKind, CanonicalFieldValue, FromFieldErrorReturn, IEncoderAdapter};
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

/// Contract: Chain identifier → a refused chain identifier is returned as
///   `AdmitContextsErrorReturn::ChainIdentifier(error)` with the form's
///   refusal unchanged.
/// Arrange: the context overriding `chain` with the text value.
/// Act:     `admit_contexts`, once.
/// Assert:  `result.err()` equals `Some(AdmitContextsErrorReturn::
///   ChainIdentifier(FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }))`.
#[test]
fn admit_contexts_refuses_a_chain_identifier_of_another_kind() {
    // Arrange
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
            assert_eq!(
                result.err(),
                Some(AdmitContextsErrorReturn::ChainIdentifier(
                    FromFieldErrorReturn::WrongKind {
                        expected: CanonicalFieldKind::Unsigned256,
                    }
                ))
            );
        }
    }

    struct CaseConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer<'_> {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        context: self.context,
                    },
                    payload.encoder,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let context = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Text(
            "not a field of this form".to_string(),
        )),
        ..Default::default()
    });
    let deps =
        build_create_chain_forms_case_deps(CaseConsumer { context: &context }, Default::default());

    // Act
    let result = create_chain_forms_case(
        &deps,
        build_create_chain_forms_case_params(Default::default()),
        build_create_chain_forms_case_payload(Default::default()),
    );

    // Assert
    assert!(result.is_ok());
    assert!(result.is_ok_and(|success| success.output.is_ok()));
    // admit_contexts_refuses_a_chain_identifier_of_another_kind
}

/// Contract: each field other than the chain identifier → the refusal is
///   `AdmitContextsErrorReturn::Field { field, error }` with the field's
///   variant and the form's own expected kind.
/// Arrange: for each of `EntitlementContract`, `SourceEntitlement`,
///   `TargetEntitlement`, `OldInterval`, `NewInterval`, `Seller`, and
///   `Buyer`, the default context with that field alone overriding to the
///   text value.
/// Act:     `admit_contexts`, once per field.
/// Assert:  each `result.err()` equals `Some(AdmitContextsErrorReturn::
///   Field { field, error: FromFieldErrorReturn::WrongKind { expected } })`
///   with `expected` `FixedBytes20` for `EntitlementContract`, `Seller`, and
///   `Buyer`, `Unsigned256` for `SourceEntitlement` and `TargetEntitlement`,
///   and `Unsigned64` for `OldInterval` and `NewInterval`.
#[test]
fn admit_contexts_names_the_field_that_held_a_value_of_another_kind() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: AdmitContextsErrorReturn,
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
            assert_eq!(result.err(), Some(self.expected));
        }
    }

    struct CaseConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: AdmitContextsErrorReturn,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer<'_> {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        context: self.context,
                        expected: self.expected,
                    },
                    payload.encoder,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Act / Assert — each form-typed field in turn.
    for (field, expected) in [
        (
            DeliveryContextField::EntitlementContract,
            CanonicalFieldKind::FixedBytes20,
        ),
        (
            DeliveryContextField::SourceEntitlement,
            CanonicalFieldKind::Unsigned256,
        ),
        (
            DeliveryContextField::TargetEntitlement,
            CanonicalFieldKind::Unsigned256,
        ),
        (
            DeliveryContextField::OldInterval,
            CanonicalFieldKind::Unsigned64,
        ),
        (
            DeliveryContextField::NewInterval,
            CanonicalFieldKind::Unsigned64,
        ),
        (
            DeliveryContextField::Seller,
            CanonicalFieldKind::FixedBytes20,
        ),
        (
            DeliveryContextField::Buyer,
            CanonicalFieldKind::FixedBytes20,
        ),
    ] {
        let refused = CanonicalFieldValue::Text("not a field of this form".to_string());
        let overrides = match field {
            DeliveryContextField::EntitlementContract => CanonicalDeliveryContextOverrides {
                entitlement_contract: Some(refused),
                ..Default::default()
            },
            DeliveryContextField::SourceEntitlement => CanonicalDeliveryContextOverrides {
                source_entitlement: Some(refused),
                ..Default::default()
            },
            DeliveryContextField::TargetEntitlement => CanonicalDeliveryContextOverrides {
                target_entitlement: Some(refused),
                ..Default::default()
            },
            DeliveryContextField::OldInterval => CanonicalDeliveryContextOverrides {
                old_interval: Some(refused),
                ..Default::default()
            },
            DeliveryContextField::NewInterval => CanonicalDeliveryContextOverrides {
                new_interval: Some(refused),
                ..Default::default()
            },
            DeliveryContextField::Seller => CanonicalDeliveryContextOverrides {
                seller: Some(refused),
                ..Default::default()
            },
            DeliveryContextField::Buyer => CanonicalDeliveryContextOverrides {
                buyer: Some(refused),
                ..Default::default()
            },
        };
        let context = build_canonical_delivery_context(overrides);
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                context: &context,
                expected: AdmitContextsErrorReturn::Field {
                    field,
                    error: FromFieldErrorReturn::WrongKind { expected },
                },
            },
            Default::default(),
        );
        let result = create_chain_forms_case(
            &deps,
            build_create_chain_forms_case_params(Default::default()),
            build_create_chain_forms_case_payload(Default::default()),
        );
        assert!(result.is_ok());
        assert!(result.is_ok_and(|success| success.output.is_ok()));
    }
    // admit_contexts_names_the_field_that_held_a_value_of_another_kind
}

/// Contract: Ordering → the first refused field in transcript order returns
///   and the later fields are not built.
/// Arrange: the default context overriding to the text value, in turn,
///   `chain` and `buyer`; `entitlement_contract` and `seller`;
///   `source_entitlement` and `new_interval`; `target_entitlement` and
///   `old_interval`; `old_interval` and `buyer`; and `seller` and `buyer`.
/// Act:     `admit_contexts`, once per context.
/// Assert:  the results equal the refusals of `chain` as `ChainIdentifier`,
///   `EntitlementContract`, `SourceEntitlement`, `TargetEntitlement`,
///   `OldInterval`, and `Seller`, each with the form's own expected kind.
#[test]
fn admit_contexts_returns_the_first_refusal_in_transcript_order() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: AdmitContextsErrorReturn,
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
            assert_eq!(result.err(), Some(self.expected));
        }
    }

    struct CaseConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: AdmitContextsErrorReturn,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer<'_> {
        type Output = CreateChainFormsReturn<()>;

        fn consume_case<E: IEncoderAdapter>(
            &self,
            _params: ConsumeCaseParams,
            payload: ConsumeCasePayload<'_, E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        context: self.context,
                        expected: self.expected,
                    },
                    payload.encoder,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    // Act / Assert — each pair of refused fields in turn.
    for (overrides, expected) in [
        (
            CanonicalDeliveryContextOverrides {
                chain: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                buyer: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                ..Default::default()
            },
            AdmitContextsErrorReturn::ChainIdentifier(FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            }),
        ),
        (
            CanonicalDeliveryContextOverrides {
                entitlement_contract: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                seller: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                ..Default::default()
            },
            AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::EntitlementContract,
                error: FromFieldErrorReturn::WrongKind {
                    expected: CanonicalFieldKind::FixedBytes20,
                },
            },
        ),
        (
            CanonicalDeliveryContextOverrides {
                source_entitlement: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                new_interval: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                ..Default::default()
            },
            AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::SourceEntitlement,
                error: FromFieldErrorReturn::WrongKind {
                    expected: CanonicalFieldKind::Unsigned256,
                },
            },
        ),
        (
            CanonicalDeliveryContextOverrides {
                target_entitlement: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                old_interval: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                ..Default::default()
            },
            AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::TargetEntitlement,
                error: FromFieldErrorReturn::WrongKind {
                    expected: CanonicalFieldKind::Unsigned256,
                },
            },
        ),
        (
            CanonicalDeliveryContextOverrides {
                old_interval: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                buyer: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                ..Default::default()
            },
            AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::OldInterval,
                error: FromFieldErrorReturn::WrongKind {
                    expected: CanonicalFieldKind::Unsigned64,
                },
            },
        ),
        (
            CanonicalDeliveryContextOverrides {
                seller: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                buyer: Some(CanonicalFieldValue::Text(
                    "not a field of this form".to_string(),
                )),
                ..Default::default()
            },
            AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::Seller,
                error: FromFieldErrorReturn::WrongKind {
                    expected: CanonicalFieldKind::FixedBytes20,
                },
            },
        ),
    ] {
        let context = build_canonical_delivery_context(overrides);
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                context: &context,
                expected,
            },
            Default::default(),
        );
        let result = create_chain_forms_case(
            &deps,
            build_create_chain_forms_case_params(Default::default()),
            build_create_chain_forms_case_payload(Default::default()),
        );
        assert!(result.is_ok());
        assert!(result.is_ok_and(|success| success.output.is_ok()));
    }
    // admit_contexts_returns_the_first_refusal_in_transcript_order
}
