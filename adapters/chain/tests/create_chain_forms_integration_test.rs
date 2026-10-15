#![cfg(feature = "mocks")]

use chain::{
    CHAIN_FORMS_CONCRETES, CHAIN_FORMS_INTERFACE_VERSION, ChainFormsDeclaration,
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsParamsOverrides,
    IChainForms, IChainFormsConsumer, build_create_chain_forms_deps,
    build_create_chain_forms_params, build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ICanonicalField, build_from_field_params,
    build_to_field_params,
};

/// Contract: the own entry
///   `chain_forms_handed_to_the_consumer_declare_the_required_identifier`
///   over the declared set: `create_chain_forms` called with a consumer
///   returning `F::DECLARATION`, a concrete from `CHAIN_FORMS_CONCRETES`,
///   and `params.identifier` → `Ok(CreateChainFormsSuccessReturn
///   { output })` with `output.identifier` equal to `params.identifier`
///   and `output.interface_version` equal to `CHAIN_FORMS_INTERFACE_VERSION`.
/// Arrange: the block's `Consumer`, whose `consume_chain_forms` returns
///   `F::DECLARATION`; `build_create_chain_forms_deps` over it; per member,
///   `build_create_chain_forms_params` overriding `concrete`, the
///   identifier at its default for that concrete, and
///   `build_create_chain_forms_payload`, all from the crate's public
///   surface under the `mocks` feature; each member of the declared set is
///   the variation.
/// Act:     `create_chain_forms(&deps, params, payload)` per member.
/// Assert:  the `Ok` arm's `output.identifier` equals the identifier the
///   block passed and `output.interface_version` equals
///   `CHAIN_FORMS_INTERFACE_VERSION`.
/// Boundary: the public `create_chain_forms`; the real chain is
///   `create_chain_forms`, the concrete's producer, and
///   `consume_chain_forms`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_forms_declaring_the_required_identifier_to_an_outside_consumer() {
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

    // Arrange
    let deps = build_create_chain_forms_deps(Consumer, Default::default());
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let payload = build_create_chain_forms_payload(Default::default());
        let identifier = params.identifier;

        // Act
        let result = create_chain_forms(&deps, params, payload);

        // Assert
        let output = result.ok().map(|success| success.output);
        assert_eq!(
            output.map(|declaration| declaration.identifier),
            Some(identifier)
        );
        assert_eq!(
            output.map(|declaration| declaration.interface_version),
            Some(CHAIN_FORMS_INTERFACE_VERSION)
        );
    }
}

/// Contract: the own entry
///   `chain_forms_identity_survives_its_field_round_trip` over the declared
///   set: `F::Identity` built by `from_field` over `CanonicalFieldValue::
///   FixedBytes20` of the ascending bytes and yielded back by `to_field` →
///   the field returned equals the field given, and `F::Identity::KIND` is
///   `CanonicalFieldKind::FixedBytes20`.
/// Arrange: the block's `Consumer`, whose `consume_chain_forms` builds the
///   form by `from_field` with `build_from_field_params` and the ascending
///   field, takes the form from the `Ok` arm, yields the field back by
///   `to_field` with `build_to_field_params`, and asserts in place that the
///   returned field equals the arranged field and that `F::Identity::KIND`
///   is `CanonicalFieldKind::FixedBytes20`; deps, params, and payload as in
///   the declaration block; each member of the declared set is the
///   variation.
/// Act:     `create_chain_forms(&deps, params, payload)` per member.
/// Assert:  the result is `Ok` per member; the in-place asserts prove the
///   round trip and the form's `KIND`.
/// Boundary: the public `create_chain_forms`; the real chain is
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the identity form's `from_field`, `to_field`, and `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_an_identity_form_that_survives_its_field_round_trip()
 {
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

    // Arrange
    let deps = build_create_chain_forms_deps(Consumer, Default::default());
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let payload = build_create_chain_forms_payload(Default::default());

        // Act
        let result = create_chain_forms(&deps, params, payload);

        // Assert
        assert!(result.is_ok());
    }
}

