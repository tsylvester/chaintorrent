#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFieldsOverrides, IEncodingContract,
    build_canonical_fields, build_from_fields_params, build_to_fields_params,
};
use envelope::{
    KeyAgreementDeclaration, KeyAgreementDeclarationOverrides, build_key_agreement_declaration,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DomainTagTryNewErrorReturn, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope,
    SetupErrorReturn, build_create_kem_params,
};
use pairing::{IPairingArithmetic, IPairingReference, SampleUniformScalarErrorReturn};
use proof::{
    DeliveryProofDeclaration, DeliveryProofDeclarationOverrides, build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::interface::{
    HashToScalarVectorFromFieldsErrorReturn, IMappingVectorsCaseConsumer,
    IdentityMappingVectorFromFieldsErrorReturn, MappingVectorsErrorReturn, MappingVectorsReturn,
};
use super::mapping_vectors;
use super::mock::{
    HashToScalarVectorOverrides, IdentityMappingVectorOverrides, MappingVectorsCaseParamsOverrides,
    MappingVectorsParamsOverrides, build_hash_to_scalar_vector,
    build_hash_to_scalar_vector_description, build_identity_mapping_vector,
    build_identity_mapping_vector_description, build_mapping_vectors_case_deps,
    build_mapping_vectors_case_params, build_mapping_vectors_case_payload,
    build_mapping_vectors_deps, build_mapping_vectors_params, build_mapping_vectors_payload,
    run_mapping_vectors_case,
};
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose `values` are
///   `CanonicalFieldValue::Bytes` of a clone of `tag`, `CanonicalFieldValue::Bytes` of a
///   clone of `message`, and `CanonicalFieldValue::Unsigned256` of `scalar`, in that order.
/// Arrange: `build_hash_to_scalar_vector` with `tag` `vec![0x0A]`, `message`
///   `vec![0x0B, 0x0C]`, and `scalar` `[0x0D; 32]`, and
///   `build_hash_to_scalar_vector_description(Default::default())`.
/// Act:    `to_fields` over `build_to_fields_params(Default::default())` and the record.
/// Assert: `fields` equals `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B, 0x0C])`, and `Unsigned256([0x0D; 32])` in that order.
#[test]
fn hash_to_scalar_vector_description_maps_tag_message_and_scalar_in_order() {
    // Arrange
    let vector = build_hash_to_scalar_vector(HashToScalarVectorOverrides {
        tag: Some(vec![0x0A]),
        message: Some(vec![0x0B, 0x0C]),
        scalar: Some([0x0D; 32]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());

    // Act
    let fields = description
        .to_fields(build_to_fields_params(Default::default()), &vector)
        .expect("the description maps the record")
        .fields;

    // Assert
    assert_eq!(
        fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Bytes(vec![0x0A]),
                CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
                CanonicalFieldValue::Unsigned256([0x0D; 32]),
            ]),
        })
    );
}
// hash_to_scalar_vector_description_maps_tag_message_and_scalar_in_order

/// Contract: `payload.values` converts into the field array and each value is the kind
///   `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described: HashToScalarVector { tag, message,
///   scalar } })`.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B, 0x0C])`, and `Unsigned256([0x0D; 32])`, and
///   `build_hash_to_scalar_vector_description(Default::default())`.
/// Act:    `fields_to_value` over `build_from_fields_params(Default::default())`.
/// Assert: `described` equals `build_hash_to_scalar_vector` with `tag` `vec![0x0A]`,
///   `message` `vec![0x0B, 0x0C]`, and `scalar` `[0x0D; 32]`.
#[test]
fn hash_to_scalar_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());

    // Act
    let described = description
        .fields_to_value(build_from_fields_params(Default::default()), payload)
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(
        described,
        build_hash_to_scalar_vector(HashToScalarVectorOverrides {
            tag: Some(vec![0x0A]),
            message: Some(vec![0x0B, 0x0C]),
            scalar: Some([0x0D; 32]),
        })
    );
}
// hash_to_scalar_vector_description_rebuilds_the_record_from_its_fields

