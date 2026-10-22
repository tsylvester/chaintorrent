#![allow(clippy::expect_used)]

use domain::AssetIdentityHashTryNewErrorReturn;
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

use super::interface::RerandomizationErrorReturn;
use super::mock::{
    RerandomizationParamsOverrides, build_rerandomization_deps, build_rerandomization_params,
    build_rerandomization_payload,
};
use super::rerandomization;
use crate::{VectorCount, build_vector_count};

/// Contract: Identity under `Asset`,
///   `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity)` fails →
///   `Err(RerandomizationErrorReturn::AssetIdentityLength { actual })`,
///   `actual` the identity's length.
/// Arrange: the scope `IdentityScope::Asset` and the identity
///   `vec![0x44; 31]`; the count by `build_vector_count` at its default.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::AssetIdentityLength { actual: 31 })`
///   whole by `assert_eq!`; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn rerandomization_refuses_an_asset_identity_shorter_than_32_bytes() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Asset;
    let identity: Vec<u8> = vec![0x44; 31];

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::AssetIdentityLength { actual: 31 })
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        count: self.count,
                        identity: self.identity,
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
        identity: &identity,
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
    // rerandomization_refuses_an_asset_identity_shorter_than_32_bytes
}

/// Contract: Identity under `Asset`,
///   `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity)` fails →
///   `Err(RerandomizationErrorReturn::AssetIdentityLength { actual })`,
///   `actual` the identity's length.
/// Arrange: the scope `IdentityScope::Asset` and the identity
///   `vec![0x44; 33]`; the count by `build_vector_count` at its default.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::AssetIdentityLength { actual: 33 })`
///   whole by `assert_eq!`; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn rerandomization_refuses_an_asset_identity_longer_than_32_bytes() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Asset;
    let identity: Vec<u8> = vec![0x44; 33];

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::AssetIdentityLength { actual: 33 })
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        count: self.count,
                        identity: self.identity,
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
        identity: &identity,
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
    // rerandomization_refuses_an_asset_identity_longer_than_32_bytes
}

/// Contract: Identity under `Asset`, `AssetIdentityHash::try_new` refuses →
///   `Err(RerandomizationErrorReturn::AssetIdentityHash(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Asset` and the identity
///   `vec![0x00; 32]`; the count by `build_vector_count` at its default.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::AssetIdentityHash(
///   AssetIdentityHashTryNewErrorReturn::AllZero))` whole by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn rerandomization_returns_the_refusal_of_an_all_zero_asset_identity() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Asset;
    let identity: Vec<u8> = vec![0x00; 32];

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::AssetIdentityHash(
                    AssetIdentityHashTryNewErrorReturn::AllZero
                ))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
            create_kem(
                &build_create_kem_deps(
                    &payload.adapter,
                    &*hashing.adapter,
                    KemConsumer {
                        subject: &*subject_source.adapter,
                        count: self.count,
                        identity: self.identity,
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
        identity: &identity,
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
    // rerandomization_returns_the_refusal_of_an_all_zero_asset_identity
}

/// Contract: Identity, an exhaustive `match` on `params.scope` before any
///   record.
/// Arrange: the scope `IdentityScope::Asset`, the identity
///   `vec![0x44; 31]`, and the subject's source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, a refused identity and a refusing source together,
///   so the identity decision reaching after any draw reports the draw's
///   refusal instead.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::AssetIdentityLength { actual: 31 })`
///   whole by `assert_eq!`; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, and kem's mock concretes
///   in their default modes through their factories; the subject's
///   randomness source, the family's mock concrete under
///   `FillBytesRefused`.
#[test]
fn rerandomization_refuses_the_asset_identity_before_any_draw() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Asset;
    let identity: Vec<u8> = vec![0x44; 31];
    let subject_kind =
        RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused);

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
    }

    impl<P: IPairingArithmetic> IKemConsumer<P> for KemConsumer<'_> {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::AssetIdentityLength { actual: 31 })
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
        identity: &identity,
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
    // rerandomization_refuses_the_asset_identity_before_any_draw
}

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(RerandomizationErrorReturn::FillBytes(error))`, `error` unchanged.
/// Arrange: the scope `IdentityScope::Entitlement`, the identity
///   `b"pkg@1.0.0#1"`, and the count by `build_vector_count` at its default;
///   the subject's source and the independent source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; the expected `error` the refusal of the independent
///   source's `fill_bytes` under the declared uniform length.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::FillBytes(error))` whole by
///   `assert_eq!`, `error` the expected refusal; the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, and kem's mock concretes
///   in their default modes through their factories; the subject's
///   randomness source and the independent source, each the family's mock
///   concrete under `FillBytesRefused`.
#[test]
fn rerandomization_returns_the_draw_refusal_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let source_kind = RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused);

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::FillBytes(expected))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
    // rerandomization_returns_the_draw_refusal_whole
}