/// Contract: the own entry
///   `chain_forms_entitlement_survives_its_field_round_trip` over the
///   declared set: `F::Entitlement` built by `from_field` over
///   `CanonicalFieldValue::Unsigned256` of the ascending bytes and yielded
///   back by `to_field` → the field returned equals the field given, and
///   `F::Entitlement::KIND` is `CanonicalFieldKind::Unsigned256`.
/// Arrange: as the identity block, over `F::Entitlement`,
///   `CanonicalFieldValue::Unsigned256`, and the thirty-two-byte ascending
///   literal; each member of the declared set is the variation.
/// Act:     `create_chain_forms(&deps, params, payload)` per member.
/// Assert:  the result is `Ok` per member; the in-place asserts prove the
///   round trip and the form's `KIND`.
/// Boundary: the public `create_chain_forms`; the real chain is
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the entitlement form's `from_field`, `to_field`, and `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_an_entitlement_form_that_survives_its_field_round_trip()
 {
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

    // Arrange
    let deps = build_create_chain_forms_deps(Consumer, Default::default());
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let payload = build_create_chain_forms_payload(Default::default());

        // Act
        let result = create_chain_forms(&deps, params, payload);

        // Assert
        assert!(result.is_ok());
    }
}

/// Contract: the own entry
///   `chain_forms_interval_survives_its_field_round_trip` over the declared
///   set: `F::Interval` built by `from_field` over `CanonicalFieldValue::
///   Unsigned64` of `0x0102_0304_0506_0708` and yielded back by `to_field`
///   → the field returned equals the field given, and `F::Interval::KIND`
///   is `CanonicalFieldKind::Unsigned64`.
/// Arrange: as the identity block, over `F::Interval`,
///   `CanonicalFieldValue::Unsigned64`, and the value
///   `0x0102_0304_0506_0708`; each member of the declared set is the
///   variation.
/// Act:     `create_chain_forms(&deps, params, payload)` per member.
/// Assert:  the result is `Ok` per member; the in-place asserts prove the
///   round trip and the form's `KIND`.
/// Boundary: the public `create_chain_forms`; the real chain is
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the interval form's `from_field`, `to_field`, and `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_an_interval_form_that_survives_its_field_round_trip()
 {
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

    // Arrange
    let deps = build_create_chain_forms_deps(Consumer, Default::default());
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let payload = build_create_chain_forms_payload(Default::default());

        // Act
        let result = create_chain_forms(&deps, params, payload);

        // Assert
        assert!(result.is_ok());
    }
}

/// Contract: the own entry
///   `chain_forms_chain_identifier_survives_its_field_round_trip` over the
///   declared set: `F::ChainIdentifier` built by `from_field` over
///   `CanonicalFieldValue::Unsigned256` of the ascending bytes and yielded
///   back by `to_field` → the field returned equals the field given, and
///   `F::ChainIdentifier::KIND` is `CanonicalFieldKind::Unsigned256`.
/// Arrange: as the identity block, over `F::ChainIdentifier`,
///   `CanonicalFieldValue::Unsigned256`, and the thirty-two-byte ascending
///   literal; each member of the declared set is the variation.
/// Act:     `create_chain_forms(&deps, params, payload)` per member.
/// Assert:  the result is `Ok` per member; the in-place asserts prove the
///   round trip and the form's `KIND`.
/// Boundary: the public `create_chain_forms`; the real chain is
///   `create_chain_forms`, the concrete's producer, `consume_chain_forms`,
///   and the chain-identifier form's `from_field`, `to_field`, and
///   `try_new`.
/// Mocked:  nothing.
#[test]
fn create_chain_forms_hands_an_outside_consumer_a_chain_identifier_form_that_survives_its_field_round_trip()
 {
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

    // Arrange
    let deps = build_create_chain_forms_deps(Consumer, Default::default());
    for concrete in CHAIN_FORMS_CONCRETES.iter().copied() {
        let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        });
        let payload = build_create_chain_forms_payload(Default::default());

        // Act
        let result = create_chain_forms(&deps, params, payload);

        // Assert
        assert!(result.is_ok());
    }
}