/// Contract: `payload.values` does not convert into the field array →
///   `Err(HashToScalarVectorFromFieldsErrorReturn::FieldCount { expected, actual })`.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0A])` and
///   `Bytes(vec![0x0B])`.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldCount { expected: 3, actual: 2 }`.
#[test]
fn hash_to_scalar_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(HashToScalarVectorFromFieldsErrorReturn::FieldCount {
            expected: 3,
            actual: 2,
        })
    );
}
// hash_to_scalar_vector_description_refuses_a_wrong_field_count

/// Contract: the lowest index `i` whose value is not the kind
///   `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`.
/// Arrange: `build_canonical_fields` with `values` `Unsigned256([0x0B; 32])`,
///   `Bytes(vec![0x0C])`, and `Bytes(vec![0x0D])`, so the indexes `0` and `2` both
///   hold a wrong kind.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldKind { index: 0, expected: CanonicalFieldKind::Bytes }`.
#[test]
fn hash_to_scalar_vector_description_refuses_the_lowest_index_of_the_wrong_kind() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x0B; 32]),
            CanonicalFieldValue::Bytes(vec![0x0C]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Bytes,
        })
    );
}
// hash_to_scalar_vector_description_refuses_the_lowest_index_of_the_wrong_kind

/// Contract: the lowest index `i` whose value is not the kind
///   `HASH_TO_SCALAR_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0A])`,
///   `Bytes(vec![0x0B])`, and `Bytes(vec![0x0D])`.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldKind { index: 2, expected:
///   CanonicalFieldKind::Unsigned256 }`.
#[test]
fn hash_to_scalar_vector_description_refuses_a_byte_string_scalar() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ]),
    });
    let description = build_hash_to_scalar_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(HashToScalarVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::Unsigned256,
        })
    );
}
// hash_to_scalar_vector_description_refuses_a_byte_string_scalar

/// Contract: `to_fields` → `Ok(ToFieldsSuccessReturn { fields })` whose `values` are
///   `CanonicalFieldValue::Bytes` of a clone of `identity` and
///   `CanonicalFieldValue::Unsigned256` of `scalar`, in that order.
/// Arrange: `build_identity_mapping_vector` with `identity` `vec![0x0E]` and `scalar`
///   `[0x0F; 32]`, and `build_identity_mapping_vector_description(Default::default())`.
/// Act:    `to_fields` over `build_to_fields_params(Default::default())` and the record.
/// Assert: `fields` equals `build_canonical_fields` with `values` `Bytes(vec![0x0E])`
///   and `Unsigned256([0x0F; 32])` in that order.
#[test]
fn identity_mapping_vector_description_maps_identity_and_scalar_in_order() {
    // Arrange
    let vector = build_identity_mapping_vector(IdentityMappingVectorOverrides {
        identity: Some(vec![0x0E]),
        scalar: Some([0x0F; 32]),
    });
    let description = build_identity_mapping_vector_description(Default::default());

    // Act
    let fields = description
        .to_fields(build_to_fields_params(Default::default()), &vector)
        .expect("the description maps the record")
        .fields;

    // Assert
    assert_eq!(
        fields,
        build_canonical_fields(CanonicalFieldsOverrides {
            values: Some(vec![
                CanonicalFieldValue::Bytes(vec![0x0E]),
                CanonicalFieldValue::Unsigned256([0x0F; 32]),
            ]),
        })
    );
}
// identity_mapping_vector_description_maps_identity_and_scalar_in_order

