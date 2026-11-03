#![allow(clippy::expect_used)]

use hash_to_scalar::IHashToScalarAdapter;
use kem::{
    CreateKemParamsOverrides, DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IdentityScope,
    IssuePayloadOverrides, SetupParamsOverrides, SetupPayloadOverrides, build_create_kem_params,
    build_credential_components_params, build_credential_components_payload,
    build_derive_identity_params, build_derive_identity_payload, build_issue_params,
    build_issue_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, build_fill_bytes_payload,
};

use super::interface::INonConvertibilityCaseConsumer;
use super::mock::{
    NonConvertibilityCaseParamsOverrides, NonConvertibilityCasePayloadOverrides,
    NonConvertibilityParamsOverrides, NonConvertibilityPayloadOverrides,
    build_non_convertibility_case_deps, build_non_convertibility_case_params,
    build_non_convertibility_case_payload, build_non_convertibility_deps,
    build_non_convertibility_params, build_non_convertibility_payload, run_non_convertibility_case,
};
use super::non_convertibility;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::{VectorCount, VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Computed → `Ok(NonConvertibilitySuccessReturn { vectors })`, a
///   record for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3`, a count other than the builder's default, so a
///   list sized from the default reports otherwise; the case's defaults.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  `vectors` holds `3` records by `assert_eq!`; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn non_convertibility_emits_the_configured_count_of_records() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.vectors.len(), 3);
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
        build_non_convertibility_case_params(Default::default()),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_emits_the_configured_count_of_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are the master, `u0`,
///   `u1`, and issuance uniforms concatenated.
/// Arrange: `count` of `1`.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the record's `draws` hold `4` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn non_convertibility_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert_eq!(
                record.draws.len(),
                4 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
        build_non_convertibility_case_params(Default::default()),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_draws_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the records in index order, within a record its
///   draws in the order master, `u0`, `u1`, and issuance.
/// Arrange: `count` of `2`; the expected draws from the independent source,
///   four consecutive calls grouped for each record in turn, so a
///   transposed record or draw reports otherwise.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  each record's `draws` equal its expected draws by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn non_convertibility_records_the_draws_in_the_sequence_the_source_yields() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
                for _ in 0..4 {
                    draws.extend_from_slice(draw().expose());
                }
                expected.push(draws);
            }
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
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
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
        build_non_convertibility_case_params(Default::default()),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_records_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Scope under `SetupScope::Entitlement`, Record, and Record
///   outcome → the record holds the credential `issue` returned over its
///   draws for the admitted identity's element, a `true` verdict against
///   that element, and a `false` verdict against the other admitted
///   identity's element.
/// Arrange: `count` of `1`; the replay over the independent source's first
///   four draws under the admitted identity's `setup_scope` and `identity`.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the encoded `a` and `b` of the record's `credential` each equal
///   the replay's by `assert_eq!`, `is_valid` is `true`, and `other_is_valid`
///   is `false`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder; the independent source, a
///   separate adapter at the same defaults as the subject's.
#[test]
fn non_convertibility_records_the_credential_and_verdicts_its_draws_produce() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
                        scope: Some(identity.setup_scope),
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
                            identity: Some(identity.identity),
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
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
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
            let actual_credential = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&record.credential, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            let expected_credential = kem
                .credential_components(
                    build_credential_components_params(Default::default()),
                    build_credential_components_payload(&issued.credential, Default::default()),
                )
                .expect("the credential's components are infallible")
                .components;
            assert_eq!(
                encode_g1(&actual_credential.a),
                encode_g1(&expected_credential.a)
            );
            assert_eq!(
                encode_g2(&actual_credential.b),
                encode_g2(&expected_credential.b)
            );
            assert!(record.is_valid);
            assert!(!record.other_is_valid);
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
        build_non_convertibility_case_params(Default::default()),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_records_the_credential_and_verdicts_its_draws_produce
}

/// Contract: Computed, `other_is_valid` → `true` when the other admitted
///   identity's canonical bytes equal the admitted identity's.
/// Arrange: `count` of `1`; the other identity bytes `b"pkg@1.0.0#1"`, the
///   identity's bytes, against the default other identity bytes under
///   which the verdict is `false`.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  `is_valid` and `other_is_valid` are each `true`; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn non_convertibility_records_a_true_verdict_against_the_other_identity_when_it_is_the_configured_one()
 {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            let record = success.vectors.first().expect("the one record");
            // Assert
            assert!(record.is_valid);
            assert!(record.other_is_valid);
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
        build_non_convertibility_case_params(Default::default()),
        build_non_convertibility_case_payload(NonConvertibilityCasePayloadOverrides {
            other_canonical: Some(b"pkg@1.0.0#1".to_vec()),
            ..Default::default()
        }),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_records_a_true_verdict_against_the_other_identity_when_it_is_the_configured_one
}

/// Contract: Scope under `SetupScope::Asset` →
///   `Ok(NonConvertibilitySuccessReturn { vectors })`, `vectors` empty.
/// Arrange: `scope` `IdentityScope::Asset`, so the KEM level composes both
///   admitted identities under `SetupScope::Asset`, and `count` of `3`, a
///   count that yields three records under the entitlement scope.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  `vectors` is empty; the outermost `Result` `is_ok()`, proving
///   the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the KEM's `scope` named
///   through the selecting params.
#[test]
fn non_convertibility_returns_an_empty_set_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
        {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                }),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert!(success.vectors.is_empty());
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer { count }, Default::default()),
        build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_returns_an_empty_set_under_the_asset_scope
}
