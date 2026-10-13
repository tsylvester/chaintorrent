#![allow(clippy::expect_used)]

use super::create_kem;
use super::provides::{
    CreateKemErrorReturn, CreateKemParamsOverrides, CreateKemReturn, DECLARED_KEM_SELECTIONS,
    IdentityScope, KemIdentifier, MockIKemConsumer, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload,
};
use hash_to_scalar::{
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingArithmetic,
    IPairingConsumer, build_create_pairing_deps, build_create_pairing_params, create_pairing,
};

/// Contract: `create_kem`, admitted: the named concrete's declared identifier
///   is `params.identifier` and its declared scopes contain `params.scope` →
///   `Ok(CreateKemSuccessReturn { output })`.
/// Arrange: `Consumer` whose params are
///   `build_create_kem_params(CreateKemParamsOverrides { scope:
///   Some(IdentityScope::Entitlement), ..Default::default() })`, the default
///   selection of the mock concrete under the identifier it declares; the
///   other scope's admission is the other block's arrangement.
/// Act:     `create_pairing` runs the consumer, whose single call is
///   `create_kem`.
/// Assert:  the output `is_ok()`; `created.is_ok()`, proving the consumer ran.
/// Collaborators: the family's mock concrete through the factory by the
///   default selection; `MockIKemConsumer` in the deps; the pairing family's
///   mock concrete through `create_pairing` and the hash-to-scalar family's
///   mock concrete through `create_hash_to_scalar`, each by default params.
#[test]
fn create_kem_admits_the_mock_selection_under_the_entitlement_scope() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashed = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default params select the mock concrete under the identifier it declares");
            create_kem(
                &build_create_kem_deps(
                    &consumed.adapter,
                    &*hashed.adapter,
                    MockIKemConsumer,
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(IdentityScope::Entitlement),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    assert!(
        created
            .expect("the pairing factory ran the consumer")
            .output
            .is_ok()
    );
}

/// Contract: `create_kem`, admitted, as
///   `create_kem_admits_the_mock_selection_under_the_entitlement_scope`,
///   under the other scope.
/// Arrange: `Consumer` whose params are
///   `build_create_kem_params(CreateKemParamsOverrides { scope:
///   Some(IdentityScope::Asset), ..Default::default() })`, so an admission
///   that tests only one scope refuses this case.
/// Act:     `create_pairing` runs the consumer, whose single call is
///   `create_kem`.
/// Assert:  the output `is_ok()`; `created.is_ok()`, proving the consumer ran.
/// Collaborators: as
///   `create_kem_admits_the_mock_selection_under_the_entitlement_scope`.
#[test]
fn create_kem_admits_the_mock_selection_under_the_asset_scope() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashed = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default params select the mock concrete under the identifier it declares");
            create_kem(
                &build_create_kem_deps(
                    &consumed.adapter,
                    &*hashed.adapter,
                    MockIKemConsumer,
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(IdentityScope::Asset),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    assert!(
        created
            .expect("the pairing factory ran the consumer")
            .output
            .is_ok()
    );
}

/// Contract: `create_kem`, unsupported identifier: the named concrete's
///   `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateKemErrorReturn::UnsupportedKemIdentifier)`.
/// Arrange: `Consumer` whose params are
///   `build_create_kem_params(CreateKemParamsOverrides { identifier:
///   Some(KemIdentifier::Bb1DepthOneV1), ..Default::default() })`, keeping
///   the default selection of the mock concrete, which declares
///   `KemIdentifier::Mock`, so the identifier required differs from the
///   identifier declared, and an admission that ignores the identifier or
///   compares it with a constant fails.
/// Act:     `create_pairing` runs the consumer, whose single call is
///   `create_kem`.
/// Assert:  `assert_eq!(output.err(),
///   Some(CreateKemErrorReturn::UnsupportedKemIdentifier))`;
///   `created.is_ok()`, proving the consumer ran.
/// Collaborators: as
///   `create_kem_admits_the_mock_selection_under_the_entitlement_scope`.
#[test]
fn create_kem_refuses_the_mock_selection_under_an_identifier_it_does_not_declare() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashed = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default params select the mock concrete under the identifier it declares");
            create_kem(
                &build_create_kem_deps(
                    &consumed.adapter,
                    &*hashed.adapter,
                    MockIKemConsumer,
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    identifier: Some(KemIdentifier::Bb1DepthOneV1),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    assert_eq!(
        created
            .expect("the pairing factory ran the consumer")
            .output
            .err(),
        Some(CreateKemErrorReturn::UnsupportedKemIdentifier)
    );
}

/// Contract: `create_kem`, unsupported identifier: the named concrete's
///   `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateKemErrorReturn::UnsupportedKemIdentifier)`, read before any
///   construction.
/// Arrange: `Consumer` that, for each selection in `DECLARED_KEM_SELECTIONS`,
///   takes params from `build_create_kem_params(CreateKemParamsOverrides {
///   concrete: Some(selection.concrete), identifier: Some(KemIdentifier::Mock),
///   scope: Some(selection.scope) })`; no declared concrete declares `Mock`,
///   so the identifier required differs from the identifier declared for
///   every case, and an arm that constructs without comparing returns an
///   adapter it must refuse; its output is each case's result.
/// Act:     `create_pairing` runs the consumer, whose single call per case is
///   `create_kem`.
/// Assert:  per case, `assert_eq!(result.err(),
///   Some(CreateKemErrorReturn::UnsupportedKemIdentifier))`;
///   `created.is_ok()`, proving the consumer ran.
/// Collaborators: `MockIKemConsumer` in the deps and the pairing and
///   hash-to-scalar families' mock concretes as above; the refusal is decided
///   from the selected concrete's declaration constant, before any
///   construction, so no real concrete's function runs.
#[test]
fn create_kem_refuses_each_declared_concrete_under_an_identifier_it_does_not_declare() {
    struct Consumer;

    impl IPairingConsumer for Consumer {
        type Output = Vec<CreateKemReturn<()>>;

        fn consume_pairing<P: IPairingArithmetic>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashed = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default params select the mock concrete under the identifier it declares");
            DECLARED_KEM_SELECTIONS
                .iter()
                .map(|selection| {
                    create_kem(
                        &build_create_kem_deps(
                            &consumed.adapter,
                            &*hashed.adapter,
                            MockIKemConsumer,
                            Default::default(),
                        ),
                        build_create_kem_params(CreateKemParamsOverrides {
                            concrete: Some(selection.concrete),
                            identifier: Some(KemIdentifier::Mock),
                            scope: Some(selection.scope),
                        }),
                        build_create_kem_payload(Default::default()),
                    )
                })
                .collect()
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(Consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    for result in created
        .expect("the pairing factory ran the consumer")
        .output
    {
        assert_eq!(
            result.err(),
            Some(CreateKemErrorReturn::UnsupportedKemIdentifier)
        );
    }
}
