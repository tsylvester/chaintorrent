#![allow(clippy::expect_used)]

use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashConstructorParamsOverrides,
    build_asset_identity_hash, build_secret,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    create_hash_to_scalar, expected_uniform_bytes,
};
use kem::{
    CapsuleComponents, ConsumeKemParams, ConsumeKemPayload, CreateKemParams,
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    IssuePayloadOverrides, KemIdentity, RerandomizePayloadOverrides, SetupErrorReturn,
    SetupParamsOverrides, SetupPayloadOverrides, SetupScope, build_capsule_components_params,
    build_capsule_components_payload, build_create_kem_deps, build_create_kem_payload,
    build_credential_components_params, build_credential_components_payload,
    build_decapsulate_params, build_decapsulate_payload, build_derive_identity_params,
    build_derive_identity_payload, build_encapsulate_params, build_encapsulate_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_is_well_formed_params,
    build_is_well_formed_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_rerandomize_params, build_rerandomize_payload, build_setup_params, build_setup_payload,
    create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeGtParams, EncodeGtPayload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1Payload, PAIRING_CONCRETES, PairingProductParams, PairingProductPayload,
    PairingProductTerm, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, TargetGroupEncodingIdentifier, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};

use super::cross_holder;
use super::interface::CrossHolderErrorReturn;
use super::mock::{
    CrossHolderParamsOverrides, build_cross_holder_deps, build_cross_holder_params,
    build_cross_holder_payload,
};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: own entry
///   `every_record_recovers_the_encapsulated_value_from_its_issued_and_successor_credentials`;
///   `cross_holder` over a case with a count above one, every adapter real
///   and the operating-system source real → every record's credentials are
///   valid, its capsule well formed, and both decapsulated values equal the
///   encapsulated value.
/// Arrange: `count` of `2`; every case of the private surface — every member
///   of `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every selection of
///   `DECLARED_KEM_SELECTIONS`, the identity `b"pkg@1.0.0#1"` under the
///   entitlement scope and `vec![0x44; 32]` under the asset scope.
/// Act:     `cross_holder`, once per case, in the KEM consumer's method.
/// Assert:  `vectors` holds `2` records, and for each, `is_valid` over
///   `build_is_valid_payload` with the record's parameter set, identity
///   element, and `issued`, and again with `successor`, returns `true`,
///   `is_well_formed` over `build_is_well_formed_payload` with the record's
///   parameter set and capsule returns `true`, and the
///   `key_material().expose()` bytes of `issued_decapsulated` and of
///   `successor_decapsulated` each equal those of `encapsulated`; each
///   factory's result is unpacked by `expect` naming the declared selection
///   and the outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: `cross_holder` and the chain of real functions the private
///   surface names — `create_random_source` for the operating-system kind,
///   `create_pairing`, `create_hash_to_scalar`, `create_kem`, and every
///   adapter method the function calls, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_recovers_the_encapsulated_value_from_its_issued_and_successor_credentials() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                let Ok(issued_valid) = payload.adapter.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.issued,
                        Default::default(),
                    ),
                );
                assert!(issued_valid.is_valid);
                let Ok(successor_valid) = payload.adapter.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.successor,
                        Default::default(),
                    ),
                );
                assert!(successor_valid.is_valid);
                let Ok(well_formed) = payload.adapter.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &record.parameter_set,
                        &record.capsule,
                        Default::default(),
                    ),
                );
                assert!(well_formed.is_well_formed);
                assert_eq!(
                    record.issued_decapsulated.key_material().expose(),
                    record.encapsulated.key_material().expose()
                );
                assert_eq!(
                    record.successor_decapsulated.key_material().expose(),
                    record.encapsulated.key_material().expose()
                );
            }
        }
    }

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let identity: Vec<u8> = match kem_selection.scope {
                        IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                        IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                    };
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: identity.as_slice(),
                            },
                            Default::default(),
                        ),
                        *kem_selection,
                        build_create_kem_payload(Default::default()),
                    );
                    created.expect("the declared selection constructs");
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
    }
    // every_record_recovers_the_encapsulated_value_from_its_issued_and_successor_credentials
}

