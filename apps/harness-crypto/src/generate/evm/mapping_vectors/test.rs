#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use domain::AssetIdentityHashTryNewErrorReturn;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, FromFieldsParamsOverrides,
    IEncodingContract, ToFieldsParamsOverrides, build_canonical_fields, build_from_fields_params,
    build_to_fields_params,
};
use envelope::{
    KeyAgreementDeclaration, KeyAgreementDeclarationOverrides, build_key_agreement_declaration,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DomainTagTryNewErrorReturn, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides, CreateKemReturn,
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    SetupErrorReturn, build_create_kem_deps, build_create_kem_params, build_create_kem_payload,
    create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, SampleUniformScalarErrorReturn, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::{
    DeliveryProofDeclaration, DeliveryProofDeclarationOverrides, build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesErrorReturn, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use super::interface::*;
use super::mapping_vectors;
use super::mock::*;

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose
///   `values` are `CanonicalFieldValue::Bytes` of a clone of `tag`,
///   `CanonicalFieldValue::Bytes` of a clone of `message`, and
///   `CanonicalFieldValue::Unsigned256` of `scalar`, in that order.
/// Arrange:  `build_hash_to_scalar_vector` with `tag` `vec![0x0A]`, `message`
///   `vec![0x0B, 0x0C]`, and `scalar` `[0x0D; 32]`,
///   `build_hash_to_scalar_vector_description(Default::default())`, and
///   `build_to_fields_params(Default::default())`.
/// Act:      `to_fields`.
/// Assert:   the values equal `CanonicalFieldValue::Bytes(vec![0x0A])`,
///   `CanonicalFieldValue::Bytes(vec![0x0B, 0x0C])`, and
///   `CanonicalFieldValue::Unsigned256([0x0D; 32])` in that order.
/// Collaborators: none — the description is a leaf.
#[test]
fn hash_to_scalar_vector_description_maps_tag_message_and_scalar_in_order() {
    // Arrange
    let vector = build_hash_to_scalar_vector(HashToScalarVectorOverrides {
        tag: Some(vec![0x0A]),
        message: Some(vec![0x0B, 0x0C]),
        scalar: Some([0x0D; 32]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());
    let params = build_to_fields_params(Default::default());

    // Act
    let Ok(success) = description.to_fields(params, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ]
    );
    // hash_to_scalar_vector_description_maps_tag_message_and_scalar_in_order
}

/// Contract: `payload.values` converts into the field array and each value is
///   the kind `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described: HashToScalarVector { tag,
///   message, scalar } })`, each moved from its field.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B, 0x0C])`, and `Unsigned256([0x0D; 32])`, the description
///   from `build_hash_to_scalar_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   `described` equals `build_hash_to_scalar_vector` with `tag`
///   `vec![0x0A]`, `message` `vec![0x0B, 0x0C]`, and `scalar` `[0x0D; 32]`.
/// Collaborators: none — the description is a leaf.
#[test]
fn hash_to_scalar_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let success = description
        .fields_to_value(params, fields)
        .expect("fields of the declared kinds rebuild the record");

    // Assert
    assert_eq!(
        success.described,
        build_hash_to_scalar_vector(HashToScalarVectorOverrides {
            tag: Some(vec![0x0A]),
            message: Some(vec![0x0B, 0x0C]),
            scalar: Some([0x0D; 32]),
        })
    );
    // hash_to_scalar_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(HashToScalarVectorFromFieldsErrorReturn::FieldCount { expected:
///   HASH_TO_SCALAR_VECTOR_FIELD_COUNT, actual })`.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])` and
///   `Bytes(vec![0x0B])`.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `HashToScalarVectorFromFieldsErrorReturn::
///   FieldCount { expected: 3, actual: 2 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn hash_to_scalar_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a wrong field count is refused");

    // Assert
    assert_eq!(
        error,
        HashToScalarVectorFromFieldsErrorReturn::FieldCount {
            expected: 3,
            actual: 2,
        }
    );
    // hash_to_scalar_vector_description_refuses_a_wrong_field_count
}

/// Contract: the lowest index `i` whose value is not the kind
///   `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`.
/// Arrange:  `build_canonical_fields` with `values`
///   `Unsigned256([0x0B; 32])`, `Bytes(vec![0x0C])`, and
///   `Bytes(vec![0x0D])`, so the indexes `0` and `2` both hold a wrong kind.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `HashToScalarVectorFromFieldsErrorReturn::
///   FieldKind { index: 0, expected: CanonicalFieldKind::Bytes }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn hash_to_scalar_vector_description_refuses_the_lowest_index_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x0B; 32]),
            CanonicalFieldValue::Bytes(vec![0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a field of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        HashToScalarVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Bytes,
        }
    );
    // hash_to_scalar_vector_description_refuses_the_lowest_index_of_the_wrong_kind
}

/// Contract: the lowest index `i` whose value is not the kind
///   `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`, `expected` that kind.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B])`, and `Bytes(vec![0x0D])`.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `HashToScalarVectorFromFieldsErrorReturn::
///   FieldKind { index: 2, expected: CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn hash_to_scalar_vector_description_refuses_a_byte_string_scalar() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a scalar of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        HashToScalarVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::Unsigned256,
        }
    );
    // hash_to_scalar_vector_description_refuses_a_byte_string_scalar
}

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose
///   `values` are `CanonicalFieldValue::Bytes` of a clone of `identity` and
///   `CanonicalFieldValue::Unsigned256` of `scalar`, in that order.
/// Arrange:  `build_identity_mapping_vector` with `identity` `vec![0x0E]` and
///   `scalar` `[0x0F; 32]`,
///   `build_identity_mapping_vector_description(Default::default())`, and
///   `build_to_fields_params(Default::default())`.
/// Act:      `to_fields`.
/// Assert:   the values equal `CanonicalFieldValue::Bytes(vec![0x0E])` and
///   `CanonicalFieldValue::Unsigned256([0x0F; 32])` in that order.
/// Collaborators: none — the description is a leaf.
#[test]
fn identity_mapping_vector_description_maps_identity_and_scalar_in_order() {
    // Arrange
    let vector = build_identity_mapping_vector(IdentityMappingVectorOverrides {
        identity: Some(vec![0x0E]),
        scalar: Some([0x0F; 32]),
    });
    let description = build_identity_mapping_vector_description(Default::default());
    let params = build_to_fields_params(Default::default());

    // Act
    let Ok(success) = description.to_fields(params, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0E]),
            CanonicalFieldValue::Unsigned256([0x0F; 32]),
        ]
    );
    // identity_mapping_vector_description_maps_identity_and_scalar_in_order
}

