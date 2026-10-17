#![allow(clippy::expect_used)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, FromFieldsReturn,
    ICanonicalField, IEncodingContract, ToFieldsReturn, build_canonical_fields,
    build_from_fields_params, build_to_fields_params,
};

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use super::interface::{
    MintProofVector, MintProofVectorFromFieldsErrorReturn, TransferProofVector,
    TransferProofVectorFromFieldsErrorReturn,
};
use super::mock::{
    MintProofVectorOverrides, ProofVectorContextOverrides, TransferProofVectorOverrides,
    build_mint_proof_vector, build_mint_proof_vector_description, build_proof_vector_context,
    build_transfer_proof_vector, build_transfer_proof_vector_description,
};

/// Contract: `to_fields` on `MintProofVectorDescription<F>` returns
///   `Ok(ToFieldsSuccessReturn { fields })`, `fields` the record's values
///   in member order, each a `CanonicalFieldValue`.
/// Arrange:  `build_mint_proof_vector` with `context` the reference
///   context, every other field its builder's fill, every field's value
///   distinct from every other field's; the description from
///   `build_mint_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.to_fields` over the
///   record with `build_to_fields_params(Default::default())`.
/// Assert:   `success.output`'s `fields.values` equals the twenty-nine
///   reference values in member order.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn mint_proof_vector_description_maps_the_record_in_member_order() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = ToFieldsReturn;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let vector = build_mint_proof_vector(MintProofVectorOverrides {
                context: Some(build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some([0x22; 32]),
                    suite_version: Some(1),
                    chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
                    entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
                    asset_identity_hash: Some([0x44; 32]),
                    parameter_set: Some([0x33; 32]),
                    source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x41; 32])),
                    target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x42; 32])),
                    old_interval: Some(CanonicalFieldValue::Unsigned64(1)),
                    new_interval: Some(CanonicalFieldValue::Unsigned64(2)),
                    purpose: Some(3),
                    seller: Some(CanonicalFieldValue::FixedBytes20([0x61; 20])),
                    buyer: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
                    expiry: Some(1_800_000_000),
                })),
                ..Default::default()
            });
            let description = build_mint_proof_vector_description::<F>(Default::default());

            // Act
            description.to_fields(build_to_fields_params(Default::default()), &vector)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success
            .output
            .expect("the description maps every record")
            .fields
            .values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x44; 32]),
            CanonicalFieldValue::FixedBytes32([0x33; 32]),
            CanonicalFieldValue::Unsigned256([0x41; 32]),
            CanonicalFieldValue::Unsigned256([0x42; 32]),
            CanonicalFieldValue::Unsigned64(1),
            CanonicalFieldValue::Unsigned64(2),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes20([0x61; 20]),
            CanonicalFieldValue::FixedBytes20([0x62; 20]),
            CanonicalFieldValue::Unsigned64(1_800_000_000),
            CanonicalFieldValue::Bytes(vec![0x10; 128]),
            CanonicalFieldValue::Bytes(vec![0x11; 64]),
            CanonicalFieldValue::Bytes(vec![0x12; 64]),
            CanonicalFieldValue::Bytes(vec![0x13; 128]),
            CanonicalFieldValue::Bytes(vec![0x14; 64]),
            CanonicalFieldValue::Bytes(vec![0x15; 64]),
            CanonicalFieldValue::Bytes(vec![0x16; 128]),
            CanonicalFieldValue::Bytes(vec![0x17; 128]),
            CanonicalFieldValue::Unsigned256([0x20; 32]),
            CanonicalFieldValue::Unsigned256([0x21; 32]),
            CanonicalFieldValue::Unsigned256([0x22; 32]),
            CanonicalFieldValue::Unsigned256([0x23; 32]),
            CanonicalFieldValue::Unsigned256([0x24; 32]),
            CanonicalFieldValue::Bytes(Vec::new()),
        ]
    );
    // mint_proof_vector_description_maps_the_record_in_member_order
}

