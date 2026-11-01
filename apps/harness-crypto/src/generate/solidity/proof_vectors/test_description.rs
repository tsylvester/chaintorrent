#![allow(clippy::expect_used)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldValue, CanonicalFields, CanonicalFieldsOverrides, ConsumeEncodingParams,
    ConsumeEncodingPayload, FromFieldsReturn, ICanonicalField, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, IEncodingContract, build_canonical_fields, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, build_from_fields_params,
    build_to_fields_params, create_encoding,
};

use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::interface::{
    MintProofVector, MintProofVectorDescription, MintProofVectorFromFieldsErrorReturn,
    TransferProofVector, TransferProofVectorDescription, TransferProofVectorFromFieldsErrorReturn,
};
use super::mock::{
    MintProofVectorOverrides, ProofVectorContextOverrides, TransferProofVectorOverrides,
    build_mint_proof_vector, build_mint_proof_vector_description, build_proof_vector_context,
    build_transfer_proof_vector, build_transfer_proof_vector_description,
};

/// Contract: `to_fields` on `MintProofVectorDescription<F>`, one branch → `Ok`
///   of `ToFieldsSuccessReturn { fields }`, `fields` the record's values in
///   member order, each a `CanonicalFieldValue`.
/// Arrange: `build_mint_proof_vector` overriding `context` with the reference
///   context; every field holds a value distinct from every other field's, so a
///   swapped, dropped, or mis-wrapped field reports otherwise.
/// Act:    `to_fields` on the description over the record with
///   `build_to_fields_params` at its defaults, inside the consumer.
/// Assert: the fields equal the mint reference values.
#[test]
fn mint_proof_vector_description_maps_the_record_in_member_order() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        vector: &'a MintProofVector,
    }

    impl IChainFormsConsumer for ChainFormsConsumer<'_> {
        type Output = CanonicalFields;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_mint_proof_vector_description::<F>(Default::default())
                .to_fields(build_to_fields_params(Default::default()), self.vector)
                .expect("the description maps the record")
                .fields
        }
    }

    struct EncodingConsumer<'a> {
        vector: &'a MintProofVector,
    }

    impl IEncodingConsumer for EncodingConsumer<'_> {
        type Output = CanonicalFields;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        vector: self.vector,
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let vector = build_mint_proof_vector(MintProofVectorOverrides {
        context: Some(build_proof_vector_context(ProofVectorContextOverrides {
            suite_identifier: Some([0x22; 32]),
            suite_version: Some(1),
            chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
            entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
            asset_root: Some([0x01; 32]),
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

    // Act
    let fields = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { vector: &vector }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Bytes(vec![0x01; 64]),
                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                CanonicalFieldValue::Unsigned16(1),
                CanonicalFieldValue::Unsigned256([0x2A; 32]),
                CanonicalFieldValue::FixedBytes20([0x31; 20]),
                CanonicalFieldValue::FixedBytes32([0x01; 32]),
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
        })
    );
}
// mint_proof_vector_description_maps_the_record_in_member_order

/// Contract: `fields_to_value`, rebuilt: the values convert into the array of
///   the field count and each is the kind `FIELDS` holds at its index → `Ok`
///   of `FromFieldsSuccessReturn { described }`, each field moved into its
///   member.
/// Arrange: `build_canonical_fields` overriding `values` with the mint
///   reference values.
/// Act:    `fields_to_value` on the description over the fields with
///   `build_from_fields_params` at its defaults, inside the consumer.
/// Assert: `described` equals `build_mint_proof_vector` overriding `context`
///   with the reference context.
#[test]
fn mint_proof_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = MintProofVector;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_mint_proof_vector_description::<F>(Default::default())
                .fields_to_value(
                    build_from_fields_params(Default::default()),
                    self.fields.clone(),
                )
                .expect("the fields rebuild the record")
                .described
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output = MintProofVector;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let described = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        described,
        build_mint_proof_vector(MintProofVectorOverrides {
            context: Some(build_proof_vector_context(ProofVectorContextOverrides {
                suite_identifier: Some([0x22; 32]),
                suite_version: Some(1),
                chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
                entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
                asset_root: Some([0x01; 32]),
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
}
// mint_proof_vector_description_rebuilds_the_record_from_its_fields

/// Contract: `fields_to_value`, count: `payload.values` does not convert into
///   the array of the field count → `Err(MintProofVectorFromFieldsErrorReturn::FieldCount
///   { expected, actual })`, `expected` the field count and `actual` the
///   list's length.
/// Arrange: `build_canonical_fields` overriding `values` with the mint
///   reference values without the last value, every value present of its
///   declared kind, so only the count refuses.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(MintProofVectorFromFieldsErrorReturn::FieldCount
///   { expected: 29, actual: 28 })` whole.
#[test]
fn mint_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_mint_proof_vector_description::<F>(Default::default()).fields_to_value(
                build_from_fields_params(Default::default()),
                self.fields.clone(),
            )
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let result = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(MintProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 29,
            actual: 28,
        })
    );
}
// mint_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count

/// Contract: `fields_to_value`, count: `payload.values` does not convert into
///   the array of the field count → `Err(MintProofVectorFromFieldsErrorReturn::FieldCount
///   { expected, actual })`.
/// Arrange: `build_canonical_fields` overriding `values` with the mint
///   reference values followed by `Bytes(Vec::new())`, every value at indexes
///   `0` through `28` of its declared kind, so only the count refuses.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(MintProofVectorFromFieldsErrorReturn::FieldCount
///   { expected: 29, actual: 30 })` whole.
#[test]
fn mint_proof_vector_description_refuses_a_field_list_longer_than_its_field_count() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_mint_proof_vector_description::<F>(Default::default()).fields_to_value(
                build_from_fields_params(Default::default()),
                self.fields.clone(),
            )
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let result = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(MintProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 29,
            actual: 30,
        })
    );
}
// mint_proof_vector_description_refuses_a_field_list_longer_than_its_field_count

