#![allow(clippy::expect_used)]

use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload,
    build_hash_to_scalar_params, build_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides, CreateKemReturn,
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    SetupErrorReturn, build_create_kem_deps, build_create_kem_params, build_create_kem_payload,
    create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, MockIPairingAdapterFailureMode,
    PairingConcrete, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};

use super::interface::NonConvertibilityErrorReturn;
use super::mock::{
    NonConvertibilityParamsOverrides, build_non_convertibility_deps,
    build_non_convertibility_params, build_non_convertibility_payload,
};
use super::non_convertibility;
use crate::{VectorCount, build_vector_count};

/// Contract: Scope under `Asset` → no draw is made, so a refusing source is
///   not reached.
/// Arrange: the scope `IdentityScope::Asset`, the identity
///   `b"pkg@1.0.0#1"`, the other identity `b"pkg@2.0.0#7"`, and the count by
///   `build_vector_count` at its default; the subject's source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, so a draw under the asset scope reports the draw's
///   refusal instead of the empty set.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  the result `is_ok()` and its `vectors` is empty by `assert!`;
///   the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, and kem's mock concretes
///   in their default modes through their factories; the subject's
///   randomness source, the family's mock concrete under `FillBytesRefused`.
#[test]
fn non_convertibility_makes_no_draw_under_the_asset_scope() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Asset;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";
    let subject_kind =
        RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused);

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
            );
            // Assert
            let success = returned.expect("no draw is made, so the refusing source is not reached");
            assert!(success.vectors.is_empty());
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        other_identity: &'a [u8],
        subject_kind: RandomSourceKind,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(self.subject_kind),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's configured selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        count: self.count,
                        identity: self.identity,
                        other_identity: self.other_identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
        other_identity,
        subject_kind,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // non_convertibility_makes_no_draw_under_the_asset_scope
}

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(NonConvertibilityErrorReturn::FillBytes(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Entitlement`, the identity
///   `b"pkg@1.0.0#1"`, the other identity `b"pkg@2.0.0#7"`, and the count by
///   `build_vector_count` at its default; the subject's source and the
///   independent source from `build_create_random_source_params` overriding
///   `kind` with `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; the expected `error` the refusal of the independent
///   source's `fill_bytes` under the declared uniform length.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(NonConvertibilityErrorReturn::FillBytes(error))` whole by
///   `assert_eq!`, `error` the expected refusal; the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, and kem's mock concretes
///   in their default modes through their factories; the subject's
///   randomness source and the independent source, each the family's mock
///   concrete under `FillBytesRefused`.
#[test]
fn non_convertibility_returns_the_draw_refusal_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";
    let source_kind = RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused);

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let expected = self
                .independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .err()
                .expect("the independent source refuses under its named mode");
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::FillBytes(expected))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        other_identity: &'a [u8],
        subject_kind: RandomSourceKind,
        independent_kind: RandomSourceKind,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(self.subject_kind),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's configured selection constructs");
            let independent_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(self.independent_kind),
                }),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's configured selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        independent: &*independent_source.adapter,
                        count: self.count,
                        identity: self.identity,
                        other_identity: self.other_identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
        other_identity,
        subject_kind: source_kind,
        independent_kind: source_kind,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // non_convertibility_returns_the_draw_refusal_whole
}

/// Contract: Record, `deps.kem.setup` fails →
///   `Err(NonConvertibilityErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the scope `IdentityScope::Entitlement`, the
///   identity `b"pkg@1.0.0#1"`, the other identity `b"pkg@2.0.0#7"`, and
///   the count by `build_vector_count` at its default; the expected `error`
///   the `Err` of `<P::Scalar as ISampleUniformScalar>::
///   sample_from_uniform_bytes` over the independent source's first draw at
///   the declared uniform length.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(NonConvertibilityErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(error)))` whole by `assert_eq!`, `error` the
///   expected refusal; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the hash-to-scalar's and kem's mock concretes in their
///   default modes through their factories; the pairing's mock concrete
///   under `SampleScalarWrongLength`; the subject's randomness source and
///   the independent source, each the randomness family's mock concrete at
///   the default selection.
#[test]
fn non_convertibility_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let uniform = self
                .independent
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
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(expected)
                ))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's default selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            let independent_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        independent: &*independent_source.adapter,
                        count: self.count,
                        identity: self.identity,
                        other_identity: self.other_identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
        other_identity,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::SampleScalarWrongLength,
            )),
            ..Default::default()
        }),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's configured selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // non_convertibility_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Record, `deps.kem.derive_identity` fails →
///   `Err(NonConvertibilityErrorReturn::DeriveIdentity(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Entitlement`, the identity
///   `b"pkg@1.0.0#1"`, the other identity `b"pkg@2.0.0#7"`, the count by
///   `build_vector_count` at its default, and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected refusal the `Err` of `hash_to_scalar`
///   over `build_hash_to_scalar_params` and `build_hash_to_scalar_payload`
///   at their defaults on the adapter built under the same mode.
/// Act:     `non_convertibility`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(NonConvertibilityErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(error)))` whole by
///   `assert_eq!`, `error` the expected refusal; the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's and kem's mock concretes in their default
///   modes through their factories; the hash-to-scalar family's mock
///   concrete under `SamplingRefused`; the subject's randomness source, the
///   randomness family's mock concrete at the default selection.
#[test]
fn non_convertibility_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let other_identity: &[u8] = b"pkg@2.0.0#7";
    let hash_concrete =
        HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused);

    struct KemConsumer<'a, P: IPairingArithmetic> {
        subject: &'a dyn IRandomSourceAdapter,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        count: VectorCount,
        identity: &'a [u8],
        other_identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_, P> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Arrange
            let expected = self
                .hashing
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the hash-to-scalar adapter refuses under its named mode");
            // Act
            let returned = non_convertibility::<K>(
                &build_non_convertibility_deps(&payload.adapter, self.subject, Default::default()),
                build_non_convertibility_params(NonConvertibilityParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_non_convertibility_payload(
                    self.identity,
                    self.other_identity,
                    Default::default(),
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(NonConvertibilityErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::HashToScalar(expected)
                ))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
        other_identity: &'a [u8],
        hash_concrete: HashToScalarConcrete,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(self.hash_concrete),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory's configured selection constructs");
            let subject_source = create_random_source(
                &build_create_random_source_deps(Default::default()),
                build_create_random_source_params(Default::default()),
                build_create_random_source_payload(Default::default()),
            )
            .expect("the random-source factory's default selection constructs");
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        hashing: &*hashing.adapter,
                        count: self.count,
                        identity: self.identity,
                        other_identity: self.other_identity,
                    },
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        count,
        scope,
        identity,
        other_identity,
        hash_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let outcome = created.expect("the pairing factory's default selection constructs");

    // Assert
    assert!(outcome.output.is_ok());
    // non_convertibility_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole
}
