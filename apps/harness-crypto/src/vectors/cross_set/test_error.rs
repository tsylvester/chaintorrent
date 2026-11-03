#![allow(clippy::expect_used)]

use domain::build_secret;
use encoding::IEncoderAdapter;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload,
    build_hash_to_scalar_params, build_hash_to_scalar_payload, create_hash_to_scalar,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kdf::{
    CreateKeyDerivationParamsOverrides, DeriveKeyErrorReturn, IKeyDerivationAdapter, KdfConcrete,
    MockIKeyDerivationAdapterDeriveKeyErrorReturn, MockIKeyDerivationAdapterFailureMode,
    build_create_key_derivation_params,
};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, ICredentialKemAdapter, IdentityScope, IssuePayloadOverrides,
    KemIdentity, SetupErrorReturn, SetupParamsOverrides, SetupPayloadOverrides, SetupScope,
    build_create_kem_params, build_derive_identity_params, build_derive_identity_payload,
    build_encapsulate_params, build_encapsulate_payload, build_issue_params, build_issue_payload,
    build_setup_params, build_setup_payload,
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
use workflows::{DerivationContext, PIECE_GROUP_KEY_LENGTH, WrapPieceGroupKeyErrorReturn};

use super::cross_set;
use super::interface::{CrossSetErrorReturn, ICrossSetCaseConsumer};
use super::mock::{
    CrossSetCaseParamsOverrides, CrossSetCasePayloadOverrides, CrossSetParamsOverrides,
    CrossSetPayloadOverrides, build_cross_set_case_deps, build_cross_set_case_params,
    build_cross_set_case_payload, build_cross_set_deps, build_cross_set_params,
    build_cross_set_payload, run_cross_set_case,
};
use crate::delivery::provides::{AdmittedKemIdentityOverrides, build_admitted_kem_identity};
use crate::{VectorCountConstructorParamsOverrides, build_vector_count};

/// Contract: Draw → `deps.random.fill_bytes` fails at the piece-group
///   key's draw, the record's first, and `Err(CrossSetErrorReturn::FillBytes(
///   error))` returns, `error` unchanged.
/// Arrange: the subject's source and the independent source from
///   `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`; the expected `error` the refusal of the
///   independent source's `fill_bytes` for `PIECE_GROUP_KEY_LENGTH`.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the result equals `Err(CrossSetErrorReturn::FillBytes(error))`
///   whole; the outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its
///   params builder, except the randomness family's, which this block
///   names under `FillBytesRefused`.
#[test]
fn cross_set_returns_the_draw_refusal_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: crate::VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let expected = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(PIECE_GROUP_KEY_LENGTH),
                    }),
                )
                .err()
                .expect("the independent source refuses");
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err().expect("the subject's call fails"),
                CrossSetErrorReturn::FillBytes(expected)
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(CrossSetCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_returns_the_draw_refusal_whole
}

/// Contract: Member → `deps.kem.setup` fails on the master sampling and
///   `Err(CrossSetErrorReturn::Setup(error))` returns, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::
///   SampleScalarWrongLength)`; the expected `error` the `Err` of
///   `sample_from_uniform_bytes` over the independent source's second
///   draw, the first member's master uniform.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the result equals `Err(CrossSetErrorReturn::Setup(
///   SetupErrorReturn::MasterScalarSampling(error)))` whole; the
///   outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its
///   params builder, except the pairing family's, which this block names
///   under `SampleScalarWrongLength`.
#[test]
fn cross_set_returns_the_sampling_refusal_of_setup_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: crate::VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let draw = |length: usize| {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            draw(PIECE_GROUP_KEY_LENGTH);
            let master_uniform = draw(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH);
            let expected = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: build_secret(
                        master_uniform.expose().clone(),
                        Default::default(),
                    ),
                },
            )
            .err()
            .expect("the pairing's mock scalar sampling refuses");
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err().expect("the subject's call fails"),
                CrossSetErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(expected))
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(CrossSetCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_returns_the_sampling_refusal_of_setup_whole
}

