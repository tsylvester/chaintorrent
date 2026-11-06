#![allow(clippy::expect_used)]

use chain::{
    ConsumeCaseParams, ConsumeCasePayload, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsReturn, IChainForms, IChainFormsConsumer, ICreateChainFormsCaseConsumer,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms,
    create_chain_forms_case,
};
use domain::{
    PARAMETER_SET_IDENTIFIER_LENGTH, ParameterSetIdentifierConstructorParamsOverrides,
    SUITE_IDENTIFIER_LENGTH, SuiteIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{CanonicalFieldValue, ICanonicalField, IEncoderAdapter, build_to_field_params};
use hashing::{RootConstructorParamsOverrides, build_root};
use proof::{MintPurpose, TransferPurpose};

use super::admit_contexts;
use super::interface::AdmitContextsReturn;
use super::mock::{
    build_admit_contexts_deps, build_admit_contexts_params, build_admit_contexts_payload,
};
use crate::config::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, build_canonical_delivery_context,
};

/// Contract: Chain identifier → the chain identifier form is built from the
///   context's `chain`.
/// Arrange: the default context and `MintPurpose::Mint`.
/// Act:     `admit_contexts`, once, inside the consumer.
/// Assert:  the result is `Ok`; `to_field` over its `chain` returns
///   `CanonicalFieldValue::Unsigned256([0x21; 32])`.
#[test]
fn admit_contexts_rebuilds_the_chain_identifier_from_its_canonical_value() {
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
            let context = result.expect("the canonical context admits").context;
            assert_eq!(
                F::ChainIdentifier::to_field(
                    build_to_field_params(Default::default()),
                    &context.chain
                )
                .expect("the chain identifier form yields its field")
                .field,
                CanonicalFieldValue::Unsigned256([0x21; 32])
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

    let context = build_canonical_delivery_context(Default::default());
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
    // admit_contexts_rebuilds_the_chain_identifier_from_its_canonical_value
}

/// Contract: Entitlement contract → the entitlement contract form is built
///   from `entitlement_contract`, not from `seller` or `buyer`.
/// Arrange: the default context.
/// Act:     `admit_contexts`, once.
/// Assert:  `to_field` over `entitlement_contract` returns
///   `CanonicalFieldValue::FixedBytes20([0x31; 20])`, over `seller`
///   `FixedBytes20([0x61; 20])`, and over `buyer` `FixedBytes20([0x62; 20])`.
#[test]
fn admit_contexts_rebuilds_the_entitlement_contract_apart_from_the_seller_and_the_buyer() {
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
            let context = result.expect("the canonical context admits").context;
            assert_eq!(
                F::Identity::to_field(
                    build_to_field_params(Default::default()),
                    &context.entitlement_contract
                )
                .expect("the entitlement contract form yields its field")
                .field,
                CanonicalFieldValue::FixedBytes20([0x31; 20])
            );
            assert_eq!(
                F::Identity::to_field(build_to_field_params(Default::default()), &context.seller)
                    .expect("the seller form yields its field")
                    .field,
                CanonicalFieldValue::FixedBytes20([0x61; 20])
            );
            assert_eq!(
                F::Identity::to_field(build_to_field_params(Default::default()), &context.buyer)
                    .expect("the buyer form yields its field")
                    .field,
                CanonicalFieldValue::FixedBytes20([0x62; 20])
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

    let context = build_canonical_delivery_context(Default::default());
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
    // admit_contexts_rebuilds_the_entitlement_contract_apart_from_the_seller_and_the_buyer
}

/// Contract: Entitlements → the source form is built from
///   `source_entitlement` and the target form from `target_entitlement`.
/// Arrange: the default context.
/// Act:     `admit_contexts`, once.
/// Assert:  `to_field` over `source_entitlement` returns
///   `CanonicalFieldValue::Unsigned256([0x41; 32])` and over
///   `target_entitlement` `Unsigned256([0x42; 32])`.
#[test]
fn admit_contexts_rebuilds_the_source_and_target_entitlements_each_from_its_own_value() {
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
            let context = result.expect("the canonical context admits").context;
            assert_eq!(
                F::Entitlement::to_field(
                    build_to_field_params(Default::default()),
                    &context.source_entitlement
                )
                .expect("the source entitlement form yields its field")
                .field,
                CanonicalFieldValue::Unsigned256([0x41; 32])
            );
            assert_eq!(
                F::Entitlement::to_field(
                    build_to_field_params(Default::default()),
                    &context.target_entitlement
                )
                .expect("the target entitlement form yields its field")
                .field,
                CanonicalFieldValue::Unsigned256([0x42; 32])
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

    let context = build_canonical_delivery_context(Default::default());
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
    // admit_contexts_rebuilds_the_source_and_target_entitlements_each_from_its_own_value
}

/// Contract: Intervals → the old form is built from `old_interval` and the
///   new form from `new_interval`.
/// Arrange: the default context.
/// Act:     `admit_contexts`, once.
/// Assert:  `to_field` over `old_interval` returns
///   `CanonicalFieldValue::Unsigned64(1)` and over `new_interval`
///   `Unsigned64(2)`.
#[test]
fn admit_contexts_rebuilds_the_old_and_new_intervals_each_from_its_own_value() {
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
            let context = result.expect("the canonical context admits").context;
            assert_eq!(
                F::Interval::to_field(
                    build_to_field_params(Default::default()),
                    &context.old_interval
                )
                .expect("the old interval form yields its field")
                .field,
                CanonicalFieldValue::Unsigned64(1)
            );
            assert_eq!(
                F::Interval::to_field(
                    build_to_field_params(Default::default()),
                    &context.new_interval
                )
                .expect("the new interval form yields its field")
                .field,
                CanonicalFieldValue::Unsigned64(2)
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

    let context = build_canonical_delivery_context(Default::default());
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
    // admit_contexts_rebuilds_the_old_and_new_intervals_each_from_its_own_value
}

/// Contract: Domain values → the suite identifier, the asset root, and the
///   parameter-set identifier are the canonical context's.
/// Arrange: the context from `build_canonical_delivery_context` overriding
///   `suite_identifier` with `build_suite_identifier` over identifier bytes
///   each `0x55` and version `7`, `asset_root` with `build_root` over bytes
///   each `0x56`, and `parameter_set_identifier` with
///   `build_parameter_set_identifier` over bytes each `0x57`.
/// Act:     `admit_contexts`, once.
/// Assert:  the admitted `suite_identifier`, `asset_root`, and
///   `parameter_set_identifier` equal the three built in Arrange.
#[test]
fn admit_contexts_carries_the_domain_values_of_the_context_unchanged() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: &'a CanonicalDeliveryContext,
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
            assert_eq!(context.suite_identifier, self.expected.suite_identifier);
            assert_eq!(context.asset_root, self.expected.asset_root);
            assert_eq!(
                context.parameter_set_identifier,
                self.expected.parameter_set_identifier
            );
        }
    }

    struct CaseConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: &'a CanonicalDeliveryContext,
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

    let context = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x55; SUITE_IDENTIFIER_LENGTH]),
                version: Some(7),
            },
        )),
        asset_root: Some(build_root(RootConstructorParamsOverrides {
            bytes: Some([0x56; 32]),
        })),
        parameter_set_identifier: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x57; PARAMETER_SET_IDENTIFIER_LENGTH]),
            },
        )),
        ..Default::default()
    });
    let expected = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(context.suite_identifier.clone()),
        asset_root: Some(context.asset_root),
        parameter_set_identifier: Some(context.parameter_set_identifier.clone()),
        ..Default::default()
    });
    let deps = build_create_chain_forms_case_deps(
        CaseConsumer {
            context: &context,
            expected: &expected,
        },
        Default::default(),
    );

    // Act
    let result = create_chain_forms_case(
        &deps,
        build_create_chain_forms_case_params(Default::default()),
        build_create_chain_forms_case_payload(Default::default()),
    );

    // Assert
    assert!(result.is_ok());
    assert!(result.is_ok_and(|success| success.output.is_ok()));
    // admit_contexts_carries_the_domain_values_of_the_context_unchanged
}

