#![allow(clippy::expect_used, clippy::panic)]

use super::interface::Bb1DepthOneKem;
use super::mock::build_bb1_depth_one_kem;
use crate::factory::provides::{
    CapsuleComponents, CapsuleFromComponentsPayloadOverrides,
    CredentialFromComponentsPayloadOverrides, DeriveIdentityErrorReturn,
    DeriveIdentityPayloadOverrides, EncapsulatePayloadOverrides, ICredentialKemAdapter,
    IssuePayloadOverrides, KemIdentity, MasterScalarFromComponentsPayloadOverrides,
    ParameterSetFromComponentsPayloadOverrides, RerandomizePayloadOverrides, SetupParamsOverrides,
    SetupPayloadOverrides, SetupScope, build_capsule_components_params,
    build_capsule_components_payload, build_capsule_from_components_params,
    build_capsule_from_components_payload, build_credential_components_params,
    build_credential_components_payload, build_credential_from_components_params,
    build_credential_from_components_payload, build_decapsulate_params, build_decapsulate_payload,
    build_derive_identity_params, build_derive_identity_payload, build_encapsulate_params,
    build_encapsulate_payload, build_is_valid_params, build_is_valid_payload,
    build_is_well_formed_params, build_is_well_formed_payload, build_issue_params,
    build_issue_payload, build_master_scalar_components_params,
    build_master_scalar_components_payload, build_master_scalar_from_components_params,
    build_master_scalar_from_components_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_parameter_set_from_components_params,
    build_parameter_set_from_components_payload, build_rerandomize_params,
    build_rerandomize_payload, build_setup_params, build_setup_payload,
};
use domain::{
    AssetIdentityHashConstructorParamsOverrides, Secret, build_asset_identity_hash, build_secret,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, ISampleUniformScalar,
    PAIRING_CONCRETES, build_create_pairing_deps, build_create_pairing_params, create_pairing,
};

fn scalar_bytes<S: ISampleUniformScalar>(value: u8) -> Secret<Vec<u8>> {
    let mut bytes = vec![0u8; S::UNIFORM_BYTES_LENGTH];
    bytes[S::UNIFORM_BYTES_LENGTH - 1] = value;
    build_secret(bytes, Default::default())
}