/// Contract: `payload.values` converts into the field array and each value is the kind
///   `IDENTITY_MAPPING_VECTOR_FIELD_KINDS` holds at its index →
///   `Ok(FromFieldsSuccessReturn { described: IdentityMappingVector { identity,
///   scalar } })`.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0E])` and
///   `Unsigned256([0x0F; 32])`, and
///   `build_identity_mapping_vector_description(Default::default())`.
/// Act:    `fields_to_value` over `build_from_fields_params(Default::default())`.
/// Assert: `described` equals `build_identity_mapping_vector` with `identity`
///   `vec![0x0E]` and `scalar` `[0x0F; 32]`.
#[test]
fn identity_mapping_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0E]),
            CanonicalFieldValue::Unsigned256([0x0F; 32]),
        ]),
    });
    let description = build_identity_mapping_vector_description(Default::default());

    // Act
    let described = description
        .fields_to_value(build_from_fields_params(Default::default()), payload)
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(
        described,
        build_identity_mapping_vector(IdentityMappingVectorOverrides {
            identity: Some(vec![0x0E]),
            scalar: Some([0x0F; 32]),
        })
    );
}
// identity_mapping_vector_description_rebuilds_the_record_from_its_fields

/// Contract: `payload.values` does not convert into the field array →
///   `Err(IdentityMappingVectorFromFieldsErrorReturn::FieldCount { expected, actual })`.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0E])`.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldCount { expected: 2, actual: 1 }`.
#[test]
fn identity_mapping_vector_description_refuses_a_wrong_field_count() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![CanonicalFieldValue::Bytes(vec![0x0E])]),
    });
    let description = build_identity_mapping_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(IdentityMappingVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1,
        })
    );
}
// identity_mapping_vector_description_refuses_a_wrong_field_count

/// Contract: the lowest index `i` whose value is not the kind
///   `IDENTITY_MAPPING_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`.
/// Arrange: `build_canonical_fields` with `values` `Unsigned256([0x0E; 32])` and
///   `Bytes(vec![0x0F])`, so the indexes `0` and `1` both hold a wrong kind.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldKind { index: 0, expected: CanonicalFieldKind::Bytes }`.
#[test]
fn identity_mapping_vector_description_refuses_the_lowest_index_of_the_wrong_kind() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Unsigned256([0x0E; 32]),
            CanonicalFieldValue::Bytes(vec![0x0F]),
        ]),
    });
    let description = build_identity_mapping_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Bytes,
        })
    );
}
// identity_mapping_vector_description_refuses_the_lowest_index_of_the_wrong_kind

/// Contract: the lowest index `i` whose value is not the kind
///   `IDENTITY_MAPPING_VECTOR_FIELD_KINDS` holds at `i` →
///   `Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind { index: i, expected })`.
/// Arrange: `build_canonical_fields` with `values` `Bytes(vec![0x0E])` and
///   `Bytes(vec![0x0F])`.
/// Act:    `fields_to_value`.
/// Assert: the error equals `FieldKind { index: 1, expected:
///   CanonicalFieldKind::Unsigned256 }`.
#[test]
fn identity_mapping_vector_description_refuses_a_byte_string_scalar() {
    // Arrange
    let payload = build_canonical_fields(CanonicalFieldsOverrides {
        values: Some(vec![
            CanonicalFieldValue::Bytes(vec![0x0E]),
            CanonicalFieldValue::Bytes(vec![0x0F]),
        ]),
    });
    let description = build_identity_mapping_vector_description(Default::default());

    // Act
    let result = description.fields_to_value(build_from_fields_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result,
        Err(IdentityMappingVectorFromFieldsErrorReturn::FieldKind {
            index: 1,
            expected: CanonicalFieldKind::Unsigned256,
        })
    );
}
// identity_mapping_vector_description_refuses_a_byte_string_scalar

/// Contract: each name in `HASH_TO_SCALAR_VECTOR_MEMBER_NAMES` that
///   `SolidityMemberName::try_new` admits → `Ok` of the names in order.
/// Arrange: `build_hash_to_scalar_vector_description(Default::default())`.
/// Act:    `member_names()`.
/// Assert: the result equals `Ok` of the names `tag`, `message`, and `scalar`, each
///   through `build_solidity_member_name`.
#[test]
fn hash_to_scalar_vector_description_declares_tag_message_and_scalar_member_names() {
    // Arrange
    let description = build_hash_to_scalar_vector_description(Default::default());

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(
        result,
        Ok(vec![
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("tag".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("message".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("scalar".to_string()),
            }),
        ])
    );
}
// hash_to_scalar_vector_description_declares_tag_message_and_scalar_member_names

