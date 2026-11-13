#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use domain::build_secret;
use encoding::{IEncodingContract, build_from_fields_params, build_to_fields_params};
use envelope::{
    KeyAgreementDeclaration, KeyAgreementDeclarationOverrides, build_key_agreement_declaration,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarIdentifier, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, build_domain_tag, expected_uniform_bytes,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{CreateKemParams, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter, KemIdentity};
use pairing::{
    CreatePairingParamsOverrides, EncodeScalarParams, EncodeScalarPayloadOverrides,
    IPairingArithmetic, IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES,
    SampleUniformScalarParams, SampleUniformScalarPayloadOverrides, build_create_pairing_params,
    build_encode_scalar_payload, build_sample_uniform_scalar_payload,
};
use proof::{
    DeliveryProofDeclaration, DeliveryProofDeclarationOverrides, build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::super::super::factory::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, build_solidity_member_name,
};
use super::interface::IMappingVectorsCaseConsumer;
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

/// Contract: own entry `each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag`;
///   `mapping_vectors` over the operating-system source → each `hash_to_scalar` record's
///   `tag` is the declared bytes of its position and its `scalar` is the reference's
///   encoding over its own `tag` and `message`.
/// Arrange: `hash_to_scalar_count` from `build_vector_count` with `count` `2`, so more
///   than one message is drawn, the declared tags distinct from one another, and every
///   case the block runs; the reference built per record from the record's `tag` and
///   `message`.
/// Act:    `mapping_vectors::<P, K>` once per case, inside the case consumer's method.
/// Assert: in place, the result is `Ok`; the `tag` of the records of each message are
///   `K::DECLARATION.identity_tag`, `b"test-possession-g1"`, `b"test-possession-g2"`,
///   `b"test-challenge"`, and `b"test-weight"` in that order; each record's `scalar`
///   equals the reference's encoded bytes by `assert_eq!`.
/// Boundary: `create_pairing`, the arm's real concrete, `create_hash_to_scalar`,
///   `create_kem`, the selected KEM concrete, and `mapping_vectors`.
/// Mocked:  nothing; the operating system's generator is the outer edge and runs real,
///   so this block does not prove the generator's quality.
#[test]
fn each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag() {
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
        type Output = ();

        fn consume_mapping_vectors_case<P, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let expected_tags = [
                K::DECLARATION.identity_tag,
                b"test-possession-g1".as_slice(),
                b"test-possession-g2".as_slice(),
                b"test-challenge".as_slice(),
                b"test-weight".as_slice(),
            ];

            // Act
            let result = mapping_vectors::<P, K>(
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
            );

            // Assert
            let success = result.expect("the mapping succeeds");
            assert_eq!(success.hash_to_scalar.len(), 10);
            for (index, record) in success.hash_to_scalar.iter().enumerate() {
                assert_eq!(record.tag, expected_tags[index % 5].to_vec());
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(record.tag.clone()),
                });
                let uniform =
                    expected_uniform_bytes(&tag, &record.message, P::Scalar::UNIFORM_BYTES_LENGTH);
                let secret = build_secret(uniform, Default::default());
                let scalar = P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                        uniform: Some(secret),
                    }),
                )
                .expect("the reference scalar samples")
                .scalar
                .expose()
                .clone();
                let encoded = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        build_encode_scalar_payload(
                            pairing,
                            EncodeScalarPayloadOverrides {
                                scalar: Some(scalar),
                            },
                        ),
                    )
                    .expect("the reference scalar encodes");
                assert_eq!(record.scalar.as_slice(), encoded.bytes.expose().as_ref());
            }
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
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for kem_selection in DECLARED_KEM_SELECTIONS {
                let kem: CreateKemParams = *kem_selection;
                let result = run_mapping_vectors_case(
                    &build_mapping_vectors_case_deps(
                        Consumer {
                            key_agreement: consumer.key_agreement,
                            delivery_proof: consumer.delivery_proof,
                            params_overrides: MappingVectorsParamsOverrides {
                                hash_to_scalar_count: consumer
                                    .params_overrides
                                    .hash_to_scalar_count,
                                message_length: consumer.params_overrides.message_length,
                            },
                        },
                        Default::default(),
                    ),
                    build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                            CreateHashToScalarParamsOverrides {
                                concrete: Some(hash_selection.concrete),
                                identifier: Some(hash_selection.identifier),
                            },
                        )),
                        kem_hash_to_scalar: Some(build_create_hash_to_scalar_params(
                            CreateHashToScalarParamsOverrides {
                                concrete: Some(hash_selection.concrete),
                                identifier: Some(hash_selection.identifier),
                            },
                        )),
                        kem: Some(kem),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                    }),
                    build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}
// each_hash_to_scalar_record_is_the_references_mapping_of_its_message_under_its_tag