/// Contract: `fields_to_value`, rebuilt: the values convert into the array
///   of the field count and each is the kind `FIELDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described })`, each field moved into
///   its member.
/// Arrange:  `build_canonical_fields` with `values` the mint reference
///   values; the description from
///   `build_mint_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output`'s `described` equals `build_mint_proof_vector`
///   with `context` the reference context.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn mint_proof_vector_description_rebuilds_the_record_from_its_fields() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x10; 128]),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x21; 32]),
                    CanonicalFieldValue::Unsigned256([0x22; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                    CanonicalFieldValue::Bytes(Vec::new()),
                ]),
            });
            let description = build_mint_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success
            .output
            .expect("fields of the declared kinds rebuild the record")
            .described,
        build_mint_proof_vector(MintProofVectorOverrides {
            context: Some(build_proof_vector_context(ProofVectorContextOverrides {
                suite_identifier: Some([0x22; 32]),
                suite_version: Some(1),
                chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
                entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
                asset_identity_hash: Some([0x44; 32]),
                parameter_set: Some([0x33; 32]),
                source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x41; 32])),
                target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x42; 32])),
                old_interval: Some(CanonicalFieldValue::Unsigned64(1)),
                new_interval: Some(CanonicalFieldValue::Unsigned64(2)),
                purpose: Some(3),
                seller: Some(CanonicalFieldValue::FixedBytes20([0x61; 20])),
                buyer: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
                expiry: Some(1_800_000_000),
            })),
            ..Default::default()
        })
    );
    // mint_proof_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `fields_to_value`, count: `payload.values` does not convert
///   into the array of the field count →
///   `Err(MintProofVectorFromFieldsErrorReturn::FieldCount { expected,
///   actual })`, `expected` the field count and `actual` the list's
///   length.
/// Arrange:  `build_canonical_fields` with `values` the mint reference
///   values without the last value, every value present of its declared
///   kind, so only the count refuses; the description from
///   `build_mint_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output` equals
///   `Err(MintProofVectorFromFieldsErrorReturn::FieldCount { expected: 29,
///   actual: 28 })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn mint_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x10; 128]),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x21; 32]),
                    CanonicalFieldValue::Unsigned256([0x22; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                ]),
            });
            let description = build_mint_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success.output.err(),
        Some(MintProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 29,
            actual: 28,
        })
    );
    // mint_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count
}

/// Contract: `fields_to_value`, count: `payload.values` does not convert
///   into the array of the field count →
///   `Err(MintProofVectorFromFieldsErrorReturn::FieldCount { expected,
///   actual })`.
/// Arrange:  `build_canonical_fields` with `values` the mint reference
///   values followed by `Bytes(Vec::new())`, every value at indexes `0`
///   through `28` of its declared kind, so only the count refuses; the
///   description from `build_mint_proof_vector_description::<F>` at its
///   defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output` equals
///   `Err(MintProofVectorFromFieldsErrorReturn::FieldCount { expected: 29,
///   actual: 30 })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn mint_proof_vector_description_refuses_a_field_list_longer_than_its_field_count() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x10; 128]),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x21; 32]),
                    CanonicalFieldValue::Unsigned256([0x22; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                    CanonicalFieldValue::Bytes(Vec::new()),
                    CanonicalFieldValue::Bytes(Vec::new()),
                ]),
            });
            let description = build_mint_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success.output.err(),
        Some(MintProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 29,
            actual: 30,
        })
    );
    // mint_proof_vector_description_refuses_a_field_list_longer_than_its_field_count
}

