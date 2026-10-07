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
use super::interface::{
    HashToScalarVectorFromFieldsErrorReturn, IdentityMappingVectorFromFieldsErrorReturn,
    MappingVectorsDeps, MappingVectorsErrorReturn, MappingVectorsPayload, MappingVectorsReturn,
};
use super::mapping_vectors;
use super::mock::{
    HashToScalarVectorOverrides, IdentityMappingVectorOverrides, MappingVectorsParamsOverrides,
    build_hash_to_scalar_vector, build_hash_to_scalar_vector_description,
    build_identity_mapping_vector, build_identity_mapping_vector_description,
    build_mapping_vectors_params,
};
use domain::AssetIdentityHashTryNewErrorReturn;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, FromFieldsParams, IEncodingContract,
    ToFieldsParams,
};
use envelope::build_key_agreement_declaration;
use envelope::{KeyAgreementDeclaration, KeyAgreementDeclarationOverrides};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, DomainTag, DomainTagConstructorParams,
    HashToScalarParams, HashToScalarPayload, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemParamsOverrides, CreateKemPayload,
    ICredentialKemAdapter, IKemConsumer, IdentityScope, build_create_kem_params, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, PairingConcrete, build_create_pairing_params,
    create_pairing,
};
use proof::build_delivery_proof_declaration;
use proof::{DeliveryProofDeclaration, DeliveryProofDeclarationOverrides};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

struct MappingVectorsProbeOutput {
    result: MappingVectorsReturn,
    hash_references: Vec<[u8; 32]>,
    identity_references: Vec<[u8; 32]>,
}

struct MappingVectorsPairingProbe {
    params: super::interface::MappingVectorsParams,
    key_agreement: KeyAgreementDeclaration,
    delivery_proof: DeliveryProofDeclaration,
    identities: Vec<Vec<u8>>,
    random: Box<dyn IRandomSourceAdapter>,
}

impl IPairingConsumer for MappingVectorsPairingProbe {
    type Output = MappingVectorsProbeOutput;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is admitted")
        };
        let Ok(kem) = create_kem(
            &CreateKemDeps {
                pairing: &payload.adapter,
                hash_to_scalar: hashing.adapter.as_ref(),
                consumer: MappingVectorsKemProbe {
                    pairing: &payload.adapter,
                    hash_to_scalar: hashing.adapter.as_ref(),
                    probe: self,
                },
            },
            build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(self.params.scope),
                ..Default::default()
            }),
            CreateKemPayload,
        ) else {
            panic!("the KEM concrete is admitted for the scope")
        };
        kem.output
    }
}

struct MappingVectorsKemProbe<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    probe: &'a MappingVectorsPairingProbe,
}

impl<P: IPairingArithmetic> IKemConsumer<P> for MappingVectorsKemProbe<'_, P> {
    type Output = MappingVectorsProbeOutput;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let mut output = MappingVectorsProbeOutput {
            result: mapping_vectors::<P, K>(
                &MappingVectorsDeps {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    kem: &payload.adapter,
                    random: self.probe.random.as_ref(),
                },
                self.probe.params,
                MappingVectorsPayload {
                    key_agreement: &self.probe.key_agreement,
                    delivery_proof: &self.probe.delivery_proof,
                    identities: &self.probe.identities,
                },
            ),
            hash_references: Vec::new(),
            identity_references: Vec::new(),
        };
        let Ok(success) = &output.result else {
            return output;
        };
        for record in &success.hash_to_scalar {
            let Ok(tag) = DomainTag::try_new(DomainTagConstructorParams {
                bytes: record.tag.clone(),
            }) else {
                panic!("each record's tag is admitted")
            };
            let Ok(hashed) = self.hash_to_scalar.hash_to_scalar(
                HashToScalarParams { tag: &tag },
                HashToScalarPayload {
                    message: &record.message,
                },
            ) else {
                panic!("each reference hash succeeds")
            };
            let Ok(encoded) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: hashed.scalar,
                },
            );
            let Ok(reference) = <[u8; 32]>::try_from(encoded.bytes.expose().as_ref()) else {
                panic!("each scalar encodes to 32 bytes")
            };
            output.hash_references.push(reference);
        }
        for record in &success.identity_mapping {
            let Ok(tag) = DomainTag::try_new(DomainTagConstructorParams {
                bytes: K::DECLARATION.identity_tag.to_vec(),
            }) else {
                panic!("the identity tag is admitted")
            };
            let Ok(hashed) = self.hash_to_scalar.hash_to_scalar(
                HashToScalarParams { tag: &tag },
                HashToScalarPayload {
                    message: &record.identity,
                },
            ) else {
                panic!("each reference hash succeeds")
            };
            let Ok(encoded) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: hashed.scalar,
                },
            );
            let Ok(reference) = <[u8; 32]>::try_from(encoded.bytes.expose().as_ref()) else {
                panic!("each scalar encodes to 32 bytes")
            };
            output.identity_references.push(reference);
        }
        output
    }
}

