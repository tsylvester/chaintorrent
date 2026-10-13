#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{Secret, build_asset_identity_hash, build_derivation_context, build_secret};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    build_derivation_context_description, build_mock_i_encoding_consumer, create_encoding,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides,
    HashToScalarIdentifier, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_payload,
    build_domain_tag, create_hash_to_scalar, expected_uniform_bytes,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemErrorReturn, CreateKemParams,
    CreateKemParamsOverrides, DECLARED_KEM_SELECTIONS, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    IssuePayloadOverrides, KEM_INTERFACE_VERSION, KemIdentifier, KemIdentity, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, build_decapsulate_params, build_decapsulate_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_is_well_formed_params, build_is_well_formed_payload, build_issue_params,
    build_issue_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingConsumer,
    ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    TargetGroupEncodingIdentifier, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload, create_random_source,
};

/// Contract: own entry
///   `every_concrete_is_constructed_under_the_identifier_and_scope_it_declares`,
///   over the declared set; `create_kem` under a case's selection, declared
///   identifier, and declared scope yields `Ok` whose consumer reads
///   `K::DECLARATION.identifier` equal to `params.identifier`,
///   `K::DECLARATION.interface_version` equal to `KEM_INTERFACE_VERSION`,
///   `K::DECLARATION.identity_scopes` containing `params.scope`, and
///   `payload.scope()` equal to `params.scope`.
/// Arrange: each case of `DECLARED_KEM_SELECTIONS` with its declared identifier
///   and scope, over every case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` and
///   every member of `PAIRING_CONCRETES`; the cases' declared identifiers
///   differ, so a construction of another concrete reports another identifier.
/// Act:     `create_kem` per case, inside `PairingConsumer`.
/// Assert:  in `KemConsumer`, the four declaration and scope reads above by
///   `assert_eq!`/`assert!`; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `create_pairing`,
///   the arm's concrete through its `try_new`, `create_hash_to_scalar`, the
///   selected concrete's `try_new`, `create_kem`.
/// Mocked:  nothing.
#[test]
fn every_concrete_is_constructed_under_the_identifier_and_scope_it_declares() {
    // Arrange
    struct KemConsumer {
        params: CreateKemParams,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            assert!(K::DECLARATION.identifier == self.params.identifier);
            assert_eq!(K::DECLARATION.interface_version, KEM_INTERFACE_VERSION);
            assert!(K::DECLARATION.identity_scopes.contains(&self.params.scope));
            assert_eq!(payload.scope(), self.params.scope);
        }
    }

    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in DECLARED_KEM_SELECTIONS {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer { params: *case },
                            Default::default(),
                        ),
                        *case,
                        build_create_kem_payload(Default::default()),
                    );
                    assert!(created.is_ok());
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: own entry
///   `every_concrete_is_refused_under_an_identifier_it_does_not_declare`, over
///   the declared set; `create_kem` under a case's selection and an identifier
///   no declared concrete declares yields
///   `Err(CreateKemErrorReturn::UnsupportedKemIdentifier)` and the consumer
///   never runs.
/// Arrange: each case of `DECLARED_KEM_SELECTIONS` with `KemIdentifier::Mock`,
///   which no declared concrete declares, over every case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` and every member of
///   `PAIRING_CONCRETES`; `KemConsumer` is the deps' consumer and its body
///   fails the block if run.
/// Act:     `create_kem` per case, inside `PairingConsumer`.
/// Assert:  `assert_eq!(result.err(),
///   Some(CreateKemErrorReturn::UnsupportedKemIdentifier))` per case; outside
///   the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route as
///   `every_concrete_is_constructed_under_the_identifier_and_scope_it_declares`.
/// Mocked:  nothing.
#[test]
fn every_concrete_is_refused_under_an_identifier_it_does_not_declare() {
    // Arrange
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            _payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            unreachable!("the refusal returns before the consumer runs")
        }
    }

    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in DECLARED_KEM_SELECTIONS {
                    let result = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
                            Default::default(),
                        ),
                        build_create_kem_params(CreateKemParamsOverrides {
                            concrete: Some(case.concrete),
                            identifier: Some(KemIdentifier::Mock),
                            scope: Some(case.scope),
                        }),
                        build_create_kem_payload(Default::default()),
                    );
                    assert_eq!(
                        result.err(),
                        Some(CreateKemErrorReturn::UnsupportedKemIdentifier)
                    );
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: own entry `a_kem_from_the_factory_round_trips_from_operating_system_draws`;
///   every uniform draw `setup`, `issue`, and `encapsulate` consume is the bytes
///   `fill_bytes` returns for `P::Scalar::UNIFORM_BYTES_LENGTH` from the adapter
///   `create_random_source` returns for the operating-system kind, one draw per
///   sampler, and the round trip `setup`, `derive_identity`, `issue`,
///   `encapsulate`, `is_valid`, `is_well_formed`, `decapsulate` returns `true`
///   for the verdicts and `encapsulate`'s bytes for `decapsulate`.
/// Arrange: each case of `DECLARED_KEM_SELECTIONS` over every member of
///   `PAIRING_CONCRETES` and every case of `DECLARED_HASH_TO_SCALAR_SELECTIONS`;
///   the random source the operating system's generator backs, created once in
///   the arrangement.
/// Act:     in `KemConsumer`, `decapsulate` with the credential and capsule the
///   arrangement produced.
/// Assert:  `is_valid` and `is_well_formed` returned `true`, and `decapsulate`'s
///   `key_material().expose()` bytes equal `encapsulate`'s; outside the
///   consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `create_random_source`,
///   the operating-system concrete's `try_new` and `fill_bytes`,
///   `create_pairing`, the arm's concrete through its `try_new`,
///   `create_hash_to_scalar`, the selected concrete's `try_new`, `create_kem`,
///   and every method the consumer calls.
/// Mocked:  nothing; the operating system's generator is the outer edge and
///   runs real, so this block proves the draws reach the samplers and not the
///   generator's quality.
#[test]
fn a_kem_from_the_factory_round_trips_from_operating_system_draws() {
    // Arrange
    let random = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the operating-system kind constructs its source")
    .adapter;

    struct KemConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let draw = || -> Secret<Vec<u8>> {
                self.random
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(P::Scalar::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the operating-system draw fills the uniform input")
                    .bytes
            };
            let scope = payload.scope();
            let hash = build_asset_identity_hash(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset {
                                identity_hash: &hash,
                            },
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("setup admits the scope the case selects and the drawn inputs");
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        match scope {
                            IdentityScope::Entitlement => DeriveIdentityPayloadOverrides::default(),
                            IdentityScope::Asset => DeriveIdentityPayloadOverrides {
                                identity: Some(KemIdentity::Asset {
                                    identity_hash: &hash,
                                }),
                            },
                        },
                    ),
                )
                .expect("the identity the set fixes derives");
            let issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("issue admits the credential inputs");
            let encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("encapsulate admits the drawn input");
            let Ok(valid) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &issued.credential,
                    Default::default(),
                ),
            );
            assert!(valid.is_valid);
            let Ok(well_formed) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &setup.parameter_set,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(well_formed.is_well_formed);

            // Act
            let Ok(decapsulated) = payload.adapter.decapsulate(
                build_decapsulate_params(Default::default()),
                build_decapsulate_payload(
                    &derived.identity_element,
                    &issued.credential,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );

            // Assert
            assert_eq!(
                decapsulated.encapsulated.key_material().expose(),
                encapsulated.encapsulated.key_material().expose()
            );
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in DECLARED_KEM_SELECTIONS {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer {
                                random: self.random,
                            },
                            Default::default(),
                        ),
                        *case,
                        build_create_kem_payload(Default::default()),
                    );
                    assert!(created.is_ok());
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps =
            build_create_pairing_deps(PairingConsumer { random: &*random }, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: carried `pairing/factory` entry
///   `the_consumer_receives_the_concrete_the_params_name`, restated by the spec;
///   the pairing the adapter `create_kem` hands the consumer computes over is
///   the concrete the pairing params name, the consumer's own computation with
///   the pairing it holds agreeing with the adapter's.
/// Arrange: each member of `PAIRING_CONCRETES` with each case of
///   `DECLARED_KEM_SELECTIONS`, over every case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the consumer holds the pairing the
///   deps borrow and reads its `g1_generator` and `g2_generator`; `setup` runs
///   in the arrangement under the case's scope.
/// Act:     `parameter_set_components` on the parameter set `setup` returned.
/// Assert:  `encode_g1` of the components' `g1` equals `encode_g1` of the
///   consumer's `g1_generator`, and `encode_g2` of `g2` equals `encode_g2` of
///   its `g2_generator`, so the adapter computes over the same pairing
///   concrete; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `create_pairing`, the
///   arm's concrete through its `try_new`, `create_hash_to_scalar`, the
///   selected concrete's `try_new`, `create_kem`, `setup`, and
///   `parameter_set_components`.
/// Mocked:  nothing.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    // Arrange
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
            let hash = build_asset_identity_hash(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset {
                                identity_hash: &hash,
                            },
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("setup admits the scope the case selects and the uniform inputs");
            let Ok(components) = payload.adapter.parameter_set_components(
                build_parameter_set_components_params(Default::default()),
                build_parameter_set_components_payload(&setup.parameter_set, Default::default()),
            );
            let Ok(g1_generator) = self
                .pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2_generator) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(set_g1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: components.components.g1,
                },
            );
            let Ok(own_g1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: g1_generator.point,
                },
            );
            assert_eq!(
                set_g1.bytes.as_ref().to_vec(),
                own_g1.bytes.as_ref().to_vec()
            );
            let Ok(set_g2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: components.components.g2,
                },
            );
            let Ok(own_g2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: g2_generator.point,
                },
            );
            assert_eq!(
                set_g2.bytes.as_ref().to_vec(),
                own_g2.bytes.as_ref().to_vec()
            );
        }
    }

    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in DECLARED_KEM_SELECTIONS {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer {
                                pairing: &consumed.adapter,
                            },
                            Default::default(),
                        ),
                        *case,
                        build_create_kem_payload(Default::default()),
                    );
                    assert!(created.is_ok());
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: carried `pairing/factory` entry
///   `libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes`,
///   restated by the spec; `setup` then `encapsulate` over identical uniform
///   bytes under any two pairing concretes whose declared target-group encoding
///   identifiers are equal return `EncapsulatedValue`s whose `key_material()`
///   bytes are equal.
/// Arrange: every member of `PAIRING_CONCRETES`, each with identical uniform
///   bytes for `setup` and `encapsulate` from the payload builders' defaults,
///   under one scope of the declared set, over every case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the `KemConsumer` returns the
///   pairing's `P::DECLARATION.target_group_encoding` with the `key_material()`
///   bytes, collected across the members.
/// Act:     `encapsulate`, inside `KemConsumer`, over the parameter set `setup`
///   returned.
/// Assert:  any two members whose identifiers are equal return equal bytes;
///   outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `create_pairing`, the
///   arm's concrete through its `try_new`, `create_hash_to_scalar`, the
///   selected concrete's `try_new`, `create_kem`, `setup`, `encapsulate`,
///   `pairing_product`, and `encode_gt`.
/// Mocked:  nothing.
#[test]
fn libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes() {
    // Arrange
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = (TargetGroupEncodingIdentifier, Vec<u8>);

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
            let hash = build_asset_identity_hash(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset {
                                identity_hash: &hash,
                            },
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("setup admits the scope the case selects and the uniform inputs");
            let encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("encapsulate admits the uniform input");
            (
                P::DECLARATION.target_group_encoding,
                encapsulated.encapsulated.key_material().expose().clone(),
            )
        }
    }

    struct PairingConsumer {
        case: CreateKemParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = (TargetGroupEncodingIdentifier, Vec<u8>);

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut observed = None;
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                let created = create_kem(
                    &build_create_kem_deps(
                        &consumed.adapter,
                        &*hashed.adapter,
                        KemConsumer,
                        Default::default(),
                    ),
                    self.case,
                    build_create_kem_payload(Default::default()),
                )
                .expect("the declared case admits its concrete");
                if let Some(previous) = &observed {
                    assert_eq!(*previous, created.output);
                }
                observed = Some(created.output);
            }
            observed.expect("the declared hash-to-scalar set holds a case")
        }
    }

    let case = *DECLARED_KEM_SELECTIONS
        .first()
        .expect("the declared set holds a case");
    let mut observed: Vec<(TargetGroupEncodingIdentifier, Vec<u8>)> = Vec::new();
    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(PairingConsumer { case }, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
        observed.push(created.expect("the pairing constructed").output);
    }
    for left in &observed {
        for right in &observed {
            if left.0 == right.0 {
                assert_eq!(left.1, right.1);
            }
        }
    }
}