/// Contract: Expiry → `expiry` is carried as the canonical context holds it.
/// Arrange: the context overriding `expiry` with `42`.
/// Act:     `admit_contexts`, once.
/// Assert:  the admitted `expiry` equals `42`.
#[test]
fn admit_contexts_carries_the_expiry_of_the_context_unchanged() {
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
            let context = result.expect("the canonical context admits").context;
            assert_eq!(context.expiry, 42);
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
        expiry: Some(42),
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
    // admit_contexts_carries_the_expiry_of_the_context_unchanged
}

/// Contract: Purpose → a mint context carries the purpose handed, whichever
///   of the relation's purposes it is.
/// Arrange: the default context with `MintPurpose::Mint`, then with
///   `MintPurpose::Replacement`.
/// Act:     `admit_contexts`, once per purpose.
/// Assert:  the admitted `purpose.code()` equals `1` and `4`.
#[test]
fn admit_contexts_carries_each_mint_relation_purpose_as_handed() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        purpose: MintPurpose,
        expected_code: u16,
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
                build_admit_contexts_payload(self.context, self.purpose, Default::default()),
            );
            let context = result.expect("the canonical context admits").context;
            assert_eq!(context.purpose.code(), self.expected_code);
        }
    }

    struct CaseConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        purpose: MintPurpose,
        expected_code: u16,
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
                        purpose: self.purpose,
                        expected_code: self.expected_code,
                    },
                    payload.encoder,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let context = build_canonical_delivery_context(Default::default());

    // Act / Assert — each of the relation's purposes in turn.
    for (purpose, expected_code) in [(MintPurpose::Mint, 1), (MintPurpose::Replacement, 4)] {
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                context: &context,
                purpose,
                expected_code,
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
    // admit_contexts_carries_each_mint_relation_purpose_as_handed
}