fn run_mapping_vectors(
    probe: MappingVectorsPairingProbe,
    concrete: PairingConcrete,
) -> MappingVectorsProbeOutput {
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer: probe },
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        }),
        CreatePairingPayload,
    ) else {
        panic!("the pairing concrete is admitted")
    };
    success.output
}

/// Contract: the hash-to-scalar description's mapped branch carries tag,
///   message, and scalar as Bytes, Bytes, and Unsigned256 fields, in member
///   order.
/// Arrange: a hash-to-scalar vector with tag [0x0A], message [0x0B, 0x0C], and
///   scalar [0x0D; 32].
/// Act:     the description's to_fields over the record.
/// Assert:  the values are Bytes([0x0A]), Bytes([0x0B, 0x0C]), and
///   Unsigned256([0x0D; 32]) in that order.
#[test]
fn hash_to_scalar_vector_description_maps_tag_message_and_scalar_in_order() {
    // Arrange
    let vector = build_hash_to_scalar_vector(HashToScalarVectorOverrides {
        tag: Some(vec![0x0A]),
        message: Some(vec![0x0B, 0x0C]),
        scalar: Some([0x0D; 32]),
    });
    let description = build_hash_to_scalar_vector_description();

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ]
    );
}

/// Contract: the hash-to-scalar description's rebuilt branch moves each field
///   back into its record member; a field vector of two takes the count branch;
///   a Bytes scalar takes the kind branch expecting Unsigned256.
/// Arrange: canonical fields Bytes([0x0A]), Bytes([0x0B, 0x0C]), and
///   Unsigned256([0x0D; 32]); the first two alone; and Bytes([0x0A]),
///   Bytes([0x0B]), and Bytes([0x0D]).
/// Act:     the description's fields_to_value over each field set.
/// Assert:  the first described record holds the three values; the second
///   error is FieldCount { expected: 3, actual: 2 }; the third is FieldKind
///   { index: 2, expected: CanonicalFieldKind::Unsigned256 }.
#[test]
fn hash_to_scalar_vector_description_rebuilds_the_record_and_refuses_a_wrong_count_and_kind() {
    // Arrange
    let rebuilt = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
            CanonicalFieldValue::Unsigned256([0x0D; 32]),
        ],
    };
    let short = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B, 0x0C]),
        ],
    };
    let wrong_kind = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0A]),
            CanonicalFieldValue::Bytes(vec![0x0B]),
            CanonicalFieldValue::Bytes(vec![0x0D]),
        ],
    };
    let description = build_hash_to_scalar_vector_description();

    // Act
    let rebuilt = description.fields_to_value(FromFieldsParams, rebuilt);
    let short = description.fields_to_value(FromFieldsParams, short);
    let wrong_kind = description.fields_to_value(FromFieldsParams, wrong_kind);

    // Assert
    let Ok(success) = rebuilt else {
        panic!("the fields rebuild the record")
    };
    assert_eq!(
        success.described,
        build_hash_to_scalar_vector(HashToScalarVectorOverrides {
            tag: Some(vec![0x0A]),
            message: Some(vec![0x0B, 0x0C]),
            scalar: Some([0x0D; 32]),
        })
    );
    let Err(error) = short else {
        panic!("the wrong field count is refused")
    };
    assert_eq!(
        error,
        HashToScalarVectorFromFieldsErrorReturn::FieldCount {
            expected: 3,
            actual: 2
        }
    );
    let Err(error) = wrong_kind else {
        panic!("the wrong field kind is refused")
    };
    assert_eq!(
        error,
        HashToScalarVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::Unsigned256,
        }
    );
}

