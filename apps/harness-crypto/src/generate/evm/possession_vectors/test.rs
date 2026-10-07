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
    PossessionVectorFromFieldsErrorReturn, PossessionVectorsDeps, PossessionVectorsParams,
    PossessionVectorsPayload, PossessionVectorsReturn,
};
use super::mock::{
    PossessionVectorsParamsOverrides, build_possession_vector, build_possession_vector_description,
    build_possession_vectors_params,
};
use super::possession_vectors;
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CanonicalFields, ConsumeEncodingParams,
    ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload, FromFieldsParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, IEncodingContract, ToFieldsParams,
    build_create_encoding_params, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementDeps,
    CreateKeyAgreementPayload, IKeyAgreementAdapter, IKeyAgreementConsumer, PossessionComponents,
    PossessionFromComponentsParams, PossessionFromComponentsPayload, PublicKeysComponents,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload,
    build_create_key_agreement_params, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeG1Params, DecodeG2Params, DecodeScalarParams, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, PairingConcrete,
    build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

struct PossessionVectorsProbeOutput {
    result: PossessionVectorsReturn,
    pk1_references: Vec<Vec<u8>>,
    pk2_references: Vec<Vec<u8>>,
    admitted_records: usize,
}

struct PossessionVectorsPairingProbe {
    params: PossessionVectorsParams,
    random: Box<dyn IRandomSourceAdapter>,
}

impl IPairingConsumer for PossessionVectorsPairingProbe {
    type Output = PossessionVectorsProbeOutput;

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
        let Ok(encoding) = create_encoding(
            &CreateEncodingDeps {
                consumer: PossessionVectorsEncodingProbe {
                    pairing: &payload.adapter,
                    hash_to_scalar: hashing.adapter.as_ref(),
                    probe: self,
                },
            },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is admitted")
        };
        encoding.output
    }
}

struct PossessionVectorsEncodingProbe<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    probe: &'a PossessionVectorsPairingProbe,
}

impl<P: IPairingArithmetic> IEncodingConsumer for PossessionVectorsEncodingProbe<'_, P> {
    type Output = PossessionVectorsProbeOutput;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(key_agreement) = create_key_agreement(
            &CreateKeyAgreementDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.probe.random.as_ref(),
                consumer: PossessionVectorsKeyAgreementProbe {
                    pairing: self.pairing,
                    probe: self.probe,
                },
            },
            build_create_key_agreement_params(Default::default()),
            CreateKeyAgreementPayload,
        ) else {
            panic!("the key-agreement concrete is admitted")
        };
        key_agreement.output
    }
}

struct PossessionVectorsKeyAgreementProbe<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    probe: &'a PossessionVectorsPairingProbe,
}

impl<P: IPairingArithmetic> IKeyAgreementConsumer<P> for PossessionVectorsKeyAgreementProbe<'_, P> {
    type Output = PossessionVectorsProbeOutput;

    fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<K>,
    ) -> Self::Output {
        let mut output = PossessionVectorsProbeOutput {
            result: possession_vectors::<P, K>(
                &PossessionVectorsDeps {
                    pairing: self.pairing,
                    key_agreement: &payload.adapter,
                    random: self.probe.random.as_ref(),
                },
                self.probe.params,
                PossessionVectorsPayload,
            ),
            pk1_references: Vec::new(),
            pk2_references: Vec::new(),
            admitted_records: 0,
        };
        let Ok(success) = &output.result else {
            return output;
        };
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        for record in &success.records {
            let Ok(x) = self.pairing.decode_scalar(DecodeScalarParams, &record.x) else {
                panic!("each record's x decodes")
            };
            let Ok(y) = self.pairing.decode_scalar(DecodeScalarParams, &record.y) else {
                panic!("each record's y decodes")
            };
            let Ok(pk1) = self.pairing.decode_g1(DecodeG1Params, &record.pk1) else {
                panic!("each record's pk1 decodes")
            };
            let Ok(pk2) = self.pairing.decode_g2(DecodeG2Params, &record.pk2) else {
                panic!("each record's pk2 decodes")
            };
            let Ok(r1) = self.pairing.decode_g1(DecodeG1Params, &record.r1) else {
                panic!("each record's r1 decodes")
            };
            let Ok(r2) = self.pairing.decode_g2(DecodeG2Params, &record.r2) else {
                panic!("each record's r2 decodes")
            };
            let Ok(z1) = self.pairing.decode_scalar(DecodeScalarParams, &record.z1) else {
                panic!("each record's z1 decodes")
            };
            let Ok(z2) = self.pairing.decode_scalar(DecodeScalarParams, &record.z2) else {
                panic!("each record's z2 decodes")
            };
            let Ok(pk1_product) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point.clone(),
                    scalar: x.scalar.clone(),
                },
            );
            let Ok(pk1_encoded) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: pk1_product.product,
                },
            );
            output
                .pk1_references
                .push(pk1_encoded.bytes.as_ref().to_vec());
            let Ok(pk2_product) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: y.scalar.clone(),
                },
            );
            let Ok(pk2_encoded) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: pk2_product.product,
                },
            );
            output
                .pk2_references
                .push(pk2_encoded.bytes.as_ref().to_vec());
            let Ok(possession) = payload.adapter.possession_from_components(
                PossessionFromComponentsParams,
                PossessionFromComponentsPayload {
                    components: PossessionComponents {
                        r1: r1.point,
                        z1: z1.scalar,
                        r2: r2.point,
                        z2: z2.scalar,
                    },
                },
            );
            let Ok(_) = payload.adapter.public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: pk1.point,
                        pk2: pk2.point,
                    },
                    possession: &possession.possession,
                },
            ) else {
                panic!("each record's keys are admitted")
            };
            output.admitted_records += 1;
        }
        output
    }
}

