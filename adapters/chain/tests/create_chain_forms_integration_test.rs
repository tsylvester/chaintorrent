#![cfg(feature = "mocks")]

use chain::{
    CHAIN_FORMS_CONCRETES, CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration, ConsumeCaseParams,
    ConsumeCasePayload, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsCaseParamsOverrides, CreateChainFormsErrorReturn, CreateChainFormsParams,
    CreateChainFormsParamsOverrides, CreateChainFormsReturn, DeriveKeyIdentifierParams,
    DeriveKeyIdentifierPayload, IChainForms, IChainFormsConsumer, ICreateChainFormsCaseConsumer,
    IIdentityForm, IntentCall, IntentField, SignedIntentParams, SignedIntentPayload,
    build_create_chain_forms_case_deps, build_create_chain_forms_case_params,
    build_create_chain_forms_case_payload, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms,
    create_chain_forms_case,
};
use domain::{
    SignatureSchemeIdentifierConstructorParamsOverrides, build_signature_scheme_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CreateEncodingParams, DECLARED_ENCODING_SELECTIONS,
    ICanonicalField, IEncoderAdapter, build_from_field_params, build_to_field_params,
};

/// Contract: the own entry
///   `chain_forms_handed_to_the_consumer_declare_the_required_identifier`
///   over the declared set: `create_chain_forms` called with a consumer
///   returning `F::DECLARATION`, a concrete from `CHAIN_FORMS_CONCRETES`,
///   and `params.identifier` → `Ok(CreateChainFormsSuccessReturn
///   { output })` with `output.identifier` equal to `params.identifier`
///   and `output.interface_version` equal to
///   `CHAIN_FORMS_INTERFACE_VERSION`.
/// Arrange: the block's `Consumer`, whose `consume_chain_forms` returns
///   `F::DECLARATION`; the case consumer holds the block's
///   `CreateChainFormsParams` and the forms consumer, builds deps with
///   `build_create_chain_forms_deps(consumer, payload.encoder, …)`, and
///   returns `create_chain_forms` whole; per member,
///   `build_create_chain_forms_params` overriding `concrete`, the
///   identifier at its default, and `build_create_chain_forms_payload`,
///   all from the crate's public surface under the `mocks` feature; each
///   member of `CHAIN_FORMS_CONCRETES`, paired with the member of
///   `DECLARED_ENCODING_SELECTIONS` its declaration requires, is the
///   variation.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member and required encoding.
/// Assert:  for each member the case result is `Ok`, the inner result is
///   `Ok`, the output's `identifier` equals the identifier the block
///   passed, and its `interface_version` equals
///   `CHAIN_FORMS_INTERFACE_VERSION`.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, the real ABI concrete,
///   `create_chain_forms`, the concrete's producer, and
///   `consume_chain_forms`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_forms_declaring_the_required_identifier_to_an_outside_consumer() {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ChainFormsDeclaration;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            F::DECLARATION
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
        type Output = CreateChainFormsReturn<ChainFormsDeclaration>;

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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        for selection in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    }),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            let declaration = result.ok().and_then(|success| success.output.ok());
            assert_eq!(
                declaration.as_ref().map(|declaration| {
                    (
                        declaration.output.identifier,
                        declaration.output.interface_version,
                    )
                }),
                Some((params.identifier, CHAIN_FORMS_INTERFACE_VERSION))
            );
        }
    }
}
// create_chain_forms_hands_forms_declaring_the_required_identifier_to_an_outside_consumer

/// Contract: the own entry `chain_forms_identity_survives_its_field_round_trip`
///   over the declared set: `F::Identity` built by `from_field` over
///   `CanonicalFieldValue::FixedBytes20` of the ascending bytes and yielded
///   back by `to_field` → the field returned equals the field given, and
///   `F::Identity::KIND` is `CanonicalFieldKind::FixedBytes20`.
/// Arrange: the block's `Consumer`, whose `consume_chain_forms` builds the
///   form by `from_field` with `build_from_field_params` and the ascending
///   field, takes the form from the `Ok` arm, yields the field back by
///   `to_field` with `build_to_field_params`, and asserts in place that the
///   returned field equals the arranged field and that `F::Identity::KIND`
///   is `CanonicalFieldKind::FixedBytes20`; the case consumer, case params,
///   and payloads as in the declaration block; each member of
///   `CHAIN_FORMS_CONCRETES`, paired with the member of
///   `DECLARED_ENCODING_SELECTIONS` its declaration requires, is the
///   variation.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member and required encoding.
/// Assert:  for each member the case result is `Ok`, the inner result is
///   `Ok`, the field `to_field` returns equals the arranged field, and the
///   form's `KIND` is the kind the contract states.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, the real ABI concrete,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the identity form's `from_field`, `to_field`, and `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_an_identity_form_that_survives_its_field_round_trip()
 {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let bytes = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
            ];
            let yielded = F::Identity::from_field(
                build_from_field_params(Default::default()),
                CanonicalFieldValue::FixedBytes20(bytes),
            )
            .ok()
            .and_then(|built| {
                F::Identity::to_field(build_to_field_params(Default::default()), &built.value)
                    .ok()
                    .map(|returned| returned.field)
            });
            assert_eq!(yielded, Some(CanonicalFieldValue::FixedBytes20(bytes)));
            assert_eq!(F::Identity::KIND, CanonicalFieldKind::FixedBytes20);
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        for selection in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    }),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}
// create_chain_forms_hands_an_outside_consumer_an_identity_form_that_survives_its_field_round_trip