/// Contract: `payload.values` converts into the field array and each value is
///   the kind `IDENTITY_MAPPING_VECTOR_FIELD_KINDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described: IdentityMappingVector {
///   identity, scalar } })`, each moved from its field.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0E])` and
///   `Unsigned256([0x0F; 32])`, the description from
///   `build_identity_mapping_vector_description(Default::default())`, and
///   `build_from_fields_params(Default::default())`.
/// Act:      `fields_to_value`.
/// Assert:   `described` equals `build_identity_mapping_vector` with
///   `identity` `vec![0x0E]` and `scalar` `[0x0F; 32]`.
/// Collaborators: none — the description is a leaf.
#[test]
fn identity_mapping_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0E]),
            CanonicalFieldValue::Unsigned256([0x0F; 32]),
        ]),
    });
    let description = build_identity_mapping_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let success = description
        .fields_to_value(params, fields)
        .expect("fields of the declared kinds rebuild the record");

    // Assert
    assert_eq!(
        success.described,
        build_identity_mapping_vector(IdentityMappingVectorOverrides {
            identity: Some(vec![0x0E]),
            scalar: Some([0x0F; 32]),
        })
    );
    // identity_mapping_vector_description_rebuilds_the_record_from_its_fields
}

/// Contract: `payload.values` does not convert into the field array →
///   `Err(IdentityMappingVectorFromFieldsErrorReturn::FieldCount { expected:
///   IDENTITY_MAPPING_VECTOR_FIELD_COUNT, actual })`.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0E])`.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `IdentityMappingVectorFromFieldsErrorReturn::
///   FieldCount { expected: 2, actual: 1 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn identity_mapping_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![CanonicalFieldValue::Bytes(vec![0x0E])]),
    });
    let description = build_identity_mapping_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a wrong field count is refused");

    // Assert
    assert_eq!(
        error,
        IdentityMappingVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1,
        }
    );
    // identity_mapping_vector_description_refuses_a_wrong_field_count
}

