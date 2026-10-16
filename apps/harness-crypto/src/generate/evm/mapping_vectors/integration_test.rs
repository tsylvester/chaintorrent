#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

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
use domain::{SecretConstructorParamsOverrides, build_secret};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, FromFieldsParamsOverrides, IEncodingContract,
    ToFieldsParamsOverrides, build_from_fields_params, build_to_fields_params,
};
use envelope::{
    KeyAgreementDeclaration, KeyAgreementDeclarationOverrides, build_key_agreement_declaration,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarIdentifier, IHashToScalarAdapter,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, build_domain_tag, create_hash_to_scalar,
    expected_uniform_bytes,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IKemConsumer, IdentityScope, build_create_kem_deps, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayloadOverrides, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES,
    SampleUniformScalarParams, SampleUniformScalarPayloadOverrides, build_create_pairing_deps,
    build_create_pairing_params, build_encode_scalar_payload, build_sample_uniform_scalar_payload,
    create_pairing,
};
use proof::{
    DeliveryProofDeclaration, DeliveryProofDeclarationOverrides, build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

/// The canonical kind a `CanonicalFieldValue` carries, limited to the kinds
/// the two descriptions emit.
fn field_kind(value: &CanonicalFieldValue) -> CanonicalFieldKind {
    match value {
        CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
        CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
        _ => panic!("the descriptions emit only Bytes and Unsigned256"),
    }
}

/// The reference encoding for a tag's bytes and a message: expand the tag
/// and message through `expected_uniform_bytes`, sample the uniform bytes
/// with `P::Scalar::sample_from_uniform_bytes`, and encode the sampled
/// scalar through the pairing — an independent path from the one
/// `mapping_vectors` takes through the hash-to-scalar adapter.
fn reference_scalar_encoding<P: IPairingArithmetic>(
    pairing: &P,
    tag: &[u8],
    message: &[u8],
) -> [u8; 32] {
    let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
        bytes: Some(tag.to_vec()),
    });
    let uniform = build_secret(
        expected_uniform_bytes(
            &tag,
            message,
            <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
        ),
        SecretConstructorParamsOverrides,
    );
    let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
            uniform: Some(uniform),
        }),
    )
    .expect("a uniform input of UNIFORM_BYTES_LENGTH samples a scalar");
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        build_encode_scalar_payload(
            pairing,
            EncodeScalarPayloadOverrides {
                scalar: Some(sampled.scalar.expose().clone()),
            },
        ),
    );
    encoded
        .bytes
        .expose()
        .as_ref()
        .try_into()
        .expect("the scalar encoding is thirty-two bytes")
}

/// Contract: entry
///   `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag`;
///   `mapping_vectors` over the operating-system source → each
///   `hash_to_scalar` record's `tag` is the declared bytes of its position
///   and its `scalar` is the reference's encoding over its own `tag` and
///   `message`.
/// Arrange: `hash_to_scalar_count` from `build_vector_count` with `count`
///   `2`, so more than one message is drawn, the declared tags distinct
///   from one another, and every case of the consumer's loops; the
///   reference built per record from the record's `tag` and `message`.
/// Act:     `mapping_vectors::<P, K>` once per case, inside `KemConsumer`.
/// Assert:  in place, the result is `Ok`; the `tag` of the records of each
///   message are `K::DECLARATION.identity_tag`, `b"test-possession-g1"`,
///   `b"test-possession-g2"`, `b"test-challenge"`, and `b"test-weight"` in
///   that order; each record's `scalar` equals the reference's encoded
///   bytes by `assert_eq!`.
/// Boundary: `create_pairing`, the arm's real concrete,
///   `create_hash_to_scalar`, `create_kem`, the selected KEM concrete, and
///   `mapping_vectors`.
/// Mocked:  nothing; the operating system's generator is the outer edge
///   and runs real, so this block does not prove the generator's quality.
#[test]
fn each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
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
    let params_overrides = MappingVectorsParamsOverrides {
        hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(2),
        })),
        ..Default::default()
    };

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: &'a MappingVectorsParamsOverrides,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let identities: Vec<Vec<u8>> = Vec::new();

            // Act
            let result = mapping_vectors::<P, K>(
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
                    &identities,
                    Default::default(),
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let tags = [
                K::DECLARATION.identity_tag,
                b"test-possession-g1".as_slice(),
                b"test-possession-g2".as_slice(),
                b"test-challenge".as_slice(),
                b"test-weight".as_slice(),
            ];
            assert_eq!(success.hash_to_scalar.len(), 10);
            for (index, record) in success.hash_to_scalar.iter().enumerate() {
                assert_eq!(record.tag.as_slice(), tags[index % tags.len()]);
                assert_eq!(
                    record.scalar,
                    reference_scalar_encoding(
                        self.pairing,
                        record.tag.as_slice(),
                        record.message.as_slice(),
                    )
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        params_overrides: &'a MappingVectorsParamsOverrides,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for case in DECLARED_KEM_SELECTIONS.iter().copied() {
                    let Ok(_) = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                pairing: &consumed.adapter,
                                hash_to_scalar: &*hashing.adapter,
                                random: self.random.as_ref(),
                                key_agreement: self.key_agreement,
                                delivery_proof: self.delivery_proof,
                                params_overrides: self.params_overrides,
                            },
                            Default::default(),
                        ),
                        case,
                        build_create_kem_payload(Default::default()),
                    ) else {
                        panic!("the declared KEM selection is created")
                    };
                }
            }
        }
    }

    let consumer = PairingConsumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: &params_overrides,
        random: source,
    };
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag
}