/// Contract: `fields_to_value`, kind: the lowest index `i` whose value is
///   not the kind `FIELDS` holds at `i` →
///   `Err(MintProofVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`.
/// Arrange:  `build_canonical_fields` with `values` the mint reference
///   values, the value at index `4` replaced by `Unsigned256([0x31; 32])`
///   and the value at index `12` by `Unsigned256([0x61; 32])`, two values
///   of a kind the identity form does not declare, so the lower index is
///   the one reported; the description from
///   `build_mint_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   in place, the result equals
///   `Err(MintProofVectorFromFieldsErrorReturn::FieldKind { index: 4,
///   expected: <F::Identity as ICanonicalField>::KIND })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn mint_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::Unsigned256([0x31; 32]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::Unsigned256([0x61; 32]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x10; 128]),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x21; 32]),
                    CanonicalFieldValue::Unsigned256([0x22; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                    CanonicalFieldValue::Bytes(Vec::new()),
                ]),
            });
            let description = build_mint_proof_vector_description::<F>(Default::default());

            // Act
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);

            // Assert
            assert_eq!(
                result.err(),
                Some(MintProofVectorFromFieldsErrorReturn::FieldKind {
                    index: 4,
                    expected: <F::Identity as ICanonicalField>::KIND,
                })
            );
        }
    }

    // Act
    create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");
    // mint_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind
}

/// Contract: `fields_to_value`, count then kind: a list of the wrong
///   count is refused by the count branch before any kind is read.
/// Arrange:  `build_canonical_fields` with `values` the mint reference
///   values without the last value and with the value at index `3`
///   replaced by `FixedBytes20([0x21; 20])`, a list both short and
///   holding a wrong kind; the description from
///   `build_mint_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output` equals
///   `Err(MintProofVectorFromFieldsErrorReturn::FieldCount { expected: 29,
///   actual: 28 })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn mint_proof_vector_description_reports_the_count_before_a_wrong_kind() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::FixedBytes20([0x21; 20]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x10; 128]),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x21; 32]),
                    CanonicalFieldValue::Unsigned256([0x22; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                ]),
            });
            let description = build_mint_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success.output.err(),
        Some(MintProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 29,
            actual: 28,
        })
    );
    // mint_proof_vector_description_reports_the_count_before_a_wrong_kind
}