/// Contract: own entry `every_record_draw_replays_to_the_values_it_holds`;
///   the record's `draws`, split at `UNIFORM_BYTES_LENGTH` into the uniforms
///   of its calls in draw order and replayed through the case's KEM over the
///   configured identity, reproduce the record's values, the encapsulated
///   value is the pairing's target-group encoding of the product over the
///   encapsulation scalar and `hpub`, and the identity element's scalar is
///   the identity mapping under the declared tag.
/// Arrange: `count` of `1`; every case of the private surface, each run
///   under the identities `b"pkg@1.0.0#1"` and `b"pkg@2.0.0#7"` under the
///   entitlement scope and `vec![0x44; 32]` and `vec![0x45; 32]` under the
///   asset scope, so a replay over an identity other than the configured one
///   reports otherwise.
/// Act:     `cross_holder`, once per case and identity, in the KEM
///   consumer's method.
/// Assert:  the encoded `hpub`, identity element `scalar` and `element`,
///   credentials' `a` and `b`, and capsule members of the replay each equal
///   the record's, the replayed `key_material().expose()` bytes equal the
///   record's three, and `encapsulated`'s bytes equal `encode_gt` of
///   `pairing_product` over `PairingProductTerm { g1: mul_g1 of g1_generator
///   by the scalar sample_from_uniform_bytes returns for the sixth uniform,
///   g2: the record's hpub }`; under the entitlement scope for each
///   selection declaring `HashToScalarIdentifier::Keccak256V1`,
///   `encode_scalar` of the record's identity element's `scalar` equals
///   `encode_scalar` of the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds with
///   `DomainTagConstructorParamsOverrides { bytes: Some(K::DECLARATION.
///   identity_tag.to_vec()) }`, the configured identity bytes, and
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`; each
///   factory's result is unpacked by `expect` naming the declared selection
///   and the outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: `cross_holder` and the chain of real functions the private
///   surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_values_it_holds() {
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        h2s_identifier: HashToScalarIdentifier,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            let hash;
            let (setup_scope, identity) = match payload.scope() {
                IdentityScope::Entitlement => (
                    SetupScope::Entitlement,
                    KemIdentity::Entitlement {
                        canonical: self.identity,
                    },
                ),
                IdentityScope::Asset => {
                    hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                        bytes: Some(
                            <[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(self.identity)
                                .expect("the configured asset identity is a hash length"),
                        ),
                    });
                    (
                        SetupScope::Asset {
                            identity_hash: &hash,
                        },
                        KemIdentity::Asset {
                            identity_hash: &hash,
                        },
                    )
                }
            };
            let mut uniforms = record
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                .map(|chunk| build_secret(chunk.to_vec(), Default::default()));
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold six uniforms"),
                        ),
                        u0_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold six uniforms"),
                        ),
                        u1_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold six uniforms"),
                        ),
                    }),
                )
                .expect("the replay's setup succeeds");
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(identity),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold six uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's issuance succeeds");
            let rerandomized = payload
                .adapter
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        RerandomizePayloadOverrides {
                            uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold six uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
            let encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(
                                uniforms
                                    .next()
                                    .expect("the record's draws hold six uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
            let issued_decapsulated = payload
                .adapter
                .decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &derived.identity_element,
                        &issued.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                )
                .expect("the replay's decapsulation is infallible");
            let successor_decapsulated = payload
                .adapter
                .decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &derived.identity_element,
                        &rerandomized.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                )
                .expect("the replay's decapsulation is infallible");
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                self.pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: &P::G2| -> Vec<u8> {
                self.pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                self.pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: scalar.clone(),
                        },
                    )
                    .expect("the scalar encoding is infallible")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let expected_parameter_set = payload
                .adapter
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            let actual_parameter_set = payload
                .adapter
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &record.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the record's parameter set's components are infallible")
                .components;
            assert_eq!(
                encode_g2(&actual_parameter_set.hpub),
                encode_g2(&expected_parameter_set.hpub)
            );
            let expected_identity_element = payload
                .adapter
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let actual_identity_element = payload
                .adapter
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &record.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the record's identity element's components are infallible")
                .components;
            assert_eq!(
                encode_scalar(&actual_identity_element.scalar),
                encode_scalar(&expected_identity_element.scalar)
            );
            assert_eq!(
                encode_g1(&actual_identity_element.element),
                encode_g1(&expected_identity_element.element)
            );
            let components_of = |credential: &K::Credential| {
                payload
                    .adapter
                    .credential_components(
                        build_credential_components_params(Default::default()),
                        build_credential_components_payload(credential, Default::default()),
                    )
                    .expect("a credential's components are infallible")
                    .components
            };
            let expected_issued = components_of(&issued.credential);
            let actual_issued = components_of(&record.issued);
            assert_eq!(encode_g1(&actual_issued.a), encode_g1(&expected_issued.a));
            assert_eq!(encode_g2(&actual_issued.b), encode_g2(&expected_issued.b));
            let expected_successor = components_of(&rerandomized.credential);
            let actual_successor = components_of(&record.successor);
            assert_eq!(
                encode_g1(&actual_successor.a),
                encode_g1(&expected_successor.a)
            );
            assert_eq!(
                encode_g2(&actual_successor.b),
                encode_g2(&expected_successor.b)
            );
            let expected_capsule = payload
                .adapter
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&encapsulated.capsule, Default::default()),
                )
                .expect("the capsule's components are infallible")
                .components;
            let actual_capsule = payload
                .adapter
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&record.capsule, Default::default()),
                )
                .expect("the record's capsule's components are infallible")
                .components;
            assert_eq!(
                core::mem::discriminant(&actual_capsule),
                core::mem::discriminant(&expected_capsule)
            );
            match (actual_capsule, expected_capsule) {
                (
                    CapsuleComponents::Entitlement {
                        u: actual_u,
                        v: actual_v,
                        w: actual_w,
                    },
                    CapsuleComponents::Entitlement {
                        u: expected_u,
                        v: expected_v,
                        w: expected_w,
                    },
                ) => {
                    assert_eq!(encode_g2(&actual_u), encode_g2(&expected_u));
                    assert_eq!(encode_g1(&actual_v), encode_g1(&expected_v));
                    assert_eq!(encode_g1(&actual_w), encode_g1(&expected_w));
                }
                (
                    CapsuleComponents::Asset {
                        u: actual_u,
                        v: actual_v,
                    },
                    CapsuleComponents::Asset {
                        u: expected_u,
                        v: expected_v,
                    },
                ) => {
                    assert_eq!(encode_g2(&actual_u), encode_g2(&expected_u));
                    assert_eq!(encode_g1(&actual_v), encode_g1(&expected_v));
                }
                _ => {}
            }
            assert_eq!(
                record.encapsulated.key_material().expose(),
                encapsulated.encapsulated.key_material().expose()
            );
            assert_eq!(
                record.issued_decapsulated.key_material().expose(),
                issued_decapsulated.encapsulated.key_material().expose()
            );
            assert_eq!(
                record.successor_decapsulated.key_material().expose(),
                successor_decapsulated.encapsulated.key_material().expose()
            );
            let encapsulation_scalar =
                <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            record
                                .draws
                                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                                .nth(5)
                                .expect("the record's draws hold six uniforms")
                                .to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the sixth uniform samples")
                .scalar
                .expose()
                .clone();
            let Ok(g1) = self
                .pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(multiplied) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: encapsulation_scalar,
                },
            );
            let Ok(product) = self.pairing.pairing_product(
                PairingProductParams,
                PairingProductPayload {
                    terms: vec![PairingProductTerm {
                        g1: multiplied.product,
                        g2: actual_parameter_set.hpub,
                    }],
                },
            );
            let Ok(encoded) = self.pairing.encode_gt(
                EncodeGtParams,
                EncodeGtPayload {
                    value: product.product,
                },
            );
            assert_eq!(
                record.encapsulated.key_material().expose().as_slice(),
                encoded.bytes.expose().as_ref()
            );
            if self.h2s_identifier == HashToScalarIdentifier::Keccak256V1
                && payload.scope() == IdentityScope::Entitlement
            {
                let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                    bytes: Some(K::DECLARATION.identity_tag.to_vec()),
                });
                let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            expected_uniform_bytes(
                                &tag,
                                self.identity,
                                <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            ),
                            Default::default(),
                        ),
                    },
                )
                .expect("the expected uniform bytes sample")
                .scalar
                .expose()
                .clone();
                assert_eq!(
                    encode_scalar(&actual_identity_element.scalar),
                    encode_scalar(&expected)
                );
            }
        }
    }

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let identities: Vec<Vec<u8>> = match kem_selection.scope {
                        IdentityScope::Entitlement => {
                            vec![b"pkg@1.0.0#1".to_vec(), b"pkg@2.0.0#7".to_vec()]
                        }
                        IdentityScope::Asset => vec![
                            vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                            vec![0x45; ASSET_IDENTITY_HASH_LENGTH],
                        ],
                    };
                    for identity in &identities {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    pairing: &payload.adapter,
                                    subject: &*subject_source.adapter,
                                    count: self.count,
                                    identity: identity.as_slice(),
                                    h2s_identifier: h2s_selection.identifier,
                                },
                                Default::default(),
                            ),
                            *kem_selection,
                            build_create_kem_payload(Default::default()),
                        );
                        created.expect("the declared selection constructs");
                    }
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
    }
    // every_record_draw_replays_to_the_values_it_holds
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`;
///   the list holds the configured count, each record's `draws` hold its six
///   uniforms each `UNIFORM_BYTES_LENGTH` long, and no two records share
///   `draws`.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES` crossed
///   with the declared hash-to-scalar and KEM selections, the identity of
///   each selection's scope.
/// Act:     `cross_holder`, once per case, in the KEM consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `6` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: `cross_holder` and the chain of real functions the private
///   surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
            for record in &success.vectors {
                assert_eq!(
                    record.draws.len(),
                    6 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                );
            }
            for left in &success.vectors {
                for right in &success.vectors {
                    if !core::ptr::eq(left, right) {
                        assert_ne!(left.draws, right.draws);
                    }
                }
            }
        }
    }

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    let identity: Vec<u8> = match kem_selection.scope {
                        IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                        IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                    };
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: identity.as_slice(),
                            },
                            Default::default(),
                        ),
                        *kem_selection,
                        build_create_kem_payload(Default::default()),
                    );
                    created.expect("the declared selection constructs");
                }
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
    }
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`;
///   a hash-to-scalar refusal reaches the caller inside the function's
///   `DeriveIdentity` variant under the entitlement scope and its `Setup`
///   variant under the asset scope, whole.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   with `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `cross_holder`, once per case, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(CrossHolderErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::
///   HashToScalar(error)))` under the entitlement scope and
///   `Some(CrossHolderErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` under the asset scope, whole; each factory's
///   result is unpacked by `expect` naming the declared selection and the
///   outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: `cross_holder` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing at the outer edge — the operating-system generator runs
///   real; the hash-to-scalar family's mock concrete is selected by
///   configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(match payload.scope() {
                    IdentityScope::Entitlement => CrossHolderErrorReturn::DeriveIdentity(
                        DeriveIdentityErrorReturn::HashToScalar(expected),
                    ),
                    IdentityScope::Asset =>
                        CrossHolderErrorReturn::Setup(SetupErrorReturn::HashToScalar(expected),),
                })
            );
        }
    }

    struct PairingConsumer {
        count: VectorCount,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the configured selection constructs");
            for kem_selection in DECLARED_KEM_SELECTIONS {
                let identity: Vec<u8> = match kem_selection.scope {
                    IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
                    IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
                };
                let created = create_kem(
                    &build_create_kem_deps(
                        &payload.adapter,
                        &*hashing.adapter,
                        KemConsumer {
                            subject: &*subject_source.adapter,
                            count: self.count,
                            identity: identity.as_slice(),
                        },
                        Default::default(),
                    ),
                    *kem_selection,
                    build_create_kem_payload(Default::default()),
                );
                created.expect("the declared selection constructs");
            }
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    for pairing_concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*pairing_concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        // Assert
        assert!(created.is_ok());
    }
    // a_hash_to_scalar_refusal_reaches_the_caller_whole
}