/// Contract: each name in `IDENTITY_MAPPING_VECTOR_MEMBER_NAMES` that
///   `SolidityMemberName::try_new` admits → `Ok` of the names in order.
/// Arrange: `build_identity_mapping_vector_description(Default::default())`.
/// Act:    `member_names()`.
/// Assert: the result equals `Ok` of the names `identity` and `scalar`, each through
///   `build_solidity_member_name`.
#[test]
fn identity_mapping_vector_description_declares_identity_and_scalar_member_names() {
    // Arrange
    let description = build_identity_mapping_vector_description(Default::default());

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(
        result,
        Ok(vec![
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("identity".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("scalar".to_string()),
            }),
        ])
    );
}
// identity_mapping_vector_description_declares_identity_and_scalar_member_names

/// Contract: for each index below `params.hash_to_scalar_count.get()`, one record per
///   tag — the KEM's identity tag, `possession_g1_tag`, `possession_g2_tag`,
///   `challenge_tag`, then `weight_tag` → `hash_to_scalar` holds the records message by
///   message in that order.
/// Arrange: `hash_to_scalar_count` from `build_vector_count` with `count` `2`; the
///   declarations built with the distinct test tags.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `hash_to_scalar` holds `10` records, and the `tag` of the records at indices
///   `0` through `4` and `5` through `9` are `b"ChainTorrent-v1-kem-identity-mock"`,
///   `b"test-possession-g1"`, `b"test-possession-g2"`, `b"test-challenge"`, and
///   `b"test-weight"` in that order.
#[test]
fn mapping_vectors_emit_one_record_per_declared_tag_for_each_message_in_tag_order() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(Default::default()),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the mapping succeeds");
    assert_eq!(success.hash_to_scalar.len(), 10);
    let tags: Vec<&[u8]> = success
        .hash_to_scalar
        .iter()
        .map(|record| record.tag.as_slice())
        .collect();
    assert_eq!(
        tags,
        vec![
            b"ChainTorrent-v1-kem-identity-mock".as_slice(),
            b"test-possession-g1".as_slice(),
            b"test-possession-g2".as_slice(),
            b"test-challenge".as_slice(),
            b"test-weight".as_slice(),
            b"ChainTorrent-v1-kem-identity-mock".as_slice(),
            b"test-possession-g1".as_slice(),
            b"test-possession-g2".as_slice(),
            b"test-challenge".as_slice(),
            b"test-weight".as_slice(),
        ]
    );
}
// mapping_vectors_emit_one_record_per_declared_tag_for_each_message_in_tag_order

/// Contract: each message is a draw of `params.message_length` bytes.
/// Arrange: `hash_to_scalar_count` of `2` and `message_length` of `24`.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: each `hash_to_scalar` record's `message` holds `24` bytes.
#[test]
fn mapping_vectors_draw_each_message_at_the_configured_length() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            message_length: Some(24),
        },
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(Default::default()),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the mapping succeeds");
    assert!(
        success
            .hash_to_scalar
            .iter()
            .all(|record| record.message.len() == 24)
    );
}
// mapping_vectors_draw_each_message_at_the_configured_length

