#![allow(clippy::expect_used)]

use super::create_kem;
use super::provides::{
    CapsuleComponents, CapsuleFromComponentsPayloadOverrides, ConsumeKemParams, ConsumeKemPayload,
    CreateKemErrorReturn, CreateKemParams, CreateKemParamsOverrides,
    CredentialFromComponentsPayloadOverrides, DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn,
    DeriveIdentityPayloadOverrides, EncapsulateErrorReturn, EncapsulatePayloadOverrides,
    ICredentialKemAdapter, IKemConsumer, IdentityScope, IssueErrorReturn, IssuePayloadOverrides,
    KEM_INTERFACE_VERSION, KemIdentity, MasterScalarFromComponentsPayloadOverrides,
    ParameterSetFromComponentsPayloadOverrides, RerandomizeErrorReturn,
    RerandomizePayloadOverrides, SetupErrorReturn, SetupParamsOverrides, SetupPayloadOverrides,
    SetupScope, build_capsule_components_params, build_capsule_components_payload,
    build_capsule_from_components_params, build_capsule_from_components_payload,
    build_create_kem_deps, build_create_kem_params, build_create_kem_payload,
    build_credential_components_params, build_credential_components_payload,
    build_credential_from_components_params, build_credential_from_components_payload,
    build_decapsulate_params, build_decapsulate_payload, build_derive_identity_params,
    build_derive_identity_payload, build_encapsulate_params, build_encapsulate_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_is_well_formed_params,
    build_is_well_formed_payload, build_issue_params, build_issue_payload,
    build_master_scalar_components_params, build_master_scalar_components_payload,
    build_master_scalar_from_components_params, build_master_scalar_from_components_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_parameter_set_from_components_params, build_parameter_set_from_components_payload,
    build_rerandomize_params, build_rerandomize_payload, build_setup_params, build_setup_payload,
};
use domain::{
    AssetIdentityHashConstructorParamsOverrides, build_asset_identity_hash, build_secret,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, build_domain_tag,
    create_hash_to_scalar, expected_uniform_bytes,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingParamsOverrides, CreatePairingPayload, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeGtParams, EncodeGtPayload,
    EncodeScalarParams, EncodeScalarPayload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, IPairingConsumer, ISampleUniformScalar, MulG1Params, MulG1Payload,
    MulG2Params, MulG2Payload, PAIRING_CONCRETES, PairingProductParams, PairingProductPayload,
    PairingProductTerm, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};