/// Contract: the own entry
///   `chain_forms_entitlement_survives_its_field_round_trip` over the
///   declared set: `F::Entitlement` built by `from_field` over
///   `CanonicalFieldValue::Unsigned256` of the ascending bytes and yielded
///   back by `to_field` → the field returned equals the field given, and
///   `F::Entitlement::KIND` is `CanonicalFieldKind::Unsigned256`.
/// Arrange: as the identity block, over `F::Entitlement`,
///   `CanonicalFieldValue::Unsigned256`, and the thirty-two-byte ascending
///   literal; each member of the declared set is the variation.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member and required encoding.
/// Assert:  for each member the case result is `Ok`, the inner result is
///   `Ok`, the field `to_field` returns equals the arranged field, and the
///   form's `KIND` is the kind the contract states.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, the real ABI concrete,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the entitlement form's `from_field`, `to_field`, and `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_an_entitlement_form_that_survives_its_field_round_trip()
 {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let bytes = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, 0x20,
            ];
            let yielded = F::Entitlement::from_field(
                build_from_field_params(Default::default()),
                CanonicalFieldValue::Unsigned256(bytes),
            )
            .ok()
            .and_then(|built| {
                F::Entitlement::to_field(build_to_field_params(Default::default()), &built.value)
                    .ok()
                    .map(|returned| returned.field)
            });
            assert_eq!(yielded, Some(CanonicalFieldValue::Unsigned256(bytes)));
            assert_eq!(F::Entitlement::KIND, CanonicalFieldKind::Unsigned256);
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        for selection in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    }),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}
// create_chain_forms_hands_an_outside_consumer_an_entitlement_form_that_survives_its_field_round_trip

/// Contract: the own entry `chain_forms_interval_survives_its_field_round_trip`
///   over the declared set: `F::Interval` built by `from_field` over
///   `CanonicalFieldValue::Unsigned64` of `0x0102_0304_0506_0708` and
///   yielded back by `to_field` → the field returned equals the field
///   given, and `F::Interval::KIND` is `CanonicalFieldKind::Unsigned64`.
/// Arrange: as the identity block, over `F::Interval`,
///   `CanonicalFieldValue::Unsigned64`, and the value
///   `0x0102_0304_0506_0708`; each member of the declared set is the
///   variation.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member and required encoding.
/// Assert:  for each member the case result is `Ok`, the inner result is
///   `Ok`, the field `to_field` returns equals the arranged field, and the
///   form's `KIND` is the kind the contract states.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, the real ABI concrete,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the interval form's `from_field`, `to_field`, and `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_an_interval_form_that_survives_its_field_round_trip()
 {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let value = 0x0102_0304_0506_0708;
            let yielded = F::Interval::from_field(
                build_from_field_params(Default::default()),
                CanonicalFieldValue::Unsigned64(value),
            )
            .ok()
            .and_then(|built| {
                F::Interval::to_field(build_to_field_params(Default::default()), &built.value)
                    .ok()
                    .map(|returned| returned.field)
            });
            assert_eq!(yielded, Some(CanonicalFieldValue::Unsigned64(value)));
            assert_eq!(F::Interval::KIND, CanonicalFieldKind::Unsigned64);
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        for selection in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    }),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}
// create_chain_forms_hands_an_outside_consumer_an_interval_form_that_survives_its_field_round_trip

