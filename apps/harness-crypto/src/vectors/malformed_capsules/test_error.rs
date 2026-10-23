#![allow(clippy::expect_used)]

use domain::AssetIdentityHashTryNewErrorReturn;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides, CreateKemReturn,
    ICredentialKemAdapter, IKemConsumer, IdentityScope, SetupErrorReturn, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, IPairingArithmetic,
    IPairingConsumer, IPairingReference, ISampleUniformScalar, MockIPairingAdapterFailureMode,
    PairingConcrete, SampleUniformScalarErrorReturn, build_create_pairing_deps,
    build_create_pairing_params, build_create_pairing_payload, create_pairing,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_deps,
    build_create_random_source_params, build_create_random_source_payload, create_random_source,
};

use super::interface::MalformedCapsulesErrorReturn;
use super::malformed_capsules;
use super::mock::{
    MalformedCapsulesParamsOverrides, build_malformed_capsules_deps,
    build_malformed_capsules_params, build_malformed_capsules_payload,
};
use crate::{VectorCount, build_vector_count};

/// Contract: Scope under `Asset`,
///   `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity)` fails
///   → `Err(MalformedCapsulesErrorReturn::AssetIdentityLength { actual })`,
///   `actual` the identity's length.
/// Arrange: the scope `IdentityScope::Asset` and the identity
///   `vec![0x44; 31]`; the count by `build_vector_count` at its default.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::AssetIdentityLength { actual: 31 })`
///   whole by `assert_eq!`; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn malformed_capsules_refuses_an_asset_identity_shorter_than_32_bytes() {
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
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::AssetIdentityLength { actual: 31 })
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
    // malformed_capsules_refuses_an_asset_identity_shorter_than_32_bytes
}

/// Contract: Scope under `Asset`,
///   `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity)` fails
///   → `Err(MalformedCapsulesErrorReturn::AssetIdentityLength { actual })`,
///   `actual` the identity's length.
/// Arrange: the scope `IdentityScope::Asset` and the identity
///   `vec![0x44; 33]`; the count by `build_vector_count` at its default.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::AssetIdentityLength { actual: 33 })`
///   whole by `assert_eq!`; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn malformed_capsules_refuses_an_asset_identity_longer_than_32_bytes() {
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
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::AssetIdentityLength { actual: 33 })
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
    // malformed_capsules_refuses_an_asset_identity_longer_than_32_bytes
}

/// Contract: Scope under `Asset`, `AssetIdentityHash::try_new` refuses →
///   `Err(MalformedCapsulesErrorReturn::AssetIdentityHash(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Asset` and the identity
///   `vec![0x00; 32]`; the count by `build_vector_count` at its default.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::AssetIdentityHash(
///   AssetIdentityHashTryNewErrorReturn::AllZero))` whole by `assert_eq!`;
///   the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, kem's, and randomness
///   family's mock concretes in their default modes through their factories;
///   the subject's randomness source, the family's mock concrete at the
///   default selection.
#[test]
fn malformed_capsules_returns_the_refusal_of_an_all_zero_asset_identity() {
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
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::AssetIdentityHash(
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
    // malformed_capsules_returns_the_refusal_of_an_all_zero_asset_identity
}

/// Contract: Scope under `Asset`, a refused identity, decided before any
///   draw → `Err(MalformedCapsulesErrorReturn::AssetIdentityLength
///   { actual })`, with no `FillBytes` refusal.
/// Arrange: the scope `IdentityScope::Asset`, the identity
///   `vec![0x44; 31]`, and the subject's source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, a refused identity and a refusing source together,
///   so the identity decision reaching after any draw reports the draw's
///   refusal instead; the count by `build_vector_count` at its default.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::AssetIdentityLength { actual: 31 })`
///   whole by `assert_eq!`; the outermost `Result` `is_ok()`, proving the
///   consumers ran.
/// Collaborators: the pairing's, hash-to-scalar's, and kem's mock concretes
///   in their default modes through their factories; the subject's
///   randomness source, the family's mock concrete under
///   `FillBytesRefused`.
#[test]
fn malformed_capsules_refuses_the_asset_identity_before_any_draw() {
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
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::AssetIdentityLength { actual: 31 })
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
    // malformed_capsules_refuses_the_asset_identity_before_any_draw
}

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(MalformedCapsulesErrorReturn::FillBytes(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Entitlement`, the identity
///   `b"pkg@1.0.0#1"`, and the count by `build_vector_count` at its default;
///   the subject's source from `build_create_random_source_params`
///   overriding `kind` with `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`; the expected
///   `error` the literal `FillBytesErrorReturn::MockIRandomSourceAdapter`.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))` whole by
///   `assert_eq!`; the outermost `Result` `is_ok()`, proving the consumers
///   ran.
/// Collaborators: the pairing's, hash-to-scalar's, and kem's mock concretes
///   in their default modes through their factories; the subject's
///   randomness source, the family's mock concrete under
///   `FillBytesRefused`.
#[test]
fn malformed_capsules_returns_the_draw_refusal_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";
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
            // Arrange
            let expected = FillBytesErrorReturn::MockIRandomSourceAdapter;
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MalformedCapsulesErrorReturn::FillBytes(expected))
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
        identity,
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
    // malformed_capsules_returns_the_draw_refusal_whole
}