/// Contract: Purpose → a transfer context carries the purpose handed,
///   whichever of the relation's purposes it is.
/// Arrange: the default context with `TransferPurpose::Transfer`, then with
///   `TransferPurpose::Grant`.
/// Act:     `admit_contexts`, once per purpose.
/// Assert:  the admitted `purpose.code()` equals `2` and `3`.
#[test]
fn admit_contexts_carries_each_transfer_relation_purpose_as_handed() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        purpose: TransferPurpose,
        expected_code: u16,
    }

    impl IChainFormsConsumer for ChainFormsConsumer<'_> {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let result: AdmitContextsReturn<F, TransferPurpose> = admit_contexts(
                &build_admit_contexts_deps(Default::default()),
                build_admit_contexts_params(Default::default()),
                build_admit_contexts_payload(self.context, self.purpose, Default::default()),
            );
            let context = result.expect("the canonical context admits").context;
            assert_eq!(context.purpose.code(), self.expected_code);
        }
    }

    struct CaseConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        purpose: TransferPurpose,
        expected_code: u16,
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
                        purpose: self.purpose,
                        expected_code: self.expected_code,
                    },
                    payload.encoder,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let context = build_canonical_delivery_context(Default::default());

    // Act / Assert — each of the relation's purposes in turn.
    for (purpose, expected_code) in [(TransferPurpose::Transfer, 2), (TransferPurpose::Grant, 3)] {
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                context: &context,
                purpose,
                expected_code,
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
    // admit_contexts_carries_each_transfer_relation_purpose_as_handed
}

/// Contract: the borrow → the canonical context is unchanged after the call.
/// Arrange: the default context and a clone of it.
/// Act:     `admit_contexts`, once.
/// Assert:  the borrowed context equals the clone.
#[test]
fn admit_contexts_does_not_alter_the_canonical_context_it_borrows() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: &'a CanonicalDeliveryContext,
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
            result.expect("the canonical context admits");
            assert_eq!(self.context, self.expected);
        }
    }

    struct CaseConsumer<'a> {
        context: &'a CanonicalDeliveryContext,
        expected: &'a CanonicalDeliveryContext,
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

    let context = build_canonical_delivery_context(Default::default());
    let expected = context.clone();
    let deps = build_create_chain_forms_case_deps(
        CaseConsumer {
            context: &context,
            expected: &expected,
        },
        Default::default(),
    );

    // Act
    let result = create_chain_forms_case(
        &deps,
        build_create_chain_forms_case_params(Default::default()),
        build_create_chain_forms_case_payload(Default::default()),
    );

    // Assert
    assert!(result.is_ok());
    assert!(result.is_ok_and(|success| success.output.is_ok()));
    // admit_contexts_does_not_alter_the_canonical_context_it_borrows
}