/// Contract: the identity-mapping description's mapped branch carries identity
///   and scalar as a Bytes field and an Unsigned256 field, in member order.
/// Arrange: an identity-mapping vector with identity [0x0E] and scalar
///   [0x0F; 32].
/// Act:     the description's to_fields over the record.
/// Assert:  the values are Bytes([0x0E]) and Unsigned256([0x0F; 32]) in that
///   order.
#[test]
fn identity_mapping_vector_description_maps_identity_and_scalar_in_order() {
    // Arrange
    let vector = build_identity_mapping_vector(IdentityMappingVectorOverrides {
        identity: Some(vec![0x0E]),
        scalar: Some([0x0F; 32]),
    });
    let description = build_identity_mapping_vector_description();

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Bytes(vec![0x0E]),
            CanonicalFieldValue::Unsigned256([0x0F; 32]),
        ]
    );
}

/// Contract: the identity-mapping description's rebuilt branch moves each field
///   back into its record member; a lone field takes the count branch; an
///   Unsigned256 identity takes the kind branch expecting Bytes.
/// Arrange: canonical fields Bytes([0x0E]) and Unsigned256([0x0F; 32]); the
///   first alone; and Unsigned256([0x0E; 32]) and Unsigned256([0x0F; 32]).
/// Act:     the description's fields_to_value over each field set.
/// Assert:  the first described record holds the two values; the second error
///   is FieldCount { expected: 2, actual: 1 }; the third is FieldKind
///   { index: 0, expected: CanonicalFieldKind::Bytes }.
#[test]
fn identity_mapping_vector_description_rebuilds_the_record_and_refuses_a_wrong_count_and_kind() {
    // Arrange
    let rebuilt = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Bytes(vec![0x0E]),
            CanonicalFieldValue::Unsigned256([0x0F; 32]),
        ],
    };
    let short = CanonicalFields {
        values: vec![CanonicalFieldValue::Bytes(vec![0x0E])],
    };
    let wrong_kind = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Unsigned256([0x0E; 32]),
            CanonicalFieldValue::Unsigned256([0x0F; 32]),
        ],
    };
    let description = build_identity_mapping_vector_description();

    // Act
    let rebuilt = description.fields_to_value(FromFieldsParams, rebuilt);
    let short = description.fields_to_value(FromFieldsParams, short);
    let wrong_kind = description.fields_to_value(FromFieldsParams, wrong_kind);

    // Assert
    let Ok(success) = rebuilt else {
        panic!("the fields rebuild the record")
    };
    assert_eq!(
        success.described,
        build_identity_mapping_vector(IdentityMappingVectorOverrides {
            identity: Some(vec![0x0E]),
            scalar: Some([0x0F; 32]),
        })
    );
    let Err(error) = short else {
        panic!("the wrong field count is refused")
    };
    assert_eq!(
        error,
        IdentityMappingVectorFromFieldsErrorReturn::FieldCount {
            expected: 2,
            actual: 1
        }
    );
    let Err(error) = wrong_kind else {
        panic!("the wrong field kind is refused")
    };
    assert_eq!(
        error,
        IdentityMappingVectorFromFieldsErrorReturn::FieldKind {
            index: 0,
            expected: CanonicalFieldKind::Bytes,
        }
    );
}

/// Contract: each description's members branch pairs its member names with its
///   field kind at the same index — tag, message, scalar for the hash-to-scalar
///   record and identity, scalar for the identity-mapping record.
/// Arrange: the hash-to-scalar and identity-mapping descriptions.
/// Act:     members() on each description.
/// Assert:  the first equals members tag and message of kind Bytes and scalar
///   of kind Unsigned256; the second equals identity of kind Bytes and scalar
///   of kind Unsigned256.
#[test]
fn mapping_vector_descriptions_declare_their_members() {
    // Arrange
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };

    // Act
    let Ok(hash_to_scalar) = build_hash_to_scalar_vector_description().members() else {
        panic!("the member names are admitted")
    };
    let Ok(identity_mapping) = build_identity_mapping_vector_description().members() else {
        panic!("the member names are admitted")
    };

    // Assert
    assert_eq!(
        hash_to_scalar,
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("tag", CanonicalFieldKind::Bytes),
                member("message", CanonicalFieldKind::Bytes),
                member("scalar", CanonicalFieldKind::Unsigned256),
            ]),
        })
    );
    assert_eq!(
        identity_mapping,
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("identity", CanonicalFieldKind::Bytes),
                member("scalar", CanonicalFieldKind::Unsigned256),
            ]),
        })
    );
}