/// Contract: one draw is made per index and every tag is mapped over it.
/// Arrange: `hash_to_scalar_count` of `2` and `message_length` of `24`.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: the records at indices `0` through `4` share one `message`, the records at
///   indices `5` through `9` share one `message`, and the two messages differ.
#[test]
fn mapping_vectors_share_one_message_across_the_tags_of_a_draw_and_draw_a_new_one_for_the_next() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            message_length: Some(24),
        },
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(Default::default()),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the mapping succeeds");
    let first = &success.hash_to_scalar[0].message;
    let second = &success.hash_to_scalar[5].message;
    assert!(
        success.hash_to_scalar[0..5]
            .iter()
            .all(|record| &record.message == first)
    );
    assert!(
        success.hash_to_scalar[5..10]
            .iter()
            .all(|record| &record.message == second)
    );
    assert_ne!(first, second);
}
// mapping_vectors_share_one_message_across_the_tags_of_a_draw_and_draw_a_new_one_for_the_next

/// Contract: each record holds the scalar `hash_to_scalar` returns for its tag and
///   message.
/// Arrange: `hash_to_scalar_count` of `2`, over the hash-to-scalar mock concrete, whose
///   scalar changes with the tag and the message.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: the `scalar` of the records at indices `0` through `4` are pairwise
///   different, and for each tag position `j` from `0` through `4` the `scalar` at
///   index `j` differs from the `scalar` at index `j + 5`.
#[test]
fn mapping_vectors_record_scalars_that_follow_their_tag_and_their_message() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            ..Default::default()
        },
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(Default::default()),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the mapping succeeds");
    for i in 0..5 {
        for j in (i + 1)..5 {
            assert_ne!(
                success.hash_to_scalar[i].scalar,
                success.hash_to_scalar[j].scalar
            );
        }
        assert_ne!(
            success.hash_to_scalar[i].scalar,
            success.hash_to_scalar[i + 5].scalar
        );
    }
}
// mapping_vectors_record_scalars_that_follow_their_tag_and_their_message

/// Contract: each admitted identity of the entitlement form is set up under its
///   `SetupScope::Entitlement` and derived as its `KemIdentity::Entitlement` → one
///   `IdentityMappingVector` per identity in order, holding its canonical bytes.
/// Arrange: the case's `kem` selection `build_create_kem_params` with `scope`
///   `IdentityScope::Entitlement`, and the case payload's `canonicals`
///   `b"pkg@1.0.0#7"` and `b"pkg@1.0.0#8"`, the two distinct entitlement identities.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` is `Ok` and `identity_mapping` holds two records whose `identity`
///   are `b"pkg@1.0.0#7".to_vec()` and `b"pkg@1.0.0#8".to_vec()` in order.
#[test]
fn mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_entitlement_scope()
 {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 2] = [b"pkg@1.0.0#7", b"pkg@1.0.0#8"];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Entitlement),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the mapping succeeds");
    assert_eq!(success.identity_mapping.len(), 2);
    assert_eq!(
        success.identity_mapping[0].identity,
        b"pkg@1.0.0#7".to_vec()
    );
    assert_eq!(
        success.identity_mapping[1].identity,
        b"pkg@1.0.0#8".to_vec()
    );
}
// mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_entitlement_scope

/// Contract: each admitted identity of the asset form is set up under its
///   `SetupScope::Asset` and derived as its `KemIdentity::Asset` → one
///   `IdentityMappingVector` per identity in order, holding its hash's bytes.
/// Arrange: the case's `kem` selection `build_create_kem_params` with `scope`
///   `IdentityScope::Asset`, and the case payload's `roots` the two `build_root`
///   builds with `bytes` `[0x11; 32]` and `[0x22; 32]`, the two distinct asset hashes.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` is `Ok` and `identity_mapping` holds two records whose `identity`
///   are `vec![0x11; 32]` and `vec![0x22; 32]` in order.
#[test]
fn mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_asset_scope() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    let root_a = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let root_b = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x22; 32]),
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 0] = [];
    let roots = [&root_a, &root_b];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the mapping succeeds");
    assert_eq!(success.identity_mapping.len(), 2);
    assert_eq!(success.identity_mapping[0].identity, vec![0x11; 32]);
    assert_eq!(success.identity_mapping[1].identity, vec![0x22; 32]);
}
// mapping_vectors_emit_one_identity_record_per_identity_in_payload_order_under_the_asset_scope

