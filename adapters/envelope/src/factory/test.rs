#![allow(clippy::expect_used)]

use super::create_key_agreement;
use super::provides::{
    CreateKeyAgreementCasePayload, CreateKeyAgreementErrorReturn,
    CreateKeyAgreementParamsOverrides, CreateKeyAgreementReturn, DECLARED_KEY_AGREEMENT_SELECTIONS,
    ICreateKeyAgreementCaseConsumer, KeyAgreementIdentifier, MockIKeyAgreementConsumer,
    build_create_key_agreement_case_deps, build_create_key_agreement_case_params,
    build_create_key_agreement_case_payload, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload,
    run_create_key_agreement_case,
};
use encoding::{IDecoderAdapter, IEncoderAdapter};
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use random::IRandomSourceAdapter;

/// Contract: `create_key_agreement`, admitted: the arm's `try_new` returns
///   `Ok(adapter)` → `Ok(CreateKeyAgreementSuccessReturn { output })`, the
///   consumer's output held whole.
/// Arrange: `Consumer` whose params are `build_create_key_agreement_params(
///   Default::default())`, the default selection of the family's mock
///   concrete under the identifier and algebra its declaration admits; the
///   consumer is `MockIKeyAgreementConsumer`, whose `type Output = ()`.
/// Act:     `run_create_key_agreement_case` runs the case, whose consumer's
///   single call is `create_key_agreement`.
/// Assert:  the case success's `output` is `Ok`, holding the `()` the mock
///   consumer declares.
/// Collaborators: the pairing from `create_pairing`, the encoder from
///   `create_encoding`, the hash-to-scalar adapter from
///   `create_hash_to_scalar`, and the randomness source from
///   `create_random_source`, each reached through the case runner at the
///   defaults of its official params builder, which select that family's
///   mock concrete; the `kem` borrow is unused by the unit blocks.
#[test]
fn create_key_agreement_returns_the_consumers_output_for_the_mock_selection() {
    struct Consumer;

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<()>;

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            create_key_agreement(
                &build_create_key_agreement_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    MockIKeyAgreementConsumer,
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            )
        }
    }

    let created = run_create_key_agreement_case(
        &build_create_key_agreement_case_deps(Consumer, Default::default()),
        build_create_key_agreement_case_params(Default::default()),
        build_create_key_agreement_case_payload(Default::default()),
    );

    assert!(created.is_ok());
    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert!(success.output.is_ok());
}

/// Contract: `create_key_agreement`, unsupported identifier: the `Mock`
///   arm's declared identifier is not `params.identifier` →
///   `Err(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)`,
///   with nothing constructed and the consumer not called.
/// Arrange: `Consumer` that, for each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, takes params from
///   `build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
///   identifier: Some(member.identifier), ..Default::default() })`, keeping
///   the default selection of the mock concrete, which declares
///   `KeyAgreementIdentifier::Mock`, so the identifier required differs
///   from the identifier declared for every case.
/// Act:     `run_create_key_agreement_case` runs the case, whose consumer's
///   single call per case is `create_key_agreement`.
/// Assert:  per case and for the returned output,
///   `assert_eq!(output.err(),
///   Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier))`.
/// Collaborators: as
///   `create_key_agreement_returns_the_consumers_output_for_the_mock_selection`.
#[test]
fn create_key_agreement_refuses_the_mock_selection_whose_identifier_the_mock_does_not_declare() {
    struct Consumer;

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<()>;

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            let mut output = None;
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = create_key_agreement(
                    &build_create_key_agreement_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        MockIKeyAgreementConsumer,
                        Default::default(),
                    ),
                    build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                        identifier: Some(selection.identifier),
                        ..Default::default()
                    }),
                    build_create_key_agreement_payload(Default::default()),
                );
                assert_eq!(
                    result.as_ref().err(),
                    Some(&CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)
                );
                output = Some(result);
            }
            let Some(output) = output else {
                panic!("the declared set holds a case")
            };
            output
        }
    }

    let created = run_create_key_agreement_case(
        &build_create_key_agreement_case_deps(Consumer, Default::default()),
        build_create_key_agreement_case_params(Default::default()),
        build_create_key_agreement_case_payload(Default::default()),
    );

    assert!(created.is_ok());
    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert_eq!(
        success.output.err(),
        Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)
    );
}

/// Contract: `create_key_agreement`, unsupported identifier: the
///   `PairingElGamal` arm's declared identifier is not `params.identifier` →
///   `Err(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)`,
///   with nothing constructed and the consumer not called.
/// Arrange: `Consumer` that, for each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, takes params from
///   `build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
///   concrete: Some(member.concrete), identifier:
///   Some(KeyAgreementIdentifier::Mock), algebra: Some(member.algebra) })`,
///   the identifier `build_create_key_agreement_params` carries at its
///   defaults, which no declared concrete declares, so the identifier
///   required differs from the identifier declared for every case.
/// Act:     `run_create_key_agreement_case` runs the case, whose consumer's
///   single call per case is `create_key_agreement`.
/// Assert:  per case and for the returned output,
///   `assert_eq!(output.err(),
///   Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier))`.
/// Collaborators: as
///   `create_key_agreement_returns_the_consumers_output_for_the_mock_selection`;
///   the refusal is decided from the selected concrete's declaration
///   constant, before any construction, so no real concrete's function runs.
#[test]
fn create_key_agreement_refuses_a_declared_selection_whose_identifier_the_concrete_does_not_declare()
 {
    struct Consumer;

    impl ICreateKeyAgreementCaseConsumer for Consumer {
        type Output = CreateKeyAgreementReturn<()>;

        fn consume_create_key_agreement_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            K: ICredentialKemAdapter<Pairing = P>,
        >(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _kem: &K,
            _payload: &CreateKeyAgreementCasePayload,
        ) -> Self::Output {
            let mut output = None;
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = create_key_agreement(
                    &build_create_key_agreement_deps(
                        pairing,
                        hash_to_scalar,
                        encoder,
                        random,
                        MockIKeyAgreementConsumer,
                        Default::default(),
                    ),
                    build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(KeyAgreementIdentifier::Mock),
                        algebra: Some(selection.algebra),
                    }),
                    build_create_key_agreement_payload(Default::default()),
                );
                assert_eq!(
                    result.as_ref().err(),
                    Some(&CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)
                );
                output = Some(result);
            }
            let Some(output) = output else {
                panic!("the declared set holds a case")
            };
            output
        }
    }

    let created = run_create_key_agreement_case(
        &build_create_key_agreement_case_deps(Consumer, Default::default()),
        build_create_key_agreement_case_params(Default::default()),
        build_create_key_agreement_case_payload(Default::default()),
    );

    assert!(created.is_ok());
    let Ok(success) = created else {
        panic!("the case ran")
    };
    assert_eq!(
        success.output.err(),
        Some(CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier)
    );
}