/// Contract: `members`, every name admitted → `Ok` of the members, each
///   admitted name paired with `FIELDS` at the same index as a
///   `SolidityRecordMember { name, kind }`.
/// Arrange:  the expected members from `build_solidity_record_members`,
///   each member from `build_solidity_record_member` over
///   `build_solidity_member_name`, the names and kinds written out in
///   member order; the description from
///   `build_mint_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.members()`.
/// Assert:   in place, the result equals `Ok` of the expected members,
///   whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn mint_proof_vector_description_declares_its_members() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let expected =
                build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
                    members: Some(vec![
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("draws".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("suiteIdentifier".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::FixedBytes32),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("suiteVersion".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned16),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("chain".to_string()),
                                },
                            )),
                            kind: Some(<F::ChainIdentifier as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("entitlementContract".to_string()),
                                },
                            )),
                            kind: Some(<F::Identity as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("assetIdentityHash".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::FixedBytes32),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("parameterSet".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::FixedBytes32),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("sourceEntitlement".to_string()),
                                },
                            )),
                            kind: Some(<F::Entitlement as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("targetEntitlement".to_string()),
                                },
                            )),
                            kind: Some(<F::Entitlement as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("oldInterval".to_string()),
                                },
                            )),
                            kind: Some(<F::Interval as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("newInterval".to_string()),
                                },
                            )),
                            kind: Some(<F::Interval as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("purpose".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned16),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("seller".to_string()),
                                },
                            )),
                            kind: Some(<F::Identity as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("buyer".to_string()),
                                },
                            )),
                            kind: Some(<F::Identity as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("expiry".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned64),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("hpub".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("identityElement".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("buyerPk1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("buyerPk2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("envelopeC1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("envelopeC2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("envelopeD1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("envelopeD2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("challenge".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("alpha".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("r".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("rho".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("sigma".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("secondGroupFirstMessages".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                    ]),
                });
            let description = build_mint_proof_vector_description::<F>(Default::default());

            // Act
            let result = description.members();

            // Assert
            assert_eq!(result, Ok(expected));
        }
    }

    // Act
    create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");
    // mint_proof_vector_description_declares_its_members
}

/// Contract: `to_fields` on `TransferProofVectorDescription<F>` returns
///   `Ok(ToFieldsSuccessReturn { fields })`, `fields` the record's values
///   in member order, each a `CanonicalFieldValue`.
/// Arrange:  `build_transfer_proof_vector` with `context` the reference
///   context, every other field its builder's fill, every field's value
///   distinct from every other field's; the description from
///   `build_transfer_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.to_fields` over the
///   record with `build_to_fields_params(Default::default())`.
/// Assert:   `success.output`'s `fields.values` equals the thirty-five
///   reference values in member order.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn transfer_proof_vector_description_maps_the_record_in_member_order() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = ToFieldsReturn;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let vector = build_transfer_proof_vector(TransferProofVectorOverrides {
                context: Some(build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some([0x22; 32]),
                    suite_version: Some(1),
                    chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
                    entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
                    asset_identity_hash: Some([0x44; 32]),
                    parameter_set: Some([0x33; 32]),
                    source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x41; 32])),
                    target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x42; 32])),
                    old_interval: Some(CanonicalFieldValue::Unsigned64(1)),
                    new_interval: Some(CanonicalFieldValue::Unsigned64(2)),
                    purpose: Some(3),
                    seller: Some(CanonicalFieldValue::FixedBytes20([0x61; 20])),
                    buyer: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
                    expiry: Some(1_800_000_000),
                })),
                ..Default::default()
            });
            let description = build_transfer_proof_vector_description::<F>(Default::default());

            // Act
            description.to_fields(build_to_fields_params(Default::default()), &vector)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success
            .output
            .expect("the description maps every record")
            .fields
            .values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x44; 32]),
            CanonicalFieldValue::FixedBytes32([0x33; 32]),
            CanonicalFieldValue::Unsigned256([0x41; 32]),
            CanonicalFieldValue::Unsigned256([0x42; 32]),
            CanonicalFieldValue::Unsigned64(1),
            CanonicalFieldValue::Unsigned64(2),
            CanonicalFieldValue::Unsigned16(3),
            CanonicalFieldValue::FixedBytes20([0x61; 20]),
            CanonicalFieldValue::FixedBytes20([0x62; 20]),
            CanonicalFieldValue::Unsigned64(1_800_000_000),
            CanonicalFieldValue::Bytes(vec![0x11; 64]),
            CanonicalFieldValue::Bytes(vec![0x12; 64]),
            CanonicalFieldValue::Bytes(vec![0x13; 128]),
            CanonicalFieldValue::Bytes(vec![0x18; 64]),
            CanonicalFieldValue::Bytes(vec![0x19; 128]),
            CanonicalFieldValue::Bytes(vec![0x14; 64]),
            CanonicalFieldValue::Bytes(vec![0x15; 64]),
            CanonicalFieldValue::Bytes(vec![0x16; 128]),
            CanonicalFieldValue::Bytes(vec![0x17; 128]),
            CanonicalFieldValue::Bytes(vec![0x1A; 64]),
            CanonicalFieldValue::Bytes(vec![0x1B; 64]),
            CanonicalFieldValue::Bytes(vec![0x1C; 128]),
            CanonicalFieldValue::Bytes(vec![0x1D; 128]),
            CanonicalFieldValue::Unsigned256([0x20; 32]),
            CanonicalFieldValue::Unsigned256([0x25; 32]),
            CanonicalFieldValue::Unsigned256([0x26; 32]),
            CanonicalFieldValue::Unsigned256([0x27; 32]),
            CanonicalFieldValue::Unsigned256([0x23; 32]),
            CanonicalFieldValue::Unsigned256([0x24; 32]),
            CanonicalFieldValue::Bytes(Vec::new()),
        ]
    );
    // transfer_proof_vector_description_maps_the_record_in_member_order
}