/// Contract: the own entry
///   `chain_forms_chain_identifier_survives_its_field_round_trip` over the
///   declared set: `F::ChainIdentifier` built by `from_field` over
///   `CanonicalFieldValue::Unsigned256` of the ascending bytes and yielded
///   back by `to_field` → the field returned equals the field given, and
///   `F::ChainIdentifier::KIND` is `CanonicalFieldKind::Unsigned256`.
/// Arrange: as the identity block, over `F::ChainIdentifier`,
///   `CanonicalFieldValue::Unsigned256`, and the thirty-two-byte ascending
///   literal; each member of the declared set is the variation.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member and required encoding.
/// Assert:  for each member the case result is `Ok`, the inner result is
///   `Ok`, the field `to_field` returns equals the arranged field, and the
///   form's `KIND` is the kind the contract states.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, the real ABI concrete,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the chain-identifier form's `from_field`, `to_field`, and
///   `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_a_chain_identifier_form_that_survives_its_field_round_trip()
 {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let bytes = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, 0x20,
            ];
            let yielded = F::ChainIdentifier::from_field(
                build_from_field_params(Default::default()),
                CanonicalFieldValue::Unsigned256(bytes),
            )
            .ok()
            .and_then(|built| {
                F::ChainIdentifier::to_field(
                    build_to_field_params(Default::default()),
                    &built.value,
                )
                .ok()
                .map(|returned| returned.field)
            });
            assert_eq!(yielded, Some(CanonicalFieldValue::Unsigned256(bytes)));
            assert_eq!(F::ChainIdentifier::KIND, CanonicalFieldKind::Unsigned256);
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        for selection in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    }),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}
// create_chain_forms_hands_an_outside_consumer_a_chain_identifier_form_that_survives_its_field_round_trip

/// Contract: the own entry
///   `chain_forms_key_identifier_survives_its_field_round_trip` over the
///   declared set: `<F::Identity as IIdentityForm>::derive_key_identifier`
///   over a scheme and a key, its key identifier yielded by `to_field` and
///   rebuilt by `from_field` → the rebuilt key identifier equals the
///   derived one, and its `KIND` is `CanonicalFieldKind::FixedBytes32`.
/// Arrange: as the private integration test's key identifier block, over
///   each member of `CHAIN_FORMS_CONCRETES` and the member of
///   `DECLARED_ENCODING_SELECTIONS` its declaration requires.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member and required encoding.
/// Assert:  for each member the case result is `Ok`, the inner result is
///   `Ok`, and the in-place assertions of the forms consumer hold.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, the real ABI concrete,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   `derive_key_identifier`, and the key identifier form's `to_field` and
///   `from_field`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_a_key_identifier_that_survives_its_field_round_trip()
 {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let key = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, 0x20, 0x21,
            ];
            let derived = <F::Identity as IIdentityForm>::derive_key_identifier(
                DeriveKeyIdentifierParams,
                DeriveKeyIdentifierPayload {
                    scheme: build_signature_scheme_identifier(
                        SignatureSchemeIdentifierConstructorParamsOverrides {
                            identifier: Some(0x5A),
                        },
                    ),
                    key: &key,
                },
            )
            .ok()
            .map(|success| success.key_identifier);
            let yielded = derived.as_ref().and_then(|key_identifier| {
                <F::Identity as IIdentityForm>::KeyIdentifier::to_field(
                    build_to_field_params(Default::default()),
                    key_identifier,
                )
                .ok()
                .map(|returned| returned.field)
            });
            let rebuilt = yielded.and_then(|field| {
                <F::Identity as IIdentityForm>::KeyIdentifier::from_field(
                    build_from_field_params(Default::default()),
                    field,
                )
                .ok()
                .map(|success| success.value)
            });
            assert!(derived.is_some());
            assert_eq!(rebuilt, derived);
            assert_eq!(
                <F::Identity as IIdentityForm>::KeyIdentifier::KIND,
                CanonicalFieldKind::FixedBytes32
            );
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        for selection in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    }),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}
// create_chain_forms_hands_an_outside_consumer_a_key_identifier_that_survives_its_field_round_trip