/// Contract: own entry
///   `records_under_pairings_sharing_a_target_group_encoding_hold_the_same_encapsulated_bytes`;
///   identical draws under two pairing concretes whose declared target-group
///   encoding identifiers are equal yield records whose `encapsulated` bytes
///   are equal.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES`, each run
///   with its own fresh source from `create_random_source` at the defaults
///   of `build_create_random_source_params`, the randomness family's mock
///   concrete, so every run draws the same uniforms; each selection of
///   `DECLARED_KEM_SELECTIONS` under the one scope it declares; the KEM
///   consumer returns the pairing's declared target-group encoding
///   identifier with the `key_material()` bytes of the record's
///   `encapsulated`, collected across the members.
/// Act:     `cross_holder`, once per pairing member and selection, in the
///   KEM consumer's method.
/// Assert:  any two members whose identifiers are equal hold equal bytes;
///   each factory's result is unpacked by `expect` naming the declared
///   selection and the outermost `Result` `is_ok()`, proving the consumers
///   ran.
/// Boundary: `cross_holder` and the chain of real functions the private
///   surface names.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn records_under_pairings_sharing_a_target_group_encoding_hold_the_same_encapsulated_bytes() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = (TargetGroupEncodingIdentifier, Vec<u8>);

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(&payload.adapter, self.subject, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_cross_holder_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            (
                P::DECLARATION.target_group_encoding,
                record.encapsulated.key_material().expose().clone(),
            )
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        kem_selection: CreateKemParams,
        identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = Vec<(TargetGroupEncodingIdentifier, Vec<u8>)>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut collected = Vec::new();
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                let subject_source = create_random_source(
                    &build_create_random_source_deps(Default::default()),
                    build_create_random_source_params(Default::default()),
                    build_create_random_source_payload(Default::default()),
                )
                .expect("the random-source factory's default selection constructs");
                let created = create_kem(
                    &build_create_kem_deps(
                        &payload.adapter,
                        &*hashing.adapter,
                        KemConsumer {
                            subject: &*subject_source.adapter,
                            count: self.count,
                            identity: self.identity,
                        },
                        Default::default(),
                    ),
                    self.kem_selection,
                    build_create_kem_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                collected.push(created.output);
            }
            collected
        }
    }

    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    for kem_selection in DECLARED_KEM_SELECTIONS {
        let identity: Vec<u8> = match kem_selection.scope {
            IdentityScope::Entitlement => b"pkg@1.0.0#1".to_vec(),
            IdentityScope::Asset => vec![0x44; ASSET_IDENTITY_HASH_LENGTH],
        };
        let mut collected: Vec<(TargetGroupEncodingIdentifier, Vec<u8>)> = Vec::new();
        for pairing_concrete in PAIRING_CONCRETES {
            // Act
            let created = create_pairing(
                &build_create_pairing_deps(
                    PairingConsumer {
                        count,
                        kem_selection: *kem_selection,
                        identity: identity.as_slice(),
                    },
                    Default::default(),
                ),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                build_create_pairing_payload(Default::default()),
            );
            // Assert
            assert!(created.is_ok());
            let outcome = created.expect("the declared selection constructs");
            collected.extend(outcome.output);
        }
        for left in &collected {
            for right in &collected {
                if left.0 == right.0 {
                    assert_eq!(left.1, right.1);
                }
            }
        }
    }
    // records_under_pairings_sharing_a_target_group_encoding_hold_the_same_encapsulated_bytes
}
