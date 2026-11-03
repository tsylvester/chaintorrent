#![allow(clippy::expect_used)]

use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IdentityScope,
    IssuePayloadOverrides, KemIdentity, RerandomizePayloadOverrides, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_create_kem_params, build_credential_components_params,
    build_credential_components_payload, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
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

use super::interface::IRerandomizationCaseConsumer;
use super::mock::{
    RerandomizationCaseParamsOverrides, RerandomizationParamsOverrides,
    RerandomizationPayloadOverrides, build_rerandomization_case_deps,
    build_rerandomization_case_params, build_rerandomization_case_payload,
    build_rerandomization_deps, build_rerandomization_params, build_rerandomization_payload,
    run_rerandomization_case,
};
use super::rerandomization;
use crate::delivery::provides::{
    AdmittedKemIdentity, AdmittedKemIdentityOverrides, build_admitted_kem_identity,
};
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(RerandomizationSuccessReturn { vectors })`, a
///   record for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3`, a count other than the params builder's default,
///   so a list sized from the default reports otherwise; the case's defaults.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`, so a list of
///   another length fails; the outermost `Result` `is_ok()`, proving the
///   runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn rerandomization_emits_the_configured_count_of_records() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(CaseConsumer { count }, Default::default()),
        build_rerandomization_case_params(Default::default()),
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_emits_the_configured_count_of_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are the master, `u0`,
///   `u1`, issuance, and rerandomization uniforms concatenated.
/// Arrange: `count` of `1`.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  the record's `draws` hold `5` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn rerandomization_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert_eq!(
                record.draws.len(),
                5 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(CaseConsumer { count }, Default::default()),
        build_rerandomization_case_params(Default::default()),
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the records in index order, within a record its
///   draws in the order master, `u0`, `u1`, issuance, and rerandomization.
/// Arrange: `count` of `2`; the expected draws from the independent source,
///   five consecutive calls grouped for each record in turn, so a transposed
///   record or draw reports otherwise.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn rerandomization_records_the_draws_in_the_sequence_the_source_yields() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
                for _ in 0..5 {
                    draws.extend_from_slice(draw().expose());
                }
                expected.push(draws);
            }
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
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
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(CaseConsumer { count }, Default::default()),
        build_rerandomization_case_params(Default::default()),
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Record under an entitlement identity → setup takes
///   `payload.identity.setup_scope`, `SetupScope::Entitlement`, and
///   derivation `payload.identity.identity`,
///   `KemIdentity::Entitlement { canonical }`; the record holds the
///   parameter set, identity element, issued credential, and rerandomized
///   credential its calls returned over its draws, and a `true` verdict for
///   each credential.
/// Arrange: `count` of `1`; the admitted entitlement identity over
///   `b"pkg@1.0.0#1"`; the replay over the independent source's first five
///   draws under that identity's `setup_scope` and `identity`.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  the encoded `hpub` of the record's parameter set, the encoded
///   `scalar` and `element` of its identity element, and the encoded `a` and
///   `b` of `issued` and of `rerandomized` each equal the replay's by
///   `assert_eq!`, and `issued_is_valid` and `rerandomized_is_valid` are
///   each `true`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn rerandomization_records_the_values_and_verdicts_its_draws_produce_under_the_entitlement_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let expected_identity = build_admitted_kem_identity(Default::default());

    struct CaseConsumer<'a> {
        count: VectorCount,
        expected_identity: AdmittedKemIdentity<'a>,
    }

    impl IRerandomizationCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
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
                .expect("the parameter set's components are infallible")
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
                .expect("the identity element's components are infallible")
                .components;
            assert_eq!(
                encode_scalar(&actual_identity_element.scalar),
                encode_scalar(&expected_identity_element.scalar)
            );
            assert_eq!(
                encode_g1(&actual_identity_element.element),
                encode_g1(&expected_identity_element.element)
            );
            let expected_issued = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&issued.credential, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            let actual_issued = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&record.issued, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            assert_eq!(encode_g1(&actual_issued.a), encode_g1(&expected_issued.a));
            assert_eq!(encode_g2(&actual_issued.b), encode_g2(&expected_issued.b));
            let expected_rerandomized = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(
                        &rerandomized.credential,
                        Default::default(),
                    ),
                )
                .expect("the credential's components are infallible")
                .components;
            let actual_rerandomized = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&record.rerandomized, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            assert_eq!(
                encode_g1(&actual_rerandomized.a),
                encode_g1(&expected_rerandomized.a)
            );
            assert_eq!(
                encode_g2(&actual_rerandomized.b),
                encode_g2(&expected_rerandomized.b)
            );
            assert!(record.issued_is_valid);
            assert!(record.rerandomized_is_valid);
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(
            CaseConsumer {
                count,
                expected_identity,
            },
            Default::default(),
        ),
        build_rerandomization_case_params(Default::default()),
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_records_the_values_and_verdicts_its_draws_produce_under_the_entitlement_scope
}

/// Contract: Record under an asset identity → setup takes
///   `payload.identity.setup_scope`, `SetupScope::Asset { root }`, and
///   derivation `payload.identity.identity`, `KemIdentity::Asset { root }`;
///   the record holds the values its calls returned over its draws and a
///   `true` verdict for each credential.
/// Arrange: the KEM's `scope` `IdentityScope::Asset`, `count` of `1`; the
///   admitted asset identity over the root `build_root` builds from
///   `[0x44; 32]`; the replay as the entitlement block builds it, under that
///   identity's `setup_scope` and `identity`.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  as the entitlement block, against this replay; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn rerandomization_records_the_values_and_verdicts_its_draws_produce_under_the_asset_scope() {
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

    impl IRerandomizationCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
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
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                }),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
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
                .expect("the parameter set's components are infallible")
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
                .expect("the identity element's components are infallible")
                .components;
            assert_eq!(
                encode_scalar(&actual_identity_element.scalar),
                encode_scalar(&expected_identity_element.scalar)
            );
            assert_eq!(
                encode_g1(&actual_identity_element.element),
                encode_g1(&expected_identity_element.element)
            );
            let expected_issued = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&issued.credential, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            let actual_issued = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&record.issued, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            assert_eq!(encode_g1(&actual_issued.a), encode_g1(&expected_issued.a));
            assert_eq!(encode_g2(&actual_issued.b), encode_g2(&expected_issued.b));
            let expected_rerandomized = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(
                        &rerandomized.credential,
                        Default::default(),
                    ),
                )
                .expect("the credential's components are infallible")
                .components;
            let actual_rerandomized = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&record.rerandomized, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            assert_eq!(
                encode_g1(&actual_rerandomized.a),
                encode_g1(&expected_rerandomized.a)
            );
            assert_eq!(
                encode_g2(&actual_rerandomized.b),
                encode_g2(&expected_rerandomized.b)
            );
            assert!(record.issued_is_valid);
            assert!(record.rerandomized_is_valid);
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(
            CaseConsumer {
                count,
                expected_identity,
            },
            Default::default(),
        ),
        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_records_the_values_and_verdicts_its_draws_produce_under_the_asset_scope
}