/// Contract: entry
///   `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`;
///   `mapping_vectors` under the scope of a declared KEM case → each
///   `identity_mapping` record's `identity` is its configured bytes and its
///   `scalar` is the reference's encoding over `K::DECLARATION.identity_tag`
///   and those bytes.
/// Arrange: every case of `DECLARED_KEM_SELECTIONS`, so each scope the
///   declared set holds runs; identities chosen in `KemConsumer` by an
///   exhaustive `match` on `payload.scope()`,
///   `vec![b"pkg@1.0.0#7".to_vec(), b"pkg@1.0.0#8".to_vec()]` under the
///   entitlement scope and `vec![vec![0x11; 32], vec![0x22; 32]]` under the
///   asset scope, so two identities differ and a scalar swapped between
///   records fails; the reference built per record from the tag
///   `K::DECLARATION.identity_tag` and the record's `identity`.
/// Act:     `mapping_vectors::<P, K>` once per case, inside `KemConsumer`.
/// Assert:  in place, the result is `Ok`; `identity_mapping` holds the
///   identities in order; each record's `scalar` equals the reference's
///   encoded bytes by `assert_eq!`, the setup draws differing per
///   identity.
/// Boundary: `create_pairing`, the arm's real concrete,
///   `create_hash_to_scalar`, `create_kem`, the selected KEM concrete's
///   `setup`, `derive_identity`, and `identity_element_components`,
///   `AssetIdentityHash::try_new`, and `mapping_vectors`.
/// Mocked:  nothing; the operating system's generator is the outer edge
///   and runs real.
#[test]
fn each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
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

    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let identities: Vec<Vec<u8>> = match payload.scope() {
                IdentityScope::Entitlement => {
                    vec![b"pkg@1.0.0#7".to_vec(), b"pkg@1.0.0#8".to_vec()]
                }
                IdentityScope::Asset => vec![vec![0x11; 32], vec![0x22; 32]],
            };
            let expected: Vec<[u8; 32]> = identities
                .iter()
                .map(|identity| {
                    reference_scalar_encoding(
                        self.pairing,
                        K::DECLARATION.identity_tag,
                        identity.as_slice(),
                    )
                })
                .collect();

            // Act
            let result = mapping_vectors::<P, K>(
                &build_mapping_vectors_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_mapping_vectors_params(MappingVectorsParamsOverrides {
                    scope: Some(payload.scope()),
                    ..Default::default()
                }),
                build_mapping_vectors_payload(
                    self.key_agreement,
                    self.delivery_proof,
                    &identities,
                    Default::default(),
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(success.identity_mapping.len(), identities.len());
            for (record, (identity, expected)) in success
                .identity_mapping
                .iter()
                .zip(identities.iter().zip(expected.iter()))
            {
                assert_eq!(&record.identity, identity);
                assert_eq!(record.scalar, *expected);
            }
            assert_ne!(
                success.identity_mapping[0].scalar, success.identity_mapping[1].scalar,
                "distinct identities map to distinct scalars whatever the setup draws"
            );
        }
    }

    struct PairingConsumer<'a> {
        key_agreement: &'a KeyAgreementDeclaration,
        delivery_proof: &'a DeliveryProofDeclaration,
        random: Box<dyn IRandomSourceAdapter>,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for case in DECLARED_KEM_SELECTIONS.iter().copied() {
                    let Ok(_) = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                pairing: &consumed.adapter,
                                hash_to_scalar: &*hashing.adapter,
                                random: self.random.as_ref(),
                                key_agreement: self.key_agreement,
                                delivery_proof: self.delivery_proof,
                            },
                            Default::default(),
                        ),
                        case,
                        build_create_kem_payload(Default::default()),
                    ) else {
                        panic!("the declared KEM selection is created")
                    };
                }
            }
        }
    }

    let consumer = PairingConsumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        random: source,
    };
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag
}

