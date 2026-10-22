#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashConstructorParamsOverrides,
    AssetIdentityHashTryNewErrorReturn, build_asset_identity_hash, build_derivation_context,
    build_secret,
};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_mock_i_encoding_consumer, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    create_hash_to_scalar, expected_uniform_bytes,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn,
    DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    IssuePayloadOverrides, KemIdentity, RerandomizePayloadOverrides, SetupErrorReturn,
    SetupParamsOverrides, SetupPayloadOverrides, SetupScope, build_create_kem_deps,
    build_create_kem_payload, build_credential_components_params,
    build_credential_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_issue_params, build_issue_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_rerandomize_params, build_rerandomize_payload,
    build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload,
    EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, MockIPairingAdapterFailureMode, MulG1Params, MulG1Payload, MulG2Params,
    MulG2Payload, PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, FillBytesParams,
    FillBytesPayloadOverrides, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};

use harness_crypto::{
    RerandomizationErrorReturn, RerandomizationParamsOverrides, VectorCount,
    VectorCountConstructorParamsOverrides, build_rerandomization_deps,
    build_rerandomization_params, build_rerandomization_payload, build_vector_count,
    rerandomization,
};

/// Contract: own entry
///   `every_record_holds_true_verdicts_for_its_issued_and_rerandomized_credentials`,
///   as an outside caller observes it — `rerandomization` over a case with a
///   count above one, every adapter real and the operating-system source
///   real → every record's verdicts are `true`, its credentials valid under
///   its parameter set and identity element, and its rerandomized credential
///   distinct from its issued one.
/// Arrange: `count` of `2` from `build_vector_count` with
///   `VectorCountConstructorParamsOverrides`; every case — every member of
///   `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every selection of
///   `DECLARED_KEM_SELECTIONS` — the identity `b"pkg@1.0.0#1"` under the
///   entitlement scope and `vec![0x44; 32]` under the asset scope.
/// Act:     `rerandomization`, once per case, in the KEM consumer's method.
/// Assert:  `vectors` holds `2` records, and for each `issued_is_valid` and
///   `rerandomized_is_valid` are `true`, `is_valid` over
///   `build_is_valid_params` and `build_is_valid_payload` with the record's
///   parameter set, identity element, and `issued`, and again with
///   `rerandomized`, returns `true`, and the `encode_g2` bytes of the `b`
///   `credential_components` reads from `rerandomized` differ from those of
///   `issued`; each factory's result is unpacked by `expect` naming the
///   declared selection and the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Boundary: the public call `rerandomization`, with the route
///   `create_random_source`, the operating-system concrete's `try_new` and
///   `fill_bytes`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `create_kem`, the selected concrete's `try_new`, `rerandomization`, and
///   every KEM method it and the block call.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_holds_true_verdicts_for_its_issued_and_rerandomized_credentials() {
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
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
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.issued_is_valid);
                assert!(record.rerandomized_is_valid);
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
                let Ok(rerandomized_valid) = payload.adapter.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &record.parameter_set,
                        &record.identity_element,
                        &record.rerandomized,
                        Default::default(),
                    ),
                );
                assert!(rerandomized_valid.is_valid);
                assert_ne!(
                    encode_g2(&components_of(&record.issued).b),
                    encode_g2(&components_of(&record.rerandomized).b)
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
                                pairing: &payload.adapter,
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
    // every_record_holds_true_verdicts_for_its_issued_and_rerandomized_credentials
}