/// Contract: Record, `deps.kem.setup` fails →
///   `Err(MalformedCapsulesErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the scope `IdentityScope::Entitlement`, the
///   identity `b"pkg@1.0.0#1"`, and the count by `build_vector_count` at its
///   default; the expected `error` the literal
///   `SampleUniformScalarErrorReturn::WrongLength { expected: 16, actual:
///   16 }`.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   MasterScalarSampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected: 16, actual: 16 })))` whole by `assert_eq!`, the master
///   sampling the first refused; the outermost `Result` `is_ok()`, proving
///   the consumers ran.
/// Collaborators: the hash-to-scalar's and kem's mock concretes in their
///   default modes through their factories; the pairing's mock concrete
///   under `SampleScalarWrongLength`; the subject's randomness source, the
///   randomness family's mock concrete at the default selection.
#[test]
fn malformed_capsules_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Entitlement;
    let identity: &[u8] = b"pkg@1.0.0#1";

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
            // Arrange
            let expected = SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 16,
            };
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
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
    // malformed_capsules_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Record under `Asset`, `deps.kem.setup` fails on the identity
///   hash → `Err(MalformedCapsulesErrorReturn::Setup(error))`, `error`
///   unchanged.
/// Arrange: the scope `IdentityScope::Asset`, the identity
///   `vec![0x44; 32]`, the count by `build_vector_count` at its default, and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `malformed_capsules`, once, in the KEM consumer's method.
/// Assert:  the result's `Err` equals
///   `Some(MalformedCapsulesErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(error)))` whole by `assert_eq!`, `error` the expected
///   literal; the outermost `Result` `is_ok()`, proving the consumers ran.
/// Collaborators: the pairing's and kem's mock concretes in their default
///   modes through their factories; the hash-to-scalar family's mock
///   concrete under `SamplingRefused`; the subject's randomness source, the
///   randomness family's mock concrete at the default selection.
#[test]
fn malformed_capsules_returns_the_hash_to_scalar_refusal_of_asset_setup_whole() {
    // Arrange
    let count = build_vector_count(Default::default());
    let scope = IdentityScope::Asset;
    let identity: Vec<u8> = vec![0x44; 32];
    let hash_concrete =
        HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused);

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
            // Arrange
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            // Act
            let returned = malformed_capsules::<K>(
                &build_malformed_capsules_deps(&payload.adapter, self.subject, Default::default()),
                build_malformed_capsules_params(MalformedCapsulesParamsOverrides {
                    count: Some(self.count),
                    scope: Some(payload.scope()),
                }),
                build_malformed_capsules_payload(self.identity, Default::default()),
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
    // malformed_capsules_returns_the_hash_to_scalar_refusal_of_asset_setup_whole
}
