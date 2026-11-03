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

use super::cross_holder;
use super::interface::{CrossHolderErrorReturn, ICrossHolderCaseConsumer};
use super::mock::{
    CrossHolderCaseParamsOverrides, CrossHolderPayloadOverrides, build_cross_holder_case_deps,
    build_cross_holder_case_params, build_cross_holder_case_payload, build_cross_holder_deps,
    build_cross_holder_params, build_cross_holder_payload, run_cross_holder_case,
};
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(CrossHolderErrorReturn::FillBytes(error))`, `error` unchanged.
/// Arrange: the subject's source and the independent source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; the expected `error` the refusal of the
///   independent source's `fill_bytes` under the declared uniform length.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(CrossHolderErrorReturn::FillBytes(error))` whole by
///   `assert_eq!`, `error` the expected refusal; the outermost `Result`
///   `is_ok()`, proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the randomness
///   selection, which this block names; the subject's source and the
///   independent source, each the family's mock concrete under
///   `FillBytesRefused`.
#[test]
fn cross_holder_returns_the_draw_refusal_whole() {
    // Arrange

    struct CaseConsumer;

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
            let expected = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .err()
                .expect("the independent source refuses under its named mode");
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(Default::default()),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(CrossHolderErrorReturn::FillBytes(expected))
            );
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(CaseConsumer, Default::default()),
        build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_returns_the_draw_refusal_whole
}

/// Contract: Record, `deps.kem.setup` fails →
///   `Err(CrossHolderErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the expected `error` the `Err` of
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes` over
///   the independent source's first draw at the declared uniform length.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(CrossHolderErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(error)))` whole by `assert_eq!`, `error` the
///   expected refusal; the outermost `Result` `is_ok()`, proving the
///   runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the pairing, which this
///   block names under `SampleScalarWrongLength`.
#[test]
fn cross_holder_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange

    struct CaseConsumer;

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
            let uniform = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .expect("the independent source fills")
                .bytes;
            let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            );
            let expected = sampled
                .err()
                .expect("the pairing's scalar sampling refuses under its named mode");
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(Default::default()),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(CrossHolderErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(expected)
                ))
            );
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(CaseConsumer, Default::default()),
        build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Record, `deps.kem.derive_identity` fails →
///   `Err(CrossHolderErrorReturn::DeriveIdentity(error))`, `error`
///   unchanged.
/// Arrange: `build_create_hash_to_scalar_params` overriding `concrete`
///   with `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected refusal the `Err` of `hash_to_scalar`
///   over `build_hash_to_scalar_params` and `build_hash_to_scalar_payload`
///   at their defaults on an adapter built under the same mode.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(CrossHolderErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::
///   HashToScalar(error)))` whole by `assert_eq!`, `error` the expected
///   refusal; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the hash-to-scalar
///   family, which this block names under `SamplingRefused`.
#[test]
fn cross_holder_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    // Arrange

    struct CaseConsumer;

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
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
            let expected = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the hash-to-scalar adapter refuses under its named mode");
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(Default::default()),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(CrossHolderErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::HashToScalar(expected)
                ))
            );
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(CaseConsumer, Default::default()),
        build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
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
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole
}

/// Contract: Record under `Asset`, `deps.kem.setup` fails on the root →
///   `Err(CrossHolderErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `scope` `IdentityScope::Asset`, the admitted asset identity
///   over the root `build_root` builds from `[0x44; 32]` — the case
///   payload's default — and the hash-to-scalar selection and expected
///   refusal of the derivation block.
/// Act:     `cross_holder`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(CrossHolderErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` whole by `assert_eq!`, `error` the expected
///   refusal; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the hash-to-scalar
///   family under `SamplingRefused` and the KEM's scope, which this block
///   names.
#[test]
fn cross_holder_returns_the_hash_to_scalar_refusal_of_asset_setup_whole() {
    // Arrange

    struct CaseConsumer;

    impl ICrossHolderCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_holder_case<P, H, K>(
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
            let expected = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the hash-to-scalar adapter refuses under its named mode");
            // Act
            let returned = cross_holder::<K>(
                &build_cross_holder_deps(kem, random, Default::default()),
                build_cross_holder_params(Default::default()),
                build_cross_holder_payload(CrossHolderPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(CrossHolderErrorReturn::Setup(
                    SetupErrorReturn::HashToScalar(expected)
                ))
            );
        }
    }

    // Act
    let returned = run_cross_holder_case(
        &build_cross_holder_case_deps(CaseConsumer, Default::default()),
        build_cross_holder_case_params(CrossHolderCaseParamsOverrides {
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
        build_cross_holder_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_holder_returns_the_hash_to_scalar_refusal_of_asset_setup_whole
}
