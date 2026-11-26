#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use domain::{Secret, build_secret};
use encoding::{
    CanonicalFieldsOverrides, CreateEncodingParamsOverrides, ENCODING_CONCRETES,
    EncodingConcreteOverrides, EncodingIdentifier, LookupEncodingDeclarationParamsOverrides,
    MockIEncodingContract, build_canonical_fields, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, build_encoding_concrete,
    build_lookup_encoding_declaration_deps, build_lookup_encoding_declaration_params,
    build_lookup_encoding_declaration_payload, build_mock_i_encoding_consumer, create_encoding,
    lookup_encoding_declaration,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DomainTagConstructorParamsOverrides,
    HASH_TO_SCALAR_CONCRETES, HashToScalarConcreteOverrides, HashToScalarIdentifier,
    IHashToScalarAdapter, LookupHashToScalarDeclarationParamsOverrides,
    build_create_hash_to_scalar_params, build_domain_tag, build_hash_to_scalar_concrete,
    build_lookup_hash_to_scalar_declaration_deps, build_lookup_hash_to_scalar_declaration_params,
    build_lookup_hash_to_scalar_declaration_payload, expected_uniform_bytes,
    lookup_hash_to_scalar_declaration,
};
use hashing::build_root;
use kem::{
    AdmitKemSelectionParamsOverrides, ConsumeCreateKemCaseParams, ConsumeCreateKemCasePayload,
    ConsumeKemParams, ConsumeKemPayload, CreateKemCaseParamsOverrides, CreateKemDepsOverrides,
    CreateKemParams, CreateKemParamsOverrides, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, ICreateKemCaseConsumer, ICredentialKemAdapter, IKemConsumer,
    IdentityScope, IssuePayloadOverrides, KEM_CONCRETES, KEM_INTERFACE_VERSION, KemIdentity,
    LookupKemDeclarationParamsOverrides, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    admit_kem_composition, admit_kem_selection, build_admit_kem_selection_deps,
    build_admit_kem_selection_params, build_create_kem_case_deps, build_create_kem_case_params,
    build_create_kem_case_payload, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, build_decapsulate_params, build_decapsulate_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_is_valid_params, build_is_valid_payload,
    build_is_well_formed_params, build_is_well_formed_payload, build_issue_params,
    build_issue_payload, build_lookup_kem_declaration_deps, build_lookup_kem_declaration_params,
    build_lookup_kem_declaration_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_setup_params, build_setup_payload, create_kem,
    lookup_kem_declaration, run_create_kem_case,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, LookupPairingDeclarationParamsOverrides, PAIRING_CONCRETES,
    PairingConcrete, SampleUniformScalarParams, SampleUniformScalarPayload,
    TargetGroupEncodingIdentifier, build_create_pairing_params,
    build_lookup_pairing_declaration_deps, build_lookup_pairing_declaration_params,
    build_lookup_pairing_declaration_payload, lookup_pairing_declaration,
};
use random::{
    ConsumeRandomSourceParams, ConsumeRandomSourcePayload, CreateRandomSourceParamsOverrides,
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, IRandomSourceConsumer,
    RANDOM_SOURCE_KINDS, build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};
use serde_json::Value;