/// Contract: own entry
///   `every_concrete_is_constructed_under_the_identifier_and_scope_it_declares`;
///   given a case's selection, declared identifier, and declared scope,
///   `create_kem` yields `Ok` whose consumer reads `K::DECLARATION.identifier`
///   equal to `params.identifier`, `K::DECLARATION.interface_version` equal to
///   `KEM_INTERFACE_VERSION`, `K::DECLARATION.identity_scopes` containing
///   `params.scope`, and `payload.scope()` equal to `params.scope`.
/// Arrange: every case of `cases`, each with the identifier its own declaration
///   names, over every case of `DECLARED_HASH_TO_SCALAR_SELECTIONS`; the cases'
///   declared identifiers differ, so a construction of another concrete reports
///   another identifier.
/// Act:     `create_kem` per case, inside `PairingConsumer`.
/// Assert:  in `KemConsumer`, the four declaration and scope reads above by
///   `assert_eq!`/`assert!`; outside the consumer, `created.is_ok()`.
/// Boundary: the private surface through `create_kem` and the selected
///   concrete's `try_new`.
/// Mocked:  nothing; the mock selection is the family's mock concrete reached
///   by configuration.
#[test]
fn every_concrete_is_constructed_under_the_identifier_and_scope_it_declares() {
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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
///   `every_concrete_is_refused_under_an_identifier_it_does_not_declare`; given
///   a case's selection and another case's declared identifier, `create_kem`
///   yields `Err(CreateKemErrorReturn::UnsupportedKemIdentifier)` and the
///   consumer never runs.
/// Arrange: each case of `cases` paired with the declared identifier of every
///   other case whose declared identifier differs from its own; `KemConsumer`
///   is the deps' consumer and its body fails the block if run.
/// Act:     `create_kem` per pairing, inside `PairingConsumer`.
/// Assert:  `assert_eq!(result.err(),
///   Some(CreateKemErrorReturn::UnsupportedKemIdentifier))` per pairing;
///   outside the consumer, `created.is_ok()`.
/// Boundary: the private surface through `create_kem`.
/// Mocked:  nothing.
#[test]
fn every_concrete_is_refused_under_an_identifier_it_does_not_declare() {
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    for other in &cases {
                        if other.identifier == case.identifier {
                            continue;
                        }
                        let result = create_kem(
                            &build_create_kem_deps(
                                &consumed.adapter,
                                &*hashed.adapter,
                                KemConsumer,
                                Default::default(),
                            ),
                            build_create_kem_params(CreateKemParamsOverrides {
                                concrete: Some(case.concrete),
                                identifier: Some(other.identifier),
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

/// Contract: own entry `every_concrete_issues_a_credential_valid_for_its_identity`;
///   `setup`, `derive_identity`, and `issue` with the master scalar `setup`
///   returned yield a credential `is_valid` accepts.
/// Arrange: each case of `cases`; `KemIdentity::Entitlement` of the payload
///   builder's default canonical bytes under the entitlement scope and
///   `KemIdentity::Asset` of `build_asset_identity_hash(Default::default())`
///   under the asset scope, the same hash given to `setup`; `setup`,
///   `derive_identity`, and `issue` run in the arrangement.
/// Act:     `is_valid` on the credential `issue` returned.
/// Assert:  `assert!(valid.is_valid)` after binding each earlier call's `Ok`
///   through `expect`.
/// Boundary: the private surface through `setup`, `derive_identity`, `issue`,
///   and `is_valid`.
/// Mocked:  nothing; uniform bytes are caller-supplied.
#[test]
fn every_concrete_issues_a_credential_valid_for_its_identity() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
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
                        Default::default(),
                    ),
                )
                .expect("issue admits the credential inputs");
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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
///   `every_concrete_rerandomizes_a_credential_to_one_valid_for_its_identity`;
///   `rerandomize` over an issued credential with no master scalar yields a
///   credential `is_valid` accepts while the original stays valid.
/// Arrange: as `every_concrete_issues_a_credential_valid_for_its_identity`,
///   with the `rerandomize` payload builder's default uniform bytes, which
///   differ from the `issue` payload's; `issue` and `rerandomize` run in the
///   arrangement.
/// Act:     `is_valid` on the rerandomized credential.
/// Assert:  `true` for the rerandomized credential and for the original; the
///   offset's encoding differs from the randomness's encoding.
/// Boundary: the private surface through `issue`, `rerandomize`, and
///   `is_valid`.
/// Mocked:  nothing.
#[test]
fn every_concrete_rerandomizes_a_credential_to_one_valid_for_its_identity() {
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
                        Default::default(),
                    ),
                )
                .expect("issue admits the credential inputs");
            let rerandomized = payload
                .adapter
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        Default::default(),
                    ),
                )
                .expect("rerandomize admits the credential inputs");
            let Ok(new_valid) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &rerandomized.credential,
                    Default::default(),
                ),
            );
            assert!(new_valid.is_valid);
            let Ok(original_valid) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &issued.credential,
                    Default::default(),
                ),
            );
            assert!(original_valid.is_valid);
            let Ok(offset_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: rerandomized.offset.expose().clone(),
                },
            );
            let Ok(randomness_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: issued.randomness.expose().clone(),
                },
            );
            assert_ne!(
                offset_encoding.bytes.expose().as_ref(),
                randomness_encoding.bytes.expose().as_ref()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry
///   `every_concrete_decapsulates_a_well_formed_capsule_to_the_encapsulated_bytes`;
///   `encapsulate` then `decapsulate` with the issued and the rerandomized
///   credential returns `is_well_formed` `true` and each `key_material()`
///   equal to `encapsulate`'s.
/// Arrange: each case; under the asset scope a second asset identity element,
///   derived under a parameter set fixing another hash, passed to one
///   `decapsulate`.
/// Act:     `decapsulate` per credential.
/// Assert:  `is_well_formed` is `true`; each `key_material().expose()` bytes
///   equal the bytes `encapsulate` returned.
/// Boundary: the private surface through `encapsulate`, `is_well_formed`, and
///   `decapsulate`.
/// Mocked:  nothing.
#[test]
fn every_concrete_decapsulates_a_well_formed_capsule_to_the_encapsulated_bytes() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
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
                        Default::default(),
                    ),
                )
                .expect("issue admits the credential inputs");
            let rerandomized = payload
                .adapter
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        Default::default(),
                    ),
                )
                .expect("rerandomize admits the credential inputs");
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
            let Ok(well_formed) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &setup.parameter_set,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(well_formed.is_well_formed);
            for credential in [&issued.credential, &rerandomized.credential] {
                let Ok(decapsulated) = payload.adapter.decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &derived.identity_element,
                        credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                );
                assert_eq!(
                    decapsulated.encapsulated.key_material().expose(),
                    encapsulated.encapsulated.key_material().expose()
                );
            }
            if scope == IdentityScope::Asset {
                let other_hash =
                    build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                        bytes: Some([0xa1; 32]),
                    });
                let other_setup = payload
                    .adapter
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(SetupScope::Asset {
                                identity_hash: &other_hash,
                            }),
                        }),
                        build_setup_payload::<P::Scalar>(Default::default()),
                    )
                    .expect("the second asset setup admits the uniform inputs");
                let other_derived = payload
                    .adapter
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &other_setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(KemIdentity::Asset {
                                    identity_hash: &other_hash,
                                }),
                            },
                        ),
                    )
                    .expect("the second asset identity derives under the set it fixes");
                let Ok(decapsulated) = payload.adapter.decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &other_derived.identity_element,
                        &issued.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                );
                assert_eq!(
                    decapsulated.encapsulated.key_material().expose(),
                    encapsulated.encapsulated.key_material().expose()
                );
            }
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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
///   `every_concrete_recovers_one_value_from_the_credentials_of_different_entitlements`;
///   entitlement scope, credentials of distinct entitlements, one capsule —
///   every `decapsulate` returns `encapsulate`'s bytes.
/// Arrange: each case that declares the entitlement scope; canonical bytes
///   from the payload builder's default and from an override differing from it.
/// Act:     `decapsulate` per credential.
/// Assert:  each `key_material()` bytes equal the bytes `encapsulate`
///   returned; the two identity scalars encode differently.
/// Boundary: the private surface through `derive_identity`, `issue`,
///   `encapsulate`, and `decapsulate`.
/// Mocked:  nothing.
#[test]
fn every_concrete_recovers_one_value_from_the_credentials_of_different_entitlements() {
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
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the entitlement setup admits the uniform inputs");
            let first = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(&setup.parameter_set, Default::default()),
                )
                .expect("the first entitlement identity derives");
            let second = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Entitlement {
                                canonical: b"entitlement-two",
                            }),
                        },
                    ),
                )
                .expect("the second entitlement identity derives");
            let first_issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &first.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the first issuance admits the uniform input");
            let second_issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &second.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the second issuance admits the uniform input");
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
            for (identity_element, credential) in [
                (&first.identity_element, &first_issued.credential),
                (&second.identity_element, &second_issued.credential),
            ] {
                let Ok(decapsulated) = payload.adapter.decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        identity_element,
                        credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                );
                assert_eq!(
                    decapsulated.encapsulated.key_material().expose(),
                    encapsulated.encapsulated.key_material().expose()
                );
            }
            let Ok(first_components) = payload.adapter.identity_element_components(
                build_identity_element_components_params(Default::default()),
                build_identity_element_components_payload(
                    &first.identity_element,
                    Default::default(),
                ),
            );
            let Ok(second_components) = payload.adapter.identity_element_components(
                build_identity_element_components_params(Default::default()),
                build_identity_element_components_payload(
                    &second.identity_element,
                    Default::default(),
                ),
            );
            let Ok(first_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: first_components.components.scalar,
                },
            );
            let Ok(second_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: second_components.components.scalar,
                },
            );
            assert_ne!(
                first_encoding.bytes.expose().as_ref(),
                second_encoding.bytes.expose().as_ref()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in cases
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Entitlement)
                {
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

/// Contract: own entry
///   `every_concrete_rejects_a_credential_for_another_identity_and_under_another_parameter_set`;
///   `is_valid` for another identity element and under another parameter set
///   returns `false`.
/// Arrange: each case; the other identity from an override of the identity,
///   the other parameter set from `setup` over uniform bytes overridden to
///   differ.
/// Act:     `is_valid` per variation.
/// Assert:  `false` for each, and `true` for the credential under its own
///   identity element and parameter set.
/// Boundary: the private surface through `setup`, `derive_identity`, `issue`,
///   and `is_valid`.
/// Mocked:  nothing.
#[test]
fn every_concrete_rejects_a_credential_for_another_identity_and_under_another_parameter_set() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
            let hash = build_asset_identity_hash(Default::default());
            let setup_scope = || match scope {
                IdentityScope::Entitlement => SetupScope::Entitlement,
                IdentityScope::Asset => SetupScope::Asset {
                    identity_hash: &hash,
                },
            };
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(setup_scope()),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("setup admits the scope the case selects and the uniform inputs");
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
                        Default::default(),
                    ),
                )
                .expect("issue admits the credential inputs");
            let Ok(own_valid) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &issued.credential,
                    Default::default(),
                ),
            );
            assert!(own_valid.is_valid);
            match scope {
                IdentityScope::Entitlement => {
                    let other = payload
                        .adapter
                        .derive_identity(
                            build_derive_identity_params(Default::default()),
                            build_derive_identity_payload(
                                &setup.parameter_set,
                                DeriveIdentityPayloadOverrides {
                                    identity: Some(KemIdentity::Entitlement {
                                        canonical: b"entitlement-two",
                                    }),
                                },
                            ),
                        )
                        .expect("the other entitlement identity derives");
                    let Ok(other_valid) = payload.adapter.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &other.identity_element,
                            &issued.credential,
                            Default::default(),
                        ),
                    );
                    assert!(!other_valid.is_valid);
                }
                IdentityScope::Asset => {
                    let other_hash =
                        build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                            bytes: Some([0xa1; 32]),
                        });
                    let other_setup = payload
                        .adapter
                        .setup(
                            build_setup_params(SetupParamsOverrides {
                                scope: Some(SetupScope::Asset {
                                    identity_hash: &other_hash,
                                }),
                            }),
                            build_setup_payload::<P::Scalar>(Default::default()),
                        )
                        .expect("the other asset setup admits the uniform inputs");
                    let other = payload
                        .adapter
                        .derive_identity(
                            build_derive_identity_params(Default::default()),
                            build_derive_identity_payload(
                                &other_setup.parameter_set,
                                DeriveIdentityPayloadOverrides {
                                    identity: Some(KemIdentity::Asset {
                                        identity_hash: &other_hash,
                                    }),
                                },
                            ),
                        )
                        .expect("the other asset identity derives under the set it fixes");
                    let Ok(other_valid) = payload.adapter.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &other.identity_element,
                            &issued.credential,
                            Default::default(),
                        ),
                    );
                    assert!(!other_valid.is_valid);
                }
            }
            let other_setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(setup_scope()),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(build_secret(
                            vec![0x9u8; P::Scalar::UNIFORM_BYTES_LENGTH],
                            Default::default(),
                        )),
                        ..Default::default()
                    }),
                )
                .expect("the other parameter set's setup admits its uniform inputs");
            let Ok(other_set_valid) = payload.adapter.is_valid(
                build_is_valid_params(Default::default()),
                build_is_valid_payload(
                    &other_setup.parameter_set,
                    &derived.identity_element,
                    &issued.credential,
                    Default::default(),
                ),
            );
            assert!(!other_set_valid.is_valid);
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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
///   `every_concrete_rejects_a_capsule_with_exchanged_identity_base_elements`;
///   an entitlement capsule with `v` and `w` exchanged is not well-formed.
/// Arrange: each case that declares the entitlement scope; the capsule read by
///   `capsule_components`, `v` and `w` exchanged, and rebuilt by
///   `capsule_from_components`.
/// Act:     `is_well_formed` on the rebuilt capsule.
/// Assert:  `false` for the rebuilt capsule and `true` for the original.
/// Boundary: the private surface through `encapsulate`, `capsule_components`,
///   `capsule_from_components`, and `is_well_formed`.
/// Mocked:  nothing.
#[test]
fn every_concrete_rejects_a_capsule_with_exchanged_identity_base_elements() {
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
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the entitlement setup admits the uniform inputs");
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
            let Ok(components) = payload.adapter.capsule_components(
                build_capsule_components_params(Default::default()),
                build_capsule_components_payload(&encapsulated.capsule, Default::default()),
            );
            let swapped = match components.components {
                CapsuleComponents::Entitlement { u, v, w } => {
                    CapsuleComponents::Entitlement { u, v: w, w: v }
                }
                CapsuleComponents::Asset { u, v } => CapsuleComponents::Asset { u, v },
            };
            let Ok(rebuilt) = payload.adapter.capsule_from_components(
                build_capsule_from_components_params(Default::default()),
                build_capsule_from_components_payload(
                    self.pairing,
                    CapsuleFromComponentsPayloadOverrides {
                        components: Some(swapped),
                    },
                ),
            );
            let Ok(swapped_verdict) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &setup.parameter_set,
                    &rebuilt.capsule,
                    Default::default(),
                ),
            );
            assert!(!swapped_verdict.is_well_formed);
            let Ok(original_verdict) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &setup.parameter_set,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(original_verdict.is_well_formed);
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in cases
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Entitlement)
                {
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

/// Contract: own entry
///   `every_concrete_rejects_a_capsule_under_another_parameter_set`;
///   `is_well_formed` under a parameter set `setup` returned for other uniform
///   bytes returns `false`.
/// Arrange: each case; a second `setup` over overridden uniform bytes.
/// Act:     `is_well_formed` under the other set.
/// Assert:  `false`, and `true` under the capsule's own set.
/// Boundary: the private surface through `setup`, `encapsulate`, and
///   `is_well_formed`.
/// Mocked:  nothing.
#[test]
fn every_concrete_rejects_a_capsule_under_another_parameter_set() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
            let hash = build_asset_identity_hash(Default::default());
            let setup_scope = || match scope {
                IdentityScope::Entitlement => SetupScope::Entitlement,
                IdentityScope::Asset => SetupScope::Asset {
                    identity_hash: &hash,
                },
            };
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(setup_scope()),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("setup admits the scope the case selects and the uniform inputs");
            let other_setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(setup_scope()),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(build_secret(
                            vec![0x9u8; P::Scalar::UNIFORM_BYTES_LENGTH],
                            Default::default(),
                        )),
                        u0_uniform: Some(build_secret(
                            vec![0xau8; P::Scalar::UNIFORM_BYTES_LENGTH],
                            Default::default(),
                        )),
                        u1_uniform: Some(build_secret(
                            vec![0xbu8; P::Scalar::UNIFORM_BYTES_LENGTH],
                            Default::default(),
                        )),
                        ..Default::default()
                    }),
                )
                .expect("the other parameter set's setup admits its uniform inputs");
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
            let Ok(other_verdict) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &other_setup.parameter_set,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(!other_verdict.is_well_formed);
            let Ok(own_verdict) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &setup.parameter_set,
                    &encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(own_verdict.is_well_formed);
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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