fn run_possession_vectors(
    probe: PossessionVectorsPairingProbe,
    concrete: PairingConcrete,
) -> PossessionVectorsProbeOutput {
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

/// Contract: the description's mapped branch carries x and y as Unsigned256,
///   pk1, pk2, and r1 as Bytes, z1 as Unsigned256, r2 as Bytes, and z2 as
///   Unsigned256, in member order.
/// Arrange: the default possession vector.
/// Act:     the description's to_fields over the record.
/// Assert:  the values are Unsigned256([0x01; 32]), Unsigned256([0x02; 32]),
///   Bytes(vec![0x03; 64]), Bytes(vec![0x04; 128]), Bytes(vec![0x05; 64]),
///   Unsigned256([0x06; 32]), Bytes(vec![0x07; 128]), and
///   Unsigned256([0x08; 32]) in that order.
#[test]
fn possession_vector_description_maps_secrets_keys_and_proof_components_in_member_order() {
    // Arrange
    let vector = build_possession_vector(Default::default());
    let description = build_possession_vector_description();

    // Act
    let Ok(success) = description.to_fields(ToFieldsParams, &vector);

    // Assert
    assert_eq!(
        success.fields.values,
        vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Bytes(vec![0x03; 64]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
            CanonicalFieldValue::Unsigned256([0x08; 32]),
        ]
    );
}

/// Contract: the description's rebuilt branch moves each field back into its
///   record member.
/// Arrange: the eight canonical fields of the default possession vector.
/// Act:     the description's fields_to_value over them.
/// Assert:  the described record equals the default possession vector.
#[test]
fn possession_vector_description_rebuilds_the_record_from_its_fields() {
    // Arrange
    let fields = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Bytes(vec![0x03; 64]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
            CanonicalFieldValue::Unsigned256([0x08; 32]),
        ],
    };
    let description = build_possession_vector_description();

    // Act
    let rebuilt = description.fields_to_value(FromFieldsParams, fields);

    // Assert
    let Ok(success) = rebuilt else {
        panic!("the fields rebuild the record")
    };
    assert_eq!(
        success.described,
        build_possession_vector(Default::default())
    );
}

/// Contract: the description's count branch refuses a field vector that does
///   not hold eight values, and its kind branch refuses the lowest index whose
///   value is not the kind the field kinds hold there.
/// Arrange: the first seven of the default record's canonical fields; and the
///   eight with the value at index 2 replaced by Unsigned256([0x03; 32]).
/// Act:     the description's fields_to_value over each field set.
/// Assert:  the first error is FieldCount { expected: 8, actual: 7 }; the
///   second is FieldKind { index: 2, expected: CanonicalFieldKind::Bytes }.
#[test]
fn possession_vector_description_refuses_a_wrong_field_count_and_a_field_of_the_wrong_kind() {
    // Arrange
    let short = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Bytes(vec![0x03; 64]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
        ],
    };
    let wrong_kind = CanonicalFields {
        values: vec![
            CanonicalFieldValue::Unsigned256([0x01; 32]),
            CanonicalFieldValue::Unsigned256([0x02; 32]),
            CanonicalFieldValue::Unsigned256([0x03; 32]),
            CanonicalFieldValue::Bytes(vec![0x04; 128]),
            CanonicalFieldValue::Bytes(vec![0x05; 64]),
            CanonicalFieldValue::Unsigned256([0x06; 32]),
            CanonicalFieldValue::Bytes(vec![0x07; 128]),
            CanonicalFieldValue::Unsigned256([0x08; 32]),
        ],
    };
    let description = build_possession_vector_description();

    // Act
    let short = description.fields_to_value(FromFieldsParams, short);
    let wrong_kind = description.fields_to_value(FromFieldsParams, wrong_kind);

    // Assert
    let Err(error) = short else {
        panic!("the wrong field count is refused")
    };
    assert_eq!(
        error,
        PossessionVectorFromFieldsErrorReturn::FieldCount {
            expected: 8,
            actual: 7
        }
    );
    let Err(error) = wrong_kind else {
        panic!("the wrong field kind is refused")
    };
    assert_eq!(
        error,
        PossessionVectorFromFieldsErrorReturn::FieldKind {
            index: 2,
            expected: CanonicalFieldKind::Bytes,
        }
    );
}