/// Contract: entry `every_credential_issued_under_a_setup_parameter_set_is_valid_for_its_identity`;
///   `setup`, `derive_identity`, and `issue` with the master scalar `setup`
///   returned, under each scope, satisfy the validity relation, so `is_valid`
///   returns `Ok(IsValidSuccessReturn { is_valid: true })`.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` from `create_hash_to_scalar::<P::Scalar>` under the case's
///   concrete and identifier and `kem` from `build_bb1_depth_one_kem`, then per
///   scope — `SetupScope::Entitlement` with `b"entitlement-one"` and
///   `SetupScope::Asset` with the hash `build_asset_identity_hash` returns for
///   `[0xa1; 32]` — runs `setup` with a distinct `scalar_bytes(n)` for the
///   master, `u0`, and `u1` samplers, `derive_identity` over the scope's
///   identity, and `issue` with the master scalar `setup` returned and a
///   further distinct `scalar_bytes(n)`, so a reused or reordered scalar
///   breaks the relation.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's call under test is `is_valid`
///   over the parameter set, the identity element, and the issued credential.
/// Assert:  in the consumer, `assert!(success.is_valid)`; outside the consumer,
///   `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `derive_identity`, `issue`, and `is_valid`.
/// Mocked:  none.
#[test]
fn every_credential_issued_under_a_setup_parameter_set_is_valid_for_its_identity() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                for (scope, identity) in [
                    (
                        SetupScope::Entitlement,
                        KemIdentity::Entitlement {
                            canonical: b"entitlement-one",
                        },
                    ),
                    (
                        SetupScope::Asset {
                            identity_hash: &hash,
                        },
                        KemIdentity::Asset {
                            identity_hash: &hash,
                        },
                    ),
                ] {
                    let setup = kem.setup(
                        build_setup_params(SetupParamsOverrides { scope: Some(scope) }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                        }),
                    );
                    let Ok(setup) = setup else {
                        panic!("setup admits distinct nonzero uniform bytes under each scope")
                    };
                    let derived = kem.derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(identity),
                            },
                        ),
                    );
                    let Ok(derived) = derived else {
                        panic!("the identity the parameter set's scope selects derives")
                    };
                    let issued = kem.issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(4)),
                            },
                        ),
                    );
                    let Ok(issued) = issued else {
                        panic!("issuance admits distinct nonzero uniform bytes")
                    };

                    // Act
                    let result = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &issued.credential,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(result.expect("the validity check never refuses").is_valid);
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `a_rerandomized_credential_is_valid_for_its_identity`;
///   `rerandomize` over an issued credential, with no master scalar, yields a
///   credential that still satisfies the validity relation for its identity,
///   and the credential it was made from stays valid.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the block above, then per scope runs `setup`,
///   `derive_identity`, and `issue` with distinct `scalar_bytes(n)` per
///   sampler, and `rerandomize` over the issued credential with a
///   `scalar_bytes(n)` distinct from the issuance sampler's.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's call under test is `is_valid` on
///   the rerandomized credential, then on the credential it was made from.
/// Assert:  in the consumer, `assert!(success.is_valid)` for the rerandomized
///   credential and for the original credential by the same call; outside the
///   consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `derive_identity`, `issue`, `rerandomize`, and `is_valid`.
/// Mocked:  none.
#[test]
fn a_rerandomized_credential_is_valid_for_its_identity() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                for (scope, identity) in [
                    (
                        SetupScope::Entitlement,
                        KemIdentity::Entitlement {
                            canonical: b"entitlement-one",
                        },
                    ),
                    (
                        SetupScope::Asset {
                            identity_hash: &hash,
                        },
                        KemIdentity::Asset {
                            identity_hash: &hash,
                        },
                    ),
                ] {
                    let setup = kem.setup(
                        build_setup_params(SetupParamsOverrides { scope: Some(scope) }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                        }),
                    );
                    let Ok(setup) = setup else {
                        panic!("setup admits distinct nonzero uniform bytes under each scope")
                    };
                    let derived = kem.derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(identity),
                            },
                        ),
                    );
                    let Ok(derived) = derived else {
                        panic!("the identity the parameter set's scope selects derives")
                    };
                    let issued = kem.issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(4)),
                            },
                        ),
                    );
                    let Ok(issued) = issued else {
                        panic!("issuance admits distinct nonzero uniform bytes")
                    };
                    let rerandomized = kem.rerandomize(
                        build_rerandomize_params(Default::default()),
                        build_rerandomize_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &issued.credential,
                            RerandomizePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(5)),
                            },
                        ),
                    );
                    let Ok(rerandomized) = rerandomized else {
                        panic!("rerandomization admits distinct nonzero uniform bytes")
                    };

                    // Act
                    let result = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &rerandomized.credential,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(result.expect("the validity check never refuses").is_valid);

                    // Act
                    let result = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &issued.credential,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(result.expect("the validity check never refuses").is_valid);
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_valid_credential_decapsulates_a_well_formed_capsule_to_the_encapsulated_bytes`;
///   `encapsulate` yields a capsule `is_well_formed` accepts, and `decapsulate`
///   of that capsule with the issued credential and with the rerandomized
///   credential each recover the bytes `encapsulate` returned; under the asset
///   scope the payload's identity element is the one `derive_identity` returns
///   for the other asset hash, which the asset branch does not read.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, then per scope runs `setup`,
///   `derive_identity`, `issue`, `rerandomize`, and `encapsulate` with distinct
///   `scalar_bytes(n)` per sampler; under the asset scope a second `setup`
///   over `SetupScope::Asset` with the hash for `[0xb2; 32]` and a
///   `derive_identity` over it supply the identity element the `decapsulate`
///   payload carries, since the asset branch reads no identity element.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are
///   `is_well_formed` on the capsule and `decapsulate` with each credential.
/// Assert:  in the consumer, `assert!(success.is_well_formed)`, and
///   `assert_eq!` of each `success.encapsulated.key_material().expose()`
///   against the bytes `encapsulate`'s `encapsulated.key_material().expose()`
///   returned; outside the consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `derive_identity`, `issue`, `rerandomize`, `encapsulate`,
///   `is_well_formed`, and `decapsulate`.
/// Mocked:  none.
#[test]
fn every_valid_credential_decapsulates_a_well_formed_capsule_to_the_encapsulated_bytes() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                let other_hash =
                    build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                        bytes: Some([0xb2; 32]),
                    });
                for (scope, identity) in [
                    (
                        SetupScope::Entitlement,
                        KemIdentity::Entitlement {
                            canonical: b"entitlement-one",
                        },
                    ),
                    (
                        SetupScope::Asset {
                            identity_hash: &hash,
                        },
                        KemIdentity::Asset {
                            identity_hash: &hash,
                        },
                    ),
                ] {
                    let scope_is_asset = matches!(scope, SetupScope::Asset { .. });
                    let setup = kem.setup(
                        build_setup_params(SetupParamsOverrides { scope: Some(scope) }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                        }),
                    );
                    let Ok(setup) = setup else {
                        panic!("setup admits distinct nonzero uniform bytes under each scope")
                    };
                    let derived = kem.derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(identity),
                            },
                        ),
                    );
                    let Ok(derived) = derived else {
                        panic!("the identity the parameter set's scope selects derives")
                    };
                    let issued = kem.issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(4)),
                            },
                        ),
                    );
                    let Ok(issued) = issued else {
                        panic!("issuance admits distinct nonzero uniform bytes")
                    };
                    let rerandomized = kem.rerandomize(
                        build_rerandomize_params(Default::default()),
                        build_rerandomize_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &issued.credential,
                            RerandomizePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(5)),
                            },
                        ),
                    );
                    let Ok(rerandomized) = rerandomized else {
                        panic!("rerandomization admits distinct nonzero uniform bytes")
                    };
                    let encapsulated = kem.encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            EncapsulatePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(6)),
                            },
                        ),
                    );
                    let Ok(encapsulated) = encapsulated else {
                        panic!("encapsulation admits distinct nonzero uniform bytes")
                    };
                    let other_identity_element = if scope_is_asset {
                        let other_setup = kem.setup(
                            build_setup_params(SetupParamsOverrides {
                                scope: Some(SetupScope::Asset {
                                    identity_hash: &other_hash,
                                }),
                            }),
                            build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                                master_uniform: Some(scalar_bytes::<P::Scalar>(7)),
                                u0_uniform: Some(scalar_bytes::<P::Scalar>(8)),
                                u1_uniform: Some(scalar_bytes::<P::Scalar>(9)),
                            }),
                        );
                        let Ok(other_setup) = other_setup else {
                            panic!("the second asset setup admits its uniform bytes")
                        };
                        let other_derived = kem.derive_identity(
                            build_derive_identity_params(Default::default()),
                            build_derive_identity_payload(
                                &other_setup.parameter_set,
                                DeriveIdentityPayloadOverrides {
                                    identity: Some(KemIdentity::Asset {
                                        identity_hash: &other_hash,
                                    }),
                                },
                            ),
                        );
                        let Ok(other_derived) = other_derived else {
                            panic!("the other asset hash derives under the set it fixes")
                        };
                        Some(other_derived.identity_element)
                    } else {
                        None
                    };
                    let decapsulation_identity_element = other_identity_element
                        .as_ref()
                        .unwrap_or(&derived.identity_element);

                    // Act
                    let result = kem.is_well_formed(
                        build_is_well_formed_params(Default::default()),
                        build_is_well_formed_payload(
                            &setup.parameter_set,
                            &encapsulated.capsule,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(
                        result
                            .expect("the well-formedness check never refuses")
                            .is_well_formed
                    );

                    // Act
                    let result = kem.decapsulate(
                        build_decapsulate_params(Default::default()),
                        build_decapsulate_payload(
                            decapsulation_identity_element,
                            &issued.credential,
                            &encapsulated.capsule,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result
                            .expect("decapsulation never refuses")
                            .encapsulated
                            .key_material()
                            .expose(),
                        encapsulated.encapsulated.key_material().expose()
                    );

                    // Act
                    let result = kem.decapsulate(
                        build_decapsulate_params(Default::default()),
                        build_decapsulate_payload(
                            decapsulation_identity_element,
                            &rerandomized.credential,
                            &encapsulated.capsule,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert_eq!(
                        result
                            .expect("decapsulation never refuses")
                            .encapsulated
                            .key_material()
                            .expose(),
                        encapsulated.encapsulated.key_material().expose()
                    );
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `credentials_of_different_entitlements_under_one_parameter_set_recover_the_same_bytes`;
///   under the entitlement scope a credential `issue`d for each of two
///   entitlement identities decapsulates one capsule to the bytes `encapsulate`
///   returned, `I` differing between the credentials.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, runs one `setup` under
///   `SetupScope::Entitlement`, `derive_identity` over `b"entitlement-one"`
///   and over `b"entitlement-two"`, `issue` for each identity element with a
///   distinct `scalar_bytes(n)`, and one `encapsulate`.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are
///   `decapsulate` of the one capsule with each credential and its identity
///   element.
/// Assert:  in the consumer, `assert_eq!` of each decapsulation's
///   `key_material().expose()` against the bytes `encapsulate` returned and
///   against each other; outside the consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `derive_identity`, `issue`, `encapsulate`, and `decapsulate`.
/// Mocked:  none.
#[test]
fn credentials_of_different_entitlements_under_one_parameter_set_recover_the_same_bytes() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let setup = kem.setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                        u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                        u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                    }),
                );
                let Ok(setup) = setup else {
                    panic!("setup admits distinct nonzero uniform bytes")
                };
                let first_derived = kem.derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Entitlement {
                                canonical: b"entitlement-one",
                            }),
                        },
                    ),
                );
                let Ok(first_derived) = first_derived else {
                    panic!("the first entitlement identity derives")
                };
                let second_derived = kem.derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Entitlement {
                                canonical: b"entitlement-two",
                            }),
                        },
                    ),
                );
                let Ok(second_derived) = second_derived else {
                    panic!("the second entitlement identity derives")
                };
                let first_issued = kem.issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &first_derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(scalar_bytes::<P::Scalar>(4)),
                        },
                    ),
                );
                let Ok(first_issued) = first_issued else {
                    panic!("the first issuance admits distinct nonzero uniform bytes")
                };
                let second_issued = kem.issue(
                    build_issue_params(Default::default()),
                    build_issue_payload::<P::Scalar, _, _, _>(
                        &setup.parameter_set,
                        &setup.master_scalar,
                        &second_derived.identity_element,
                        IssuePayloadOverrides {
                            uniform: Some(scalar_bytes::<P::Scalar>(5)),
                        },
                    ),
                );
                let Ok(second_issued) = second_issued else {
                    panic!("the second issuance admits distinct nonzero uniform bytes")
                };
                let encapsulated = kem.encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(scalar_bytes::<P::Scalar>(6)),
                        },
                    ),
                );
                let Ok(encapsulated) = encapsulated else {
                    panic!("encapsulation admits distinct nonzero uniform bytes")
                };

                // Act
                let first = kem.decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &first_derived.identity_element,
                        &first_issued.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                );

                // Assert
                let first_bytes = first
                    .expect("decapsulation never refuses")
                    .encapsulated
                    .key_material()
                    .expose()
                    .clone();
                assert_eq!(
                    &first_bytes,
                    encapsulated.encapsulated.key_material().expose()
                );

                // Act
                let second = kem.decapsulate(
                    build_decapsulate_params(Default::default()),
                    build_decapsulate_payload(
                        &second_derived.identity_element,
                        &second_issued.credential,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                );

                // Assert
                let second_bytes = second
                    .expect("decapsulation never refuses")
                    .encapsulated
                    .key_material()
                    .expose()
                    .clone();
                assert_eq!(
                    &second_bytes,
                    encapsulated.encapsulated.key_material().expose()
                );
                assert_eq!(first_bytes, second_bytes);
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `a_credential_is_invalid_for_another_identity_and_under_another_parameter_set`;
///   `is_valid` over an issued credential with the identity element of another
///   identity, and under a parameter set `setup` returned for other uniform
///   bytes, each return `Ok(IsValidSuccessReturn { is_valid: false })`.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, then per scope runs `setup`,
///   `derive_identity`, and `issue` with distinct `scalar_bytes(n)` per
///   sampler; the other identity is `b"entitlement-two"` under the entitlement
///   scope and the hash `[0xb2; 32]` under the asset scope, whose identity
///   element a second asset `setup` and `derive_identity` supply; the other
///   parameter set comes from `setup` with a different `scalar_bytes(n)` for
///   each sampler.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are `is_valid`
///   with the other identity element under the issued set, then `is_valid`
///   with the issued identity element under the other parameter set.
/// Assert:  in the consumer, `assert!(!success.is_valid)` for each call;
///   outside the consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `derive_identity`, `issue`, and `is_valid`.
/// Mocked:  none.
#[test]
fn a_credential_is_invalid_for_another_identity_and_under_another_parameter_set() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                let other_hash =
                    build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                        bytes: Some([0xb2; 32]),
                    });
                for (scope, identity) in [
                    (
                        SetupScope::Entitlement,
                        KemIdentity::Entitlement {
                            canonical: b"entitlement-one",
                        },
                    ),
                    (
                        SetupScope::Asset {
                            identity_hash: &hash,
                        },
                        KemIdentity::Asset {
                            identity_hash: &hash,
                        },
                    ),
                ] {
                    let scope_is_asset = matches!(scope, SetupScope::Asset { .. });
                    let setup = kem.setup(
                        build_setup_params(SetupParamsOverrides { scope: Some(scope) }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                        }),
                    );
                    let Ok(setup) = setup else {
                        panic!("setup admits distinct nonzero uniform bytes under each scope")
                    };
                    let derived = kem.derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(identity),
                            },
                        ),
                    );
                    let Ok(derived) = derived else {
                        panic!("the identity the parameter set's scope selects derives")
                    };
                    let issued = kem.issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(4)),
                            },
                        ),
                    );
                    let Ok(issued) = issued else {
                        panic!("issuance admits distinct nonzero uniform bytes")
                    };
                    let other_setup = kem.setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(if scope_is_asset {
                                SetupScope::Asset {
                                    identity_hash: &other_hash,
                                }
                            } else {
                                SetupScope::Entitlement
                            }),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(5)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(6)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(7)),
                        }),
                    );
                    let Ok(other_setup) = other_setup else {
                        panic!("the other setup admits distinct nonzero uniform bytes")
                    };
                    let other_identity_element = if scope_is_asset {
                        let other_derived = kem.derive_identity(
                            build_derive_identity_params(Default::default()),
                            build_derive_identity_payload(
                                &other_setup.parameter_set,
                                DeriveIdentityPayloadOverrides {
                                    identity: Some(KemIdentity::Asset {
                                        identity_hash: &other_hash,
                                    }),
                                },
                            ),
                        );
                        let Ok(other_derived) = other_derived else {
                            panic!("the other asset hash derives under the set it fixes")
                        };
                        other_derived.identity_element
                    } else {
                        let other_derived = kem.derive_identity(
                            build_derive_identity_params(Default::default()),
                            build_derive_identity_payload(
                                &setup.parameter_set,
                                DeriveIdentityPayloadOverrides {
                                    identity: Some(KemIdentity::Entitlement {
                                        canonical: b"entitlement-two",
                                    }),
                                },
                            ),
                        );
                        let Ok(other_derived) = other_derived else {
                            panic!("the other entitlement identity derives")
                        };
                        other_derived.identity_element
                    };

                    // Act
                    let result = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &other_identity_element,
                            &issued.credential,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(!result.expect("the validity check never refuses").is_valid);

                    // Act
                    let result = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &other_setup.parameter_set,
                            &derived.identity_element,
                            &issued.credential,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(!result.expect("the validity check never refuses").is_valid);
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `a_capsule_with_exchanged_identity_base_elements_is_ill_formed`;
///   under the entitlement scope, a capsule `encapsulate` returned, read by
///   `capsule_components`, its `v` and `w` exchanged, and rebuilt by
///   `capsule_from_components` fails the well-formedness checks the original
///   passes.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, runs `setup` under
///   `SetupScope::Entitlement` and `encapsulate` with distinct
///   `scalar_bytes(n)` per sampler, reads the capsule's components, and
///   rebuilds it from `CapsuleComponents::Entitlement { u, v: w, w: v }`, the
///   exchange being the only difference from the original.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are
///   `is_well_formed` on the original capsule and on the rebuilt one.
/// Assert:  in the consumer, `assert!(success.is_well_formed)` for the
///   original and `assert!(!success.is_well_formed)` for the rebuilt; outside
///   the consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `encapsulate`, `capsule_components`, `capsule_from_components`, and
///   `is_well_formed`.
/// Mocked:  none.
#[test]
fn a_capsule_with_exchanged_identity_base_elements_is_ill_formed() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let setup = kem.setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                        u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                        u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                    }),
                );
                let Ok(setup) = setup else {
                    panic!("setup admits distinct nonzero uniform bytes")
                };
                let encapsulated = kem.encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(scalar_bytes::<P::Scalar>(4)),
                        },
                    ),
                );
                let Ok(encapsulated) = encapsulated else {
                    panic!("encapsulation admits distinct nonzero uniform bytes")
                };
                let components = kem
                    .capsule_components(
                        build_capsule_components_params(Default::default()),
                        build_capsule_components_payload(&encapsulated.capsule, Default::default()),
                    )
                    .expect("extracting components never refuses")
                    .components;
                let CapsuleComponents::Entitlement { u, v, w } = components else {
                    panic!("an entitlement capsule yields the entitlement components")
                };
                let rebuilt = kem
                    .capsule_from_components(
                        build_capsule_from_components_params(Default::default()),
                        build_capsule_from_components_payload(
                            &consumed.adapter,
                            CapsuleFromComponentsPayloadOverrides {
                                components: Some(CapsuleComponents::Entitlement { u, v: w, w: v }),
                            },
                        ),
                    )
                    .expect("rebuilding components never refuses")
                    .capsule;

                // Act
                let result = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &setup.parameter_set,
                        &encapsulated.capsule,
                        Default::default(),
                    ),
                );

                // Assert
                assert!(
                    result
                        .expect("the well-formedness check never refuses")
                        .is_well_formed
                );

                // Act
                let result = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &setup.parameter_set,
                        &rebuilt,
                        Default::default(),
                    ),
                );

                // Assert
                assert!(
                    !result
                        .expect("the well-formedness check never refuses")
                        .is_well_formed
                );
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `a_capsule_is_ill_formed_under_another_parameter_set`;
///   `is_well_formed` over a capsule `encapsulate` returned, under a parameter
///   set `setup` returned for other uniform bytes, yields
///   `Ok(IsWellFormedSuccessReturn { is_well_formed: false })`, the capsule
///   being well formed under the set it was made against.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, then per scope runs `setup`
///   and `encapsulate` with distinct `scalar_bytes(n)` per sampler, and a
///   second `setup` under the same scope with a different `scalar_bytes(n)`
///   for each sampler.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are
///   `is_well_formed` under the other parameter set, then under the capsule's
///   own.
/// Assert:  in the consumer, `assert!(!success.is_well_formed)` under the
///   other set and `assert!(success.is_well_formed)` under the capsule's own;
///   outside the consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `encapsulate`, and `is_well_formed`.
/// Mocked:  none.
#[test]
fn a_capsule_is_ill_formed_under_another_parameter_set() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                for scope in [
                    SetupScope::Entitlement,
                    SetupScope::Asset {
                        identity_hash: &hash,
                    },
                ] {
                    let scope_is_asset = matches!(scope, SetupScope::Asset { .. });
                    let setup = kem.setup(
                        build_setup_params(SetupParamsOverrides { scope: Some(scope) }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                        }),
                    );
                    let Ok(setup) = setup else {
                        panic!("setup admits distinct nonzero uniform bytes under each scope")
                    };
                    let encapsulated = kem.encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            EncapsulatePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(4)),
                            },
                        ),
                    );
                    let Ok(encapsulated) = encapsulated else {
                        panic!("encapsulation admits distinct nonzero uniform bytes")
                    };
                    let other_setup = kem.setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(if scope_is_asset {
                                SetupScope::Asset {
                                    identity_hash: &hash,
                                }
                            } else {
                                SetupScope::Entitlement
                            }),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(5)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(6)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(7)),
                        }),
                    );
                    let Ok(other_setup) = other_setup else {
                        panic!("the other setup admits distinct nonzero uniform bytes")
                    };

                    // Act
                    let result = kem.is_well_formed(
                        build_is_well_formed_params(Default::default()),
                        build_is_well_formed_payload(
                            &other_setup.parameter_set,
                            &encapsulated.capsule,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(
                        !result
                            .expect("the well-formedness check never refuses")
                            .is_well_formed
                    );

                    // Act
                    let result = kem.is_well_formed(
                        build_is_well_formed_params(Default::default()),
                        build_is_well_formed_payload(
                            &setup.parameter_set,
                            &encapsulated.capsule,
                            Default::default(),
                        ),
                    );

                    // Assert
                    assert!(
                        result
                            .expect("the well-formedness check never refuses")
                            .is_well_formed
                    );
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `a_capsule_of_the_other_scope_is_ill_formed`;
///   `is_well_formed` over an asset capsule under an entitlement parameter set
///   and over an entitlement capsule under an asset parameter set each yield
///   `Ok(IsWellFormedSuccessReturn { is_well_formed: false })`, each capsule
///   being well formed under a parameter set of its own scope.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, runs one `setup` under
///   `SetupScope::Entitlement` and one under `SetupScope::Asset` with the hash
///   for `[0xa1; 32]`, each with a distinct `scalar_bytes(n)` per sampler, and
///   `encapsulate` under each parameter set.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are
///   `is_well_formed` of each capsule under its own set, then across the
///   scopes in both directions.
/// Assert:  in the consumer, `assert!(success.is_well_formed)` for each
///   capsule under its own set and `assert!(!success.is_well_formed)` for
///   each direction across; outside the consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `encapsulate`, and `is_well_formed`.
/// Mocked:  none.
#[test]
fn a_capsule_of_the_other_scope_is_ill_formed() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                let entitlement_setup = kem.setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                        u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                        u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                    }),
                );
                let Ok(entitlement_setup) = entitlement_setup else {
                    panic!("the entitlement setup admits distinct nonzero uniform bytes")
                };
                let entitlement_capsule = kem.encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &entitlement_setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(scalar_bytes::<P::Scalar>(4)),
                        },
                    ),
                );
                let Ok(entitlement_capsule) = entitlement_capsule else {
                    panic!("the entitlement encapsulation admits its uniform bytes")
                };
                let asset_setup = kem.setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Asset {
                            identity_hash: &hash,
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(scalar_bytes::<P::Scalar>(5)),
                        u0_uniform: Some(scalar_bytes::<P::Scalar>(6)),
                        u1_uniform: Some(scalar_bytes::<P::Scalar>(7)),
                    }),
                );
                let Ok(asset_setup) = asset_setup else {
                    panic!("the asset setup admits distinct nonzero uniform bytes")
                };
                let asset_capsule = kem.encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &asset_setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(scalar_bytes::<P::Scalar>(8)),
                        },
                    ),
                );
                let Ok(asset_capsule) = asset_capsule else {
                    panic!("the asset encapsulation admits its uniform bytes")
                };

                // Act
                let result = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &entitlement_setup.parameter_set,
                        &entitlement_capsule.capsule,
                        Default::default(),
                    ),
                );

                // Assert
                assert!(
                    result
                        .expect("the well-formedness check never refuses")
                        .is_well_formed
                );

                // Act
                let result = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &asset_setup.parameter_set,
                        &asset_capsule.capsule,
                        Default::default(),
                    ),
                );

                // Assert
                assert!(
                    result
                        .expect("the well-formedness check never refuses")
                        .is_well_formed
                );

                // Act
                let result = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &asset_setup.parameter_set,
                        &entitlement_capsule.capsule,
                        Default::default(),
                    ),
                );

                // Assert
                assert!(
                    !result
                        .expect("the well-formedness check never refuses")
                        .is_well_formed
                );

                // Act
                let result = kem.is_well_formed(
                    build_is_well_formed_params(Default::default()),
                    build_is_well_formed_payload(
                        &entitlement_setup.parameter_set,
                        &asset_capsule.capsule,
                        Default::default(),
                    ),
                );

                // Assert
                assert!(
                    !result
                        .expect("the well-formedness check never refuses")
                        .is_well_formed
                );
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry `derive_identity_refuses_a_trivial_identity_element`;
///   an entitlement parameter set `setup` returned for zero uniform bytes for
///   `u0` and `u1` has both as the identity element, so `F` is the identity
///   element whatever `I` is and `derive_identity` over an entitlement
///   identity yields `Err(DeriveIdentityErrorReturn::TrivialIdentityElement)`,
///   while the same call under a set from nonzero bytes returns `Ok`.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, runs `setup` under
///   `SetupScope::Entitlement` with `u0_uniform` and `u1_uniform` each
///   `Some(scalar_bytes(0))` against the default nonzero bytes the second
///   `setup` uses.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are
///   `derive_identity` over `b"entitlement-one"` under each set.
/// Assert:  in the consumer, `assert_eq!` of the whole error against
///   `DeriveIdentityErrorReturn::TrivialIdentityElement` under the zero-byte
///   set, and `result.is_ok()` under the other; outside the consumer,
///   `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`, and
///   `derive_identity`.
/// Mocked:  none.
#[test]
fn derive_identity_refuses_a_trivial_identity_element() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let trivial_setup = kem.setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                        u0_uniform: Some(scalar_bytes::<P::Scalar>(0)),
                        u1_uniform: Some(scalar_bytes::<P::Scalar>(0)),
                    }),
                );
                let Ok(trivial_setup) = trivial_setup else {
                    panic!("the entitlement scope skips the trivial-element check")
                };
                let setup = kem.setup(
                    build_setup_params(Default::default()),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                        u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                        u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                    }),
                );
                let Ok(setup) = setup else {
                    panic!("setup admits distinct nonzero uniform bytes")
                };

                // Act
                let result = kem.derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(&trivial_setup.parameter_set, Default::default()),
                );

                // Assert
                let Err(error) = result else {
                    panic!("a trivial identity element is refused")
                };
                assert_eq!(error, DeriveIdentityErrorReturn::TrivialIdentityElement);

                // Act
                let result = kem.derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(&setup.parameter_set, Default::default()),
                );

                // Assert
                assert!(result.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `derive_identity_derives_under_the_asset_scope_only_the_identity_the_set_fixes`;
///   an asset parameter set `setup` returned for an asset identity hash fixes
///   that identity's element, so `derive_identity` over the same hash yields
///   `Ok(DeriveIdentitySuccessReturn { identity_element })` and over another
///   hash yields `Err(DeriveIdentityErrorReturn::OutsideAssetScope)`.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above and runs `setup` under
///   `SetupScope::Asset` with the hash for `[0xa1; 32]`, the other hash being
///   `[0xb2; 32]`, each sampler fed a distinct `scalar_bytes(n)`.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are
///   `derive_identity` over the hash the set fixes, then over the other.
/// Assert:  in the consumer, `result.is_ok()` for the hash the set fixes and
///   `assert_eq!` of the whole error against
///   `DeriveIdentityErrorReturn::OutsideAssetScope` for the other; outside the
///   consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`, and
///   `derive_identity`.
/// Mocked:  none.
#[test]
fn derive_identity_derives_under_the_asset_scope_only_the_identity_the_set_fixes() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                let other_hash =
                    build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                        bytes: Some([0xb2; 32]),
                    });
                let setup = kem.setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Asset {
                            identity_hash: &hash,
                        }),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                        u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                        u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                    }),
                );
                let Ok(setup) = setup else {
                    panic!("the asset setup admits distinct nonzero uniform bytes")
                };

                // Act
                let result = kem.derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Asset {
                                identity_hash: &hash,
                            }),
                        },
                    ),
                );

                // Assert
                assert!(result.is_ok());

                // Act
                let result = kem.derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(KemIdentity::Asset {
                                identity_hash: &other_hash,
                            }),
                        },
                    ),
                );

                // Assert
                let Err(error) = result else {
                    panic!("an identity outside the scope the set fixes is refused")
                };
                assert_eq!(error, DeriveIdentityErrorReturn::OutsideAssetScope);
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}