/// Contract: Record, `deps.kem.setup` fails →
///   `Err(RerandomizationErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the scope `IdentityScope::Entitlement`, the
///   identity `b"pkg@1.0.0#1"`, and the count by `build_vector_count` at its
///   default; the expected `error` the `Err` of
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes` over
///   the independent source's first draw at the declared uniform length.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(error)))` whole by `assert_eq!`, `error` the
///   expected refusal; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the hash-to-scalar's and kem's mock concretes in their
///   default modes through their factories; the pairing's mock concrete
///   under `SampleScalarWrongLength`; the subject's randomness source and
///   the independent source, each the randomness family's mock concrete at
///   the default selection.
#[test]
fn rerandomization_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";

    struct KemConsumer<'a> {
        subject: &'a dyn IRandomSourceAdapter,
        independent: &'a dyn IRandomSourceAdapter,
        count: VectorCount,
        identity: &'a [u8],
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::Setup(
                    SetupErrorReturn::MasterScalarSampling(expected)
                ))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
    // rerandomization_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Record, `deps.kem.derive_identity` fails →
///   `Err(RerandomizationErrorReturn::DeriveIdentity(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Entitlement`, the identity
///   `b"pkg@1.0.0#1"`, the count by `build_vector_count` at its default, and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected refusal the `Err` of `hash_to_scalar`
///   over `build_hash_to_scalar_params` and `build_hash_to_scalar_payload`
///   at their defaults on the adapter built under the same mode.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(error)))` whole by
///   `assert_eq!`, `error` the expected refusal; the outermost `Result`
///   `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's and kem's mock concretes in their default
///   modes through their factories; the hash-to-scalar family's mock
///   concrete under `SamplingRefused`; the subject's randomness source, the
///   randomness family's mock concrete at the default selection.
#[test]
fn rerandomization_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
    let hash_concrete =
        HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused);

    struct KemConsumer<'a, P: IPairingArithmetic> {
        subject: &'a dyn IRandomSourceAdapter,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        count: VectorCount,
        identity: &'a [u8],
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::HashToScalar(expected)
                ))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
    // rerandomization_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole
}

/// Contract: Record under `Asset`, `deps.kem.setup` fails on the identity
///   hash → `Err(RerandomizationErrorReturn::Setup(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Asset`, the identity
///   `vec![0x44; 32]`, the count by `build_vector_count` at its default, and
///   the hash-to-scalar selection and expected refusal of the derivation
///   block.
/// Act:     `rerandomization`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(RerandomizationErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` whole by `assert_eq!`, `error` the expected
///   refusal; the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's and kem's mock concretes in their default
///   modes through their factories; the hash-to-scalar family's mock
///   concrete under `SamplingRefused`; the subject's randomness source, the
///   randomness family's mock concrete at the default selection.
#[test]
fn rerandomization_returns_the_hash_to_scalar_refusal_of_asset_setup_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Asset;
    let identity: Vec<u8> = vec![0x44; 32];
    let hash_concrete =
        HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused);

    struct KemConsumer<'a, P: IPairingArithmetic> {
        subject: &'a dyn IRandomSourceAdapter,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        count: VectorCount,
        identity: &'a [u8],
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
            let returned = rerandomization::<K>(
                &build_rerandomization_deps(&payload.adapter, self.subject, Default::default()),
                build_rerandomization_params(RerandomizationParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_rerandomization_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(RerandomizationErrorReturn::Setup(
                    SetupErrorReturn::HashToScalar(expected)
                ))
            );
        }
    }

    struct PairingConsumer<'a> {
        count: VectorCount,
        scope: IdentityScope,
        identity: &'a [u8],
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
        identity: &identity,
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
    // rerandomization_returns_the_hash_to_scalar_refusal_of_asset_setup_whole
}