/// Contract: `fields_to_value`, rebuilt: the values convert into the array
///   of the field count and each is the kind `FIELDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described })`, each field moved into
///   its member.
/// Arrange:  `build_canonical_fields` with `values` the transfer reference
///   values; the description from
///   `build_transfer_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output`'s `described` equals
///   `build_transfer_proof_vector` with `context` the reference context.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn transfer_proof_vector_description_rebuilds_the_record_from_its_fields() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x18; 64]),
                    CanonicalFieldValue::Bytes(vec![0x19; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1A; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1B; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1C; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1D; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x25; 32]),
                    CanonicalFieldValue::Unsigned256([0x26; 32]),
                    CanonicalFieldValue::Unsigned256([0x27; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                    CanonicalFieldValue::Bytes(Vec::new()),
                ]),
            });
            let description = build_transfer_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success
            .output
            .expect("fields of the declared kinds rebuild the record")
            .described,
        build_transfer_proof_vector(TransferProofVectorOverrides {
            context: Some(build_proof_vector_context(ProofVectorContextOverrides {
                suite_identifier: Some([0x22; 32]),
                suite_version: Some(1),
                chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
                entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
                asset_identity_hash: Some([0x44; 32]),
                parameter_set: Some([0x33; 32]),
                source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x41; 32])),
                target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x42; 32])),
                old_interval: Some(CanonicalFieldValue::Unsigned64(1)),
                new_interval: Some(CanonicalFieldValue::Unsigned64(2)),
                purpose: Some(3),
                seller: Some(CanonicalFieldValue::FixedBytes20([0x61; 20])),
                buyer: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
                expiry: Some(1_800_000_000),
            })),
            ..Default::default()
        })
    );
    // transfer_proof_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `fields_to_value`, count: `payload.values` does not convert
///   into the array of the field count →
///   `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount { expected,
///   actual })`.
/// Arrange:  `build_canonical_fields` with `values` the transfer reference
///   values without the last value, every value present of its declared
///   kind, so only the count refuses; the description from
///   `build_transfer_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output` equals
///   `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
///   expected: 35, actual: 34 })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn transfer_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x18; 64]),
                    CanonicalFieldValue::Bytes(vec![0x19; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1A; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1B; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1C; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1D; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x25; 32]),
                    CanonicalFieldValue::Unsigned256([0x26; 32]),
                    CanonicalFieldValue::Unsigned256([0x27; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                ]),
            });
            let description = build_transfer_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success.output.err(),
        Some(TransferProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 35,
            actual: 34,
        })
    );
    // transfer_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count
}

/// Contract: `fields_to_value`, count: `payload.values` does not convert
///   into the array of the field count →
///   `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount { expected,
///   actual })`.
/// Arrange:  `build_canonical_fields` with `values` the transfer reference
///   values followed by `Bytes(Vec::new())`, every value at indexes `0`
///   through `34` of its declared kind, so only the count refuses; the
///   description from `build_transfer_proof_vector_description::<F>` at
///   its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output` equals
///   `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
///   expected: 35, actual: 36 })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn transfer_proof_vector_description_refuses_a_field_list_longer_than_its_field_count() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x18; 64]),
                    CanonicalFieldValue::Bytes(vec![0x19; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1A; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1B; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1C; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1D; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x25; 32]),
                    CanonicalFieldValue::Unsigned256([0x26; 32]),
                    CanonicalFieldValue::Unsigned256([0x27; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                    CanonicalFieldValue::Bytes(Vec::new()),
                    CanonicalFieldValue::Bytes(Vec::new()),
                ]),
            });
            let description = build_transfer_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success.output.err(),
        Some(TransferProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 35,
            actual: 36,
        })
    );
    // transfer_proof_vector_description_refuses_a_field_list_longer_than_its_field_count
}