/// Contract: entry
///   `every_value_rebuilt_from_its_components_behaves_as_the_value_it_was_read_from`;
///   the parameter set, credential, capsule, and master scalar each read by
///   its components method and rebuilt by its from-components method behave
///   as the originals: `issue` under the rebuilt parameter set with the
///   rebuilt master scalar returns a credential `is_valid` accepts under the
///   original set, the rebuilt credential is valid and decapsulates to the
///   encapsulated bytes, and the rebuilt capsule is well formed under the
///   original set.
/// Arrange: `Consumer` that, per `case` of `DECLARED_HASH_TO_SCALAR_SELECTIONS`,
///   binds `made` and `kem` as the blocks above, then per scope runs `setup`,
///   `derive_identity`, `issue`, and `encapsulate` with distinct
///   `scalar_bytes(n)` per sampler, reads each value's components, and
///   rebuilds each from the components its reader returned, so a swapped
///   field of any component breaks the relation.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)` per concrete
///   of `PAIRING_CONCRETES`, whose consumer's calls under test are `issue`
///   under the rebuilt values, `is_valid` on that credential and on the
///   rebuilt credential, `decapsulate` with the rebuilt credential and
///   capsule, and `is_well_formed` on the rebuilt capsule.
/// Assert:  in the consumer, `assert!(success.is_valid)` for both
///   credentials, `assert_eq!` of the decapsulation's `key_material().expose()`
///   against the bytes `encapsulate` returned, and
///   `assert!(success.is_well_formed)` for the rebuilt capsule; outside the
///   consumer, `created.is_ok()`.
/// Boundary: `create_pairing`, the arm's concrete, `create_hash_to_scalar`,
///   the selected concrete, `Bb1DepthOneKem::try_new`, `setup`,
///   `derive_identity`, `issue`, `encapsulate`, the components and
///   from-components methods, `is_valid`, `decapsulate`, and
///   `is_well_formed`.
/// Mocked:  none.
#[test]
fn every_value_rebuilt_from_its_components_behaves_as_the_value_it_was_read_from() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            for case in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                // Arrange
                let Ok(made) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(case.concrete),
                        identifier: Some(case.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared selection admits the concrete it names")
                };
                let kem: Bb1DepthOneKem<'_, P> =
                    build_bb1_depth_one_kem(&consumed.adapter, &*made.adapter, Default::default());
                let hash = build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0xa1; 32]),
                });
                for (scope, identity) in [
                    (
                        SetupScope::Entitlement,
                        KemIdentity::Entitlement {
                            canonical: b"entitlement-one",
                        },
                    ),
                    (
                        SetupScope::Asset {
                            identity_hash: &hash,
                        },
                        KemIdentity::Asset {
                            identity_hash: &hash,
                        },
                    ),
                ] {
                    let setup = kem.setup(
                        build_setup_params(SetupParamsOverrides { scope: Some(scope) }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(scalar_bytes::<P::Scalar>(1)),
                            u0_uniform: Some(scalar_bytes::<P::Scalar>(2)),
                            u1_uniform: Some(scalar_bytes::<P::Scalar>(3)),
                        }),
                    );
                    let Ok(setup) = setup else {
                        panic!("setup admits distinct nonzero uniform bytes under each scope")
                    };
                    let derived = kem.derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &setup.parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(identity),
                            },
                        ),
                    );
                    let Ok(derived) = derived else {
                        panic!("the identity the parameter set's scope selects derives")
                    };
                    let issued = kem.issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &setup.parameter_set,
                            &setup.master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(4)),
                            },
                        ),
                    );
                    let Ok(issued) = issued else {
                        panic!("issuance admits distinct nonzero uniform bytes")
                    };
                    let encapsulated = kem.encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            EncapsulatePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(5)),
                            },
                        ),
                    );
                    let Ok(encapsulated) = encapsulated else {
                        panic!("encapsulation admits distinct nonzero uniform bytes")
                    };
                    let parameter_set_components = kem
                        .parameter_set_components(
                            build_parameter_set_components_params(Default::default()),
                            build_parameter_set_components_payload(
                                &setup.parameter_set,
                                Default::default(),
                            ),
                        )
                        .expect("extracting components never refuses")
                        .components;
                    let master_scalar_components = kem
                        .master_scalar_components(
                            build_master_scalar_components_params(Default::default()),
                            build_master_scalar_components_payload(
                                &setup.master_scalar,
                                Default::default(),
                            ),
                        )
                        .expect("extracting components never refuses")
                        .components;
                    let credential_components = kem
                        .credential_components(
                            build_credential_components_params(Default::default()),
                            build_credential_components_payload(
                                &issued.credential,
                                Default::default(),
                            ),
                        )
                        .expect("extracting components never refuses")
                        .components;
                    let capsule_components = kem
                        .capsule_components(
                            build_capsule_components_params(Default::default()),
                            build_capsule_components_payload(
                                &encapsulated.capsule,
                                Default::default(),
                            ),
                        )
                        .expect("extracting components never refuses")
                        .components;
                    let rebuilt_parameter_set = kem
                        .parameter_set_from_components(
                            build_parameter_set_from_components_params(Default::default()),
                            build_parameter_set_from_components_payload(
                                &consumed.adapter,
                                ParameterSetFromComponentsPayloadOverrides {
                                    components: Some(parameter_set_components),
                                },
                            ),
                        )
                        .expect("rebuilding components never refuses")
                        .parameter_set;
                    let rebuilt_master_scalar = kem
                        .master_scalar_from_components(
                            build_master_scalar_from_components_params(Default::default()),
                            build_master_scalar_from_components_payload::<P::Scalar>(
                                MasterScalarFromComponentsPayloadOverrides {
                                    components: Some(master_scalar_components),
                                },
                            ),
                        )
                        .expect("rebuilding components never refuses")
                        .master_scalar;
                    let rebuilt_credential = kem
                        .credential_from_components(
                            build_credential_from_components_params(Default::default()),
                            build_credential_from_components_payload(
                                &consumed.adapter,
                                CredentialFromComponentsPayloadOverrides {
                                    components: Some(credential_components),
                                },
                            ),
                        )
                        .expect("rebuilding components never refuses")
                        .credential;
                    let rebuilt_capsule = kem
                        .capsule_from_components(
                            build_capsule_from_components_params(Default::default()),
                            build_capsule_from_components_payload(
                                &consumed.adapter,
                                CapsuleFromComponentsPayloadOverrides {
                                    components: Some(capsule_components),
                                },
                            ),
                        )
                        .expect("rebuilding components never refuses")
                        .capsule;

                    // Act
                    let reissued = kem.issue(
                        build_issue_params(Default::default()),
                        build_issue_payload::<P::Scalar, _, _, _>(
                            &rebuilt_parameter_set,
                            &rebuilt_master_scalar,
                            &derived.identity_element,
                            IssuePayloadOverrides {
                                uniform: Some(scalar_bytes::<P::Scalar>(6)),
                            },
                        ),
                    );

                    // Assert
                    let Ok(reissued) = reissued else {
                        panic!("issuance under the rebuilt values admits its sampler")
                    };
                    let result = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &reissued.credential,
                            Default::default(),
                        ),
                    );
                    assert!(result.expect("the validity check never refuses").is_valid);
                    let result = kem.is_valid(
                        build_is_valid_params(Default::default()),
                        build_is_valid_payload(
                            &setup.parameter_set,
                            &derived.identity_element,
                            &rebuilt_credential,
                            Default::default(),
                        ),
                    );
                    assert!(result.expect("the validity check never refuses").is_valid);
                    let result = kem.decapsulate(
                        build_decapsulate_params(Default::default()),
                        build_decapsulate_payload(
                            &derived.identity_element,
                            &rebuilt_credential,
                            &rebuilt_capsule,
                            Default::default(),
                        ),
                    );
                    assert_eq!(
                        result
                            .expect("decapsulation never refuses")
                            .encapsulated
                            .key_material()
                            .expose(),
                        encapsulated.encapsulated.key_material().expose()
                    );
                    let result = kem.is_well_formed(
                        build_is_well_formed_params(Default::default()),
                        build_is_well_formed_payload(
                            &setup.parameter_set,
                            &rebuilt_capsule,
                            Default::default(),
                        ),
                    );
                    assert!(
                        result
                            .expect("the well-formedness check never refuses")
                            .is_well_formed
                    );
                }
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let params = build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(*concrete),
            ..Default::default()
        });
        let deps = build_create_pairing_deps(Consumer, Default::default());

        // Act
        let created = create_pairing(&deps, params, CreatePairingPayload);

        // Assert
        assert!(created.is_ok());
    }
}
