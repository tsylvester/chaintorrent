#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DELIVERY_STATEMENT_VERSION_ONE, IDeliveryProofAdapter};

use super::create_verifier;
use super::interface::{
    CreateVerifierErrorReturn, CreateVerifierParams, CreateVerifierReturn,
    CreateVerifierSuccessReturn, ICreateVerifierCaseConsumer,
};
use super::mock::{
    CreateVerifierParamsOverrides, MockIVerifierConsumer, build_create_verifier_case_deps,
    build_create_verifier_case_params, build_create_verifier_case_payload,
    build_create_verifier_deps, build_create_verifier_params, build_create_verifier_payload,
    run_create_verifier_case,
};

/// Contract: admitted, `Mock(failure_mode)` arm: the mock concrete's
///   declaration serves the pairing's form and lists the params' version →
///   the consumer is called once with the constructed mock concrete and
///   `Ok(CreateVerifierSuccessReturn { output })` holds its output.
/// Arrange: `build_create_verifier_params(Default::default())`, the mock
///   selection under `NoFailure` at `DELIVERY_STATEMENT_VERSION_ONE`.
/// Act:     `create_verifier`, called in the innermost consumer's method.
/// Assert:  `matches!(result, Ok(CreateVerifierSuccessReturn { output: ()
///   }))`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', delivery-proof's, and random-source's mock concretes in
///   their default modes through their factories; `MockIVerifierConsumer`
///   in the deps.
#[test]
fn create_verifier_hands_the_consumer_the_concrete_the_selection_names() {
    // Arrange
    let params = build_create_verifier_params(Default::default());

    struct CaseConsumer<'a> {
        params: &'a CreateVerifierParams,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

        fn consume_create_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    pairing,
                    delivery_proof,
                    MockIVerifierConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let created = run_create_verifier_case(
        &build_create_verifier_case_deps(CaseConsumer { params: &params }, Default::default()),
        build_create_verifier_case_params(Default::default()),
        build_create_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(success) = created else {
        panic!("the case runner admits each family's default selection")
    };
    assert!(matches!(
        success.output,
        Ok(CreateVerifierSuccessReturn { output: () })
    ));
    // create_verifier_hands_the_consumer_the_concrete_the_selection_names
}

/// Contract: statement version: the declaration's `statement_versions`
///   does not contain the params' version → `Err(CreateVerifierErrorReturn
///   ::UnsupportedStatementVersion { version })`, nothing constructed.
/// Arrange: `build_create_verifier_params` with `statement_version`
///   `DELIVERY_STATEMENT_VERSION_ONE + 1`, a version no concrete of the
///   family declares.
/// Act:     `create_verifier`, called in the innermost consumer's method.
/// Assert:  `matches!(result, Err(CreateVerifierErrorReturn::
///   UnsupportedStatementVersion { version: 2 }))`, the pattern naming
///   every field because `CreateVerifierErrorReturn` omits `PartialEq`.
/// Collaborators: as
///   `create_verifier_hands_the_consumer_the_concrete_the_selection_names`;
///   the refusal is decided from the selected concrete's declaration,
///   before any construction.
#[test]
fn create_verifier_refuses_a_statement_version_the_declaration_does_not_list() {
    // Arrange
    let params = build_create_verifier_params(CreateVerifierParamsOverrides {
        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE + 1),
        ..Default::default()
    });

    struct CaseConsumer<'a> {
        params: &'a CreateVerifierParams,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

        fn consume_create_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    pairing,
                    delivery_proof,
                    MockIVerifierConsumer,
                    Default::default(),
                ),
                self.params.clone(),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let created = run_create_verifier_case(
        &build_create_verifier_case_deps(CaseConsumer { params: &params }, Default::default()),
        build_create_verifier_case_params(Default::default()),
        build_create_verifier_case_payload(Default::default()),
    );

    // Assert
    let Ok(success) = created else {
        panic!("the case runner admits each family's default selection")
    };
    assert!(matches!(
        success.output,
        Err(CreateVerifierErrorReturn::UnsupportedStatementVersion { version: 2 })
    ));
    // create_verifier_refuses_a_statement_version_the_declaration_does_not_list
}