/// Contract: the lowest index `i` whose value is not the kind
///   `IDENTITY_MAPPING_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`.
/// Arrange:  `build_canonical_fields` with `values`
///   `Unsigned256([0x0E; 32])` and `Bytes(vec![0x0F])`, so the indexes `0`
///   and `1` both hold a wrong kind.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `IdentityMappingVectorFromFieldsErrorReturn::
///   FieldKind { index: 0, expected: CanonicalFieldKind::Bytes }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn identity_mapping_vector_description_refuses_the_lowest_index_of_the_wrong_kind() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x0E; 32]),
            CanonicalFieldValue::Bytes(vec![0x0F]),
        ]),
    });
    let description = build_identity_mapping_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a field of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        IdentityMappingVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Bytes,
        }
    );
    // identity_mapping_vector_description_refuses_the_lowest_index_of_the_wrong_kind
}

/// Contract: the lowest index `i` whose value is not the kind
///   `IDENTITY_MAPPING_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind { index: i,
///   expected })`, `expected` that kind.
/// Arrange:  `build_canonical_fields` with `values` `Bytes(vec![0x0E])` and
///   `Bytes(vec![0x0F])`.
/// Act:      `fields_to_value`.
/// Assert:   `error` equals `IdentityMappingVectorFromFieldsErrorReturn::
///   FieldKind { index: 1, expected: CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: none — the description is a leaf.
#[test]
fn identity_mapping_vector_description_refuses_a_byte_string_scalar() {
    // Arrange
    let fields = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0E]),
            CanonicalFieldValue::Bytes(vec![0x0F]),
        ]),
    });
    let description = build_identity_mapping_vector_description(Default::default());
    let params = build_from_fields_params(Default::default());

    // Act
    let error = description
        .fields_to_value(params, fields)
        .err()
        .expect("a scalar of the wrong kind is refused");

    // Assert
    assert_eq!(
        error,
        IdentityMappingVectorFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Unsigned256,
        }
    );
    // identity_mapping_vector_description_refuses_a_byte_string_scalar
}

/// Contract: each name `SolidityMemberName::try_new` admits is paired with the
///   description's field kind at its index and
///   `SolidityRecordMembers::try_new` admits the members → `Ok` of the
///   members.
/// Arrange:  `build_hash_to_scalar_vector_description(Default::default())`.
/// Act:      `members()`.
/// Assert:   the result equals `Ok` of `build_solidity_record_members` over
///   `tag` and `message` of kind `CanonicalFieldKind::Bytes` and `scalar` of
///   kind `CanonicalFieldKind::Unsigned256`, each member through
///   `build_solidity_record_member` with its name from
///   `build_solidity_member_name`.
/// Collaborators: none — the description is a leaf.
#[test]
fn hash_to_scalar_vector_description_declares_tag_message_and_scalar_members() {
    // Arrange
    let description = build_hash_to_scalar_vector_description(Default::default());
    let expected = build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
        members: Some(vec![
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("tag".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("message".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("scalar".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned256),
            }),
        ]),
    });

    // Act
    let result = description.members();

    // Assert
    assert_eq!(result, Ok(expected));
    // hash_to_scalar_vector_description_declares_tag_message_and_scalar_members
}

/// Contract: each name `SolidityMemberName::try_new` admits is paired with the
///   description's field kind at its index and
///   `SolidityRecordMembers::try_new` admits the members → `Ok` of the
///   members.
/// Arrange:  `build_identity_mapping_vector_description(Default::default())`.
/// Act:      `members()`.
/// Assert:   the result equals `Ok` of `build_solidity_record_members` over
///   `identity` of kind `CanonicalFieldKind::Bytes` and `scalar` of kind
///   `CanonicalFieldKind::Unsigned256`, each member through
///   `build_solidity_record_member` with its name from
///   `build_solidity_member_name`.
/// Collaborators: none — the description is a leaf.
#[test]
fn identity_mapping_vector_description_declares_identity_and_scalar_members() {
    // Arrange
    let description = build_identity_mapping_vector_description(Default::default());
    let expected = build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
        members: Some(vec![
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("identity".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Bytes),
            }),
            build_solidity_record_member(SolidityRecordMemberOverrides {
                name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some("scalar".to_string()),
                    },
                )),
                kind: Some(CanonicalFieldKind::Unsigned256),
            }),
        ]),
    });

    // Act
    let result = description.members();

    // Assert
    assert_eq!(result, Ok(expected));
    // identity_mapping_vector_description_declares_identity_and_scalar_members
}