/// Contract: own entry `every_concrete_rejects_a_capsule_of_the_other_scope`;
///   a capsule of one scope under a parameter set of the other scope is not
///   well-formed.
/// Arrange: each case that declares both scopes; a capsule and a parameter
///   set from `setup` and `encapsulate` under each scope.
/// Act:     `is_well_formed` with each capsule under the other scope's set.
/// Assert:  `false` in each direction and `true` under each capsule's own set.
/// Boundary: the private surface through `setup`, `encapsulate`, and
///   `is_well_formed`.
/// Mocked:  nothing.
#[test]
fn every_concrete_rejects_a_capsule_of_the_other_scope() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            if !(K::DECLARATION
                .identity_scopes
                .contains(&IdentityScope::Entitlement)
                && K::DECLARATION
                    .identity_scopes
                    .contains(&IdentityScope::Asset))
            {
                return;
            }
            let hash = build_asset_identity_hash(Default::default());
            let entitlement_setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the entitlement setup admits the uniform inputs");
            let asset_setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Asset {
                            identity_hash: &hash,
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the asset setup admits the uniform inputs");
            let entitlement_encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &entitlement_setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the entitlement encapsulation admits the uniform input");
            let asset_encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &asset_setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the asset encapsulation admits the uniform input");
            let Ok(under_entitlement) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &entitlement_setup.parameter_set,
                    &asset_encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(!under_entitlement.is_well_formed);
            let Ok(under_asset) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &asset_setup.parameter_set,
                    &entitlement_encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(!under_asset.is_well_formed);
            let Ok(entitlement_own) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &entitlement_setup.parameter_set,
                    &entitlement_encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(entitlement_own.is_well_formed);
            let Ok(asset_own) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &asset_setup.parameter_set,
                    &asset_encapsulated.capsule,
                    Default::default(),
                ),
            );
            assert!(asset_own.is_well_formed);
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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