/// Contract: `fields_to_value`, kind: the lowest index `i` whose value is not
///   the kind `FIELDS` holds at `i` → `Err(MintProofVectorFromFieldsErrorReturn::FieldKind
///   { index: i, expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the mint
///   reference values with the value at index `4` replaced by
///   `Unsigned256([0x31; 32])` and the value at index `12` replaced by
///   `Unsigned256([0x61; 32])`, two values of a kind the identity form does not
///   declare, so the lower index is the one reported.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(MintProofVectorFromFieldsErrorReturn::FieldKind
///   { index: 4, expected: <F::Identity as ICanonicalField>::KIND })` whole,
///   asserted in place.
#[test]
fn mint_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let result = build_mint_proof_vector_description::<F>(Default::default())
                .fields_to_value(
                    build_from_fields_params(Default::default()),
                    self.fields.clone(),
                );

            // Assert
            assert_eq!(
                result,
                Err(MintProofVectorFromFieldsErrorReturn::FieldKind {
                    index: 4,
                    expected: <F::Identity as ICanonicalField>::KIND,
                })
            );
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms");
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::Unsigned256([0x31; 32]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder");
}
// mint_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind

/// Contract: `fields_to_value`, count then kind: a list of the wrong count is
///   refused by the count branch before any kind is read.
/// Arrange: `build_canonical_fields` overriding `values` with the mint
///   reference values without the last value and with the value at index `3`
///   replaced by `FixedBytes20([0x21; 20])`, a list both short and holding a
///   wrong kind.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(MintProofVectorFromFieldsErrorReturn::FieldCount
///   { expected: 29, actual: 28 })` whole.
#[test]
fn mint_proof_vector_description_reports_the_count_before_a_wrong_kind() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_mint_proof_vector_description::<F>(Default::default()).fields_to_value(
                build_from_fields_params(Default::default()),
                self.fields.clone(),
            )
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output = FromFieldsReturn<MintProofVector, MintProofVectorFromFieldsErrorReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::FixedBytes20([0x21; 20]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let result = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(MintProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 29,
            actual: 28,
        })
    );
}
// mint_proof_vector_description_reports_the_count_before_a_wrong_kind