/// Contract: the identity element's scalar is a function of the tag and the bytes
///   alone, so the draws of each parameter set do not reach the record.
/// Arrange: the case's `kem` selection `build_create_kem_params` with `scope`
///   `IdentityScope::Entitlement`, and the case payload's `canonicals`
///   `b"pkg@1.0.0#7"`, `b"pkg@1.0.0#8"`, and `b"pkg@1.0.0#7"`, so equal bytes are set
///   up from different draws.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: the `scalar` of the records at indices `0` and `2` are equal and the
///   `scalar` at index `0` differs from the `scalar` at index `1`.
#[test]
fn mapping_vectors_record_one_scalar_per_identity_bytes_whatever_the_setup_draws() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 3] = [b"pkg@1.0.0#7", b"pkg@1.0.0#8", b"pkg@1.0.0#7"];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Entitlement),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    let success = output.expect("the mapping succeeds");
    assert_eq!(
        success.identity_mapping[0].scalar,
        success.identity_mapping[2].scalar
    );
    assert_ne!(
        success.identity_mapping[0].scalar,
        success.identity_mapping[1].scalar
    );
}
// mapping_vectors_record_one_scalar_per_identity_bytes_whatever_the_setup_draws

/// Contract: `DomainTag::try_new` refuses a declared tag →
///   `Err(MappingVectorsErrorReturn::DomainTag(error))`, unchanged.
/// Arrange: `build_delivery_proof_declaration` with `weight_tag` `Some(b"")`, the last
///   tag, so a function that admits only the first tags fails.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` equals
///   `Err(MappingVectorsErrorReturn::DomainTag(DomainTagTryNewErrorReturn::Empty))`.
#[test]
fn mapping_vectors_return_the_refusal_of_a_declared_tag_unchanged() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b""),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(Default::default()),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(MappingVectorsErrorReturn::DomainTag(
            DomainTagTryNewErrorReturn::Empty
        ))
    );
}
// mapping_vectors_return_the_refusal_of_a_declared_tag_unchanged

/// Contract: the tags are admitted before any draw → a refused tag is returned though
///   every draw would be refused.
/// Arrange: `weight_tag` `Some(b"")` as above and the `random_source` selection with
///   `kind` `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` equals
///   `Err(MappingVectorsErrorReturn::DomainTag(DomainTagTryNewErrorReturn::Empty))`.
#[test]
fn mapping_vectors_refuse_a_declared_tag_before_any_draw() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b""),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(MappingVectorsErrorReturn::DomainTag(
            DomainTagTryNewErrorReturn::Empty
        ))
    );
}
// mapping_vectors_refuse_a_declared_tag_before_any_draw

/// Contract: `deps.random.fill_bytes` fails →
///   `Err(MappingVectorsErrorReturn::FillBytes(error))`, unchanged.
/// Arrange: the `random_source` selection with `kind` `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, a mode that differs
///   from the builder's default `Succeeds`.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` equals `Err(MappingVectorsErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))`.
#[test]
fn mapping_vectors_return_the_fill_refusal_unchanged() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(MappingVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
}
// mapping_vectors_return_the_fill_refusal_unchanged

/// Contract: `deps.hash_to_scalar.hash_to_scalar` fails →
///   `Err(MappingVectorsErrorReturn::HashToScalar(error))`, unchanged.
/// Arrange: the `hash_to_scalar` selection's `concrete` `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)`.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` equals `Err(MappingVectorsErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
///   expected: 16, actual: 0 })))`.
#[test]
fn mapping_vectors_return_the_hash_refusal_unchanged() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 0] = [];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(MappingVectorsErrorReturn::HashToScalar(
            HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 0,
            })
        ))
    );
}
// mapping_vectors_return_the_hash_refusal_unchanged