/// Contract: own entry `each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag`;
///   `mapping_vectors` under the scope of a declared KEM case → each `identity_mapping`
///   record's `identity` is its admitted identity's bytes and its `scalar` is the
///   reference's encoding over `K::DECLARATION.identity_tag` and those bytes.
/// Arrange: every case of `DECLARED_KEM_SELECTIONS`, so each scope the declared set
///   holds runs; the two admitted identities the runner's KEM level composes under the
///   scope the received KEM reports, from the case payload's `canonicals`
///   `b"pkg@1.0.0#7"` and `b"pkg@1.0.0#8"` under the entitlement scope and from the
///   case payload's `roots`, `build_root` over `[0x11; 32]` and `[0x22; 32]`, under
///   the asset scope, so two identities differ and a scalar swapped between records
///   fails; the reference built per record from the tag `K::DECLARATION.identity_tag`
///   and the record's `identity`.
/// Act:    `mapping_vectors::<P, K>` once per case, inside the case consumer's method.
/// Assert: in place, the result is `Ok`; `identity_mapping` holds the identities' bytes
///   in order; each record's `scalar` equals the reference's encoded bytes by
///   `assert_eq!`, the setup draws differing per identity.
/// Boundary: `create_pairing`, the arm's real concrete, `create_hash_to_scalar`,
///   `create_kem`, the selected KEM concrete's `setup`, `derive_identity`, and
///   `identity_element_components`, and `mapping_vectors`.
/// Mocked:  nothing; the operating system's generator is the outer edge and runs real.
#[test]
fn each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag() {
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
        type Output = ();

        fn consume_mapping_vectors_case<P, K>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let expected_identities: Vec<Vec<u8>> = identities
                .iter()
                .map(|admitted| match admitted.identity {
                    KemIdentity::Entitlement { canonical } => canonical.to_vec(),
                    KemIdentity::Asset { root } => root.as_bytes().to_vec(),
                })
                .collect();

            // Act
            let result = mapping_vectors::<P, K>(
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
            );

            // Assert
            let success = result.expect("the mapping succeeds");
            assert_eq!(success.identity_mapping.len(), expected_identities.len());
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            for (index, record) in success.identity_mapping.iter().enumerate() {
                assert_eq!(record.identity, expected_identities[index]);
                let uniform =
                    expected_uniform_bytes(&tag, &record.identity, P::Scalar::UNIFORM_BYTES_LENGTH);
                let secret = build_secret(uniform, Default::default());
                let scalar = P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    build_sample_uniform_scalar_payload(SampleUniformScalarPayloadOverrides {
                        uniform: Some(secret),
                    }),
                )
                .expect("the reference scalar samples")
                .scalar
                .expose()
                .clone();
                let encoded = pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        build_encode_scalar_payload(
                            pairing,
                            EncodeScalarPayloadOverrides {
                                scalar: Some(scalar),
                            },
                        ),
                    )
                    .expect("the reference scalar encodes");
                assert_eq!(record.scalar.as_slice(), encoded.bytes.expose().as_ref());
            }
        }
    }

    let consumer = Consumer {
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        params_overrides: Default::default(),
    };
    let canonicals: [&[u8]; 2] = [b"pkg@1.0.0#7", b"pkg@1.0.0#8"];
    let roots = [&root_a, &root_b];

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for kem_selection in DECLARED_KEM_SELECTIONS {
                let kem: CreateKemParams = *kem_selection;
                let result = run_mapping_vectors_case(
                    &build_mapping_vectors_case_deps(
                        Consumer {
                            key_agreement: consumer.key_agreement,
                            delivery_proof: consumer.delivery_proof,
                            params_overrides: MappingVectorsParamsOverrides {
                                hash_to_scalar_count: consumer
                                    .params_overrides
                                    .hash_to_scalar_count,
                                message_length: consumer.params_overrides.message_length,
                            },
                        },
                        Default::default(),
                    ),
                    build_mapping_vectors_case_params(MappingVectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                            CreateHashToScalarParamsOverrides {
                                concrete: Some(hash_selection.concrete),
                                identifier: Some(hash_selection.identifier),
                            },
                        )),
                        kem_hash_to_scalar: Some(build_create_hash_to_scalar_params(
                            CreateHashToScalarParamsOverrides {
                                concrete: Some(hash_selection.concrete),
                                identifier: Some(hash_selection.identifier),
                            },
                        )),
                        kem: Some(kem),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                    }),
                    build_mapping_vectors_case_payload(&canonicals, &roots, Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}
// each_identity_record_is_the_references_mapping_of_its_identity_under_the_identity_tag