/// Contract: own entry `every_concrete_refuses_a_trivial_identity_element`;
///   zero `u0` and `u1` uniform bytes yield `derive_identity`
///   `Err(DeriveIdentityErrorReturn::TrivialIdentityElement)`.
/// Arrange: each case that declares the entitlement scope; `setup` with `u0`
///   and `u1` uniform bytes from `build_secret` of the scalar type's length
///   with every byte zero, against the payload builder's defaults.
/// Act:     `derive_identity` over an entitlement identity.
/// Assert:  `assert_eq!(result.err(),
///   Some(DeriveIdentityErrorReturn::TrivialIdentityElement))` for every case;
///   `Ok` under the default bytes.
/// Boundary: the private surface through `setup` and `derive_identity`.
/// Mocked:  nothing.
#[test]
fn every_concrete_refuses_a_trivial_identity_element() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let zeros = || {
                build_secret(
                    vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH],
                    Default::default(),
                )
            };
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        u0_uniform: Some(zeros()),
                        u1_uniform: Some(zeros()),
                        ..Default::default()
                    }),
                )
                .expect("the entitlement scope skips the trivial-element check");
            let result = payload.adapter.derive_identity(
                build_derive_identity_params(Default::default()),
                build_derive_identity_payload(&setup.parameter_set, Default::default()),
            );
            assert_eq!(
                result.err(),
                Some(DeriveIdentityErrorReturn::TrivialIdentityElement)
            );
            let default_setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the default bytes set up");
            payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(&default_setup.parameter_set, Default::default()),
                )
                .expect("the default bytes derive an identity element");
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in cases
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Entitlement)
                {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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
///   `every_concrete_refuses_a_trivial_identity_element_at_asset_setup`; zero
///   `u0` and `u1` uniform bytes under the asset scope yield `setup`
///   `Err(SetupErrorReturn::TrivialIdentityElement)`.
/// Arrange: each case whose scope is the asset scope;
///   `SetupScope::Asset { identity_hash: &hash }`, `hash` from
///   `build_asset_identity_hash` with `bytes: Some([0xa1; 32])`; `u0` and `u1`
///   uniform bytes from `build_secret` of the scalar type's length with every
///   byte zero, the master bytes the payload builder's default.
/// Act:     `setup`.
/// Assert:  `assert_eq!(result.err(),
///   Some(SetupErrorReturn::TrivialIdentityElement))` for every case; `Ok`
///   under the default bytes.
/// Boundary: the private surface through `setup`.
/// Mocked:  nothing.
#[test]
fn every_concrete_refuses_a_trivial_identity_element_at_asset_setup() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xa1; 32]),
            });
            let zeros = || {
                build_secret(
                    vec![0u8; P::Scalar::UNIFORM_BYTES_LENGTH],
                    Default::default(),
                )
            };
            let result = payload.adapter.setup(
                build_setup_params(SetupParamsOverrides {
                    scope: Some(SetupScope::Asset {
                        identity_hash: &hash,
                    }),
                }),
                build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                    u0_uniform: Some(zeros()),
                    u1_uniform: Some(zeros()),
                    ..Default::default()
                }),
            );
            assert_eq!(result.err(), Some(SetupErrorReturn::TrivialIdentityElement));
            payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Asset {
                            identity_hash: &hash,
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the default bytes set up the asset scope");
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in cases
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Asset)
                {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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
///   `every_concrete_derives_under_the_asset_scope_only_the_identity_the_set_fixes`;
///   an asset parameter set derives the fixed hash's identity and refuses
///   another.
/// Arrange: each case that declares the asset scope; `setup` over
///   `build_asset_identity_hash(Default::default())`, the other hash from an
///   override differing from it.
/// Act:     `derive_identity` over each hash.
/// Assert:  `Ok` for the fixed hash and `assert_eq!(result.err(),
///   Some(DeriveIdentityErrorReturn::OutsideAssetScope))` for the other, for
///   every case.
/// Boundary: the private surface through `setup` and `derive_identity`.
/// Mocked:  nothing.
#[test]
fn every_concrete_derives_under_the_asset_scope_only_the_identity_the_set_fixes() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let hash = build_asset_identity_hash(Default::default());
            let other_hash =
                build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Asset {
                            identity_hash: &hash,
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the asset setup admits the uniform inputs");
            payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Asset {
                                identity_hash: &hash,
                            }),
                        },
                    ),
                )
                .expect("the identity the set fixes derives");
            let result = payload.adapter.derive_identity(
                build_derive_identity_params(Default::default()),
                build_derive_identity_payload(
                    &setup.parameter_set,
                    DeriveIdentityPayloadOverrides {
                        identity: Some(KemIdentity::Asset {
                            identity_hash: &other_hash,
                        }),
                    },
                ),
            );
            assert_eq!(
                result.err(),
                Some(DeriveIdentityErrorReturn::OutsideAssetScope)
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in cases
                    .iter()
                    .filter(|case| case.scope == IdentityScope::Asset)
                {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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

/// Contract: own entry `every_concrete_refuses_an_identity_of_the_other_scope`;
///   an identity of the other scope yields `derive_identity`
///   `Err(DeriveIdentityErrorReturn::WrongIdentityScope)` before hashing.
/// Arrange: each case that declares both scopes; an entitlement parameter set
///   with `KemIdentity::Asset`, an asset parameter set with
///   `KemIdentity::Entitlement`.
/// Act:     `derive_identity` per direction.
/// Assert:  `assert_eq!(result.err(),
///   Some(DeriveIdentityErrorReturn::WrongIdentityScope))` for every case, in
///   each direction.
/// Boundary: the private surface through `setup` and `derive_identity`.
/// Mocked:  nothing.
#[test]
fn every_concrete_refuses_an_identity_of_the_other_scope() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            if !(K::DECLARATION
                .identity_scopes
                .contains(&IdentityScope::Entitlement)
                && K::DECLARATION
                    .identity_scopes
                    .contains(&IdentityScope::Asset))
            {
                return;
            }
            let hash = build_asset_identity_hash(Default::default());
            let entitlement_setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the entitlement setup admits the uniform inputs");
            let under_entitlement = payload.adapter.derive_identity(
                build_derive_identity_params(Default::default()),
                build_derive_identity_payload(
                    &entitlement_setup.parameter_set,
                    DeriveIdentityPayloadOverrides {
                        identity: Some(KemIdentity::Asset {
                            identity_hash: &hash,
                        }),
                    },
                ),
            );
            assert_eq!(
                under_entitlement.err(),
                Some(DeriveIdentityErrorReturn::WrongIdentityScope)
            );
            let asset_setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Asset {
                            identity_hash: &hash,
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the asset setup admits the uniform inputs");
            let under_asset = payload.adapter.derive_identity(
                build_derive_identity_params(Default::default()),
                build_derive_identity_payload(&asset_setup.parameter_set, Default::default()),
            );
            assert_eq!(
                under_asset.err(),
                Some(DeriveIdentityErrorReturn::WrongIdentityScope)
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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
///   `every_concrete_accepts_a_credential_issued_under_a_rebuilt_parameter_set_and_master_scalar`;
///   a parameter set and master scalar read by their components methods and
///   rebuilt issue a credential `is_valid` accepts.
/// Arrange: each case; `setup`, `parameter_set_components`,
///   `master_scalar_components`, `parameter_set_from_components`,
///   `master_scalar_from_components`, `derive_identity`, and `issue` over the
///   rebuilt pair run in the arrangement.
/// Act:     `is_valid` on the issued credential under the original parameter
///   set.
/// Assert:  `true`.
/// Boundary: the private surface through the parameter-set and master-scalar
///   components methods, `issue`, and `is_valid`.
/// Mocked:  nothing.
#[test]
fn every_concrete_accepts_a_credential_issued_under_a_rebuilt_parameter_set_and_master_scalar() {
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
            let Ok(set_components) = payload.adapter.parameter_set_components(
                build_parameter_set_components_params(Default::default()),
                build_parameter_set_components_payload(&setup.parameter_set, Default::default()),
            );
            let Ok(master_components) = payload.adapter.master_scalar_components(
                build_master_scalar_components_params(Default::default()),
                build_master_scalar_components_payload(&setup.master_scalar, Default::default()),
            );
            let Ok(rebuilt_set) = payload.adapter.parameter_set_from_components(
                build_parameter_set_from_components_params(Default::default()),
                build_parameter_set_from_components_payload(
                    self.pairing,
                    ParameterSetFromComponentsPayloadOverrides {
                        components: Some(set_components.components),
                    },
                ),
            );
            let Ok(rebuilt_master) = payload.adapter.master_scalar_from_components(
                build_master_scalar_from_components_params(Default::default()),
                build_master_scalar_from_components_payload(
                    MasterScalarFromComponentsPayloadOverrides {
                        components: Some(master_components.components),
                    },
                ),
            );
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &rebuilt_set.parameter_set,
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
                .expect("the identity the rebuilt set fixes derives");
            let issued = payload
                .adapter
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &rebuilt_set.parameter_set,
                        &rebuilt_master.master_scalar,
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("issue under the rebuilt pair admits the credential inputs");
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry
///   `every_concrete_decapsulates_a_rebuilt_credential_and_capsule_to_the_encapsulated_bytes`;
///   a credential and a capsule read by their components methods and rebuilt
///   decapsulate to `encapsulate`'s bytes.
/// Arrange: each case; `issue`, `encapsulate`, `credential_components`,
///   `credential_from_components`, `capsule_components`, and
///   `capsule_from_components` run in the arrangement.
/// Act:     `decapsulate` with the rebuilt credential and the rebuilt capsule.
/// Assert:  `key_material().expose()` bytes equal the bytes `encapsulate`
///   returned.
/// Boundary: the private surface through the credential and capsule
///   components methods and `decapsulate`.
/// Mocked:  nothing.
#[test]
fn every_concrete_decapsulates_a_rebuilt_credential_and_capsule_to_the_encapsulated_bytes() {
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
                        Default::default(),
                    ),
                )
                .expect("issue admits the credential inputs");
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
            let Ok(credential_components) = payload.adapter.credential_components(
                build_credential_components_params(Default::default()),
                build_credential_components_payload(&issued.credential, Default::default()),
            );
            let Ok(rebuilt_credential) = payload.adapter.credential_from_components(
                build_credential_from_components_params(Default::default()),
                build_credential_from_components_payload(
                    self.pairing,
                    CredentialFromComponentsPayloadOverrides {
                        components: Some(credential_components.components),
                    },
                ),
            );
            let Ok(capsule_components) = payload.adapter.capsule_components(
                build_capsule_components_params(Default::default()),
                build_capsule_components_payload(&encapsulated.capsule, Default::default()),
            );
            let Ok(rebuilt_capsule) = payload.adapter.capsule_from_components(
                build_capsule_from_components_params(Default::default()),
                build_capsule_from_components_payload(
                    self.pairing,
                    CapsuleFromComponentsPayloadOverrides {
                        components: Some(capsule_components.components),
                    },
                ),
            );
            let Ok(decapsulated) = payload.adapter.decapsulate(
                build_decapsulate_params(Default::default()),
                build_decapsulate_payload(
                    &derived.identity_element,
                    &rebuilt_credential.credential,
                    &rebuilt_capsule.capsule,
                    Default::default(),
                ),
            );
            assert_eq!(
                decapsulated.encapsulated.key_material().expose(),
                encapsulated.encapsulated.key_material().expose()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry `every_concrete_accepts_a_rebuilt_capsule_as_well_formed`;
///   a capsule read by `capsule_components` and rebuilt is well-formed.
/// Arrange: each case; `encapsulate`, `capsule_components`, and
///   `capsule_from_components` run in the arrangement.
/// Act:     `is_well_formed` on the rebuilt capsule under the original
///   parameter set.
/// Assert:  `true`.
/// Boundary: the private surface through `capsule_components`,
///   `capsule_from_components`, and `is_well_formed`.
/// Mocked:  nothing.
#[test]
fn every_concrete_accepts_a_rebuilt_capsule_as_well_formed() {
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
            let Ok(components) = payload.adapter.capsule_components(
                build_capsule_components_params(Default::default()),
                build_capsule_components_payload(&encapsulated.capsule, Default::default()),
            );
            let Ok(rebuilt) = payload.adapter.capsule_from_components(
                build_capsule_from_components_params(Default::default()),
                build_capsule_from_components_payload(
                    self.pairing,
                    CapsuleFromComponentsPayloadOverrides {
                        components: Some(components.components),
                    },
                ),
            );
            let Ok(verdict) = payload.adapter.is_well_formed(
                build_is_well_formed_params(Default::default()),
                build_is_well_formed_payload(
                    &setup.parameter_set,
                    &rebuilt.capsule,
                    Default::default(),
                ),
            );
            assert!(verdict.is_well_formed);
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry `every_concrete_binds_the_public_value_to_the_master_scalar`;
///   `hpub` is the master scalar times `g2`.
/// Arrange: each case; `setup` and `master_scalar_components` run in the
///   arrangement, and the consumer computes `mul_g2` of `g2` by the master
///   scalar with the pairing it holds.
/// Act:     `parameter_set_components` on the parameter set.
/// Assert:  `encode_g2` of the computed point equals `encode_g2` of `hpub`,
///   each through `AsRef<[u8]>` into a `Vec<u8>`.
/// Boundary: the private surface through `setup`, `parameter_set_components`,
///   and `master_scalar_components`.
/// Mocked:  nothing.
#[test]
fn every_concrete_binds_the_public_value_to_the_master_scalar() {
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
            let Ok(set_components) = payload.adapter.parameter_set_components(
                build_parameter_set_components_params(Default::default()),
                build_parameter_set_components_payload(&setup.parameter_set, Default::default()),
            );
            let Ok(master_components) = payload.adapter.master_scalar_components(
                build_master_scalar_components_params(Default::default()),
                build_master_scalar_components_payload(&setup.master_scalar, Default::default()),
            );
            let Ok(computed) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: set_components.components.g2,
                    scalar: master_components.components.value.expose().clone(),
                },
            );
            let Ok(computed_encoding) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: computed.product,
                },
            );
            let Ok(hpub_encoding) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: set_components.components.hpub,
                },
            );
            assert_eq!(
                computed_encoding.bytes.as_ref().to_vec(),
                hpub_encoding.bytes.as_ref().to_vec()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry
///   `every_concrete_returns_the_randomness_its_credential_was_formed_with`;
///   `issue`'s randomness is the scalar its credential's `b` was formed with.
/// Arrange: each case; the `issue` payload's uniform bytes read back from the
///   builder's default; `issue` runs in the arrangement.
/// Act:     `credential_components` on the issued credential.
/// Assert:  `encode_g2` of `b` equals `encode_g2` of `mul_g2` of `g2` by the
///   randomness, and `encode_scalar` of the randomness equals `encode_scalar`
///   of the scalar `sample_from_uniform_bytes` returns for those bytes.
/// Boundary: the private surface through `issue` and `credential_components`.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_the_randomness_its_credential_was_formed_with() {
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
            let issue_payload = build_issue_payload::<P::Scalar, _, _, _>(
                &setup.parameter_set,
                &setup.master_scalar,
                &derived.identity_element,
                Default::default(),
            );
            let uniform_bytes = issue_payload.uniform.expose().clone();
            let issued = payload
                .adapter
                .issue(build_issue_params(Default::default()), issue_payload)
                .expect("issue admits the credential inputs");
            let Ok(components) = payload.adapter.credential_components(
                build_credential_components_params(Default::default()),
                build_credential_components_payload(&issued.credential, Default::default()),
            );
            let Ok(g2) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(product) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: issued.randomness.expose().clone(),
                },
            );
            let Ok(b_encoding) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: components.components.b,
                },
            );
            let Ok(product_encoding) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: product.product,
                },
            );
            assert_eq!(
                b_encoding.bytes.as_ref().to_vec(),
                product_encoding.bytes.as_ref().to_vec()
            );
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(uniform_bytes, Default::default()),
                },
            )
            .expect("the issue uniform input samples a scalar");
            let Ok(randomness_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: issued.randomness.expose().clone(),
                },
            );
            let Ok(sampled_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            assert_eq!(
                randomness_encoding.bytes.expose().as_ref().to_vec(),
                sampled_encoding.bytes.expose().as_ref().to_vec()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry
///   `every_concrete_returns_the_offset_it_applied_under_rerandomization`; the
///   new credential's `b` is the old `b` plus the offset times `g2`.
/// Arrange: each case; an issued credential, its components read, and
///   `rerandomize` run in the arrangement.
/// Act:     `credential_components` on the rerandomized credential.
/// Assert:  `encode_g2` of the new `b` equals `encode_g2` of `add_g2` of the
///   old `b` and `mul_g2` of `g2` by the offset.
/// Boundary: the private surface through `rerandomize` and
///   `credential_components`.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_the_offset_it_applied_under_rerandomization() {
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
                        Default::default(),
                    ),
                )
                .expect("issue admits the credential inputs");
            let Ok(old_components) = payload.adapter.credential_components(
                build_credential_components_params(Default::default()),
                build_credential_components_payload(&issued.credential, Default::default()),
            );
            let rerandomized = payload
                .adapter
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        Default::default(),
                    ),
                )
                .expect("rerandomize admits the credential inputs");
            let Ok(new_components) = payload.adapter.credential_components(
                build_credential_components_params(Default::default()),
                build_credential_components_payload(&rerandomized.credential, Default::default()),
            );
            let Ok(g2) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(offset_term) = self.pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: rerandomized.offset.expose().clone(),
                },
            );
            let Ok(expected_b) = self.pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: old_components.components.b,
                    right: offset_term.product,
                },
            );
            let Ok(new_b_encoding) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: new_components.components.b,
                },
            );
            let Ok(expected_encoding) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: expected_b.sum,
                },
            );
            assert_eq!(
                new_b_encoding.bytes.as_ref().to_vec(),
                expected_encoding.bytes.as_ref().to_vec()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry
///   `every_concrete_exposes_the_target_group_encoding_of_the_encapsulated_value`;
///   `key_material()` is the pairing's encoding of `e(t·g1, hpub)`.
/// Arrange: each case; the `encapsulate` payload's uniform bytes read back
///   from the builder's default, the sampled `t` from
///   `sample_from_uniform_bytes`.
/// Act:     `encapsulate`.
/// Assert:  `key_material().expose()` bytes equal the bytes of `encode_gt`
///   over `pairing_product` of `mul_g1` of `g1` by `t` with `hpub`.
/// Boundary: the private surface through `encapsulate`.
/// Mocked:  nothing.
#[test]
fn every_concrete_exposes_the_target_group_encoding_of_the_encapsulated_value() {
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
            let encapsulate_payload =
                build_encapsulate_payload::<P::Scalar, _>(&setup.parameter_set, Default::default());
            let uniform_bytes = encapsulate_payload.uniform.expose().clone();
            let encapsulated = payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    encapsulate_payload,
                )
                .expect("encapsulate admits the uniform input");
            let Ok(set_components) = payload.adapter.parameter_set_components(
                build_parameter_set_components_params(Default::default()),
                build_parameter_set_components_payload(&setup.parameter_set, Default::default()),
            );
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(uniform_bytes, Default::default()),
                },
            )
            .expect("the encapsulate uniform input samples a scalar");
            let Ok(point) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: set_components.components.g1,
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            let Ok(product) = self.pairing.pairing_product(
                PairingProductParams,
                PairingProductPayload {
                    terms: vec![PairingProductTerm {
                        g1: point.product,
                        g2: set_components.components.hpub,
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
                encoded.bytes.expose().as_ref(),
                encapsulated.encapsulated.key_material().expose().as_slice()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
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

/// Contract: own entry `every_concrete_maps_an_identity_under_its_declared_tag`;
///   the identity scalar is the oracle's scalar under
///   `K::DECLARATION.identity_tag`.
/// Arrange: each case, over each case of `DECLARED_HASH_TO_SCALAR_SELECTIONS`
///   that declares `HashToScalarIdentifier::Keccak256V1`; the tag from
///   `build_domain_tag` over the bytes of `K::DECLARATION.identity_tag`;
///   `setup` and `derive_identity` run in the arrangement.
/// Act:     `identity_element_components` on the derived identity element.
/// Assert:  `encode_scalar` of the scalar equals `encode_scalar` of the scalar
///   sampled from `expected_uniform_bytes(&tag, canonical,
///   P::Scalar::UNIFORM_BYTES_LENGTH)`, and `encode_g1` of the element equals
///   `encode_g1` of `add_g1` of `u0` and `mul_g1` of `u1` by that scalar.
/// Boundary: the private surface through `create_hash_to_scalar`,
///   `derive_identity`, and `identity_element_components`.
/// Mocked:  nothing.
#[test]
fn every_concrete_maps_an_identity_under_its_declared_tag() {
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
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
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
            let Ok(element_components) = payload.adapter.identity_element_components(
                build_identity_element_components_params(Default::default()),
                build_identity_element_components_payload(
                    &derived.identity_element,
                    Default::default(),
                ),
            );
            let Ok(set_components) = payload.adapter.parameter_set_components(
                build_parameter_set_components_params(Default::default()),
                build_parameter_set_components_payload(&setup.parameter_set, Default::default()),
            );
            let message: Vec<u8> = match scope {
                IdentityScope::Entitlement => b"entitlement-one".to_vec(),
                IdentityScope::Asset => hash.as_bytes().to_vec(),
            };
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(
                        expected_uniform_bytes(&tag, &message, P::Scalar::UNIFORM_BYTES_LENGTH),
                        Default::default(),
                    ),
                },
            )
            .expect("the oracle uniform bytes sample a scalar");
            let Ok(term) = self.pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: set_components.components.u1,
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            let Ok(expected_element) = self.pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: set_components.components.u0,
                    right: term.product,
                },
            );
            let Ok(scalar_encoding) = self.pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: element_components.components.scalar,
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
            let Ok(element_encoding) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: element_components.components.element,
                },
            );
            let Ok(expected_element_encoding) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: expected_element.sum,
                },
            );
            assert_eq!(
                element_encoding.bytes.as_ref().to_vec(),
                expected_element_encoding.bytes.as_ref().to_vec()
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
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
                for case in &cases {
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

/// Contract: own entry `every_concrete_returns_a_sampling_refusal_whole`;
///   uniform bytes one short yields the method's sampling flavor carrying
///   `WrongLength { expected, actual }`.
/// Arrange: each case; `setup` with each of its three uniform inputs
///   shortened in turn, and `issue`, `rerandomize`, and `encapsulate` with
///   theirs shortened.
/// Act:     each method.
/// Assert:  for every case, `assert_eq!` of the error against the method's
///   flavor, `SetupErrorReturn::MasterScalarSampling`,
///   `SetupErrorReturn::U0Sampling`, `SetupErrorReturn::U1Sampling`,
///   `IssueErrorReturn::Sampling`, `RerandomizeErrorReturn::Sampling`, or
///   `EncapsulateErrorReturn::Sampling`, holding
///   `SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual }`, `actual` one less; `Ok`
///   under the full-length input.
/// Boundary: the private surface through the four sampling methods.
/// Mocked:  nothing.
#[test]
fn every_concrete_returns_a_sampling_refusal_whole() {
    struct KemConsumer;

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let expected = P::Scalar::UNIFORM_BYTES_LENGTH;
            let actual = expected - 1;
            let short = || build_secret(vec![0u8; actual], Default::default());
            let master_result = payload.adapter.setup(
                build_setup_params(Default::default()),
                build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                    master_uniform: Some(short()),
                    ..Default::default()
                }),
            );
            assert_eq!(
                master_result.err(),
                Some(SetupErrorReturn::MasterScalarSampling(
                    SampleUniformScalarErrorReturn::WrongLength { expected, actual }
                ))
            );
            let u0_result = payload.adapter.setup(
                build_setup_params(Default::default()),
                build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                    u0_uniform: Some(short()),
                    ..Default::default()
                }),
            );
            assert_eq!(
                u0_result.err(),
                Some(SetupErrorReturn::U0Sampling(
                    SampleUniformScalarErrorReturn::WrongLength { expected, actual }
                ))
            );
            let u1_result = payload.adapter.setup(
                build_setup_params(Default::default()),
                build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                    u1_uniform: Some(short()),
                    ..Default::default()
                }),
            );
            assert_eq!(
                u1_result.err(),
                Some(SetupErrorReturn::U1Sampling(
                    SampleUniformScalarErrorReturn::WrongLength { expected, actual }
                ))
            );
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(Default::default()),
                )
                .expect("the full-length inputs set up");
            let derived = payload
                .adapter
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(&setup.parameter_set, Default::default()),
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
                        Default::default(),
                    ),
                )
                .expect("the full-length input issues a credential");
            let issue_result = payload.adapter.issue(
                build_issue_params(Default::default()),
                build_issue_payload::<P::Scalar, _, _, _>(
                    &setup.parameter_set,
                    &setup.master_scalar,
                    &derived.identity_element,
                    IssuePayloadOverrides {
                        uniform: Some(short()),
                    },
                ),
            );
            assert_eq!(
                issue_result.err(),
                Some(IssueErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength { expected, actual }
                ))
            );
            payload
                .adapter
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        Default::default(),
                    ),
                )
                .expect("the full-length input rerandomizes");
            let rerandomize_result = payload.adapter.rerandomize(
                build_rerandomize_params(Default::default()),
                build_rerandomize_payload::<P::Scalar, _, _, _>(
                    &setup.parameter_set,
                    &derived.identity_element,
                    &issued.credential,
                    RerandomizePayloadOverrides {
                        uniform: Some(short()),
                    },
                ),
            );
            assert_eq!(
                rerandomize_result.err(),
                Some(RerandomizeErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength { expected, actual }
                ))
            );
            payload
                .adapter
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the full-length input encapsulates");
            let encapsulate_result = payload.adapter.encapsulate(
                build_encapsulate_params(Default::default()),
                build_encapsulate_payload::<P::Scalar, _>(
                    &setup.parameter_set,
                    EncapsulatePayloadOverrides {
                        uniform: Some(short()),
                    },
                ),
            );
            assert_eq!(
                encapsulate_result.err(),
                Some(EncapsulateErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength { expected, actual }
                ))
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            for h2s_case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    *h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the declared selection admits its concrete");
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer,
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