/// Contract: entry `a_record_survives_its_description_round_trip`; each
///   description's `fields_to_value`, called with the fields its
///   `to_fields` returns for a record whose fields differ from one another
///   in value and length, returns `Ok` whose `described` equals the
///   record.
/// Arrange: `build_hash_to_scalar_vector` with `tag` `vec![0x0A, 0x0B]`,
///   `message` `vec![0x0C]`, and `scalar` `[0x0E; 32]`, and
///   `build_identity_mapping_vector` with `identity` `vec![0x0A, 0x0B,
///   0x0C]` and `scalar` `[0x0D; 32]`, each with its description from
///   `build_hash_to_scalar_vector_description(Default::default())` or
///   `build_identity_mapping_vector_description(Default::default())` and
///   with `build_to_fields_params(Default::default())` and
///   `build_from_fields_params(Default::default())`.
/// Act:     `to_fields` on each record, then `fields_to_value` on the
///   fields it returned.
/// Assert:  each `described` equals the record it was built from.
/// Boundary: each description's `to_fields` then `fields_to_value`, both
///   real.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_record_survives_its_description_round_trip() {
    // Arrange
    let hash_vector = build_hash_to_scalar_vector(HashToScalarVectorOverrides {
        tag: Some(vec![0x0A, 0x0B]),
        message: Some(vec![0x0C]),
        scalar: Some([0x0E; 32]),
    });
    let identity_vector = build_identity_mapping_vector(IdentityMappingVectorOverrides {
        identity: Some(vec![0x0A, 0x0B, 0x0C]),
        scalar: Some([0x0D; 32]),
    });
    let hash_description = build_hash_to_scalar_vector_description(Default::default());
    let identity_description = build_identity_mapping_vector_description(Default::default());

    // Act
    let Ok(hash_fields) = hash_description.to_fields(
        build_to_fields_params(ToFieldsParamsOverrides),
        &hash_vector,
    );
    let rebuilt_hash = hash_description
        .fields_to_value(
            build_from_fields_params(FromFieldsParamsOverrides),
            hash_fields.fields,
        )
        .expect("the hash-to-scalar record's fields rebuild it");
    let Ok(identity_fields) = identity_description.to_fields(
        build_to_fields_params(ToFieldsParamsOverrides),
        &identity_vector,
    );
    let rebuilt_identity = identity_description
        .fields_to_value(
            build_from_fields_params(FromFieldsParamsOverrides),
            identity_fields.fields,
        )
        .expect("the identity-mapping record's fields rebuild it");

    // Assert
    assert_eq!(rebuilt_hash.described, hash_vector);
    assert_eq!(rebuilt_identity.described, identity_vector);
    // a_record_survives_its_description_round_trip
}

/// Contract: entry `members_declare_the_kinds_to_fields_emits`; on each
///   description over a built record, the kinds of `members()` in order are
///   the kinds of the values `to_fields` returns.
/// Arrange: `build_hash_to_scalar_vector` and
///   `build_identity_mapping_vector` at their defaults, each with its
///   description and `build_to_fields_params(Default::default())`.
/// Act:     `to_fields` and `members()` on each description.
/// Assert:  each `members()` result equals `Ok` of
///   `build_solidity_record_members` over `build_solidity_record_member`
///   values named `tag`, `message`, and `scalar` for the hash-to-scalar
///   description and `identity` and `scalar` for the identity-mapping
///   description through `build_solidity_member_name`, whose kinds are, in
///   order, `CanonicalFieldKind::Bytes` for each `CanonicalFieldValue::Bytes`
///   and `CanonicalFieldKind::Unsigned256` for each
///   `CanonicalFieldValue::Unsigned256` among the values `to_fields`
///   returned.
/// Boundary: `members()` over the real `SolidityMemberName::try_new` and
///   `SolidityRecordMembers::try_new`, and `to_fields`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn members_declare_the_kinds_to_fields_emits() {
    // Arrange
    let hash_vector = build_hash_to_scalar_vector(Default::default());
    let identity_vector = build_identity_mapping_vector(Default::default());
    let hash_description = build_hash_to_scalar_vector_description(Default::default());
    let identity_description = build_identity_mapping_vector_description(Default::default());

    // Act
    let Ok(hash_fields) = hash_description.to_fields(
        build_to_fields_params(ToFieldsParamsOverrides),
        &hash_vector,
    );
    let hash_members = hash_description.members();
    let Ok(identity_fields) = identity_description.to_fields(
        build_to_fields_params(ToFieldsParamsOverrides),
        &identity_vector,
    );
    let identity_members = identity_description.members();

    // Assert
    let expected_hash =
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(
                HASH_TO_SCALAR_VECTOR_MEMBER_NAMES
                    .iter()
                    .zip(hash_fields.fields.values.iter())
                    .map(|(name, value)| {
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some((*name).to_string()),
                                },
                            )),
                            kind: Some(field_kind(value)),
                        })
                    })
                    .collect(),
            ),
        });
    assert_eq!(hash_members, Ok(expected_hash));
    let expected_identity =
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(
                IDENTITY_MAPPING_VECTOR_MEMBER_NAMES
                    .iter()
                    .zip(identity_fields.fields.values.iter())
                    .map(|(name, value)| {
                        build_solidity_record_member(SolidityRecordMemberOverrides {
                            name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some((*name).to_string()),
                                },
                            )),
                            kind: Some(field_kind(value)),
                        })
                    })
                    .collect(),
            ),
        });
    assert_eq!(identity_members, Ok(expected_identity));
    // members_declare_the_kinds_to_fields_emits
}
