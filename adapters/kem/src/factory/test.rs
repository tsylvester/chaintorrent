#![allow(clippy::expect_used)]

use super::create_kem;
use super::provides::{
    CreateKemCasePayload, CreateKemErrorReturn, CreateKemParamsOverrides, CreateKemReturn,
    DECLARED_KEM_SELECTIONS, ICreateKemCaseConsumer, IdentityScope, KemIdentifier,
    MockIKemConsumer, build_create_kem_case_deps, build_create_kem_case_params,
    build_create_kem_case_payload, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, run_create_kem_case,
};
use hash_to_scalar::IHashToScalarAdapter;
use pairing::{IPairingArithmetic, IPairingReference};

/// Contract: `create_kem`, admitted: the named concrete's declared identifier
///   is `params.identifier` and its declared scopes contain `params.scope` →
///   `Ok(CreateKemSuccessReturn { output })`.
/// Arrange: `Consumer` whose params are `build_create_kem_params(
///   CreateKemParamsOverrides { scope: Some(IdentityScope::Entitlement),
///   ..Default::default() })`, the default selection of the mock concrete
///   under the identifier it declares; the other scope's admission is the
///   other block's arrangement.
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

        fn consume_create_kem_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &CreateKemCasePayload,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    pairing,
                    hash_to_scalar,
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
///   ..Default::default() })`, so an admission that tests only one scope
///   refuses this case.
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

        fn consume_create_kem_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &CreateKemCasePayload,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    pairing,
                    hash_to_scalar,
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

/// Contract: `create_kem`, unsupported identifier: the named concrete's
///   `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateKemErrorReturn::UnsupportedKemIdentifier)`.
/// Arrange: `Consumer` whose params are `build_create_kem_params(
///   CreateKemParamsOverrides { identifier: Some(KemIdentifier::Bb1DepthOneV1),
///   ..Default::default() })`, keeping the default selection of the mock
///   concrete, which declares `KemIdentifier::Mock`, so the identifier
///   required differs from the identifier declared, and an admission that
///   ignores the identifier or compares it with a constant fails.
/// Act:     `run_create_kem_case` runs the case, whose consumer's single call
///   is `create_kem`.
/// Assert:  `assert_eq!(output.err(),
///   Some(CreateKemErrorReturn::UnsupportedKemIdentifier))`.
/// Collaborators: as
///   `create_kem_admits_the_mock_selection_under_the_entitlement_scope`.
#[test]
fn create_kem_refuses_the_mock_selection_under_an_identifier_it_does_not_declare() {
    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_create_kem_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &CreateKemCasePayload,
        ) -> Self::Output {
            create_kem(
                &build_create_kem_deps(
                    pairing,
                    hash_to_scalar,
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
/// Act:     `run_create_kem_case` runs the case, whose consumer's single call
///   per case is `create_kem`.
/// Assert:  per case, `assert_eq!(result.err(),
///   Some(CreateKemErrorReturn::UnsupportedKemIdentifier))`.
/// Collaborators: `MockIKemConsumer` in the deps and the pairing and
///   hash-to-scalar families' mock concretes as above; the refusal is decided
///   from the selected concrete's declaration constant, before any
///   construction, so no real concrete's function runs.
#[test]
fn create_kem_refuses_each_declared_concrete_under_an_identifier_it_does_not_declare() {
    struct Consumer;

    impl ICreateKemCaseConsumer for Consumer {
        type Output = CreateKemReturn<()>;

        fn consume_create_kem_case<P: IPairingArithmetic + IPairingReference>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            _payload: &CreateKemCasePayload,
        ) -> Self::Output {
            let mut output = None;
            for selection in DECLARED_KEM_SELECTIONS {
                let result = create_kem(
                    &build_create_kem_deps(
                        pairing,
                        hash_to_scalar,
                        MockIKemConsumer,
                        Default::default(),
                    ),
                    build_create_kem_params(CreateKemParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(KemIdentifier::Mock),
                        scope: Some(selection.scope),
                    }),
                    build_create_kem_payload(Default::default()),
                );
                assert_eq!(
                    result.as_ref().err(),
                    Some(&CreateKemErrorReturn::UnsupportedKemIdentifier)
                );
                output = Some(result);
            }
            let Some(output) = output else {
                panic!("the declared set holds a case")
            };
            output
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
        Some(CreateKemErrorReturn::UnsupportedKemIdentifier)
    );
}