/// Contract: `deps.kem.setup` fails → `Err(MappingVectorsErrorReturn::Setup(error))`,
///   unchanged.
/// Arrange: the case's `kem` selection `build_create_kem_params` with `scope`
///   `IdentityScope::Asset`, the case payload's `roots` the one `build_root` build
///   with `bytes` `[0x11; 32]`, the `kem_hash_to_scalar` selection the refusing
///   adapter's and `hash_to_scalar` the succeeding one, so the hash-to-scalar
///   records succeed and the KEM's asset setup is refused.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` equals `Err(MappingVectorsErrorReturn::Setup(
///   SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 0 }))))`.
#[test]
fn mapping_vectors_return_the_setup_refusal_unchanged_under_the_asset_scope() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 0] = [];
    let roots = [&root];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(Default::default())),
            kem_hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(MappingVectorsErrorReturn::Setup(
            SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 16,
                    actual: 0,
                }
            ))
        ))
    );
}
// mapping_vectors_return_the_setup_refusal_unchanged_under_the_asset_scope

/// Contract: `deps.kem.derive_identity` fails →
///   `Err(MappingVectorsErrorReturn::DeriveIdentity(error))`, unchanged.
/// Arrange: the case's `kem` selection `build_create_kem_params` with `scope`
///   `IdentityScope::Entitlement`, the case payload's `canonicals`
///   `b"pkg@1.0.0#7"`, the `kem_hash_to_scalar` selection the refusing adapter's and
///   `hash_to_scalar` the succeeding one, so the setup succeeds and the derivation is
///   refused.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` equals `Err(MappingVectorsErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual: 0 }))))`.
#[test]
fn mapping_vectors_return_the_derivation_refusal_unchanged_under_the_entitlement_scope() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 1] = [b"pkg@1.0.0#7"];
    let roots: [&Root; 0] = [];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(Default::default())),
            kem_hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Entitlement),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(MappingVectorsErrorReturn::DeriveIdentity(
            DeriveIdentityErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 16,
                    actual: 0,
                }
            ))
        ))
    );
}
// mapping_vectors_return_the_derivation_refusal_unchanged_under_the_entitlement_scope

/// Contract: the hash-to-scalar records precede the identity-mapping records and the
///   first refusal is returned.
/// Arrange: the `hash_to_scalar` and `kem_hash_to_scalar` selections both the
///   refusing adapter's, the case's `kem` selection `build_create_kem_params` with
///   `scope` `IdentityScope::Asset`, and the case payload's `roots` the one
///   `build_root` build with `bytes` `[0x11; 32]`, an identity whose setup the
///   refusing KEM would refuse.
/// Act:    `mapping_vectors` in the case consumer's method.
/// Assert: `result` equals `Err(MappingVectorsErrorReturn::HashToScalar(
///   HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
///   expected: 16, actual: 0 })))`.
#[test]
fn mapping_vectors_return_the_hash_refusal_before_mapping_any_identity() {
    // Arrange
    let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
        possession_g1_tag: Some(b"test-possession-g1"),
        possession_g2_tag: Some(b"test-possession-g2"),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b"test-challenge"),
        weight_tag: Some(b"test-weight"),
        ..Default::default()
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    struct Consumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: MappingVectorsParamsOverrides,
    }

    impl IMappingVectorsCaseConsumer for Consumer<'_> {
        type Output = MappingVectorsReturn;

        fn consume_mapping_vectors_case<P, H, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    pairing,
                    hash_to_scalar,
                    kem,
                    random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    hash_to_scalar_count: self.params_overrides.hash_to_scalar_count,
                    message_length: self.params_overrides.message_length,
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            )
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 0] = [];
    let roots = [&root];

    // Act
    let output = run_mapping_vectors_case(
        &build_mapping_vectors_case_deps(consumer, Default::default()),
        build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            kem_hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
    )
    .expect("the case runs")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(MappingVectorsErrorReturn::HashToScalar(
            HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 0,
            })
        ))
    );
}
// mapping_vectors_return_the_hash_refusal_before_mapping_any_identity