/// Contract: own entry `a_record_survives_its_description_round_trip`; each
///   description's `fields_to_value`, called with the fields its `to_fields` returns
///   for a record whose fields differ from one another in value and length, returns
///   `Ok` whose `described` equals the record.
/// Arrange: `build_hash_to_scalar_vector` with `tag` `vec![0x0A, 0x0B]`, `message`
///   `vec![0x0C]`, and `scalar` `[0x0E; 32]`, and `build_identity_mapping_vector` with
///   `identity` `vec![0x0A, 0x0B, 0x0C]` and `scalar` `[0x0D; 32]`, each with its
///   description from `build_hash_to_scalar_vector_description(Default::default())` or
///   `build_identity_mapping_vector_description(Default::default())` and with
///   `build_to_fields_params(Default::default())` and
///   `build_from_fields_params(Default::default())`.
/// Act:    `to_fields` on each record, then `fields_to_value` on the fields it
///   returned.
/// Assert: each `described` equals the record it was built from.
/// Boundary: each description's `to_fields` then `fields_to_value`, both real.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn a_record_survives_its_description_round_trip() {
    // Arrange
    let hash_to_scalar = build_hash_to_scalar_vector(HashToScalarVectorOverrides {
        tag: Some(vec![0x0A, 0x0B]),
        message: Some(vec![0x0C]),
        scalar: Some([0x0E; 32]),
    });
    let identity_mapping = build_identity_mapping_vector(IdentityMappingVectorOverrides {
        identity: Some(vec![0x0A, 0x0B, 0x0C]),
        scalar: Some([0x0D; 32]),
    });
    let hash_to_scalar_description = build_hash_to_scalar_vector_description(Default::default());
    let identity_mapping_description =
        build_identity_mapping_vector_description(Default::default());

    // Act
    let hash_to_scalar_fields = hash_to_scalar_description
        .to_fields(build_to_fields_params(Default::default()), &hash_to_scalar)
        .expect("the description maps the record")
        .fields;
    let hash_to_scalar_described = hash_to_scalar_description
        .fields_to_value(
            build_from_fields_params(Default::default()),
            hash_to_scalar_fields,
        )
        .expect("the fields rebuild the record")
        .described;
    let identity_mapping_fields = identity_mapping_description
        .to_fields(
            build_to_fields_params(Default::default()),
            &identity_mapping,
        )
        .expect("the description maps the record")
        .fields;
    let identity_mapping_described = identity_mapping_description
        .fields_to_value(
            build_from_fields_params(Default::default()),
            identity_mapping_fields,
        )
        .expect("the fields rebuild the record")
        .described;

    // Assert
    assert_eq!(hash_to_scalar_described, hash_to_scalar);
    assert_eq!(identity_mapping_described, identity_mapping);
}
// a_record_survives_its_description_round_trip

/// Contract: own entry `member_names_declare_one_name_per_field`; on each description
///   over a built record, `member_names()` returns `Ok` of the declared names in
///   order, and their count equals the number of values `to_fields` returns.
/// Arrange: `build_hash_to_scalar_vector` and `build_identity_mapping_vector` at their
///   defaults, each with its description and `build_to_fields_params(Default::default())`.
/// Act:    `to_fields` and `member_names()` on each description.
/// Assert: each `member_names()` result equals `Ok` of the names `tag`, `message`, and
///   `scalar` for the hash-to-scalar description and `identity` and `scalar` for the
///   identity-mapping description through `build_solidity_member_name`, and its length
///   equals the length of the values `to_fields` returned.
/// Boundary: `member_names()` over the real `SolidityMemberName::try_new`, and
///   `to_fields`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn member_names_declare_one_name_per_field() {
    // Arrange
    let hash_to_scalar = build_hash_to_scalar_vector(Default::default());
    let identity_mapping = build_identity_mapping_vector(Default::default());
    let hash_to_scalar_description = build_hash_to_scalar_vector_description(Default::default());
    let identity_mapping_description =
        build_identity_mapping_vector_description(Default::default());
    let hash_to_scalar_expected = vec![
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("tag".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("message".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("scalar".to_string()),
        }),
    ];
    let identity_mapping_expected = vec![
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("identity".to_string()),
        }),
        build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
            text: Some("scalar".to_string()),
        }),
    ];

    // Act
    let hash_to_scalar_fields = hash_to_scalar_description
        .to_fields(build_to_fields_params(Default::default()), &hash_to_scalar)
        .expect("the description maps the record")
        .fields;
    let hash_to_scalar_names = hash_to_scalar_description
        .member_names()
        .expect("the names admit");
    let identity_mapping_fields = identity_mapping_description
        .to_fields(
            build_to_fields_params(Default::default()),
            &identity_mapping,
        )
        .expect("the description maps the record")
        .fields;
    let identity_mapping_names = identity_mapping_description
        .member_names()
        .expect("the names admit");

    // Assert
    assert_eq!(hash_to_scalar_names, hash_to_scalar_expected);
    assert_eq!(
        hash_to_scalar_names.len(),
        hash_to_scalar_fields.values.len()
    );
    assert_eq!(identity_mapping_names, identity_mapping_expected);
    assert_eq!(
        identity_mapping_names.len(),
        identity_mapping_fields.values.len()
    );
}
// member_names_declare_one_name_per_field