/// Contract: the computed branch emits one hash-to-scalar record per declared
///   tag per drawn message, the tags in declaration order and each record
///   carrying its message.
/// Arrange: a hash-to-scalar count of 2 and a message length of 24, over the
///   operating system random source.
/// Act:     mapping_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  hash_to_scalar holds 10 records; indices 0 through 4 and 5 through
///   9 carry the tags b"ChainTorrent-v1-kem-identity", b"test-possession-g1",
///   b"test-possession-g2", b"test-challenge", and b"test-weight" in that
///   order; each record's message holds 24 bytes; each group of five shares
///   one message and the two groups' messages differ.
#[test]
fn mapping_vectors_map_each_drawn_message_under_every_declared_tag_in_order() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = MappingVectorsPairingProbe {
        params: build_mapping_vectors_params(MappingVectorsParamsOverrides {
            hash_to_scalar_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(2),
            })),
            message_length: Some(24),
            ..Default::default()
        }),
        key_agreement: build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
            possession_g1_tag: Some(b"test-possession-g1"),
            possession_g2_tag: Some(b"test-possession-g2"),
            ..Default::default()
        }),
        delivery_proof: build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
            challenge_tag: Some(b"test-challenge"),
            weight_tag: Some(b"test-weight"),
            ..Default::default()
        }),
        identities: Vec::new(),
        random: random.adapter,
    };

    // Act
    let output = run_mapping_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    let tags = [
        b"ChainTorrent-v1-kem-identity".as_slice(),
        b"test-possession-g1".as_slice(),
        b"test-possession-g2".as_slice(),
        b"test-challenge".as_slice(),
        b"test-weight".as_slice(),
    ];
    assert_eq!(success.hash_to_scalar.len(), 10);
    for (index, record) in success.hash_to_scalar.iter().enumerate() {
        assert_eq!(record.tag, tags[index % 5]);
        assert_eq!(record.message.len(), 24);
    }
    for index in 0..5 {
        assert_eq!(
            success.hash_to_scalar[index].message,
            success.hash_to_scalar[0].message
        );
        assert_eq!(
            success.hash_to_scalar[5 + index].message,
            success.hash_to_scalar[5].message
        );
    }
    assert_ne!(
        success.hash_to_scalar[0].message,
        success.hash_to_scalar[5].message
    );
}

/// Contract: the computed branch's hash-to-scalar records carry the scalar the
///   record's tag maps its message to.
/// Arrange: the default params, over the operating system random source.
/// Act:     mapping_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  each hash_to_scalar record's scalar equals its entry of
///   hash_references.
#[test]
fn mapping_vectors_record_the_scalar_each_tag_maps_its_message_to() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = MappingVectorsPairingProbe {
        params: build_mapping_vectors_params(Default::default()),
        key_agreement: build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
            possession_g1_tag: Some(b"test-possession-g1"),
            possession_g2_tag: Some(b"test-possession-g2"),
            ..Default::default()
        }),
        delivery_proof: build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
            challenge_tag: Some(b"test-challenge"),
            weight_tag: Some(b"test-weight"),
            ..Default::default()
        }),
        identities: Vec::new(),
        random: random.adapter,
    };

    // Act
    let output = run_mapping_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    for (index, record) in success.hash_to_scalar.iter().enumerate() {
        assert_eq!(record.scalar, output.hash_references[index]);
    }
}

/// Contract: under the entitlement scope the computed branch's identity-mapping
///   records carry each identity's bytes and the scalar the KEM derives for it.
/// Arrange: identities b"pkg@1.0.0#7" and b"pkg@1.0.0#8", over the operating
///   system random source.
/// Act:     mapping_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  identity_mapping holds two records whose identity is each arranged
///   value in order and whose scalar equals its entry of identity_references.
#[test]
fn mapping_vectors_map_each_identity_through_the_kem_under_the_entitlement_scope() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = MappingVectorsPairingProbe {
        params: build_mapping_vectors_params(Default::default()),
        key_agreement: build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
            possession_g1_tag: Some(b"test-possession-g1"),
            possession_g2_tag: Some(b"test-possession-g2"),
            ..Default::default()
        }),
        delivery_proof: build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
            challenge_tag: Some(b"test-challenge"),
            weight_tag: Some(b"test-weight"),
            ..Default::default()
        }),
        identities: vec![b"pkg@1.0.0#7".to_vec(), b"pkg@1.0.0#8".to_vec()],
        random: random.adapter,
    };

    // Act
    let output = run_mapping_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    assert_eq!(success.identity_mapping.len(), 2);
    assert_eq!(success.identity_mapping[0].identity, b"pkg@1.0.0#7");
    assert_eq!(success.identity_mapping[1].identity, b"pkg@1.0.0#8");
    for (index, record) in success.identity_mapping.iter().enumerate() {
        assert_eq!(record.scalar, output.identity_references[index]);
    }
}