/// Contract: `member_names`, every name admitted → `Ok` of the admitted names
///   in order, one for each index of `FIELDS`.
/// Arrange: the expected names, each from `build_solidity_member_name` over
///   `draws`, `suiteIdentifier`, `suiteVersion`, `chain`, `entitlementContract`,
///   `assetRoot`, `parameterSet`, `sourceEntitlement`, `targetEntitlement`,
///   `oldInterval`, `newInterval`, `purpose`, `seller`, `buyer`, `expiry`,
///   `hpub`, `identityElement`, `buyerPk1`, `buyerPk2`, `envelopeC1`,
///   `envelopeC2`, `envelopeD1`, `envelopeD2`, `challenge`, `alpha`, `r`,
///   `rho`, `sigma`, and `secondGroupFirstMessages`, written out.
/// Act:    `member_names()` on the description, inside the consumer.
/// Assert: the result equals `Ok` of the expected names whole, asserted in
///   place, and its length equals the length of `FIELDS`.
#[test]
fn mint_proof_vector_description_declares_one_name_per_field() {
    // Arrange
    struct ChainFormsConsumer;

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let expected = vec![
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("draws".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("suiteIdentifier".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("suiteVersion".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("chain".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("entitlementContract".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("assetRoot".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("parameterSet".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("sourceEntitlement".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("targetEntitlement".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("oldInterval".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("newInterval".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("purpose".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("seller".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("buyer".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("expiry".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("hpub".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("identityElement".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("buyerPk1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("buyerPk2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("envelopeC1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("envelopeC2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("envelopeD1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("envelopeD2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("challenge".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("alpha".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("r".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("rho".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("sigma".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("secondGroupFirstMessages".to_string()),
                }),
            ];

            // Act
            let result =
                build_mint_proof_vector_description::<F>(Default::default()).member_names();

            // Assert
            assert_eq!(
                result.as_ref().map(Vec::len),
                Ok(MintProofVectorDescription::<F>::FIELDS.len())
            );
            assert_eq!(result, Ok(expected));
        }
    }

    struct EncodingConsumer;

    impl IEncodingConsumer for EncodingConsumer {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer,
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms");
        }
    }

    // Act
    create_encoding(
        &build_create_encoding_deps(EncodingConsumer),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder");
}
// mint_proof_vector_description_declares_one_name_per_field

/// Contract: `to_fields` on `TransferProofVectorDescription<F>`, one branch →
///   `Ok` of `ToFieldsSuccessReturn { fields }`, `fields` the record's values
///   in member order, each a `CanonicalFieldValue`.
/// Arrange: `build_transfer_proof_vector` overriding `context` with the
///   reference context; every field holds a value distinct from every other
///   field's.
/// Act:    `to_fields` on the description over the record with
///   `build_to_fields_params` at its defaults, inside the consumer.
/// Assert: the fields equal the transfer reference values.
#[test]
fn transfer_proof_vector_description_maps_the_record_in_member_order() {
    // Arrange
    struct ChainFormsConsumer<'a> {
        vector: &'a TransferProofVector,
    }

    impl IChainFormsConsumer for ChainFormsConsumer<'_> {
        type Output = CanonicalFields;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_transfer_proof_vector_description::<F>(Default::default())
                .to_fields(build_to_fields_params(Default::default()), self.vector)
                .expect("the description maps the record")
                .fields
        }
    }

    struct EncodingConsumer<'a> {
        vector: &'a TransferProofVector,
    }

    impl IEncodingConsumer for EncodingConsumer<'_> {
        type Output = CanonicalFields;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        vector: self.vector,
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let vector = build_transfer_proof_vector(TransferProofVectorOverrides {
        context: Some(build_proof_vector_context(ProofVectorContextOverrides {
            suite_identifier: Some([0x22; 32]),
            suite_version: Some(1),
            chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
            entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
            asset_root: Some([0x01; 32]),
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

    // Act
    let fields = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { vector: &vector }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Bytes(vec![0x01; 64]),
                CanonicalFieldValue::FixedBytes32([0x22; 32]),
                CanonicalFieldValue::Unsigned16(1),
                CanonicalFieldValue::Unsigned256([0x2A; 32]),
                CanonicalFieldValue::FixedBytes20([0x31; 20]),
                CanonicalFieldValue::FixedBytes32([0x01; 32]),
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
        })
    );
}
// transfer_proof_vector_description_maps_the_record_in_member_order

/// Contract: `fields_to_value`, rebuilt: the values convert into the array of
///   the field count and each is the kind `FIELDS` holds at its index → `Ok`
///   of `FromFieldsSuccessReturn { described }`, each field moved into its
///   member.
/// Arrange: `build_canonical_fields` overriding `values` with the transfer
///   reference values.
/// Act:    `fields_to_value` on the description over the fields with
///   `build_from_fields_params` at its defaults, inside the consumer.
/// Assert: `described` equals `build_transfer_proof_vector` overriding
///   `context` with the reference context.
#[test]
fn transfer_proof_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = TransferProofVector;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_transfer_proof_vector_description::<F>(Default::default())
                .fields_to_value(
                    build_from_fields_params(Default::default()),
                    self.fields.clone(),
                )
                .expect("the fields rebuild the record")
                .described
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output = TransferProofVector;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let described = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        described,
        build_transfer_proof_vector(TransferProofVectorOverrides {
            context: Some(build_proof_vector_context(ProofVectorContextOverrides {
                suite_identifier: Some([0x22; 32]),
                suite_version: Some(1),
                chain: Some(CanonicalFieldValue::Unsigned256([0x2A; 32])),
                entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x31; 20])),
                asset_root: Some([0x01; 32]),
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
}
// transfer_proof_vector_description_rebuilds_the_record_from_its_fields

/// Contract: `fields_to_value`, count: `payload.values` does not convert into
///   the array of the field count → `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount
///   { expected, actual })`.
/// Arrange: `build_canonical_fields` overriding `values` with the transfer
///   reference values without the last value, every value present of its
///   declared kind, so only the count refuses.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount
///   { expected: 35, actual: 34 })` whole.
#[test]
fn transfer_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_transfer_proof_vector_description::<F>(Default::default()).fields_to_value(
                build_from_fields_params(Default::default()),
                self.fields.clone(),
            )
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let result = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 35,
            actual: 34,
        })
    );
}
// transfer_proof_vector_description_refuses_a_field_list_shorter_than_its_field_count

/// Contract: `fields_to_value`, count: `payload.values` does not convert into
///   the array of the field count → `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount
///   { expected, actual })`.
/// Arrange: `build_canonical_fields` overriding `values` with the transfer
///   reference values followed by `Bytes(Vec::new())`, every value at indexes
///   `0` through `34` of its declared kind, so only the count refuses.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount
///   { expected: 35, actual: 36 })` whole.
#[test]
fn transfer_proof_vector_description_refuses_a_field_list_longer_than_its_field_count() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_transfer_proof_vector_description::<F>(Default::default()).fields_to_value(
                build_from_fields_params(Default::default()),
                self.fields.clone(),
            )
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let result = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 35,
            actual: 36,
        })
    );
}
// transfer_proof_vector_description_refuses_a_field_list_longer_than_its_field_count

/// Contract: `fields_to_value`, kind: the lowest index `i` whose value is not
///   the kind `FIELDS` holds at `i` → `Err(TransferProofVectorFromFieldsErrorReturn::FieldKind
///   { index: i, expected })`.
/// Arrange: `build_canonical_fields` overriding `values` with the transfer
///   reference values with the value at index `4` replaced by
///   `Unsigned256([0x31; 32])` and the value at index `12` replaced by
///   `Unsigned256([0x61; 32])`, two values of a kind the identity form does not
///   declare, so the lower index is the one reported.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(TransferProofVectorFromFieldsErrorReturn::FieldKind
///   { index: 4, expected: <F::Identity as ICanonicalField>::KIND })` whole,
///   asserted in place.
#[test]
fn transfer_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let result = build_transfer_proof_vector_description::<F>(Default::default())
                .fields_to_value(
                    build_from_fields_params(Default::default()),
                    self.fields.clone(),
                );

            // Assert
            assert_eq!(
                result,
                Err(TransferProofVectorFromFieldsErrorReturn::FieldKind {
                    index: 4,
                    expected: <F::Identity as ICanonicalField>::KIND,
                })
            );
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms");
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::Unsigned256([0x2A; 32]),
            CanonicalFieldValue::Unsigned256([0x31; 32]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder");
}
// transfer_proof_vector_description_refuses_the_lowest_field_of_the_wrong_kind

/// Contract: `fields_to_value`, count then kind: a list of the wrong count is
///   refused by the count branch before any kind is read.
/// Arrange: `build_canonical_fields` overriding `values` with the transfer
///   reference values without the last value and with the value at index `3`
///   replaced by `FixedBytes20([0x21; 20])`, a list both short and holding a
///   wrong kind.
/// Act:    `fields_to_value` on the description over the fields, inside the
///   consumer.
/// Assert: the result equals `Err(TransferProofVectorFromFieldsErrorReturn::FieldCount
///   { expected: 35, actual: 34 })` whole.
#[test]
fn transfer_proof_vector_description_reports_the_count_before_a_wrong_kind() {
    // Arrange
    struct ChainFormsConsumer {
        fields: CanonicalFields,
    }

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            build_transfer_proof_vector_description::<F>(Default::default()).fields_to_value(
                build_from_fields_params(Default::default()),
                self.fields.clone(),
            )
        }
    }

    struct EncodingConsumer {
        fields: CanonicalFields,
    }

    impl IEncodingConsumer for EncodingConsumer {
        type Output =
            FromFieldsReturn<TransferProofVector, TransferProofVectorFromFieldsErrorReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer {
                        fields: self.fields.clone(),
                    },
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms")
            .output
        }
    }

    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x01; 64]),
            CanonicalFieldValue::FixedBytes32([0x22; 32]),
            CanonicalFieldValue::Unsigned16(1),
            CanonicalFieldValue::FixedBytes20([0x21; 20]),
            CanonicalFieldValue::FixedBytes20([0x31; 20]),
            CanonicalFieldValue::FixedBytes32([0x01; 32]),
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

    // Act
    let result = create_encoding(
        &build_create_encoding_deps(EncodingConsumer { fields }),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(TransferProofVectorFromFieldsErrorReturn::FieldCount {
            expected: 35,
            actual: 34,
        })
    );
}
// transfer_proof_vector_description_reports_the_count_before_a_wrong_kind

/// Contract: `member_names`, every name admitted → `Ok` of the admitted names
///   in order, one for each index of `FIELDS`.
/// Arrange: the expected names, each from `build_solidity_member_name` over
///   `draws`, `suiteIdentifier`, `suiteVersion`, `chain`, `entitlementContract`,
///   `assetRoot`, `parameterSet`, `sourceEntitlement`, `targetEntitlement`,
///   `oldInterval`, `newInterval`, `purpose`, `seller`, `buyer`, `expiry`,
///   `identityElement`, `sellerPk1`, `sellerPk2`, `buyerPk1`, `buyerPk2`,
///   `oldEnvelopeC1`, `oldEnvelopeC2`, `oldEnvelopeD1`, `oldEnvelopeD2`,
///   `newEnvelopeC1`, `newEnvelopeC2`, `newEnvelopeD1`, `newEnvelopeD2`,
///   `challenge`, `x`, `y`, `s`, `rho`, `sigma`, and
///   `secondGroupFirstMessages`, written out.
/// Act:    `member_names()` on the description, inside the consumer.
/// Assert: the result equals `Ok` of the expected names whole, asserted in
///   place, and its length equals the length of `FIELDS`.
#[test]
fn transfer_proof_vector_description_declares_one_name_per_field() {
    // Arrange
    struct ChainFormsConsumer;

    impl IChainFormsConsumer for ChainFormsConsumer {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let expected = vec![
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("draws".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("suiteIdentifier".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("suiteVersion".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("chain".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("entitlementContract".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("assetRoot".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("parameterSet".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("sourceEntitlement".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("targetEntitlement".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("oldInterval".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("newInterval".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("purpose".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("seller".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("buyer".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("expiry".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("identityElement".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("sellerPk1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("sellerPk2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("buyerPk1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("buyerPk2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("oldEnvelopeC1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("oldEnvelopeC2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("oldEnvelopeD1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("oldEnvelopeD2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("newEnvelopeC1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("newEnvelopeC2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("newEnvelopeD1".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("newEnvelopeD2".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("challenge".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("x".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("y".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("s".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("rho".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("sigma".to_string()),
                }),
                build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                    text: Some("secondGroupFirstMessages".to_string()),
                }),
            ];

            // Act
            let result =
                build_transfer_proof_vector_description::<F>(Default::default()).member_names();

            // Assert
            assert_eq!(
                result.as_ref().map(Vec::len),
                Ok(TransferProofVectorDescription::<F>::FIELDS.len())
            );
            assert_eq!(result, Ok(expected));
        }
    }

    struct EncodingConsumer;

    impl IEncodingConsumer for EncodingConsumer {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            create_chain_forms(
                &build_create_chain_forms_deps(
                    ChainFormsConsumer,
                    &payload.adapter,
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
            .expect("the default selection constructs the chain forms");
        }
    }

    // Act
    create_encoding(
        &build_create_encoding_deps(EncodingConsumer),
        build_create_encoding_params(Default::default()),
        build_create_encoding_payload(Default::default()),
    )
    .expect("the default selection constructs the encoder");
}
// transfer_proof_vector_description_declares_one_name_per_field