/// Contract: own entry `every_concrete_is_constructed_under_the_scope_it_declares`,
///   over the declared set; `create_kem` under a case's selection and a scope
///   its declaration provides yields `Ok` whose consumer reads the declaration's
///   `concrete` equal to `params.concrete`, its `interface_version` equal to
///   `KEM_INTERFACE_VERSION`, its `identity_scopes` containing `params.scope`,
///   and `payload.scope()` equal to `params.scope`.
/// Arrange: each member of `KEM_CONCRETES` with its selection from
///   `admit_kem_selection` and each scope its declaration provides, run once
///   per member of `PAIRING_CONCRETES` and each member of
///   `HASH_TO_SCALAR_CONCRETES` the case params name.
/// Act:     `create_kem`, inside `Consumer`.
/// Assert:  in `KemConsumer`, the four declaration and scope reads above by
///   `assert_eq!`/`assert!`; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `admit_kem_selection`,
///   `lookup_kem_declaration`, `create_pairing`, the arm's concrete through its
///   `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`,
///   `admit_kem_composition`, `create_kem`.
/// Mocked:  nothing.
#[test]
fn every_concrete_is_constructed_under_the_scope_it_declares() {
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
            let looked_up = lookup_kem_declaration(
                &build_lookup_kem_declaration_deps(Default::default()),
                build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                    concrete: Some(self.params.concrete),
                }),
                build_lookup_kem_declaration_payload(Default::default()),
            )
            .expect("the real lookup returns the concrete's declaration");
            assert_eq!(looked_up.declaration.concrete, self.params.concrete);
            assert_eq!(
                looked_up.declaration.interface_version,
                KEM_INTERFACE_VERSION
            );
            assert!(
                looked_up
                    .declaration
                    .identity_scopes
                    .contains(&self.params.scope)
            );
            assert_eq!(payload.scope(), self.params.scope);
        }
    }

    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            let mut cases: Vec<CreateKemParams> = Vec::new();
            for member in KEM_CONCRETES {
                let admitted = admit_kem_selection(
                    &build_admit_kem_selection_deps(Default::default()),
                    build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                        concrete: Some(*member),
                    }),
                    Value::Null,
                )
                .expect("a member of the declared set is admitted");
                let looked_up = lookup_kem_declaration(
                    &build_lookup_kem_declaration_deps(Default::default()),
                    build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                        concrete: Some(admitted.selection),
                    }),
                    build_lookup_kem_declaration_payload(Default::default()),
                )
                .expect("the real lookup returns the member's declaration");
                for scope in looked_up.declaration.identity_scopes {
                    cases.push(build_create_kem_params(CreateKemParamsOverrides {
                        concrete: Some(admitted.selection),
                        scope: Some(*scope),
                    }));
                }
            }
            for case in &cases {
                let created = create_kem(
                    &build_create_kem_deps(
                        payload.pairing,
                        payload.hash_to_scalar,
                        KemConsumer { params: *case },
                        CreateKemDepsOverrides {
                            admit_kem_composition: Some(admit_kem_composition),
                            lookup_kem_declaration: Some(lookup_kem_declaration),
                        },
                    ),
                    *case,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        for member in HASH_TO_SCALAR_CONCRETES {
            // Act
            let created = run_create_kem_case(
                &build_create_kem_case_deps(Consumer, Default::default()),
                build_create_kem_case_params(CreateKemCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                HashToScalarConcreteOverrides,
                            )),
                        },
                    )),
                }),
                build_create_kem_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: carried `kem/admit_kem_composition` entry
///   `every_member_of_the_declared_set_has_a_declaration_that_names_it`,
///   restated by the spec; `create_kem` under each member's selection and each
///   scope its declaration provides yields `Ok`, and the declaration the real
///   `lookup_kem_declaration` returns for the member names the member.
/// Arrange: each member of `KEM_CONCRETES` and each scope the declaration
///   `lookup_kem_declaration` returns for it provides; the member's selection
///   is the member, run once per member of `PAIRING_CONCRETES` and each member
///   of `HASH_TO_SCALAR_CONCRETES` the case params name.
/// Act:     `create_kem`, inside `Consumer`, with the real
///   `admit_kem_composition` and `lookup_kem_declaration` in the deps.
/// Assert:  the result is `Ok`, and in `KemConsumer` the declaration
///   `lookup_kem_declaration` returns for `params.concrete` has a `concrete`
///   equal to `params.concrete`; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route
///   `lookup_kem_declaration`, `admit_kem_composition`, `create_kem`.
/// Mocked:  nothing.
#[test]
fn every_member_of_the_declared_set_has_a_declaration_that_names_it() {
    // Arrange
    struct KemConsumer {
        params: CreateKemParams,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            _payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let looked_up = lookup_kem_declaration(
                &build_lookup_kem_declaration_deps(Default::default()),
                build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                    concrete: Some(self.params.concrete),
                }),
                build_lookup_kem_declaration_payload(Default::default()),
            )
            .expect("the real lookup returns the concrete's declaration");
            assert_eq!(looked_up.declaration.concrete, self.params.concrete);
        }
    }

    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            let mut cases: Vec<CreateKemParams> = Vec::new();
            for member in KEM_CONCRETES {
                let admitted = admit_kem_selection(
                    &build_admit_kem_selection_deps(Default::default()),
                    build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                        concrete: Some(*member),
                    }),
                    Value::Null,
                )
                .expect("a member of the declared set is admitted");
                let looked_up = lookup_kem_declaration(
                    &build_lookup_kem_declaration_deps(Default::default()),
                    build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                        concrete: Some(admitted.selection),
                    }),
                    build_lookup_kem_declaration_payload(Default::default()),
                )
                .expect("the real lookup returns the member's declaration");
                for scope in looked_up.declaration.identity_scopes {
                    cases.push(build_create_kem_params(CreateKemParamsOverrides {
                        concrete: Some(admitted.selection),
                        scope: Some(*scope),
                    }));
                }
            }
            for case in &cases {
                let created = create_kem(
                    &build_create_kem_deps(
                        payload.pairing,
                        payload.hash_to_scalar,
                        KemConsumer { params: *case },
                        CreateKemDepsOverrides {
                            admit_kem_composition: Some(admit_kem_composition),
                            lookup_kem_declaration: Some(lookup_kem_declaration),
                        },
                    ),
                    *case,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        for member in HASH_TO_SCALAR_CONCRETES {
            // Act
            let created = run_create_kem_case(
                &build_create_kem_case_deps(Consumer, Default::default()),
                build_create_kem_case_params(CreateKemCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                HashToScalarConcreteOverrides,
                            )),
                        },
                    )),
                }),
                build_create_kem_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: carried `kem/admit_kem_composition` entry