/// Contract: for each index below `params.hash_to_scalar_count.get()`, one
///   record per tag, the KEM's identity tag, `possession_g1_tag`,
///   `possession_g2_tag`, `challenge_tag`, then `weight_tag` →
///   `hash_to_scalar` holds the records message by message in that order.
/// Arrange:  `hash_to_scalar_count` from `build_vector_count` with `count`
///   `2`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `hash_to_scalar` holds `10` records, and the `tag` of the
///   records at indices `0` through `4` and `5` through `9` are
///   `b"ChainTorrent-v1-kem-identity-mock"`, `b"test-possession-g1"`,
///   `b"test-possession-g2"`, `b"test-challenge"`, and `b"test-weight"` in
///   that order.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_emit_one_record_per_declared_tag_for_each_message_in_tag_order() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    let success = result.expect("the configured count is admitted");
    assert_eq!(success.hash_to_scalar.len(), 10);
    let tags = [
        b"ChainTorrent-v1-kem-identity-mock".as_slice(),
        b"test-possession-g1".as_slice(),
        b"test-possession-g2".as_slice(),
        b"test-challenge".as_slice(),
        b"test-weight".as_slice(),
    ];
    for (index, record) in success.hash_to_scalar.iter().enumerate() {
        assert_eq!(record.tag.as_slice(), tags[index % 5]);
    }
    // mapping_vectors_emit_one_record_per_declared_tag_for_each_message_in_tag_order
}

/// Contract: each message is a draw of `params.message_length` bytes.
/// Arrange:  `hash_to_scalar_count` of `2` and `message_length` of `24`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   each `hash_to_scalar` record's `message` holds `24` bytes.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_draw_each_message_at_the_configured_length() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            message_length: Some(24),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    let success = result.expect("the configured length is drawn");
    for record in &success.hash_to_scalar {
        assert_eq!(record.message.len(), 24);
    }
    // mapping_vectors_draw_each_message_at_the_configured_length
}

/// Contract: one draw is made per index and every tag is mapped over it.
/// Arrange:  `hash_to_scalar_count` of `2` and `message_length` of `24`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   the records at indices `0` through `4` share one `message`, the
///   records at indices `5` through `9` share one `message`, and the two
///   messages differ.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_share_one_message_across_the_tags_of_a_draw_and_draw_a_new_one_for_the_next() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            message_length: Some(24),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    let success = result.expect("the configured count is admitted");
    for index in 1..5 {
        assert_eq!(
            success.hash_to_scalar[index].message,
            success.hash_to_scalar[0].message
        );
    }
    for index in 6..10 {
        assert_eq!(
            success.hash_to_scalar[index].message,
            success.hash_to_scalar[5].message
        );
    }
    assert_ne!(
        success.hash_to_scalar[0].message,
        success.hash_to_scalar[5].message
    );
    // mapping_vectors_share_one_message_across_the_tags_of_a_draw_and_draw_a_new_one_for_the_next
}

/// Contract: each record holds the scalar `hash_to_scalar` returns for its
///   tag and message.
/// Arrange:  `hash_to_scalar_count` of `2`, over the hash-to-scalar mock
///   concrete, whose scalar changes with the tag and the message.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   the `scalar` of the records at indices `0` through `4` are
///   pairwise different, and for each tag position `j` from `0` through `4`
///   the `scalar` at index `j` differs from the `scalar` at index `j + 5`.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_record_scalars_that_follow_their_tag_and_their_message() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    let success = result.expect("the configured count is admitted");
    for left in 0..5 {
        for right in (left + 1)..5 {
            assert_ne!(
                success.hash_to_scalar[left].scalar,
                success.hash_to_scalar[right].scalar
            );
        }
        assert_ne!(
            success.hash_to_scalar[left].scalar,
            success.hash_to_scalar[left + 5].scalar
        );
    }
    // mapping_vectors_record_scalars_that_follow_their_tag_and_their_message
}