/// Contract: `fields_to_value`, kind: the lowest index `i` whose value is
///   not the kind `FIELDS` holds at `i` →
///   `Err(TransferProofVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`.
/// Arrange:  `build_canonical_fields` with `values` the transfer reference
///   values, the value at index `4` replaced by `Unsigned256([0x31; 32])`
///   and the value at index `12` by `Unsigned256([0x61; 32])`, two values
///   of a kind the identity form does not declare, so the lower index is
///   the one reported; the description from
///   `build_transfer_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   in place, the result equals
///   `Err(TransferProofVectorFromFieldsErrorReturn::FieldKind { index: 4,
///   expected: <F::Identity as ICanonicalField>::KIND })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn transfer_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::Unsigned256([0x2A; 32]),
                    CanonicalFieldValue::Unsigned256([0x31; 32]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::Unsigned256([0x61; 32]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x18; 64]),
                    CanonicalFieldValue::Bytes(vec![0x19; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1A; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1B; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1C; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1D; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x25; 32]),
                    CanonicalFieldValue::Unsigned256([0x26; 32]),
                    CanonicalFieldValue::Unsigned256([0x27; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                    CanonicalFieldValue::Bytes(Vec::new()),
                ]),
            });
            let description = build_transfer_proof_vector_description::<F>(Default::default());

            // Act
            let result =
                description.fields_to_value(build_from_fields_params(Default::default()), fields);

            // Assert
            assert_eq!(
                result.err(),
                Some(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                    index: 4,
                    expected: <F::Identity as ICanonicalField>::KIND,
                })
            );
        }
    }

    // Act
    create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");
    // transfer_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind
}

/// Contract: `fields_to_value`, count then kind: a list of the wrong
///   count is refused by the count branch before any kind is read.
/// Arrange:  `build_canonical_fields` with `values` the transfer reference
///   values without the last value and with the value at index `3`
///   replaced by `FixedBytes20([0x21; 20])`, a list both short and
///   holding a wrong kind; the description from
///   `build_transfer_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.fields_to_value`
///   over the fields with `build_from_fields_params(Default::default())`.
/// Assert:   `success.output` equals
///   `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
///   expected: 35, actual: 34 })` whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn transfer_proof_vector_description_reports_the_count_before_a_wrong_kind() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let fields = build_canonical_fields(CanonicalFieldsOverrides {
                values: Some(vec![
                    CanonicalFieldValue::Bytes(vec![0x01; 64]),
                    CanonicalFieldValue::FixedBytes32([0x22; 32]),
                    CanonicalFieldValue::Unsigned16(1),
                    CanonicalFieldValue::FixedBytes20([0x21; 20]),
                    CanonicalFieldValue::FixedBytes20([0x31; 20]),
                    CanonicalFieldValue::FixedBytes32([0x44; 32]),
                    CanonicalFieldValue::FixedBytes32([0x33; 32]),
                    CanonicalFieldValue::Unsigned256([0x41; 32]),
                    CanonicalFieldValue::Unsigned256([0x42; 32]),
                    CanonicalFieldValue::Unsigned64(1),
                    CanonicalFieldValue::Unsigned64(2),
                    CanonicalFieldValue::Unsigned16(3),
                    CanonicalFieldValue::FixedBytes20([0x61; 20]),
                    CanonicalFieldValue::FixedBytes20([0x62; 20]),
                    CanonicalFieldValue::Unsigned64(1_800_000_000),
                    CanonicalFieldValue::Bytes(vec![0x11; 64]),
                    CanonicalFieldValue::Bytes(vec![0x12; 64]),
                    CanonicalFieldValue::Bytes(vec![0x13; 128]),
                    CanonicalFieldValue::Bytes(vec![0x18; 64]),
                    CanonicalFieldValue::Bytes(vec![0x19; 128]),
                    CanonicalFieldValue::Bytes(vec![0x14; 64]),
                    CanonicalFieldValue::Bytes(vec![0x15; 64]),
                    CanonicalFieldValue::Bytes(vec![0x16; 128]),
                    CanonicalFieldValue::Bytes(vec![0x17; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1A; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1B; 64]),
                    CanonicalFieldValue::Bytes(vec![0x1C; 128]),
                    CanonicalFieldValue::Bytes(vec![0x1D; 128]),
                    CanonicalFieldValue::Unsigned256([0x20; 32]),
                    CanonicalFieldValue::Unsigned256([0x25; 32]),
                    CanonicalFieldValue::Unsigned256([0x26; 32]),
                    CanonicalFieldValue::Unsigned256([0x27; 32]),
                    CanonicalFieldValue::Unsigned256([0x23; 32]),
                    CanonicalFieldValue::Unsigned256([0x24; 32]),
                ]),
            });
            let description = build_transfer_proof_vector_description::<F>(Default::default());

            // Act
            description.fields_to_value(build_from_fields_params(Default::default()), fields)
        }
    }

    // Act
    let success = create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");

    // Assert
    assert_eq!(
        success.output.err(),
        Some(TransferProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 35,
            actual: 34,
        })
    );
    // transfer_proof_vector_description_reports_the_count_before_a_wrong_kind
}

