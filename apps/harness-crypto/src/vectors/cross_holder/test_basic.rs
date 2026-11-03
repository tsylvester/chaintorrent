#![allow(clippy::expect_used)]

use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CapsuleComponents, CreateKemParamsOverrides, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides,
    KemIdentity, RerandomizePayloadOverrides, SetupParamsOverrides, SetupPayloadOverrides,
    SetupScope, build_capsule_components_params, build_capsule_components_payload,
    build_create_kem_params, build_credential_components_params,
    build_credential_components_payload, build_decapsulate_params, build_decapsulate_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_issue_params, build_issue_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_rerandomize_params, build_rerandomize_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, build_fill_bytes_payload,
};

use super::cross_holder;
use super::interface::ICrossHolderCaseConsumer;
use super::mock::{
    CrossHolderCaseParamsOverrides, CrossHolderParamsOverrides, CrossHolderPayloadOverrides,
    build_cross_holder_case_deps, build_cross_holder_case_params, build_cross_holder_case_payload,
    build_cross_holder_deps, build_cross_holder_params, build_cross_holder_payload,
    run_cross_holder_case,
};
use crate::delivery::provides::{
    AdmittedKemIdentity, AdmittedKemIdentityOverrides, build_admitted_kem_identity,
};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(CrossHolderSuccessReturn { vectors })`, a record
///   for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3`, a count other than the params builder's default,
///   so a list sized from the default reports otherwise; the case's defaults.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`, so a list of
///   another length fails; the outermost `Result` `is_ok()`, proving the
///   runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn cross_holder_emits_the_configured_count_of_records() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_holder_case_params(Default::default()),
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_emits_the_configured_count_of_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are the master, `u0`,
///   `u1`, issuance, rerandomization, and encapsulation uniforms concatenated.
/// Arrange: `count` of `1`.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  the record's `draws` hold `6` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn cross_holder_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert_eq!(
                record.draws.len(),
                6 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_holder_case_params(Default::default()),
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the records in index order, within a record its
///   draws in the order master, `u0`, `u1`, issuance, rerandomization, and
///   encapsulation.
/// Arrange: `count` of `2`; the expected draws from the independent source,
///   six consecutive calls grouped for each record in turn, so a transposed
///   record or draw reports otherwise.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn cross_holder_records_the_draws_in_the_sequence_the_source_yields() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let draw = || {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let mut expected: Vec<Vec<u8>> = Vec::new();
            for _ in 0..self.count.get() {
                let mut draws = Vec::new();
                for _ in 0..6 {
                    draws.extend_from_slice(draw().expose());
                }
                expected.push(draws);
            }
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), expected.len());
            for (record, draws) in success.vectors.iter().zip(expected.iter()) {
                assert_eq!(&record.draws, draws);
            }
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_holder_case_params(Default::default()),
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Record under an entitlement identity → setup takes
///   `payload.identity.setup_scope`, `SetupScope::Entitlement`, and
///   derivation `payload.identity.identity`,
///   `KemIdentity::Entitlement { canonical }`; the record holds the
///   parameter set, identity element, issued credential, successor,
///   capsule, encapsulated value, and both decapsulated values its calls
///   returned over its draws.
/// Arrange: `count` of `1`; the admitted entitlement identity over
///   `b"pkg@1.0.0#1"`; the replay over the independent source's first six
///   draws under that identity's `setup_scope` and `identity`.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  the encoded `hpub` of the record's parameter set, the encoded
///   `scalar` and `element` of its identity element, the encoded `a` and
///   `b` of `issued` and of `successor`, and the encoded members of its
///   capsule's components each equal the replay's, and the
///   `key_material().expose()` bytes of `encapsulated`,
///   `issued_decapsulated`, and `successor_decapsulated` each equal the
///   replay's; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn cross_holder_records_the_values_its_draws_produce_under_the_entitlement_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let expected_identity = build_admitted_kem_identity(Default::default());

    struct CaseConsumer<'a> {
        count: VectorCount,
        expected_identity: AdmittedKemIdentity<'a>,
    }

    impl ICrossHolderCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let draw = || {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(self.expected_identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("the replay's setup succeeds");
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(self.expected_identity.identity),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let issued = kem
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
                .expect("the replay's issuance succeeds");
            let rerandomized = kem
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        RerandomizePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
            let encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
            let issued_decapsulated = kem
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
            let successor_decapsulated = kem
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
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
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
                pairing
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
                pairing
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
            let expected_parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            let actual_parameter_set = kem
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
            let expected_identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let actual_identity_element = kem
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
                kem.credential_components(
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
            let expected_capsule = kem
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&encapsulated.capsule, Default::default()),
                )
                .expect("the capsule's components are infallible")
                .components;
            let actual_capsule = kem
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
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(
            CaseConsumer {
                count,
                expected_identity,
            },
            Default::default(),
        ),
        build_cross_holder_case_params(Default::default()),
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_records_the_values_its_draws_produce_under_the_entitlement_scope
}

/// Contract: Record under an asset identity → setup takes
///   `payload.identity.setup_scope`, `SetupScope::Asset { root }`, and
///   derivation `payload.identity.identity`, `KemIdentity::Asset { root }`;
///   the record holds the values its calls returned over its draws.
/// Arrange: `scope` `IdentityScope::Asset`, `count` of `1`; the admitted
///   asset identity over the root `build_root` builds from `[0x44; 32]`;
///   the replay as the entitlement block builds it, under that identity's
///   `setup_scope` and `identity`.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  as the entitlement block, against this replay; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the KEM's scope, which
///   this block names; the independent source, a separate adapter at the
///   same defaults as the subject's.
#[test]
fn cross_holder_records_the_values_its_draws_produce_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    let expected_identity = build_admitted_kem_identity(AdmittedKemIdentityOverrides {
        setup_scope: Some(SetupScope::Asset { root: &root }),
        identity: Some(KemIdentity::Asset { root: &root }),
    });

    struct CaseConsumer<'a> {
        count: VectorCount,
        expected_identity: AdmittedKemIdentity<'a>,
    }

    impl ICrossHolderCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Arrange
            let draw = || {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(self.expected_identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("the replay's setup succeeds");
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(self.expected_identity.identity),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let issued = kem
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
                .expect("the replay's issuance succeeds");
            let rerandomized = kem
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &derived.identity_element,
                        &issued.credential,
                        RerandomizePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
            let encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
            let issued_decapsulated = kem
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
            let successor_decapsulated = kem
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
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(CrossHolderParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
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
                pairing
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
                pairing
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
            let expected_parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            let actual_parameter_set = kem
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
            let expected_identity_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let actual_identity_element = kem
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
                kem.credential_components(
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
            let expected_capsule = kem
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(&encapsulated.capsule, Default::default()),
                )
                .expect("the capsule's components are infallible")
                .components;
            let actual_capsule = kem
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
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(
            CaseConsumer {
                count,
                expected_identity,
            },
            Default::default(),
        ),
        build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_records_the_values_its_draws_produce_under_the_asset_scope
}