/// Contract: under `IdentityScope::Entitlement` each identity's bytes are set
///   up and derived as `KemIdentity::Entitlement` → one
///   `IdentityMappingVector` per identity in order.
/// Arrange:  `CreateKemParamsOverrides` with `scope`
///   `Some(IdentityScope::Entitlement)` and `identities`
///   `vec![b"pkg@1.0.0#7".to_vec(), b"pkg@1.0.0#8".to_vec()]`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` is `Ok` and `identity_mapping` holds two records whose
///   `identity` are the arranged values in order.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_entitlement_scope()
 {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![b"pkg@1.0.0#7".to_vec(), b"pkg@1.0.0#8".to_vec()];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: Some(IdentityScope::Entitlement),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    let success = result.expect("the identities are admitted");
    assert_eq!(success.identity_mapping.len(), 2);
    assert_eq!(
        success.identity_mapping[0].identity,
        b"pkg@1.0.0#7".to_vec()
    );
    assert_eq!(
        success.identity_mapping[1].identity,
        b"pkg@1.0.0#8".to_vec()
    );
    // mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_entitlement_scope
}

/// Contract: under `IdentityScope::Asset` each identity's bytes are admitted
///   as an `AssetIdentityHash`, set up, and derived as `KemIdentity::Asset`
///   → one `IdentityMappingVector` per identity in order.
/// Arrange:  `CreateKemParamsOverrides` with `scope`
///   `Some(IdentityScope::Asset)` and `identities` `vec![vec![0x11; 32],
///   vec![0x22; 32]]`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` is `Ok` and `identity_mapping` holds two records whose
///   `identity` are the arranged values in order.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_asset_scope() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![vec![0x11; 32], vec![0x22; 32]];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: Some(IdentityScope::Asset),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    let success = result.expect("the identities are admitted");
    assert_eq!(success.identity_mapping.len(), 2);
    assert_eq!(success.identity_mapping[0].identity, vec![0x11; 32]);
    assert_eq!(success.identity_mapping[1].identity, vec![0x22; 32]);
    // mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_asset_scope
}

/// Contract: the identity element's scalar is a function of the tag and the
///   bytes alone, so the draws of each parameter set do not reach the record.
/// Arrange:  `scope` `Some(IdentityScope::Entitlement)` and `identities`
///   `vec![b"pkg@1.0.0#7".to_vec(), b"pkg@1.0.0#8".to_vec(),
///   b"pkg@1.0.0#7".to_vec()]`, so equal bytes are set up from different
///   draws.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   the `scalar` of the records at indices `0` and `2` are equal and
///   the `scalar` at index `0` differs from the `scalar` at index `1`.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_record_one_scalar_per_identity_bytes_whatever_the_setup_draws() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![
        b"pkg@1.0.0#7".to_vec(),
        b"pkg@1.0.0#8".to_vec(),
        b"pkg@1.0.0#7".to_vec(),
    ];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: Some(IdentityScope::Entitlement),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    let success = result.expect("the identities are admitted");
    assert_eq!(
        success.identity_mapping[0].scalar,
        success.identity_mapping[2].scalar
    );
    assert_ne!(
        success.identity_mapping[0].scalar,
        success.identity_mapping[1].scalar
    );
    // mapping_vectors_record_one_scalar_per_identity_bytes_whatever_the_setup_draws
}

/// Contract: `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(bytes)` fails →
///   `Err(MappingVectorsErrorReturn::AssetIdentityLength { index, actual })`.
/// Arrange:  `scope` `Some(IdentityScope::Asset)` and `identities`
///   `vec![vec![0x11; 32], vec![0x22; 31]]`, so the first identity is mapped
///   and the second is refused.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::
///   AssetIdentityLength { index: 1, actual: 31 })`.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_refuse_an_asset_identity_that_is_not_32_bytes() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![vec![0x11; 32], vec![0x22; 31]];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: Some(IdentityScope::Asset),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::AssetIdentityLength {
            index: 1,
            actual: 31,
        })
    );
    // mapping_vectors_refuse_an_asset_identity_that_is_not_32_bytes
}

/// Contract: `AssetIdentityHash::try_new` refuses →
///   `Err(MappingVectorsErrorReturn::AssetIdentityHash(error))` unchanged.
/// Arrange:  `scope` `Some(IdentityScope::Asset)` and `identities`
///   `vec![vec![0x00; 32]]`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::
///   AssetIdentityHash(AssetIdentityHashTryNewErrorReturn::AllZero))`.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_return_the_refusal_of_an_all_zero_asset_identity() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![vec![0x00; 32]];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: Some(IdentityScope::Asset),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::AssetIdentityHash(
            AssetIdentityHashTryNewErrorReturn::AllZero
        ))
    );
    // mapping_vectors_return_the_refusal_of_an_all_zero_asset_identity
}