/// Contract: `members`, every name admitted → `Ok` of the members, each
///   admitted name paired with `FIELDS` at the same index as a
///   `SolidityRecordMember { name, kind }`.
/// Arrange:  the expected members from `build_solidity_record_members`,
///   each member from `build_solidity_record_member` over
///   `build_solidity_member_name`, the names and kinds written out in
///   member order; the description from
///   `build_transfer_proof_vector_description::<F>` at its defaults.
/// Act:      the consumer's single call, `description.members()`.
/// Assert:   in place, the result equals `Ok` of the expected members,
///   whole.
/// Collaborators: the chain family's mock concrete through
///   `create_chain_forms` under the default params.
#[test]
fn transfer_proof_vector_description_declares_its_members() {
    struct FormsConsumer;

    impl IChainFormsConsumer for FormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Arrange
            let expected =
                build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
                    members: Some(vec![
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("draws".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("suiteIdentifier".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::FixedBytes32),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("suiteVersion".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned16),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("chain".to_string()),
                                },
                            )),
                            kind: Some(<F::ChainIdentifier as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("entitlementContract".to_string()),
                                },
                            )),
                            kind: Some(<F::Identity as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("assetIdentityHash".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::FixedBytes32),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("parameterSet".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::FixedBytes32),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("sourceEntitlement".to_string()),
                                },
                            )),
                            kind: Some(<F::Entitlement as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("targetEntitlement".to_string()),
                                },
                            )),
                            kind: Some(<F::Entitlement as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("oldInterval".to_string()),
                                },
                            )),
                            kind: Some(<F::Interval as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("newInterval".to_string()),
                                },
                            )),
                            kind: Some(<F::Interval as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("purpose".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned16),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("seller".to_string()),
                                },
                            )),
                            kind: Some(<F::Identity as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("buyer".to_string()),
                                },
                            )),
                            kind: Some(<F::Identity as ICanonicalField>::KIND),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("expiry".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned64),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("identityElement".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("sellerPk1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("sellerPk2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("buyerPk1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("buyerPk2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("oldEnvelopeC1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("oldEnvelopeC2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("oldEnvelopeD1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("oldEnvelopeD2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("newEnvelopeC1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("newEnvelopeC2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("newEnvelopeD1".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("newEnvelopeD2".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("challenge".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("x".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("y".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("s".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("rho".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("sigma".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Unsigned256),
                        }),
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("secondGroupFirstMessages".to_string()),
                                },
                            )),
                            kind: Some(CanonicalFieldKind::Bytes),
                        }),
                    ]),
                });
            let description = build_transfer_proof_vector_description::<F>(Default::default());

            // Act
            let result = description.members();

            // Assert
            assert_eq!(result, Ok(expected));
        }
    }

    // Act
    create_chain_forms(
        &build_create_chain_forms_deps(FormsConsumer, Default::default()),
        build_create_chain_forms_params(Default::default()),
        build_create_chain_forms_payload(Default::default()),
    )
    .expect("the default selection constructs the chain forms");
    // transfer_proof_vector_description_declares_its_members
}
