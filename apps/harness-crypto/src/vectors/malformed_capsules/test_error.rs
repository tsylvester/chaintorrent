#![allow(clippy::expect_used)]

use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
};
use kem::{
    CreateKemParamsOverrides, ICredentialKemAdapter, IdentityScope, SetupErrorReturn,
    build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarErrorReturn,
    build_create_pairing_params,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::interface::{IMalformedCapsulesCaseConsumer, MalformedCapsulesErrorReturn};
use super::malformed_capsules;
use super::mock::{
    MalformedCapsulesCaseParamsOverrides, MalformedCapsulesParamsOverrides,
    MalformedCapsulesPayloadOverrides, build_malformed_capsules_case_deps,
    build_malformed_capsules_case_params, build_malformed_capsules_case_payload,
    build_malformed_capsules_deps, build_malformed_capsules_params,
    build_malformed_capsules_payload, run_malformed_capsules_case,
};
use crate::delivery::provides::AdmittedKemIdentity;
use crate::{VectorCount, build_vector_count};

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(MalformedCapsulesErrorReturn::FillBytes(error))`, `error`
///   unchanged.
/// Arrange: the subject's source from `build_create_random_source_params`
///   overriding `kind` with `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`, named in the
///   case params' `random_source`; the expected `error` the literal
///   `FillBytesErrorReturn::MockIRandomSourceAdapter`; the count by
///   `build_vector_count` at its default.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))` whole by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the subject's randomness
///   source, the family's mock concrete under `FillBytesRefused`.
#[test]
fn malformed_capsules_returns_the_draw_refusal_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let random_source = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
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
            // Arrange
            let expected = FillBytesErrorReturn::MockIRandomSourceAdapter;
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::FillBytes(expected))
            );
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
            random_source: Some(random_source),
            ..Default::default()
        }),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_returns_the_draw_refusal_whole
}

/// Contract: Record, `deps.kem.setup` fails →
///   `Err(MalformedCapsulesErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`, named in the case params' `pairing`; the
///   count by `build_vector_count` at its default; the expected `error`
///   the literal `SampleUniformScalarErrorReturn::WrongLength
///   { expected: 16, actual: 16 }`.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected: 16, actual: 16 })))` whole by `assert_eq!`, the master
///   sampling the first refused; the outermost `Result` `is_ok()`,
///   proving the runner's levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the pairing family's
///   mock concrete under `SampleScalarWrongLength`.
#[test]
fn malformed_capsules_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let pairing = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::SampleScalarWrongLength,
        )),
        ..Default::default()
    });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
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
            // Arrange
            let expected = SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 16,
            };
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(expected)
                ))
            );
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
            pairing: Some(pairing),
            ..Default::default()
        }),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Record under `Asset`, `deps.kem.setup` fails on the root →
///   `Err(MalformedCapsulesErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `scope` `IdentityScope::Asset` in the case params' `kem`, the
///   admitted asset identity over the root `build_root` builds from
///   `[0x44; 32]`, the case payload's default, and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`, named in the case params' `hash_to_scalar`; the
///   count by `build_vector_count` at its default; the expected `error`
///   the literal `HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength
///   { expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
///   actual: 0 })`.
/// Act:     `malformed_capsules`, once, in the case consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` whole by `assert_eq!`, `error` the expected
///   literal; the outermost `Result` `is_ok()`, proving the runner's
///   levels ran.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the KEM's `scope`
///   selection and the hash-to-scalar family's mock concrete under
///   `SamplingRefused`, each named.
#[test]
fn malformed_capsules_returns_the_hash_to_scalar_refusal_of_asset_setup_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let hash_to_scalar = build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
        concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        ..Default::default()
    });
    let kem_selection = build_create_kem_params(CreateKemParamsOverrides {
        scope: Some(IdentityScope::Asset),
        ..Default::default()
    });

    struct CaseConsumer {
        count: VectorCount,
    }

    impl IMalformedCapsulesCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_malformed_capsules_case<P, H, K>(
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
            // Arrange
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(kem, random, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                }),
                build_malformed_capsules_payload(MalformedCapsulesPayloadOverrides {
                    identity: Some(identity),
                }),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::Setup(
                    SetupErrorReturn::HashToScalar(expected)
                ))
            );
        }
    }

    // Act
    let returned = run_malformed_capsules_case(
        &build_malformed_capsules_case_deps(CaseConsumer { count }, Default::default()),
        build_malformed_capsules_case_params(MalformedCapsulesCaseParamsOverrides {
            hash_to_scalar: Some(hash_to_scalar),
            kem: Some(kem_selection),
            ..Default::default()
        }),
        build_malformed_capsules_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // malformed_capsules_returns_the_hash_to_scalar_refusal_of_asset_setup_whole
}