/// Contract: the description's members branch pairs each member name with its
///   field kind at the same index — x, y, z1, and z2 of kind Unsigned256 and
///   pk1, pk2, r1, and r2 of kind Bytes, in member order.
/// Arrange: the possession-vector description.
/// Act:     members() on it.
/// Assert:  it equals members x and y of kind Unsigned256, pk1, pk2, and r1 of
///   kind Bytes, z1 of kind Unsigned256, r2 of kind Bytes, and z2 of kind
///   Unsigned256, in that order.
#[test]
fn possession_vector_description_declares_its_members() {
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
    let Ok(members) = build_possession_vector_description().members() else {
        panic!("the member names are admitted")
    };

    // Assert
    assert_eq!(
        members,
        build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
            members: Some(vec![
                member("x", CanonicalFieldKind::Unsigned256),
                member("y", CanonicalFieldKind::Unsigned256),
                member("pk1", CanonicalFieldKind::Bytes),
                member("pk2", CanonicalFieldKind::Bytes),
                member("r1", CanonicalFieldKind::Bytes),
                member("z1", CanonicalFieldKind::Unsigned256),
                member("r2", CanonicalFieldKind::Bytes),
                member("z2", CanonicalFieldKind::Unsigned256),
            ]),
        })
    );
}

/// Contract: the computed branch emits one record per index below the
///   configured count, each drawn fresh.
/// Arrange: a count of 3, over the operating system random source.
/// Act:     possession_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  records holds three records and no two records share an x.
#[test]
fn possession_vectors_emit_the_configured_count_with_distinct_secrets() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = PossessionVectorsPairingProbe {
        params: build_possession_vectors_params(PossessionVectorsParamsOverrides {
            count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(3),
            })),
        }),
        random: random.adapter,
    };

    // Act
    let output = run_possession_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    assert_eq!(success.records.len(), 3);
    for first in 0..success.records.len() {
        for second in first + 1..success.records.len() {
            assert_ne!(success.records[first].x, success.records[second].x);
        }
    }
}

/// Contract: the computed branch carries each record's first-group encodings
///   in the EIP-196 form and its second-group encodings in the EIP-197 form.
/// Arrange: the default params, over the operating system random source.
/// Act:     possession_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  each record's pk1 and r1 hold 64 bytes and its pk2 and r2 hold
///   128 bytes.
#[test]
fn possession_vectors_size_bn254_records_in_the_eip_196_and_eip_197_encodings() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = PossessionVectorsPairingProbe {
        params: build_possession_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_possession_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    for record in &success.records {
        assert_eq!(record.pk1.len(), 64);
        assert_eq!(record.r1.len(), 64);
        assert_eq!(record.pk2.len(), 128);
        assert_eq!(record.r2.len(), 128);
    }
}

/// Contract: the computed branch carries each record's first-group encodings
///   and its second-group encodings in the EIP-2537 forms.
/// Arrange: the default params, over the operating system random source.
/// Act:     possession_vectors through the probe over
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  each record's pk1 and r1 hold 128 bytes and its pk2 and r2 hold
///   256 bytes.
#[test]
fn possession_vectors_size_bls12_381_records_in_the_eip_2537_encodings() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = PossessionVectorsPairingProbe {
        params: build_possession_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_possession_vectors(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    for record in &success.records {
        assert_eq!(record.pk1.len(), 128);
        assert_eq!(record.r1.len(), 128);
        assert_eq!(record.pk2.len(), 256);
        assert_eq!(record.r2.len(), 256);
    }
}

/// Contract: the computed branch's records carry the public keys that are each
///   group's generator multiplied by the record's secret.
/// Arrange: the default params, over the operating system random source.
/// Act:     possession_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  each record's pk1 equals its entry of pk1_references and its pk2
///   its entry of pk2_references.
#[test]
fn possession_vectors_carry_public_keys_that_are_the_generators_times_their_secrets() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = PossessionVectorsPairingProbe {
        params: build_possession_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_possession_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    for (index, record) in success.records.iter().enumerate() {
        assert_eq!(record.pk1, output.pk1_references[index]);
        assert_eq!(record.pk2, output.pk2_references[index]);
    }
}

/// Contract: the computed branch's records carry proofs of possession the key
///   agreement admits when it rebuilds each record's public keys from their
///   decoded components.
/// Arrange: the default params, over the operating system random source.
/// Act:     possession_vectors through the probe over
///   PairingConcrete::Bn254Arkworks.
/// Assert:  admitted_records equals the number of records.
#[test]
fn possession_vectors_carry_proofs_the_key_agreement_admits() {
    // Arrange
    let Ok(random) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let probe = PossessionVectorsPairingProbe {
        params: build_possession_vectors_params(Default::default()),
        random: random.adapter,
    };

    // Act
    let output = run_possession_vectors(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = &output.result else {
        panic!("the vectors are computed")
    };
    assert_eq!(output.admitted_records, success.records.len());
}