/// Contract: carried `hash-to-scalar/factory` entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`,
///   restated by the spec; `derive_identity` hashes the canonical identity bytes
///   the caller supplies unchanged, so the scalar `identity_element_components`
///   returns for the bytes `create_encoding` returns for a derivation context
///   encodes to the encoding of the scalar the received scalar type samples
///   from `expected_uniform_bytes` over the tag built from
///   `K::DECLARATION.identity_tag` and those bytes.
/// Arrange: each case of `DECLARED_KEM_SELECTIONS` under the entitlement scope,
///   over each case of `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`, each case of
///   `DECLARED_ENCODING_SELECTIONS` that declares
///   `EncodingIdentifier::EthereumAbiV1`, and every member of
///   `PAIRING_CONCRETES`; the canonical bytes are the bytes `create_encoding`
///   returns for the derivation context the `domain` builder returns, through
///   `build_mock_i_encoding_consumer`; `setup` and `derive_identity` run in the
///   arrangement.
/// Act:     `identity_element_components` on the identity element
///   `derive_identity` returned over `KemIdentity::Entitlement` of those bytes.
/// Assert:  `encode_scalar` of the scalar equals `encode_scalar` of the scalar
///   sampled from `expected_uniform_bytes` over the tag built from
///   `K::DECLARATION.identity_tag`, those bytes, and
///   `P::Scalar::UNIFORM_BYTES_LENGTH`; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `create_encoding`,
///   `create_pairing`, the arm's concrete through its `try_new`,
///   `create_hash_to_scalar`, the selected concrete's `try_new`, `create_kem`,
///   the selected concrete's `try_new`, `derive_identity`, and
///   `identity_element_components`.
/// Mocked:  the encoding family's official consumer mock, which hands the bytes
///   the encoding returned to the block.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    // Arrange
    struct KemConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        canonical: &'a [u8],
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("setup admits the entitlement scope and the uniform inputs");
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Entitlement {
                                canonical: self.canonical,
                            }),
                        },
                    ),
                )
                .expect("the identity of the encoded context derives");
            let Ok(components) = payload.adapter.identity_element_components(
                build_identity_element_components_params(Default::default()),
                build_identity_element_components_payload(
                    &derived.identity_element,
                    Default::default(),
                ),
            );
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(
                        expected_uniform_bytes(
                            &tag,
                            self.canonical,
                            P::Scalar::UNIFORM_BYTES_LENGTH,
                        ),
                        Default::default(),
                    ),
                },
            )
            .expect("the oracle uniform bytes sample a scalar");
            let Ok(scalar_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: components.components.scalar,
                },
            );
            let Ok(expected_scalar_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            assert_eq!(
                scalar_encoding.bytes.expose().as_ref().to_vec(),
                expected_scalar_encoding.bytes.expose().as_ref().to_vec()
            );
        }
    }

    struct PairingConsumer<'a> {
        canonical: &'a [u8],
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|case| case.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in DECLARED_KEM_SELECTIONS
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Entitlement)
                {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer {
                                pairing: &consumed.adapter,
                                canonical: self.canonical,
                            },
                            Default::default(),
                        ),
                        *case,
                        build_create_kem_payload(Default::default()),
                    );
                    assert!(created.is_ok());
                }
            }
        }
    }

    for encoding_selection in DECLARED_ENCODING_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
    {
        let description = build_derivation_context_description(Default::default());
        let consumer = build_mock_i_encoding_consumer(
            &description,
            build_derivation_context(Default::default()),
            Vec::new(),
        );
        let canonical = create_encoding(
            &build_create_encoding_deps(consumer),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        )
        .expect("an admitted encoding selection returns the consumer's output")
        .output
        .encoded
        .expect("the derivation context encodes")
        .bytes;

        for concrete in PAIRING_CONCRETES {
            let params = build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            });
            let deps = build_create_pairing_deps(
                PairingConsumer {
                    canonical: canonical.as_slice(),
                },
                Default::default(),
            );

            // Act
            let created = create_pairing(&deps, params, CreatePairingPayload);

            // Assert
            assert!(created.is_ok());
        }
    }
}