/// Contract: own entry
///   `every_run_emits_the_configured_count_of_records_with_distinct_draws`,
///   as an outside caller observes it — the list holds the configured count,
///   each record's `draws` hold its five uniforms each `UNIFORM_BYTES_LENGTH`
///   long, and no two records share `draws`.
/// Arrange: `count` of `3` from `build_vector_count`; every member of
///   `PAIRING_CONCRETES` with each selection and scope of
///   `DECLARED_KEM_SELECTIONS` and every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, the identity of each selection's
///   scope.
/// Act:     `rerandomization`, once per case, in the KEM consumer's method.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `5` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes, and
///   no two records share `draws`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `rerandomization`, with the route of the block
///   above.
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
            for record in &success.vectors {
                assert_eq!(
                    record.draws.len(),
                    5 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
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

/// Contract: own entry
///   `an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope`,
///   as an outside caller observes it — an identity that is not a 32-byte
///   array is refused in the function's `AssetIdentityLength` variant with
///   the identity's length, and an all-zero 32-byte identity is refused in
///   its `AssetIdentityHash` variant carrying the domain constructor's
///   `AllZero` unchanged, while a well-formed identity is admitted.
/// Arrange: each asset-scope selection of `DECLARED_KEM_SELECTIONS` over
///   every member of `PAIRING_CONCRETES` and every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the identities `vec![0x44; 31]`,
///   `vec![0x00; 32]`, and `vec![0x44; 32]` in turn, each with the `Err` it
///   must equal or `None` for the admitted identity.
/// Act:     `rerandomization`, once per identity, in the KEM consumer's
///   method.
/// Assert:  the results equal `Err(RerandomizationErrorReturn::
///   AssetIdentityLength { actual: 31 })` and
///   `Err(RerandomizationErrorReturn::AssetIdentityHash
///   (AssetIdentityHashTryNewErrorReturn::AllZero))` whole, and the third
///   `is_ok()`; each factory's result is unpacked by `expect` naming the
///   declared selection and the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Boundary: the public call `rerandomization`, with the route of the blocks
///   above.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        expected: &'a Option<RerandomizationErrorReturn>,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(returned.err(), *self.expected);
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
            let cases: [(Vec<u8>, Option<RerandomizationErrorReturn>); 3] = [
                (
                    vec![0x44; ASSET_IDENTITY_HASH_LENGTH - 1],
                    Some(RerandomizationErrorReturn::AssetIdentityLength {
                        actual: ASSET_IDENTITY_HASH_LENGTH - 1,
                    }),
                ),
                (
                    vec![0x00; ASSET_IDENTITY_HASH_LENGTH],
                    Some(RerandomizationErrorReturn::AssetIdentityHash(
                        AssetIdentityHashTryNewErrorReturn::AllZero,
                    )),
                ),
                (vec![0x44; ASSET_IDENTITY_HASH_LENGTH], None),
            ];
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    if kem_selection.scope != IdentityScope::Asset {
                        continue;
                    }
                    for (identity, expected) in &cases {
                        let created = create_kem(
                            &build_create_kem_deps(
                                &payload.adapter,
                                &*hashing.adapter,
                                KemConsumer {
                                    subject: &*subject_source.adapter,
                                    count: self.count,
                                    identity: identity.as_slice(),
                                    expected,
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
    // an_identity_that_is_not_an_asset_identity_hash_is_refused_whole_under_the_asset_scope
}

/// Contract: carried `kem/factory` entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`,
///   restated by the spec — the identity element's scalar is the scalar the
///   Keccak-256 identity mapping yields for the identity bytes the encoding
///   factory returns for the derivation context, under the tag the KEM
///   declares.
/// Arrange: each entitlement-scope selection of `DECLARED_KEM_SELECTIONS`
///   over every member of `PAIRING_CONCRETES`, each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`, and each case of
///   `DECLARED_ENCODING_SELECTIONS` that declares
///   `EncodingIdentifier::EthereumAbiV1`; the identity bytes are the bytes
///   `create_encoding` returns for the derivation context
///   `build_derivation_context` returns, under `DerivationContextDescription`,
///   through `build_mock_i_encoding_consumer`; `count` of `1`.
/// Act:     `rerandomization`, once per case, in the KEM consumer's method,
///   over `build_rerandomization_payload` of those bytes.
/// Assert:  `encode_scalar` of the record's identity element's `scalar`,
///   read through `identity_element_components`, equals `encode_scalar` of
///   the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds from
///   `K::DECLARATION.identity_tag`, those bytes, and `<P::Scalar as
///   ISampleUniformScalar>::UNIFORM_BYTES_LENGTH`; each factory's result is
///   unpacked by `expect` naming the declared selection and the outermost
///   `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `rerandomization`, with the route the spec
///   states for the entry.
/// Mocked:  the encoding family's official consumer mock, which hands the
///   bytes the encoding returned to the block; the operating-system
///   generator runs real.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
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
            assert_eq!(
                encode_scalar(&actual_identity_element.scalar),
                encode_scalar(&expected)
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
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
                if h2s_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                    continue;
                }
                let hashing = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_selection,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection constructs");
                for kem_selection in DECLARED_KEM_SELECTIONS {
                    if kem_selection.scope != IdentityScope::Entitlement {
                        continue;
                    }
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                pairing: &payload.adapter,
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: self.identity,
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
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let context = build_derivation_context(Default::default());
    for encoding_selection in DECLARED_ENCODING_SELECTIONS {
        if encoding_selection.identifier != EncodingIdentifier::EthereumAbiV1 {
            continue;
        }
        let description = build_derivation_context_description(Default::default());
        let identity = create_encoding(
            &build_create_encoding_deps(build_mock_i_encoding_consumer(
                &description,
                context.clone(),
                Vec::new(),
            )),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        )
        .expect("the declared selection constructs")
        .output
        .encoded
        .expect("the derivation context encodes")
        .bytes;
        for pairing_concrete in PAIRING_CONCRETES {
            // Act
            let created = create_pairing(
                &build_create_pairing_deps(
                    PairingConsumer {
                        count,
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
        }
    }
    // every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle
}

/// Contract: own entry `every_record_draw_replays_to_the_values_it_holds`,
///   as an outside caller observes it — the record's `draws`, split at
///   `UNIFORM_BYTES_LENGTH` into the uniforms of its calls in draw order and
///   replayed through the case's KEM over the configured identity,
///   reproduce the record's values; the credentials' `b` are the
///   second-group generator multiplied by the issuance scalar and, for the
///   rerandomization, shifted by the generator multiplied by the offset
///   scalar; and the identity element's scalar is the identity mapping
///   under the declared tag.
/// Arrange: `count` of `1` from `build_vector_count`; every case, each run
///   under the identities `b"pkg@1.0.0#1"` and `b"pkg@2.0.0#7"` under the
///   entitlement scope and `vec![0x44; 32]` and `vec![0x45; 32]` under the
///   asset scope, so a replay over an identity other than the configured one
///   reports otherwise.
/// Act:     `rerandomization`, once per case and identity, in the KEM
///   consumer's method.
/// Assert:  as the private integration test element's block of the same
///   name asserts, through the crate's public surface and the adapter
///   crates' public methods — the encoded `hpub`, identity element `scalar`
///   and `element`, and credentials' `a` and `b` of the replay each equal
///   the record's; `issued`'s `b` encodes to `mul_g2` of `g2_generator` by
///   the fourth uniform's scalar and `rerandomized`'s `b` to `add_g2` of the
///   issued `b` and `mul_g2` of `g2_generator` by the fifth uniform's
///   scalar; the identity element's `element` encodes to `add_g1` of `u0`
///   and `mul_g1` of `u1` by its `scalar`; and under the entitlement scope
///   for each `Keccak256V1` selection the identity element's `scalar`
///   encodes to the oracle's; each factory's result is unpacked by `expect`
///   naming the declared selection and the outermost `Result` `is_ok()`,
///   proving the consumers ran.
/// Boundary: the public call `rerandomization`, with the route of the blocks
///   above.
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
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
                                .expect("the record's draws hold five uniforms"),
                        ),
                        u0_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold five uniforms"),
                        ),
                        u1_uniform: Some(
                            uniforms
                                .next()
                                .expect("the record's draws hold five uniforms"),
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
                                    .expect("the record's draws hold five uniforms"),
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
                                    .expect("the record's draws hold five uniforms"),
                            ),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
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
            let expected_rerandomized = components_of(&rerandomized.credential);
            let actual_rerandomized = components_of(&record.rerandomized);
            assert_eq!(
                encode_g1(&actual_rerandomized.a),
                encode_g1(&expected_rerandomized.a)
            );
            assert_eq!(
                encode_g2(&actual_rerandomized.b),
                encode_g2(&expected_rerandomized.b)
            );
            let issuance_scalar = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(
                        record
                            .draws
                            .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                            .nth(3)
                            .expect("the record's draws hold five uniforms")
                            .to_vec(),
                        Default::default(),
                    ),
                },
            )
            .expect("the fourth uniform samples")
            .scalar
            .expose()
            .clone();
            let rerandomization_scalar =
                <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: build_secret(
                            record
                                .draws
                                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                                .nth(4)
                                .expect("the record's draws hold five uniforms")
                                .to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the fifth uniform samples")
                .scalar
                .expose()
                .clone();
            let Ok(g2) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(expected_issued_b) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: issuance_scalar,
                },
            );
            assert_eq!(
                encode_g2(&actual_issued.b),
                encode_g2(&expected_issued_b.product)
            );
            let Ok(offset_term) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: rerandomization_scalar,
                },
            );
            let Ok(expected_rerandomized_b) = self.pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: actual_issued.b.clone(),
                    right: offset_term.product,
                },
            );
            assert_eq!(
                encode_g2(&actual_rerandomized.b),
                encode_g2(&expected_rerandomized_b.sum)
            );
            let Ok(expected_identity_inner) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: actual_parameter_set.u1.clone(),
                    scalar: actual_identity_element.scalar.clone(),
                },
            );
            let Ok(expected_identity_element_point) = self.pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: actual_parameter_set.u0.clone(),
                    right: expected_identity_inner.product,
                },
            );
            assert_eq!(
                encode_g1(&actual_identity_element.element),
                encode_g1(&expected_identity_element_point.sum)
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

/// Contract: own entry `a_hash_to_scalar_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it — a hash-to-scalar refusal reaches the
///   caller inside the function's `DeriveIdentity` variant under the
///   entitlement scope and its `Setup` variant under the asset scope, whole.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`,
///   with `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `rerandomization`, once per case, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(error)))` under the entitlement
///   scope and `Some(RerandomizationErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` under the asset scope, whole; each factory's
///   result is unpacked by `expect` naming the declared selection and the
///   outermost `Result` `is_ok()`, proving the consumers ran.
/// Boundary: the public call `rerandomization`, with the route of the blocks
///   above.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration; the operating-system generator runs real.
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(match payload.scope() {
                    IdentityScope::Entitlement => RerandomizationErrorReturn::DeriveIdentity(
                        DeriveIdentityErrorReturn::HashToScalar(expected),
                    ),
                    IdentityScope::Asset =>
                        RerandomizationErrorReturn::Setup(SetupErrorReturn::HashToScalar(expected),),
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

/// Contract: own entry `a_draw_refusal_reaches_the_caller_whole`, as an
///   outside caller observes it — a `fill_bytes` refusal reaches the caller
///   inside the function's `FillBytes` variant at the master uniform's draw,
///   before any KEM call, whole.
/// Arrange: `count` of `1`; each entitlement-scope selection of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES` and
///   every selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the subject's
///   source `create_random_source` returns for
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, and, as the counterfactual, the source for
///   `RandomSourceKind::OperatingSystem`.
/// Act:     `rerandomization`, once per source, in the KEM consumer's
///   method.
/// Assert:  the result under the refusing source equals
///   `Err(RerandomizationErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` whole, and the result under the
///   operating-system source `is_ok()`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `rerandomization`, with the route of the blocks
///   above.
/// Mocked:  the randomness source, the outer edge, by the randomness
///   family's mock concrete selected by configuration.
#[test]
fn a_draw_refusal_reaches_the_caller_whole() {
    struct KemConsumer<'a> {
        refusing: &'a dyn IRandomSourceAdapter,
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
            let refused = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.refusing, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                refused.err(),
                Some(RerandomizationErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))
            );
            // Act
            let admitted = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert!(admitted.is_ok());
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
            let refusing_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the configured selection constructs");
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
                    if kem_selection.scope != IdentityScope::Entitlement {
                        continue;
                    }
                    let created = create_kem(
                        &build_create_kem_deps(
                            &payload.adapter,
                            &*hashing.adapter,
                            KemConsumer {
                                refusing: &*refusing_source.adapter,
                                subject: &*subject_source.adapter,
                                count: self.count,
                                identity: b"pkg@1.0.0#1",
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
    // a_draw_refusal_reaches_the_caller_whole
}

/// Contract: own entry `a_setup_sampling_refusal_reaches_the_caller_whole`,
///   as an outside caller observes it — a scalar-sampling refusal at setup's
///   master uniform reaches the caller inside the function's `Setup` variant
///   as `MasterScalarSampling`, whole.
/// Arrange: `count` of `1`; `create_pairing` over
///   `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`, with each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, and, as the counterfactual, each
///   member of `PAIRING_CONCRETES`; the expected `error` the `Err` that
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes`
///   returns, in the pairing consumer's method, for `UNIFORM_BYTES_LENGTH`
///   bytes `fill_bytes` draws from a further operating-system source.
/// Act:     `rerandomization`, once per pairing selection, in the KEM
///   consumer's method.
/// Assert:  the result under the refusing selection equals
///   `Err(RerandomizationErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(error)))` whole, and the result under each member
///   of `PAIRING_CONCRETES` `is_ok()`; each factory's result is unpacked by
///   `expect` naming the declared selection and the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Boundary: the public call `rerandomization`, with the route of the blocks
///   above.
/// Mocked:  the pairing family's mock concrete, selected by configuration;
///   the operating-system generator runs real.
#[test]
fn a_setup_sampling_refusal_reaches_the_caller_whole() {
    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        expected: &'a Option<SampleUniformScalarErrorReturn>,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                match self.expected {
                    Some(SampleUniformScalarErrorReturn::WrongLength { expected, actual }) => {
                        Some(RerandomizationErrorReturn::Setup(
                            SetupErrorReturn::MasterScalarSampling(
                                SampleUniformScalarErrorReturn::WrongLength {
                                    expected: *expected,
                                    actual: *actual,
                                },
                            ),
                        ))
                    }
                    None => None,
                }
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
            let probe_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::OperatingSystem),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the operating-system source constructs");
            // Arrange
            let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: probe_source
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            build_fill_bytes_payload(FillBytesPayloadOverrides {
                                length: Some(
                                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                ),
                            }),
                        )
                        .expect("the probe source fills")
                        .bytes,
                },
            )
            .err();
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
                                expected: &expected,
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
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    // Act
    let refused = create_pairing(
        &build_create_pairing_deps(PairingConsumer { count }, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::SampleScalarWrongLength,
            )),
            ..Default::default()
        }),
        build_create_pairing_payload(Default::default()),
    );
    // Assert
    assert!(refused.is_ok());
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
    // a_setup_sampling_refusal_reaches_the_caller_whole
}