/// Contract: under the asset scope the computed branch's identity-mapping
///   records carry each identity's bytes and the scalar the KEM derives for it.
/// Arrange: the asset scope and identities [0x11; 32] and [0x22; 32], over the
///   operating system random source.
/// Act:     mapping_vectors through the probe over
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  identity_mapping holds two records whose identity is each arranged
///   value in order and whose scalar equals its entry of identity_references.
#[test]
fn mapping_vectors_map_each_identity_through_the_kem_under_the_asset_scope_on_bls12_381() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = MappingVectorsPairingProbe {
        params: build_mapping_vectors_params(MappingVectorsParamsOverrides {
            scope: Some(IdentityScope::Asset),
            ..Default::default()
        }),
        key_agreement: build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
            possession_g1_tag: Some(b"test-possession-g1"),
            possession_g2_tag: Some(b"test-possession-g2"),
            ..Default::default()
        }),
        delivery_proof: build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
            challenge_tag: Some(b"test-challenge"),
            weight_tag: Some(b"test-weight"),
            ..Default::default()
        }),
        identities: vec![vec![0x11; 32], vec![0x22; 32]],
        random: random.adapter,
    };

    // Act
    let output = run_mapping_vectors(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    assert_eq!(success.identity_mapping.len(), 2);
    assert_eq!(success.identity_mapping[0].identity, vec![0x11; 32]);
    assert_eq!(success.identity_mapping[1].identity, vec![0x22; 32]);
    for (index, record) in success.identity_mapping.iter().enumerate() {
        assert_eq!(record.scalar, output.identity_references[index]);
    }
}

/// Contract: under the asset scope an identity whose bytes do not convert into
///   32 takes the asset-identity-wrong-length branch with its index and length.
/// Arrange: the asset scope and identities [0x11; 32] and [0x22; 31], over the
///   operating system random source.
/// Act:     mapping_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  the result is Err(MappingVectorsErrorReturn::AssetIdentityLength
///   { index: 1, actual: 31 }).
#[test]
fn mapping_vectors_refuse_an_asset_identity_that_is_not_32_bytes() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = MappingVectorsPairingProbe {
        params: build_mapping_vectors_params(MappingVectorsParamsOverrides {
            scope: Some(IdentityScope::Asset),
            ..Default::default()
        }),
        key_agreement: build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
            possession_g1_tag: Some(b"test-possession-g1"),
            possession_g2_tag: Some(b"test-possession-g2"),
            ..Default::default()
        }),
        delivery_proof: build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
            challenge_tag: Some(b"test-challenge"),
            weight_tag: Some(b"test-weight"),
            ..Default::default()
        }),
        identities: vec![vec![0x11; 32], vec![0x22; 31]],
        random: random.adapter,
    };

    // Act
    let output = run_mapping_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    assert_eq!(
        output.result,
        Err(MappingVectorsErrorReturn::AssetIdentityLength {
            index: 1,
            actual: 31
        })
    );
}

/// Contract: under the asset scope an all-zero identity's refusal from
///   AssetIdentityHash::try_new is returned unchanged on the
///   asset-identity-refused branch.
/// Arrange: the asset scope and the identity [0x00; 32], over the operating
///   system random source.
/// Act:     mapping_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  the result is Err(MappingVectorsErrorReturn::AssetIdentityHash(
///   AssetIdentityHashTryNewErrorReturn::AllZero)).
#[test]
fn mapping_vectors_return_the_refusal_of_an_all_zero_asset_identity() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = MappingVectorsPairingProbe {
        params: build_mapping_vectors_params(MappingVectorsParamsOverrides {
            scope: Some(IdentityScope::Asset),
            ..Default::default()
        }),
        key_agreement: build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
            possession_g1_tag: Some(b"test-possession-g1"),
            possession_g2_tag: Some(b"test-possession-g2"),
            ..Default::default()
        }),
        delivery_proof: build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
            challenge_tag: Some(b"test-challenge"),
            weight_tag: Some(b"test-weight"),
            ..Default::default()
        }),
        identities: vec![vec![0x00; 32]],
        random: random.adapter,
    };

    // Act
    let output = run_mapping_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    assert_eq!(
        output.result,
        Err(MappingVectorsErrorReturn::AssetIdentityHash(
            AssetIdentityHashTryNewErrorReturn::AllZero
        ))
    );
}
