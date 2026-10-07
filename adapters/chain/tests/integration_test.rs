#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use chain::{
    ChainFormsConcrete, ChainFormsIdentifier, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsDeps, CreateChainFormsParamsOverrides, CreateChainFormsPayload, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_params, create_chain_forms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldParams, ICanonicalField, ToFieldParams,
};

struct FormsOutcome {
    identity_kind: CanonicalFieldKind,
    entitlement_kind: CanonicalFieldKind,
    interval_kind: CanonicalFieldKind,
    chain_identifier_kind: CanonicalFieldKind,
    fields: Vec<CanonicalFieldValue>,
    identity_refuses_another_kind: bool,
}

struct FormsRoundTrip {
    identity: CanonicalFieldValue,
    entitlement: CanonicalFieldValue,
    interval: CanonicalFieldValue,
    chain_identifier: CanonicalFieldValue,
}

impl IChainFormsConsumer for FormsRoundTrip {
    type Output = FormsOutcome;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        let Ok(identity) =
            <F::Identity as ICanonicalField>::from_field(FromFieldParams, self.identity.clone())
        else {
            panic!("the identity form rebuilds from its canonical field")
        };
        let Ok(entitlement) = <F::Entitlement as ICanonicalField>::from_field(
            FromFieldParams,
            self.entitlement.clone(),
        ) else {
            panic!("the entitlement form rebuilds from its canonical field")
        };
        let Ok(interval) =
            <F::Interval as ICanonicalField>::from_field(FromFieldParams, self.interval.clone())
        else {
            panic!("the interval form rebuilds from its canonical field")
        };
        let Ok(chain_identifier) = <F::ChainIdentifier as ICanonicalField>::from_field(
            FromFieldParams,
            self.chain_identifier.clone(),
        ) else {
            panic!("the chain-identifier form rebuilds from its canonical field")
        };
        let Ok(identity_field) =
            <F::Identity as ICanonicalField>::to_field(ToFieldParams, &identity.value);
        let Ok(entitlement_field) =
            <F::Entitlement as ICanonicalField>::to_field(ToFieldParams, &entitlement.value);
        let Ok(interval_field) =
            <F::Interval as ICanonicalField>::to_field(ToFieldParams, &interval.value);
        let Ok(chain_identifier_field) = <F::ChainIdentifier as ICanonicalField>::to_field(
            ToFieldParams,
            &chain_identifier.value,
        );
        FormsOutcome {
            identity_kind: <F::Identity as ICanonicalField>::KIND,
            entitlement_kind: <F::Entitlement as ICanonicalField>::KIND,
            interval_kind: <F::Interval as ICanonicalField>::KIND,
            chain_identifier_kind: <F::ChainIdentifier as ICanonicalField>::KIND,
            fields: vec![
                identity_field.field,
                entitlement_field.field,
                interval_field.field,
                chain_identifier_field.field,
            ],
            identity_refuses_another_kind: <F::Identity as ICanonicalField>::from_field(
                FromFieldParams,
                CanonicalFieldValue::FixedBytes32([0x31; 32]),
            )
            .is_err(),
        }
    }
}

/// Contract: the forms concrete the factory constructs, admitted for the EVM
///   forms identifier, rebuilds a transcript's identity, entitlement,
///   interval, and chain-identifier fields from their canonical values and
///   yields the same values back, under the kinds the EVM suite's schemas fix,
///   through a consumer that never names the concrete.
/// Arrange: `build_create_chain_forms_params` with `concrete:
///   Some(ChainFormsConcrete::Evm)` and `identifier:
///   Some(ChainFormsIdentifier::EvmV1)`, and `CreateChainFormsDeps { consumer:
///   FormsRoundTrip { identity: FixedBytes20([0x31; 20]), entitlement:
///   Unsigned256([0x41; 32]), interval: Unsigned64(7), chain_identifier:
///   Unsigned256([0x21; 32]) } }`.
/// Act:     `create_chain_forms(&deps, params, CreateChainFormsPayload)`.
/// Assert:  `success.output.identity_kind` equals `CanonicalFieldKind::FixedBytes20`,
///   `entitlement_kind` equals `CanonicalFieldKind::Unsigned256`,
///   `interval_kind` equals `CanonicalFieldKind::Unsigned64`,
///   `chain_identifier_kind` equals `CanonicalFieldKind::Unsigned256`, `fields`
///   equals the four canonical values in identity, entitlement, interval, and
///   chain-identifier order, and `identity_refuses_another_kind` is `true`.
/// Boundary: `create_chain_forms` and the EVM forms concrete, each real,
///   reached through `IChainForms` and the canonical-field contract.
/// Mocked:   nothing; the forms concrete has no outer edge.
#[test]
fn the_evm_forms_from_the_factory_rebuild_and_yield_each_transcript_field_through_the_form_interface()
 {
    // Arrange
    let deps = CreateChainFormsDeps {
        consumer: FormsRoundTrip {
            identity: CanonicalFieldValue::FixedBytes20([0x31; 20]),
            entitlement: CanonicalFieldValue::Unsigned256([0x41; 32]),
            interval: CanonicalFieldValue::Unsigned64(7),
            chain_identifier: CanonicalFieldValue::Unsigned256([0x21; 32]),
        },
    };
    let params = build_create_chain_forms_params(CreateChainFormsParamsOverrides {
        concrete: Some(ChainFormsConcrete::Evm),
        identifier: Some(ChainFormsIdentifier::EvmV1),
    });

    // Act
    let Ok(success) = create_chain_forms(&deps, params, CreateChainFormsPayload) else {
        panic!("the EVM forms concrete is admitted for the EVM forms identifier")
    };

    // Assert
    assert_eq!(
        success.output.identity_kind,
        CanonicalFieldKind::FixedBytes20
    );
    assert_eq!(
        success.output.entitlement_kind,
        CanonicalFieldKind::Unsigned256
    );
    assert_eq!(success.output.interval_kind, CanonicalFieldKind::Unsigned64);
    assert_eq!(
        success.output.chain_identifier_kind,
        CanonicalFieldKind::Unsigned256
    );
    assert_eq!(
        success.output.fields,
        vec![
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::Unsigned256([0x41; 32]),
            CanonicalFieldValue::Unsigned64(7),
            CanonicalFieldValue::Unsigned256([0x21; 32]),
        ]
    );
    assert!(success.output.identity_refuses_another_kind);
}