/// Contract: `DomainTag::try_new` refuses a declared tag →
///   `Err(MappingVectorsErrorReturn::DomainTag(error))` unchanged.
/// Arrange:  `build_delivery_proof_declaration` with `weight_tag`
///   `Some(b"")`, the last tag, so a function that admits only the first
///   tags fails.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::DomainTag(
///   DomainTagTryNewErrorReturn::Empty))`.
/// Collaborators: the random source's, pairing's, both hash-to-scalar's, and
///   KEM's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`, and
///   `create_kem`.
#[test]
fn mapping_vectors_return_the_refusal_of_a_declared_tag_unchanged() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::DomainTag(
            DomainTagTryNewErrorReturn::Empty
        ))
    );
    // mapping_vectors_return_the_refusal_of_a_declared_tag_unchanged
}

/// Contract: the tags are admitted before any draw → a refused tag is
///   returned though every draw would be refused.
/// Arrange:  `weight_tag` `Some(b"")` as above and the random source with
///   `kind` `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::DomainTag(
///   DomainTagTryNewErrorReturn::Empty))`.
/// Collaborators: the random source's mock concrete in its `FillBytesRefused`
///   mode through `create_random_source`, and the pairing's, both
///   hash-to-scalar's, and KEM's mock concretes in their succeeding modes
///   through `create_pairing`, `create_hash_to_scalar`, and `create_kem`.
#[test]
fn mapping_vectors_refuse_a_declared_tag_before_any_draw() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
            )),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::DomainTag(
            DomainTagTryNewErrorReturn::Empty
        ))
    );
    // mapping_vectors_refuse_a_declared_tag_before_any_draw
}

/// Contract: `deps.random.fill_bytes` fails →
///   `Err(MappingVectorsErrorReturn::FillBytes(error))` unchanged.
/// Arrange:  the random source with `kind` `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, a mode that
///   differs from the builder's default `Succeeds`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))`.
/// Collaborators: the random source's mock concrete in its `FillBytesRefused`
///   mode through `create_random_source`, and the pairing's, both
///   hash-to-scalar's, and KEM's mock concretes in their succeeding modes
///   through `create_pairing`, `create_hash_to_scalar`, and `create_kem`.
#[test]
fn mapping_vectors_return_the_fill_refusal_unchanged() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
            )),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
    // mapping_vectors_return_the_fill_refusal_unchanged
}

/// Contract: `deps.hash_to_scalar.hash_to_scalar` fails →
///   `Err(MappingVectorsErrorReturn::HashToScalar(error))` unchanged.
/// Arrange:  `hashing` the adapter of `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: 16, actual: 0 })))`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`,
///   the subject hash-to-scalar's mock concrete in its `SamplingRefused`
///   mode and the KEM hash-to-scalar's in its succeeding mode through
///   `create_hash_to_scalar`, and the KEM's mock concrete in its succeeding
///   mode through `create_kem`.
#[test]
fn mapping_vectors_return_the_hash_refusal_unchanged() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities: Vec<Vec<u8>> = Vec::new();

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        kem_hashing_concrete: None,
        kem_scope: None,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::HashToScalar(
            HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 0,
            })
        ))
    );
    // mapping_vectors_return_the_hash_refusal_unchanged
}

/// Contract: `deps.kem.setup` fails → `Err(MappingVectorsErrorReturn::
///   Setup(error))` unchanged.
/// Arrange:  `scope` `Some(IdentityScope::Asset)`, `identities`
///   `vec![vec![0x11; 32]]`, `kem_hashing` the refusing adapter and `hashing`
///   the succeeding one, so the hash-to-scalar records succeed and the
///   KEM's asset setup is refused.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::Setup(
///   SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 0
///   }))))`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`,
///   the subject hash-to-scalar's mock concrete in its succeeding mode and
///   the KEM hash-to-scalar's in its `SamplingRefused` mode through
///   `create_hash_to_scalar`, and the KEM's mock concrete in its succeeding
///   mode through `create_kem`.
#[test]
fn mapping_vectors_return_the_setup_refusal_unchanged_under_the_asset_scope() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![vec![0x11; 32]];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        kem_scope: Some(IdentityScope::Asset),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::Setup(
            SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 16,
                    actual: 0,
                }
            ))
        ))
    );
    // mapping_vectors_return_the_setup_refusal_unchanged_under_the_asset_scope
}