/// Contract: own entry `every_concrete_returns_a_hash_to_scalar_refusal_whole`;
///   the mock hash-to-scalar concrete under `SamplingRefused` yields the
///   `HashToScalar` flavor from `setup` under the asset scope and from
///   `derive_identity`.
/// Arrange: each case; the hash-to-scalar adapter from
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
///   concrete: Some(HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)),
///   ..Default::default() })`, against each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` under which the same calls return
///   `Ok`.
/// Act:     `setup` under the asset scope, and `derive_identity`.
/// Assert:  for every case, `assert_eq!` of the error `setup` returns against
///   `SetupErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   P::Scalar::UNIFORM_BYTES_LENGTH, actual: 0 }))`, and of the error
///   `derive_identity` returns against `DeriveIdentityErrorReturn::HashToScalar`
///   holding the same refusal; `Ok` under the real selection.
/// Boundary: the private surface through `create_hash_to_scalar`, `setup`,
///   and `derive_identity`.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration, standing for the sampling bound's refusal.
#[test]
fn every_concrete_returns_a_hash_to_scalar_refusal_whole() {
    struct KemConsumer {
        hash_refused: bool,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let expected = P::Scalar::UNIFORM_BYTES_LENGTH;
            let hash = build_asset_identity_hash(Default::default());
            let asset_result = payload.adapter.setup(
                build_setup_params(SetupParamsOverrides {
                    scope: Some(SetupScope::Asset {
                        identity_hash: &hash,
                    }),
                }),
                build_setup_payload::<P::Scalar>(Default::default()),
            );
            let entitlement_setup = payload.adapter.setup(
                build_setup_params(Default::default()),
                build_setup_payload::<P::Scalar>(Default::default()),
            );
            if self.hash_refused {
                assert_eq!(
                    asset_result.err(),
                    Some(SetupErrorReturn::HashToScalar(
                        HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected,
                                actual: 0
                            }
                        )
                    ))
                );
                let entitlement_setup =
                    entitlement_setup.expect("the entitlement scope hashes nothing at setup");
                let derive_result = payload.adapter.derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &entitlement_setup.parameter_set,
                        Default::default(),
                    ),
                );
                assert_eq!(
                    derive_result.err(),
                    Some(DeriveIdentityErrorReturn::HashToScalar(
                        HashToScalarErrorReturn::Sampling(
                            SampleUniformScalarErrorReturn::WrongLength {
                                expected,
                                actual: 0
                            }
                        )
                    ))
                );
            } else {
                asset_result.expect("the real selection sets up the asset scope");
                let entitlement_setup =
                    entitlement_setup.expect("the real selection sets up the entitlement scope");
                payload
                    .adapter
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &entitlement_setup.parameter_set,
                            Default::default(),
                        ),
                    )
                    .expect("the real selection derives the identity the set fixes");
            }
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
            let mut cases: Vec<CreateKemParams> = DECLARED_KEM_SELECTIONS.to_vec();
            let mut scopes: Vec<IdentityScope> = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                if !scopes.contains(&selection.scope) {
                    scopes.push(selection.scope);
                }
            }
            for scope in scopes {
                cases.push(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(scope),
                    ..Default::default()
                }));
            }
            let mut h2s_cases = vec![build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )];
            h2s_cases.extend_from_slice(DECLARED_HASH_TO_SCALAR_SELECTIONS);
            for h2s_case in h2s_cases {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    h2s_case,
                    build_create_hash_to_scalar_payload(Default::default()),
                )
                .expect("the selection admits its concrete");
                let hash_refused = h2s_case.identifier == HashToScalarIdentifier::Mock;
                for case in &cases {
                    let created = create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            KemConsumer { hash_refused },
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
