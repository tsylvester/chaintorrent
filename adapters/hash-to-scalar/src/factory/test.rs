use super::create_hash_to_scalar;
use super::provides::{
    CreateHashToScalarCasePayload, CreateHashToScalarErrorReturn,
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    ICreateHashToScalarCaseConsumer, IHashToScalarAdapter, build_create_hash_to_scalar_case_deps,
    build_create_hash_to_scalar_case_params, build_create_hash_to_scalar_case_payload,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, run_create_hash_to_scalar_case,
};
use pairing::IPairingAdapter;

/// Contract: `create_hash_to_scalar`, admitted: the named concrete's declared
///   identifier is `params.identifier` →
///   `Ok(CreateHashToScalarSuccessReturn { adapter })`, whose adapter reports
///   the concrete's declaration.
/// Arrange: `Consumer` whose params are
///   `build_create_hash_to_scalar_params(Default::default())`, which select
///   the mock concrete under the identifier it declares; the adapter reports
///   the declaration of the concrete it is, so a factory that constructs
///   another concrete reports another identifier.
/// Act:     `run_create_hash_to_scalar_case` runs the case runner, whose case
///   consumer's single call is `create_hash_to_scalar::<P::Scalar>`.
/// Assert:  in the consumer, the result is `Ok`, and `adapter.declaration()`
///   has an identifier matching `HashToScalarIdentifier::Mock` by
///   `assert!(matches!(…))`, an `adapter_version` of `1`, and an
///   `interface_version` of `1`; outside the consumer, `created.is_ok()`,
///   proving the consumer ran.
/// Collaborators: the family's mock concrete through the factory by the
///   default selection; the pairing family's mock concrete through
///   `create_pairing`, supplying `P::Scalar` only.
#[test]
fn create_hash_to_scalar_returns_the_adapter_the_mock_selection_names() {
    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            _pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let result = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(success) = result else {
                panic!(
                    "the default params select the mock concrete under the identifier it declares"
                )
            };
            let adapter: &dyn IHashToScalarAdapter<P::Scalar> = &*success.adapter;
            let declaration = adapter.declaration();
            assert!(matches!(
                declaration.identifier,
                HashToScalarIdentifier::Mock
            ));
            assert_eq!(declaration.adapter_version, 1);
            assert_eq!(declaration.interface_version, 1);
        }
    }

    let created = run_create_hash_to_scalar_case(
        &build_create_hash_to_scalar_case_deps(Consumer, Default::default()),
        build_create_hash_to_scalar_case_params(Default::default()),
        build_create_hash_to_scalar_case_payload(Default::default()),
    );

    assert!(created.is_ok());
}

/// Contract: `create_hash_to_scalar`, unsupported identifier: the named
///   concrete's `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`.
/// Arrange: `Consumer` whose params are
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
///   identifier: Some(HashToScalarIdentifier::Keccak256V1),
///   ..Default::default() })`, keeping the default selection of the mock
///   concrete, which declares `HashToScalarIdentifier::Mock`, so the
///   identifier required differs from the identifier declared, and an
///   admission that ignores the identifier or compares it with a constant
///   fails.
/// Act:     `run_create_hash_to_scalar_case` runs the case runner, whose case
///   consumer's single call is `create_hash_to_scalar::<P::Scalar>`.
/// Assert:  in the consumer, `result.err()` equals
///   `Some(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`
///   by `assert_eq!`; outside the consumer, `created.is_ok()`.
/// Collaborators: as
///   `create_hash_to_scalar_returns_the_adapter_the_mock_selection_names`.
#[test]
fn create_hash_to_scalar_refuses_the_mock_selection_under_an_identifier_it_does_not_declare() {
    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            _pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            let result = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    identifier: Some(HashToScalarIdentifier::Keccak256V1),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            assert_eq!(
                result.err(),
                Some(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)
            );
        }
    }

    let created = run_create_hash_to_scalar_case(
        &build_create_hash_to_scalar_case_deps(Consumer, Default::default()),
        build_create_hash_to_scalar_case_params(Default::default()),
        build_create_hash_to_scalar_case_payload(Default::default()),
    );

    assert!(created.is_ok());
}

/// Contract: `create_hash_to_scalar`, unsupported identifier: the named
///   concrete's `DECLARATION.identifier` is not `params.identifier` →
///   `Err(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`,
///   read before any construction.
/// Arrange: `Consumer` that, for each selection in
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, takes params from
///   `build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
///   concrete: Some(selection.concrete), identifier:
///   Some(HashToScalarIdentifier::Mock) })`; no declared concrete declares
///   `Mock`, so the identifier required differs from the identifier declared
///   for every case, and an arm that constructs without comparing returns an
///   adapter it must refuse.
/// Act:     `run_create_hash_to_scalar_case` runs the case runner, whose case
///   consumer's single call per case is `create_hash_to_scalar::<P::Scalar>`.
/// Assert:  in the consumer, per case, `result.err()` equals
///   `Some(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)`
///   by `assert_eq!`; outside the consumer, `created.is_ok()`.
/// Collaborators: the pairing family's mock concrete through `create_pairing`,
///   supplying `P::Scalar` only; the refusal is decided from the selected
///   concrete's declaration constant, before any construction, so no real
///   concrete's function runs.
#[test]
fn create_hash_to_scalar_refuses_each_declared_concrete_under_an_identifier_it_does_not_declare() {
    struct Consumer;

    impl ICreateHashToScalarCaseConsumer for Consumer {
        type Output = ();

        fn consume_create_hash_to_scalar_case<P: IPairingAdapter>(
            &self,
            _pairing: &P,
            _payload: &CreateHashToScalarCasePayload,
        ) -> Self::Output {
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let result = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(HashToScalarIdentifier::Mock),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                assert_eq!(
                    result.err(),
                    Some(CreateHashToScalarErrorReturn::UnsupportedHashToScalarIdentifier)
                );
            }
        }
    }

    let created = run_create_hash_to_scalar_case(
        &build_create_hash_to_scalar_case_deps(Consumer, Default::default()),
        build_create_hash_to_scalar_case_params(Default::default()),
        build_create_hash_to_scalar_case_payload(Default::default()),
    );

    assert!(created.is_ok());
}