///   `a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set`,
///   restated by the spec; `create_kem` under each member's admitted selection
///   and each scope its declaration provides yields `Ok`.
/// Arrange: each member of `KEM_CONCRETES`, its selection from
///   `admit_kem_selection` over `Value::Null` and over `Value::Object` with no
///   entries, and each scope its declaration provides, run once per member of
///   `PAIRING_CONCRETES` and each member of `HASH_TO_SCALAR_CONCRETES` the case
///   params name.
/// Act:     `create_kem` with `params.concrete` the selection
///   `admit_kem_selection` returned, inside `Consumer`.
/// Assert:  the result is `Ok` for each pair and `payload.scope()` equals
///   `params.scope`; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `admit_kem_selection`,
///   `admit_kem_composition`, `create_kem`.
/// Mocked:  nothing.
#[test]
fn a_member_admitted_by_selection_is_compatible_with_each_scope_of_the_declared_set() {
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
            assert_eq!(payload.scope(), self.params.scope);
        }
    }

    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            let mut cases: Vec<CreateKemParams> = Vec::new();
            for member in KEM_CONCRETES {
                for settings in [Value::Null, Value::Object(Default::default())] {
                    let admitted = admit_kem_selection(
                        &build_admit_kem_selection_deps(Default::default()),
                        build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                            concrete: Some(*member),
                        }),
                        settings,
                    )
                    .expect("a member of the declared set is admitted under no settings");
                    let looked_up = lookup_kem_declaration(
                        &build_lookup_kem_declaration_deps(Default::default()),
                        build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                            concrete: Some(admitted.selection),
                        }),
                        build_lookup_kem_declaration_payload(Default::default()),
                    )
                    .expect("the real lookup returns the member's declaration");
                    for scope in looked_up.declaration.identity_scopes {
                        cases.push(build_create_kem_params(CreateKemParamsOverrides {
                            concrete: Some(admitted.selection),
                            scope: Some(*scope),
                        }));
                    }
                }
            }
            for case in &cases {
                let created = create_kem(
                    &build_create_kem_deps(
                        payload.pairing,
                        payload.hash_to_scalar,
                        KemConsumer { params: *case },
                        CreateKemDepsOverrides {
                            admit_kem_composition: Some(admit_kem_composition),
                            lookup_kem_declaration: Some(lookup_kem_declaration),
                        },
                    ),
                    *case,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        for member in HASH_TO_SCALAR_CONCRETES {
            // Act
            let created = run_create_kem_case(
                &build_create_kem_case_deps(Consumer, Default::default()),
                build_create_kem_case_params(CreateKemCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                HashToScalarConcreteOverrides,
                            )),
                        },
                    )),
                }),
                build_create_kem_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
    }
}