/// Contract: `deps.kem.derive_identity` fails →
///   `Err(MappingVectorsErrorReturn::DeriveIdentity(error))` unchanged.
/// Arrange:  `scope` `Some(IdentityScope::Entitlement)`, `identities`
///   `vec![b"pkg@1.0.0#7".to_vec()]`, `kem_hashing` the refusing adapter and
///   `hashing` the succeeding one, so the setup succeeds and the derivation
///   is refused.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::
///   DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: 16, actual: 0 }))))`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`,
///   the subject hash-to-scalar's mock concrete in its succeeding mode and
///   the KEM hash-to-scalar's in its `SamplingRefused` mode through
///   `create_hash_to_scalar`, and the KEM's mock concrete in its succeeding
///   mode through `create_kem`.
#[test]
fn mapping_vectors_return_the_derivation_refusal_unchanged_under_the_entitlement_scope() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![b"pkg@1.0.0#7".to_vec()];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: None,
        kem_hashing_concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        kem_scope: Some(IdentityScope::Entitlement),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::DeriveIdentity(
            DeriveIdentityErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 16,
                    actual: 0,
                }
            ))
        ))
    );
    // mapping_vectors_return_the_derivation_refusal_unchanged_under_the_entitlement_scope
}

/// Contract: the hash-to-scalar records precede the identity-mapping
///   records and the first refusal is returned.
/// Arrange:  `hashing` the refusing adapter, `scope`
///   `Some(IdentityScope::Asset)`, and `identities` `vec![vec![0x11; 31]]`,
///   an identity the asset scope would refuse.
/// Act:      `mapping_vectors` in the KEM consumer.
/// Assert:   `result` equals `Err(MappingVectorsErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: 16, actual: 0 })))`.
/// Collaborators: the random source's and pairing's mock concretes in their
///   succeeding modes through `create_random_source` and `create_pairing`,
///   the subject hash-to-scalar's mock concrete in its `SamplingRefused`
///   mode and the KEM hash-to-scalar's in its succeeding mode through
///   `create_hash_to_scalar`, and the KEM's mock concrete in its succeeding
///   mode through `create_kem`.
#[test]
fn mapping_vectors_return_the_hash_refusal_before_mapping_any_identity() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides { kind: None }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params");
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1".as_slice()),
        possession_g2_tag: Some(b"test-possession-g2".as_slice()),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge".as_slice()),
        weight_tag: Some(b"test-weight".as_slice()),
        ..Default::default()
    });
    let identities = vec![vec![0x11; 31]];

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = MappingVectorsReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                    scope: Some(payload.scope()),
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    self.identities,
                    Default::default(),
                ),
            )
        }
    }

    struct PairingConsumer<'a> {
        hashing_concrete: Option<HashToScalarConcrete>,
        kem_hashing_concrete: Option<HashToScalarConcrete>,
        kem_scope: Option<IdentityScope>,
        params_overrides: MappingVectorsParamsOverrides,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        identities: &'a [Vec<u8>],
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = CreateKemReturn<MappingVectorsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the subject's hash-to-scalar adapter is created")
            };
            let kem_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.kem_hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(kem_hashing) = kem_hashing else {
                panic!("the KEM's hash-to-scalar adapter is created")
            };
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*kem_hashing.adapter,
                    KemConsumer {
                        pairing: &payload.adapter,
                        hash_to_scalar: &*hashing.adapter,
                        random: self.random.as_ref(),
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        identities: self.identities,
                        params_overrides: &self.params_overrides,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: self.kem_scope,
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        hashing_concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        kem_hashing_concrete: None,
        kem_scope: Some(IdentityScope::Asset),
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        identities: &identities,
        random: source.adapter,
    };

    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );
    let result = created
        .expect("the pairing factory ran the consumer")
        .output
        .expect("the KEM factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(MappingVectorsErrorReturn::HashToScalar(
            HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 0,
            })
        ))
    );
    // mapping_vectors_return_the_hash_refusal_before_mapping_any_identity
}
