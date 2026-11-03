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

use super::interface::{IRerandomizationCaseConsumer, RerandomizationErrorReturn};
use super::mock::{
    RerandomizationCaseParamsOverrides, RerandomizationPayloadOverrides,
    build_rerandomization_case_deps, build_rerandomization_case_params,
    build_rerandomization_case_payload, build_rerandomization_deps, build_rerandomization_params,
    build_rerandomization_payload, run_rerandomization_case,
};
use super::rerandomization;
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(RerandomizationErrorReturn::FillBytes(error))`, `error` unchanged.
/// Arrange: the subject's source and the independent source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; the expected `error` the refusal of the
///   independent source's `fill_bytes`.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::FillBytes(error))` whole by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete at the default selection;
///   the subject's and independent randomness sources, the family's mock
///   concrete under `FillBytesRefused`.
#[test]
fn rerandomization_returns_the_draw_refusal_whole() {
    // Arrange
    struct CaseConsumer;

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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(Default::default()),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::FillBytes(error))
            );
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(CaseConsumer, Default::default()),
        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_returns_the_draw_refusal_whole
}

/// Contract: Record, `deps.kem.setup` fails →
///   `Err(RerandomizationErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the expected `error` the `Err` of
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes` over
///   the independent source's first draw.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(RerandomizationErrorReturn::
///   Setup(SetupErrorReturn::MasterScalarSampling(error)))` whole by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: the pairing family's mock concrete under
///   `SampleScalarWrongLength`; every other family's mock concrete at the
///   default selection; the independent source, a separate adapter at the
///   same defaults as the subject's.
#[test]
fn rerandomization_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange
    struct CaseConsumer;

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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(Default::default()),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(error)
                ))
            );
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(CaseConsumer, Default::default()),
        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Record, `deps.kem.derive_identity` fails →
///   `Err(RerandomizationErrorReturn::DeriveIdentity(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Entitlement` and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected refusal the `Err` of `hash_to_scalar`
///   over `build_hash_to_scalar_params` and `build_hash_to_scalar_payload`
///   at their defaults on the adapter built under the same mode.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(RerandomizationErrorReturn::
///   DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))`
///   whole by `assert_eq!`, `error` the expected refusal; the outermost
///   `Result` `is_ok()`, proving the runner's levels ran.
/// Collaborators: the hash-to-scalar family's mock concrete under
///   `SamplingRefused`; every other family's mock concrete at the default
///   selection.
#[test]
fn rerandomization_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    // Arrange
    struct CaseConsumer;

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
            &self,
            _pairing: &P,
            hash_to_scalar: &H,
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
            // Arrange
            let error = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the refusing adapter refuses the hash");
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(Default::default()),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::HashToScalar(error)
                ))
            );
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(CaseConsumer, Default::default()),
        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
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
        build_rerandomization_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // rerandomization_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole
}

/// Contract: Record under `Asset`, `deps.kem.setup` fails on the root →
///   `Err(RerandomizationErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: the scope `IdentityScope::Asset`, the admitted asset identity
///   over the root `build_root` builds from `[0x44; 32]` in the case
///   payload's default, and the hash-to-scalar selection and expected
///   refusal of the derivation block.
/// Act:     `rerandomization`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals `Some(RerandomizationErrorReturn::
///   Setup(SetupErrorReturn::HashToScalar(error)))` whole by `assert_eq!`,
///   `error` the expected refusal; the outermost `Result` `is_ok()`,
///   proving the runner's levels ran.
/// Collaborators: the hash-to-scalar family's mock concrete under
///   `SamplingRefused`; every other family's mock concrete at the default
///   selection.
#[test]
fn rerandomization_returns_the_hash_to_scalar_refusal_of_asset_setup_whole() {
    // Arrange
    struct CaseConsumer;

    impl IRerandomizationCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_rerandomization_case<P, H, K>(
            &self,
            _pairing: &P,
            hash_to_scalar: &H,
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
            // Arrange
            let error = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the refusing adapter refuses the hash");
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(kem, random, Default::default()),
                build_rerandomization_params(Default::default()),
                build_rerandomization_payload(RerandomizationPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::Setup(
                    SetupErrorReturn::HashToScalar(error)
                ))
            );
        }
    }

    // Act
    let returned = run_rerandomization_case(
        &build_rerandomization_case_deps(CaseConsumer, Default::default()),
        build_rerandomization_case_params(RerandomizationCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
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
    // rerandomization_returns_the_hash_to_scalar_refusal_of_asset_setup_whole
}