/// Contract: own entry `a_kem_from_the_factory_round_trips_from_operating_system_draws`;
///   every uniform draw `setup`, `issue`, and `encapsulate` consume is the bytes
///   `fill_bytes` returns for `P::Scalar::UNIFORM_BYTES_LENGTH` from the adapter
///   `create_random_source` returns for a member of `RANDOM_SOURCE_KINDS`, one
///   draw per sampler, and the round trip `setup`, `derive_identity`, `issue`,
///   `encapsulate`, `is_valid`, `is_well_formed`, `decapsulate` returns `true`
///   for the verdicts and `encapsulate`'s bytes for `decapsulate`.
/// Arrange: each member of `KEM_CONCRETES` with each scope its declaration
///   provides, over every member of `PAIRING_CONCRETES`, each member of
///   `RANDOM_SOURCE_KINDS`, and every member of `HASH_TO_SCALAR_CONCRETES` the
///   case params name; every uniform input is the bytes `fill_bytes` returns
///   for `FillBytesPayloadOverrides { length: Some(P::Scalar::UNIFORM_BYTES_LENGTH)
///   }` from the adapter `create_random_source` returns for that member's
///   `kind`, one draw per sampler.
/// Act:     in `KemConsumer`, `decapsulate` with the credential and capsule the
///   arrangement produced.
/// Assert:  `is_valid` and `is_well_formed` returned `true`, and `decapsulate`'s
///   `key_material().expose()` bytes equal `encapsulate`'s; outside the
///   consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `create_random_source`,
///   the selected member's `try_new` and `fill_bytes`, `create_pairing`, the
///   arm's concrete through its `try_new`, `create_hash_to_scalar`, the
///   selected concrete's `try_new`, `create_kem`, and every method the consumer
///   calls.
/// Mocked:  nothing; each declared member's generator is the outer edge and
///   runs real, so this block proves the draws reach the samplers and not a
///   generator's quality.
#[test]
fn a_kem_from_the_factory_round_trips_from_operating_system_draws() {
    // Arrange
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
                    .expect("the declared source's draw fills the uniform input")
                    .bytes
            };
            let scope = payload.scope();
            let root = build_root(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset { root: &root },
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
                                identity: Some(KemIdentity::Asset { root: &root }),
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

    struct Consumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
    }

    impl<'a> ICreateKemCaseConsumer for Consumer<'a> {
        type Output = ();

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            let mut cases: Vec<CreateKemParams> = Vec::new();
            for member in KEM_CONCRETES {
                let admitted = admit_kem_selection(
                    &build_admit_kem_selection_deps(Default::default()),
                    build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                        concrete: Some(*member),
                    }),
                    Value::Null,
                )
                .expect("a member of the declared set is admitted");
                let looked_up = lookup_kem_declaration(
                    &build_lookup_kem_declaration_deps(Default::default()),
                    build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                        concrete: Some(admitted.selection),
                    }),
                    build_lookup_kem_declaration_payload(Default::default()),
                )
                .expect("the real lookup returns the member's declaration");
                for scope in looked_up.declaration.identity_scopes {
                    cases.push(build_create_kem_params(CreateKemParamsOverrides {
                        concrete: Some(admitted.selection),
                        scope: Some(*scope),
                    }));
                }
            }
            for case in &cases {
                let created = create_kem(
                    &build_create_kem_deps(
                        payload.pairing,
                        payload.hash_to_scalar,
                        KemConsumer {
                            random: self.random,
                        },
                        CreateKemDepsOverrides {
                            admit_kem_composition: Some(admit_kem_composition),
                            lookup_kem_declaration: Some(lookup_kem_declaration),
                        },
                    ),
                    *case,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    struct RandomConsumer;

    impl IRandomSourceConsumer for RandomConsumer {
        type Output = ();

        fn consume_random_source<K: IRandomSourceAdapter>(
            &self,
            _params: ConsumeRandomSourceParams,
            payload: ConsumeRandomSourcePayload<K>,
        ) -> Self::Output {
            for concrete in PAIRING_CONCRETES {
                for member in HASH_TO_SCALAR_CONCRETES {
                    let created = run_create_kem_case(
                        &build_create_kem_case_deps(
                            Consumer {
                                random: &payload.source,
                            },
                            Default::default(),
                        ),
                        build_create_kem_case_params(CreateKemCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*concrete),
                                    ..Default::default()
                                },
                            )),
                            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                CreateHashToScalarParamsOverrides {
                                    concrete: Some(build_hash_to_scalar_concrete(
                                        *member,
                                        HashToScalarConcreteOverrides,
                                    )),
                                },
                            )),
                        }),
                        build_create_kem_case_payload(Default::default()),
                    );

                    assert!(created.is_ok());
                }
            }
        }
    }

    for kind in RANDOM_SOURCE_KINDS {
        // Act
        let created = create_random_source(
            &build_create_random_source_deps(RandomConsumer, Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(*kind),
            }),
            build_create_random_source_payload(Default::default()),
        );

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: carried `pairing/factory` entry
///   `the_consumer_receives_the_concrete_the_params_name`, restated by the spec;
///   the pairing the adapter `create_kem` hands the consumer computes over is
///   the concrete the pairing params name, the consumer's own computation with
///   the pairing it holds agreeing with the adapter's.
/// Arrange: each member of `PAIRING_CONCRETES` with each member of
///   `KEM_CONCRETES` under each scope its declaration provides, over every
///   member of `HASH_TO_SCALAR_CONCRETES`; `KemConsumer` holds the pairing the
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
            let root = build_root(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset { root: &root },
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

    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            let mut cases: Vec<CreateKemParams> = Vec::new();
            for member in KEM_CONCRETES {
                let admitted = admit_kem_selection(
                    &build_admit_kem_selection_deps(Default::default()),
                    build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                        concrete: Some(*member),
                    }),
                    Value::Null,
                )
                .expect("a member of the declared set is admitted");
                let looked_up = lookup_kem_declaration(
                    &build_lookup_kem_declaration_deps(Default::default()),
                    build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                        concrete: Some(admitted.selection),
                    }),
                    build_lookup_kem_declaration_payload(Default::default()),
                )
                .expect("the real lookup returns the member's declaration");
                for scope in looked_up.declaration.identity_scopes {
                    cases.push(build_create_kem_params(CreateKemParamsOverrides {
                        concrete: Some(admitted.selection),
                        scope: Some(*scope),
                    }));
                }
            }
            for case in &cases {
                let created = create_kem(
                    &build_create_kem_deps(
                        payload.pairing,
                        payload.hash_to_scalar,
                        KemConsumer {
                            pairing: payload.pairing,
                        },
                        CreateKemDepsOverrides {
                            admit_kem_composition: Some(admit_kem_composition),
                            lookup_kem_declaration: Some(lookup_kem_declaration),
                        },
                    ),
                    *case,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        for member in HASH_TO_SCALAR_CONCRETES {
            // Act
            let created = run_create_kem_case(
                &build_create_kem_case_deps(Consumer, Default::default()),
                build_create_kem_case_params(CreateKemCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                HashToScalarConcreteOverrides,
                            )),
                        },
                    )),
                }),
                build_create_kem_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
        }
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
///   under one scope of the declared set, over every member of
///   `HASH_TO_SCALAR_CONCRETES`; `KemConsumer` returns the
///   `target_group_encoding` of the declaration `lookup_pairing_declaration`
///   returns for the member's `concrete` with the `key_material()` bytes,
///   collected across the members.
/// Act:     `encapsulate`, inside `KemConsumer`, over the parameter set `setup`
///   returned.
/// Assert:  any two collected pairs whose identifiers are equal hold equal
///   bytes; outside the consumers, `created.is_ok()`.
/// Boundary: the public call `create_kem`, with the route `create_pairing`, the
///   arm's concrete through its `try_new`, `create_hash_to_scalar`, the
///   selected concrete's `try_new`, `create_kem`, `setup`, `encapsulate`,
///   `pairing_product`, and `encode_gt`.
/// Mocked:  nothing.
#[test]
fn libraries_sharing_an_identifier_encode_the_pairing_product_to_the_same_bytes() {
    // Arrange
    struct KemConsumer {
        concrete: PairingConcrete,
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer {
        type Output = (TargetGroupEncodingIdentifier, Vec<u8>);

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
            let root = build_root(Default::default());
            let setup = payload
                .adapter
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(match scope {
                            IdentityScope::Entitlement => SetupScope::Entitlement,
                            IdentityScope::Asset => SetupScope::Asset { root: &root },
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
            let Ok(looked_up) = lookup_pairing_declaration(
                &build_lookup_pairing_declaration_deps(Default::default()),
                build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                    concrete: Some(self.concrete),
                }),
                build_lookup_pairing_declaration_payload(Default::default()),
            );
            (
                looked_up.declaration.target_group_encoding,
                encapsulated.encapsulated.key_material().expose().clone(),
            )
        }
    }

    struct Consumer {
        case: CreateKemParams,
        concrete: PairingConcrete,
    }

    impl ICreateKemCaseConsumer for Consumer {
        type Output = (TargetGroupEncodingIdentifier, Vec<u8>);

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    KemConsumer {
                        concrete: self.concrete,
                    },
                    CreateKemDepsOverrides {
                        admit_kem_composition: Some(admit_kem_composition),
                        lookup_kem_declaration: Some(lookup_kem_declaration),
                    },
                ),
                self.case,
                build_create_kem_payload(Default::default()),
            )
            .expect("the declared case admits its concrete")
            .output
        }
    }

    let member = KEM_CONCRETES
        .first()
        .expect("the declared set holds a member");
    let admitted = admit_kem_selection(
        &build_admit_kem_selection_deps(Default::default()),
        build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
            concrete: Some(*member),
        }),
        Value::Null,
    )
    .expect("a member of the declared set is admitted");
    let looked_up = lookup_kem_declaration(
        &build_lookup_kem_declaration_deps(Default::default()),
        build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
            concrete: Some(admitted.selection),
        }),
        build_lookup_kem_declaration_payload(Default::default()),
    )
    .expect("the real lookup returns the member's declaration");
    let scope = *looked_up
        .declaration
        .identity_scopes
        .first()
        .expect("the declaration provides a scope");
    let case = build_create_kem_params(CreateKemParamsOverrides {
        concrete: Some(admitted.selection),
        scope: Some(scope),
    });
    let mut observed: Vec<(TargetGroupEncodingIdentifier, Vec<u8>)> = Vec::new();
    for concrete in PAIRING_CONCRETES {
        for member in HASH_TO_SCALAR_CONCRETES {
            // Act
            let created = run_create_kem_case(
                &build_create_kem_case_deps(
                    Consumer {
                        case,
                        concrete: *concrete,
                    },
                    Default::default(),
                ),
                build_create_kem_case_params(CreateKemCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(build_hash_to_scalar_concrete(
                                *member,
                                HashToScalarConcreteOverrides,
                            )),
                        },
                    )),
                }),
                build_create_kem_case_payload(Default::default()),
            );

            // Assert
            assert!(created.is_ok());
            observed.push(created.expect("the case ran").output);
        }
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
///   returns for the bytes `create_encoding` returns for a value of the
///   family's mock description encodes to the encoding of the scalar the
///   received scalar type samples from `expected_uniform_bytes` over the tag
///   built from the `identity_tag` of the declaration `lookup_kem_declaration`
///   returns for the case's concrete and those bytes.
/// Arrange: each member of `KEM_CONCRETES` whose declaration provides the
///   entitlement scope, under that scope, over each member of
///   `HASH_TO_SCALAR_CONCRETES` whose declaration declares
///   `HashToScalarIdentifier::Keccak256V1`, each member of `ENCODING_CONCRETES`
///   whose declaration, read through `lookup_encoding_declaration`, carries
///   `EncodingIdentifier::EthereumAbiV1`, and every member of
///   `PAIRING_CONCRETES`; the canonical bytes are the bytes `create_encoding`
///   returns for the value `build_canonical_fields` returns at its defaults,
///   through `build_mock_i_encoding_consumer` carrying `MockIEncodingContract`;
///   `setup` and `derive_identity` run in the arrangement.
/// Act:     `identity_element_components` on the identity element
///   `derive_identity` returned over `KemIdentity::Entitlement` of those bytes.
/// Assert:  `encode_scalar` of the scalar equals `encode_scalar` of the scalar
///   sampled from `expected_uniform_bytes` over the tag built from the
///   `identity_tag` of the declaration `lookup_kem_declaration` returns for the
///   case's concrete, those bytes, and `P::Scalar::UNIFORM_BYTES_LENGTH`;
///   outside the consumers, `created.is_ok()`.
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
        params: CreateKemParams,
    }

    impl<'a, P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'a, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let looked_up = lookup_kem_declaration(
                &build_lookup_kem_declaration_deps(Default::default()),
                build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                    concrete: Some(self.params.concrete),
                }),
                build_lookup_kem_declaration_payload(Default::default()),
            )
            .expect("the real lookup returns the concrete's declaration");
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(looked_up.declaration.identity_tag.to_vec()),
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

    struct Consumer<'a> {
        canonical: &'a [u8],
    }

    impl<'a> ICreateKemCaseConsumer for Consumer<'a> {
        type Output = ();

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            let mut cases: Vec<CreateKemParams> = Vec::new();
            for member in KEM_CONCRETES {
                let admitted = admit_kem_selection(
                    &build_admit_kem_selection_deps(Default::default()),
                    build_admit_kem_selection_params(AdmitKemSelectionParamsOverrides {
                        concrete: Some(*member),
                    }),
                    Value::Null,
                )
                .expect("a member of the declared set is admitted");
                let looked_up = lookup_kem_declaration(
                    &build_lookup_kem_declaration_deps(Default::default()),
                    build_lookup_kem_declaration_params(LookupKemDeclarationParamsOverrides {
                        concrete: Some(admitted.selection),
                    }),
                    build_lookup_kem_declaration_payload(Default::default()),
                )
                .expect("the real lookup returns the member's declaration");
                for scope in looked_up.declaration.identity_scopes {
                    cases.push(build_create_kem_params(CreateKemParamsOverrides {
                        concrete: Some(admitted.selection),
                        scope: Some(*scope),
                    }));
                }
            }
            for case in cases
                .iter()
                .filter(|case| case.scope == IdentityScope::Entitlement)
            {
                let created = create_kem(
                    &build_create_kem_deps(
                        payload.pairing,
                        payload.hash_to_scalar,
                        KemConsumer {
                            pairing: payload.pairing,
                            canonical: self.canonical,
                            params: *case,
                        },
                        CreateKemDepsOverrides {
                            admit_kem_composition: Some(admit_kem_composition),
                            lookup_kem_declaration: Some(lookup_kem_declaration),
                        },
                    ),
                    *case,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for member in ENCODING_CONCRETES.iter().filter(|member| {
        let Ok(looked_up) = lookup_encoding_declaration(
            &build_lookup_encoding_declaration_deps(Default::default()),
            build_lookup_encoding_declaration_params(LookupEncodingDeclarationParamsOverrides {
                concrete: Some(**member),
            }),
            build_lookup_encoding_declaration_payload(Default::default()),
        );
        looked_up.declaration.identifier == EncodingIdentifier::EthereumAbiV1
    }) {
        let consumer = build_mock_i_encoding_consumer(
            &MockIEncodingContract,
            build_canonical_fields(CanonicalFieldsOverrides::default()),
            Vec::new(),
        );
        let canonical = create_encoding(
            &build_create_encoding_deps(consumer, Default::default()),
            build_create_encoding_params(CreateEncodingParamsOverrides {
                selection: Some(build_encoding_concrete(*member, EncodingConcreteOverrides)),
            }),
            build_create_encoding_payload(Default::default()),
        )
        .expect("an admitted encoding selection returns the consumer's output")
        .output
        .encoded
        .expect("the canonical fields encode")
        .bytes;

        for concrete in PAIRING_CONCRETES {
            for member in HASH_TO_SCALAR_CONCRETES.iter().filter(|member| {
                let Ok(looked_up) = lookup_hash_to_scalar_declaration(
                    &build_lookup_hash_to_scalar_declaration_deps(Default::default()),
                    build_lookup_hash_to_scalar_declaration_params(
                        LookupHashToScalarDeclarationParamsOverrides {
                            concrete: Some(**member),
                        },
                    ),
                    build_lookup_hash_to_scalar_declaration_payload(Default::default()),
                );
                looked_up.declaration.identifier == HashToScalarIdentifier::Keccak256V1
            }) {
                // Act
                let created = run_create_kem_case(
                    &build_create_kem_case_deps(
                        Consumer {
                            canonical: canonical.as_slice(),
                        },
                        Default::default(),
                    ),
                    build_create_kem_case_params(CreateKemCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        })),
                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                            CreateHashToScalarParamsOverrides {
                                concrete: Some(build_hash_to_scalar_concrete(
                                    *member,
                                    HashToScalarConcreteOverrides,
                                )),
                            },
                        )),
                    }),
                    build_create_kem_case_payload(Default::default()),
                );

                // Assert
                assert!(created.is_ok());
            }
        }
    }
}