/// Contract: the own entry
///   `chain_forms_signed_intent_survives_its_field_round_trip` over the
///   declared set: `forms.signed_intent` over a chain, a contract, an
///   acting identity, a nonce, an expiry, and a call, its intent yielded
///   by `to_field` and rebuilt by `from_field` → the rebuilt intent equals
///   the one produced, and `F::SignedIntent::KIND` is
///   `CanonicalFieldKind::FixedBytes32`.
/// Arrange: as the private integration test's signed intent block, over
///   each member of `CHAIN_FORMS_CONCRETES` and the member of
///   `DECLARED_ENCODING_SELECTIONS` its declaration requires.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member and required encoding.
/// Assert:  for each member the case result is `Ok`, the inner result is
///   `Ok`, and the in-place assertions of the forms consumer hold.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, the real ABI concrete,
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   `signed_intent`, and the intent form's `to_field` and `from_field`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_forms_that_produce_a_signed_intent_that_survives_its_field_round_trip()
 {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let ascending = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, 0x20,
            ];
            let ascending_twenty = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
            ];
            let descending_twenty = [
                0x14, 0x13, 0x12, 0x11, 0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
            ];
            let chain = F::ChainIdentifier::from_field(
                build_from_field_params(Default::default()),
                CanonicalFieldValue::Unsigned256(ascending),
            )
            .ok()
            .map(|success| success.value);
            let contract = F::Identity::from_field(
                build_from_field_params(Default::default()),
                CanonicalFieldValue::FixedBytes20(ascending_twenty),
            )
            .ok()
            .map(|success| success.value);
            let identity = F::Identity::from_field(
                build_from_field_params(Default::default()),
                CanonicalFieldValue::FixedBytes20(descending_twenty),
            )
            .ok()
            .map(|success| success.value);
            let call = IntentCall {
                operation: "transfer".to_string(),
                fields: vec![IntentField {
                    name: "amount".to_string(),
                    value: CanonicalFieldValue::Unsigned256(ascending),
                }],
            };
            let produced = match (chain.as_ref(), contract.as_ref(), identity.as_ref()) {
                (Some(chain), Some(contract), Some(identity)) => payload
                    .forms
                    .signed_intent(
                        SignedIntentParams,
                        SignedIntentPayload {
                            chain,
                            contract,
                            identity,
                            nonce: CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708),
                            expiry: CanonicalFieldValue::Unsigned64(0x0807_0605_0403_0201),
                            call: &call,
                        },
                    )
                    .ok()
                    .map(|success| success.signed_intent),
                _ => None,
            };
            let yielded = produced.as_ref().and_then(|signed_intent| {
                F::SignedIntent::to_field(build_to_field_params(Default::default()), signed_intent)
                    .ok()
                    .map(|returned| returned.field)
            });
            let rebuilt = yielded.and_then(|field| {
                F::SignedIntent::from_field(build_from_field_params(Default::default()), field)
                    .ok()
                    .map(|success| success.value)
            });
            assert!(chain.is_some() && contract.is_some() && identity.is_some());
            assert!(produced.is_some());
            assert_eq!(rebuilt, produced);
            assert_eq!(F::SignedIntent::KIND, CanonicalFieldKind::FixedBytes32);
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        for selection in DECLARED_ENCODING_SELECTIONS {
            let case_params =
                build_create_chain_forms_case_params(CreateChainFormsCaseParamsOverrides {
                    encoding: Some(CreateEncodingParams {
                        concrete: selection.concrete,
                        identifier: selection.identifier,
                    }),
                });
            let case_payload = build_create_chain_forms_case_payload(Default::default());

            // Act
            let result = create_chain_forms_case(&deps, case_params, case_payload);

            // Assert
            assert!(result.is_ok());
            assert!(result.is_ok_and(|success| success.output.is_ok()));
        }
    }
}
// create_chain_forms_hands_an_outside_consumer_forms_that_produce_a_signed_intent_that_survives_its_field_round_trip

/// Contract: the own entry
///   `chain_forms_refuse_an_encoder_declaring_another_encoding_identifier`:
///   `create_chain_forms` with a concrete from `CHAIN_FORMS_CONCRETES`,
///   the forms identifier it declares, and an encoder declaring an
///   encoding identifier the concrete does not require →
///   `Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)`,
///   the forms consumer never called.
/// Arrange: the block's `Consumer` fails the test if it is called; the
///   case consumer as the statement above gives it, over the `Evm` member
///   with the case params at their defaults, which lend the mock encoding
///   concrete; the pairing is the variation.
/// Act:     `create_chain_forms_case(&deps, case_params, case_payload)`
///   per member.
/// Assert:  for each member the case result is `Ok` and the output it
///   holds is
///   `Err(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)` by
///   `assert_eq!`.
/// Boundary: the public `create_chain_forms_case`; the real chain is
///   `create_chain_forms_case`, `create_encoding`, and
///   `create_chain_forms` up to its encoding identifier check.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_refuses_an_encoder_declaring_another_encoding_identifier_for_an_outside_consumer()
 {
    #[derive(Clone, Copy)]
    struct Consumer;

    impl IChainFormsConsumer for Consumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            unreachable!("the forms consumer is never called")
        }
    }

    struct CaseConsumer {
        params: CreateChainFormsParams,
        consumer: Consumer,
    }

    impl ICreateChainFormsCaseConsumer for CaseConsumer {
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
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let deps = build_create_chain_forms_case_deps(
            CaseConsumer {
                params,
                consumer: Consumer,
            },
            Default::default(),
        );
        let case_params = build_create_chain_forms_case_params(Default::default());
        let case_payload = build_create_chain_forms_case_payload(Default::default());

        // Act
        let result = create_chain_forms_case(&deps, case_params, case_payload);

        // Assert
        assert!(result.is_ok());
        assert_eq!(
            result.ok().and_then(|success| success.output.err()),
            Some(CreateChainFormsErrorReturn::UnsupportedEncodingIdentifier)
        );
    }
}
// create_chain_forms_refuses_an_encoder_declaring_another_encoding_identifier_for_an_outside_consumer