/// Contract: Member → `deps.kem.derive_identity` fails on the hash-to-
///   scalar sampling and `Err(CrossSetErrorReturn::DeriveIdentity(error))`
///   returns, `error` unchanged.
/// Arrange: `build_create_hash_to_scalar_params` overriding `concrete`
///   with `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode
///   ::SamplingRefused)`; the expected refusal the `Err` of
///   `hash_to_scalar` at the defaults of its params and payload builders
///   on an adapter built under the same mode.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the result equals `Err(CrossSetErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(error)))` whole; the
///   outermost `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its
///   params builder, except the hash-to-scalar family's, which this block
///   names under `SamplingRefused`.
#[test]
fn cross_set_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });
    let hash_to_scalar_selection = CreateHashToScalarParamsOverrides {
        concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        ..Default::default()
    };

    struct CaseConsumer {
        count: crate::VectorCount,
        hash_to_scalar_selection: CreateHashToScalarParamsOverrides,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let expected_adapter = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hash_to_scalar_selection.concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the same-mode adapter is built");
            let expected = expected_adapter
                .adapter
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the same-mode adapter refuses");
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err().expect("the subject's call fails"),
                CrossSetErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(
                    expected
                ))
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(
            CaseConsumer {
                count,
                hash_to_scalar_selection,
            },
            Default::default(),
        ),
        build_cross_set_case_params(CrossSetCaseParamsOverrides {
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
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_returns_the_hash_to_scalar_refusal_of_identity_derivation_whole
}

/// Contract: Member under `Asset` → `deps.kem.setup` fails on the root's
///   hash-to-scalar sampling and `Err(CrossSetErrorReturn::Setup(error))`
///   returns, `error` unchanged.
/// Arrange: `scope` `IdentityScope::Asset`, the identity bytes `[0x44; 32]`,
///   over whose hash the case consumer builds the admitted asset identity,
///   and the hash-to-scalar selection and expected refusal of the
///   derivation block.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the result equals `Err(CrossSetErrorReturn::Setup(
///   SetupErrorReturn::HashToScalar(error)))` whole; the outermost
///   `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its
///   params builder, except the hash-to-scalar family's, which this block
///   names under `SamplingRefused`.
#[test]
fn cross_set_returns_the_hash_to_scalar_refusal_of_asset_setup_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: crate::VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let expected_adapter = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the same-mode adapter is built");
            let expected = expected_adapter
                .adapter
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the same-mode adapter refuses");
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err().expect("the subject's call fails"),
                CrossSetErrorReturn::Setup(SetupErrorReturn::HashToScalar(expected))
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(CrossSetCaseParamsOverrides {
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
        build_cross_set_case_payload(CrossSetCasePayloadOverrides {
            identity: Some(vec![0x44; 32]),
            ..Default::default()
        }),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_returns_the_hash_to_scalar_refusal_of_asset_setup_whole
}

/// Contract: Member → `wrap_piece_group_key` refuses on the key
///   derivation and `Err(CrossSetErrorReturn::Wrap(error))` returns,
///   `error` unchanged.
/// Arrange: `build_create_key_derivation_params` overriding `concrete`
///   with `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`; the replay of the first member's
///   `setup`, `derive_identity`, `issue`, and `encapsulate` over the
///   independent source's second to sixth draws; the expected `length`
///   the length of the replayed encapsulated value's
///   `key_material().expose()`.
/// Act:     `cross_set`, once, in the case consumer's method.
/// Assert:  the result equals `Err(CrossSetErrorReturn::Wrap(
///   WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::
///   Mock(MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length }))))` whole; the outermost
///   `Result` `is_ok()`.
/// Collaborators: every family's mock concrete at the defaults of its
///   params builder, except the key-derivation family's, which this block
///   names under `KeyMaterialLengthUnrepresentable`; the independent
///   source, a separate adapter at the same defaults as the subject's.
#[test]
fn cross_set_returns_the_key_derivation_refusal_of_the_wrap_whole() {
    // Arrange
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) });

    struct CaseConsumer {
        count: crate::VectorCount,
    }

    impl ICrossSetCaseConsumer for CaseConsumer {
        type Output = ();

        fn consume_cross_set_case<P, H, K, E>(
            &self,
            _pairing: &P,
            _hash_to_scalar: &H,
            kem: &K,
            _encoder: &E,
            _kdf: &dyn IKeyDerivationAdapter,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            scope: IdentityScope,
            identity: &[u8],
            context: &DerivationContext,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            K: ICredentialKemAdapter<Pairing = P>,
            E: IEncoderAdapter,
        {
            // Arrange
            let draw = |length: usize| {
                independent
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the independent source fills")
                    .bytes
            };
            let asset_root: Option<Root> = match scope {
                IdentityScope::Asset => Some(build_root(RootConstructorParamsOverrides {
                    bytes: Some(
                        <[u8; 32]>::try_from(identity)
                            .expect("the case payload carries 32 bytes under the asset scope"),
                    ),
                })),
                IdentityScope::Entitlement => None,
            };
            let admitted = match scope {
                IdentityScope::Entitlement => {
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Entitlement),
                        identity: Some(KemIdentity::Entitlement {
                            canonical: identity,
                        }),
                    })
                }
                IdentityScope::Asset => {
                    let root = asset_root
                        .as_ref()
                        .expect("the root is built under the asset scope");
                    build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                        setup_scope: Some(SetupScope::Asset { root }),
                        identity: Some(KemIdentity::Asset { root }),
                    })
                }
            };
            draw(PIECE_GROUP_KEY_LENGTH);
            let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(admitted.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw(uniform)),
                        u0_uniform: Some(draw(uniform)),
                        u1_uniform: Some(draw(uniform)),
                    }),
                )
                .expect("the replay's setup succeeds");
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(admitted.identity),
                        },
                    ),
                )
                .expect("the replay's identity derivation succeeds");
            let _issued = kem
                .issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(draw(uniform)),
                        },
                    ),
                )
                .expect("the replay's issuance succeeds");
            let encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw(uniform)),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
            let length = encapsulated.encapsulated.key_material().expose().len();
            // Act
            let returned = cross_set::<K, E>(
                &build_cross_set_deps(kem, random, _kdf, _encoder, Default::default()),
                build_cross_set_params(CrossSetParamsOverrides {
                    count: Some(self.count),
                }),
                build_cross_set_payload(
                    context,
                    CrossSetPayloadOverrides {
                        identity: Some(admitted),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err().expect("the subject's call fails"),
                CrossSetErrorReturn::Wrap(
                    WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::Mock(
                        MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                            length,
                        }
                    ))
                )
            );
        }
    }

    // Act
    let returned = run_cross_set_case(
        &build_cross_set_case_deps(CaseConsumer { count }, Default::default()),
        build_cross_set_case_params(CrossSetCaseParamsOverrides {
            key_derivation: Some(build_create_key_derivation_params(
                CreateKeyDerivationParamsOverrides {
                    concrete: Some(KdfConcrete::Mock(
                        MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_cross_set_case_payload(Default::default()),
    );

    // Assert
    assert!(returned.is_ok());
    // cross_set_returns_the_key_derivation_refusal_of_the_wrap_whole
}
