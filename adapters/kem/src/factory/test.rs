#![allow(clippy::expect_used)]

use super::create_kem;
use super::provides::{
    ConsumeCreateKemCaseParams, ConsumeCreateKemCasePayload, CreateKemDepsOverrides,
    CreateKemErrorReturn, CreateKemParamsOverrides, CreateKemReturn, ICreateKemCaseConsumer,
    IdentityScope, KemConcrete, MockIKemConsumer, build_create_kem_case_deps,
    build_create_kem_case_params, build_create_kem_case_payload, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, run_create_kem_case,
};
use crate::admit_kem_composition::provides::{
    AdmitKemCompositionDeps, AdmitKemCompositionErrorReturn, AdmitKemCompositionParams,
    AdmitKemCompositionPayload, AdmitKemCompositionReturn, AdmitKemCompositionSuccessReturn,
};
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};

/// Contract: `create_kem`, admitted: the composition admission returns `Ok`
///   for `params.concrete` and `params.scope` →
///   `Ok(CreateKemSuccessReturn { output })`.
/// Arrange: `Consumer` whose params are `build_create_kem_params(
///   CreateKemParamsOverrides { scope: Some(IdentityScope::Entitlement),
///   ..Default::default() })`, the default selection of the mock concrete,
///   and whose deps hold the default handles; the other scope's admission is
///   the other block's arrangement.
/// Act:     `run_create_kem_case` runs the case, whose consumer's single call
///   is `create_kem`.
/// Assert:  the output `is_ok()`.
/// Collaborators: the family's mock concrete through the factory by the
///   default selection; `MockIKemConsumer` in the deps; the pairing family's
///   mock concrete through `create_pairing` and the hash-to-scalar family's
///   mock concrete through `create_hash_to_scalar`, each by default params.
#[test]
fn create_kem_admits_the_mock_selection_under_the_entitlement_scope() {
    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
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

    let created = run_create_kem_case(
        &build_create_kem_case_deps(Consumer, Default::default()),
        build_create_kem_case_params(Default::default()),
        build_create_kem_case_payload(Default::default()),
    );

    assert!(created.is_ok());
    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert!(success.output.is_ok());
}

/// Contract: `create_kem`, admitted, as
///   `create_kem_admits_the_mock_selection_under_the_entitlement_scope`, under
///   the other scope.
/// Arrange: `Consumer` whose params are `build_create_kem_params(
///   CreateKemParamsOverrides { scope: Some(IdentityScope::Asset),
///   ..Default::default() })`, the scope the variation separating it from the
///   entitlement block's params.
/// Act:     `run_create_kem_case` runs the case, whose consumer's single call
///   is `create_kem`.
/// Assert:  the output `is_ok()`.
/// Collaborators: as
///   `create_kem_admits_the_mock_selection_under_the_entitlement_scope`.
#[test]
fn create_kem_admits_the_mock_selection_under_the_asset_scope() {
    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
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

    let created = run_create_kem_case(
        &build_create_kem_case_deps(Consumer, Default::default()),
        build_create_kem_case_params(Default::default()),
        build_create_kem_case_payload(Default::default()),
    );

    assert!(created.is_ok());
    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert!(success.output.is_ok());
}

