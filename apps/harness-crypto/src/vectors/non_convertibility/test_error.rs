#![allow(clippy::expect_used)]

use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
    build_hash_to_scalar_params, build_hash_to_scalar_payload,
};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope,
    SetupErrorReturn, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload,
};

use super::interface::{INonConvertibilityCaseConsumer, NonConvertibilityErrorReturn};
use super::mock::{
    NonConvertibilityCaseParamsOverrides, NonConvertibilityPayloadOverrides,
    build_non_convertibility_case_deps, build_non_convertibility_case_params,
    build_non_convertibility_case_payload, build_non_convertibility_deps,
    build_non_convertibility_params, build_non_convertibility_payload, run_non_convertibility_case,
};
use super::non_convertibility;
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: Scope under `SetupScope::Asset` → no draw is made, so a
///   refusing source is not reached.
/// Arrange: `scope` `IdentityScope::Asset`, so the KEM level composes both
///   admitted identities under `SetupScope::Asset`, and the subject's
///   source from `build_create_random_source_params` overriding `kind`
///   with `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the result `is_ok()` and its `vectors` is empty; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete at the default selection,
///   except the KEM's `scope` and the randomness family's mock concrete
///   under `FillBytesRefused`, named through the selecting params.
#[test]
fn non_convertibility_makes_no_draw_under_the_asset_scope() {
    // Arrange
    struct CaseConsumer;

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
                build_non_convertibility_params(Default::default()),
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
        &build_non_convertibility_case_deps(CaseConsumer, Default::default()),
        build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_makes_no_draw_under_the_asset_scope
}

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(NonConvertibilityErrorReturn::FillBytes(error))`, `error`
///   unchanged.
/// Arrange: the subject's source and the independent source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; the expected `error` the refusal of the
///   independent source's `fill_bytes`.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(NonConvertibilityErrorReturn::FillBytes(error))` whole by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete at the default selection;
///   the subject's and independent randomness sources, the family's mock
///   concrete under `FillBytesRefused`.
#[test]
fn non_convertibility_returns_the_draw_refusal_whole() {
    // Arrange
    struct CaseConsumer;

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
            let error = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .err()
                .expect("the refusing source refuses");
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(Default::default()),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::FillBytes(error))
            );
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer, Default::default()),
        build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_returns_the_draw_refusal_whole
}

/// Contract: Record, `deps.kem.setup` fails →
///   `Err(NonConvertibilityErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the expected `error` the `Err` of
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes` over
///   the independent source's first draw.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(NonConvertibilityErrorReturn::
///   Setup(SetupErrorReturn::MasterScalarSampling(error)))` whole by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: the pairing family's mock concrete under
///   `SampleScalarWrongLength`; every other family's mock concrete at the
///   default selection; the independent source, a separate adapter at the
///   same defaults as the subject's.
#[test]
fn non_convertibility_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange
    struct CaseConsumer;

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
            let uniform = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .expect("the independent source fills")
                .bytes;
            let error = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .err()
            .expect("the refusing scalar refuses the sampling");
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(Default::default()),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(error)
                ))
            );
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer, Default::default()),
        build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Record, `deps.kem.derive_identity` fails →
///   `Err(NonConvertibilityErrorReturn::DeriveIdentity(error))`, `error`
///   unchanged.
/// Arrange: `build_create_hash_to_scalar_params` overriding `concrete`
///   with `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected refusal the `Err` of `hash_to_scalar`
///   over `build_hash_to_scalar_params` and `build_hash_to_scalar_payload`
///   at their defaults on an adapter built under the same mode.
/// Act:     `non_convertibility`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(NonConvertibilityErrorReturn::
///   DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))`
///   whole by `assert_eq!`, `error` the expected refusal; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: the hash-to-scalar family's mock concrete under
///   `SamplingRefused`; every other family's mock concrete at the default
///   selection.
#[test]
fn non_convertibility_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    // Arrange
    struct CaseConsumer;

    impl INonConvertibilityCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_non_convertibility_case<P, H, K>(
            &self,
            _pairing: &P,
            hash_to_scalar: &H,
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
            // Arrange
            let error = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the refusing adapter refuses the hash");
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(kem, random, Default::default()),
                build_non_convertibility_params(Default::default()),
                build_non_convertibility_payload(NonConvertibilityPayloadOverrides {
                    identity: Some(identity),
                    other_identity: Some(other_identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::HashToScalar(error)
                ))
            );
        }
    }

    // Act
    let returned = run_non_convertibility_case(
        &build_non_convertibility_case_deps(CaseConsumer, Default::default()),
        build_non_convertibility_case_params(NonConvertibilityCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_non_convertibility_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // non_convertibility_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole
}