/// Contract: `create_kem`, composition refused:
///   `deps.admit_kem_composition` returns `Err(error)` →
///   `Err(CreateKemErrorReturn::Composition(error))`, the refusal unchanged,
///   read before any construction.
/// Arrange: `Consumer` whose deps are `build_create_kem_deps` with
///   `CreateKemDepsOverrides { admit_kem_composition: Some(refuse),
///   ..Default::default() }`, `refuse` a function declared in the block of
///   the production type `AdmitKemCompositionFn` returning
///   `Err(AdmitKemCompositionErrorReturn::Incompatible { concrete:
///   KemConcrete::Mock, declared: &[IdentityScope::Asset], required:
///   IdentityScope::Entitlement })`, and whose params are
///   `build_create_kem_params(Default::default())`; the refusal differs from
///   the default `Ok` the mock handle returns, so an arm that ignores the
///   admission fails.
/// Act:     `run_create_kem_case` runs the case, whose consumer's single call
///   is `create_kem`.
/// Assert:  `assert_eq!(output.err(),
///   Some(CreateKemErrorReturn::Composition(
///   AdmitKemCompositionErrorReturn::Incompatible { concrete:
///   KemConcrete::Mock, declared: &[IdentityScope::Asset], required:
///   IdentityScope::Entitlement })))`.
/// Collaborators: `MockIKemConsumer` in the deps and the pairing and
///   hash-to-scalar families' mock concretes as above; the refusal is decided
///   by the handle, before any construction, so no concrete's function runs.
#[test]
fn create_kem_returns_the_composition_refusal_unchanged() {
    fn refuse(
        _deps: &AdmitKemCompositionDeps,
        _params: AdmitKemCompositionParams,
        _payload: AdmitKemCompositionPayload,
    ) -> AdmitKemCompositionReturn {
        Err(AdmitKemCompositionErrorReturn::Incompatible {
            concrete: KemConcrete::Mock,
            declared: &[IdentityScope::Asset],
            required: IdentityScope::Entitlement,
        })
    }

    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    MockIKemConsumer,
                    CreateKemDepsOverrides {
                        admit_kem_composition: Some(refuse),
                        ..Default::default()
                    },
                ),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let created = run_create_kem_case(
        &build_create_kem_case_deps(Consumer, Default::default()),
        build_create_kem_case_params(Default::default()),
        build_create_kem_case_payload(Default::default()),
    );

    assert!(created.is_ok());
    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert_eq!(
        success.output.err(),
        Some(CreateKemErrorReturn::Composition(
            AdmitKemCompositionErrorReturn::Incompatible {
                concrete: KemConcrete::Mock,
                declared: &[IdentityScope::Asset],
                required: IdentityScope::Entitlement,
            }
        ))
    );
}

/// Contract: `create_kem`, the admission is called with the params' concrete
///   and scope → `Ok(CreateKemSuccessReturn { output })` only for the pair the
///   handle accepts.
/// Arrange: `Consumer` whose deps hold `accept_only`, a function declared in
///   the block of the production type `AdmitKemCompositionFn` that returns
///   `Ok` when its params are `KemConcrete::Mock` and `IdentityScope::Asset`
///   and the `Incompatible` refusal otherwise, and whose params are
///   `build_create_kem_params(CreateKemParamsOverrides {
///   scope: Some(IdentityScope::Asset), ..Default::default() })`, so an arm
///   that passes another concrete or scope to the admission is refused.
/// Act:     `run_create_kem_case` runs the case, whose consumer's single call
///   is `create_kem`.
/// Assert:  the output `is_ok()`.
/// Collaborators: as `create_kem_returns_the_composition_refusal_unchanged`.
#[test]
fn create_kem_admits_the_composition_the_params_name() {
    fn accept_only(
        _deps: &AdmitKemCompositionDeps,
        params: AdmitKemCompositionParams,
        _payload: AdmitKemCompositionPayload,
    ) -> AdmitKemCompositionReturn {
        if params.concrete == KemConcrete::Mock && params.scope == IdentityScope::Asset {
            Ok(AdmitKemCompositionSuccessReturn {
                concrete: params.concrete,
                scope: params.scope,
            })
        } else {
            Err(AdmitKemCompositionErrorReturn::Incompatible {
                concrete: params.concrete,
                declared: &[IdentityScope::Asset],
                required: params.scope,
            })
        }
    }

    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_create_kem_case<
            P: IPairingArithmetic + IPairingReference,
            H: IHashToScalarAdapter<P::Scalar>,
        >(
            &self,
            _params: ConsumeCreateKemCaseParams,
            payload: ConsumeCreateKemCasePayload<'_, P, H>,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    payload.pairing,
                    payload.hash_to_scalar,
                    MockIKemConsumer,
                    CreateKemDepsOverrides {
                        admit_kem_composition: Some(accept_only),
                        ..Default::default()
                    },
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(IdentityScope::Asset),
                    ..Default::default()
                }),
                build_create_kem_payload(Default::default()),
            )
        }
    }

    let created = run_create_kem_case(
        &build_create_kem_case_deps(Consumer, Default::default()),
        build_create_kem_case_params(Default::default()),
        build_create_kem_case_payload(Default::default()),
    );

    assert!(created.is_ok());
    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert!(success.output.is_ok());
}
