#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{IChainAdapter, IEntitlementStateAdapter};
use encoding::CanonicalFieldValue;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::IDeliveryProofAdapter;

use super::super::admit_verifier_composition::provides::{
    AdmitVerifierCompositionDeps, AdmitVerifierCompositionErrorReturn, AdmitVerifierCompositionFn,
    AdmitVerifierCompositionParams, AdmitVerifierCompositionPayload,
    AdmitVerifierCompositionReturn, ChainMemberIdentifierDisagreementOverrides,
    build_chain_member_identifier_disagreement,
};
use super::create_verifier;
use super::interface::{
    ConsumeCreateVerifierCaseParams, ConsumeCreateVerifierCasePayload, CreateVerifierErrorReturn,
    CreateVerifierParams, CreateVerifierReturn, CreateVerifierSuccessReturn,
    ICreateVerifierCaseConsumer,
};
use super::mock::{
    CreateVerifierDepsOverrides, CreateVerifierParamsOverrides, MockIVerifierConsumer,
    build_create_verifier_case_deps, build_create_verifier_case_params,
    build_create_verifier_case_payload, build_create_verifier_deps, build_create_verifier_params,
    build_create_verifier_payload, run_create_verifier_case,
};
use crate::generate::provides::{GenerateDeclarationOverrides, build_generate_declaration};

/// Contract: admitted, `Mock(failure_mode)` arm: the composition admitted
///   and the mock concrete's declaration listing the params' version →
///   the consumer is called once with the constructed mock concrete and
///   `Ok(CreateVerifierSuccessReturn { output })` holds its output.
/// Arrange: `build_create_verifier_params`, the mock selection under
///   `NoFailure` at `DELIVERY_STATEMENT_VERSION_ONE`, `verifier_address`
///   `CanonicalFieldValue::FixedBytes20([0x11; 20])` the deployed location
///   the params carry.
/// Act:     `create_verifier`, the single call inside the block's case
///   consumer, run once by `run_create_verifier_case`.
/// Assert:  `matches!(result, Ok(CreateVerifierSuccessReturn { output: ()
///   }))`.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's,
///   chain-forms', chain's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories; the
///   composition admission at its mock default; `MockIVerifierConsumer`
///   in the deps.
#[test]
fn create_verifier_hands_the_consumer_the_concrete_the_selection_names() {
    // Arrange
    let params = build_create_verifier_params(CreateVerifierParamsOverrides {
        verifier_address: Some(CanonicalFieldValue::FixedBytes20([0x11; 20])),
        ..Default::default()
    });
    let generate = build_generate_declaration(GenerateDeclarationOverrides::default());

    struct CaseConsumer<'a> {
        params: &'a CreateVerifierParams,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

        fn consume_create_verifier_case<P, C, D>(
            &self,
            _params: ConsumeCreateVerifierCaseParams,
            payload: ConsumeCreateVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    payload.pairing,
                    payload.chain,
                    payload.delivery_proof,
                    payload.encoder,
                    payload.generate,
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
        build_create_verifier_case_payload(&generate, Default::default()),
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
///   `Some(2)`, a version no concrete of the family declares.
/// Act:     `create_verifier`, the single call inside the block's case
///   consumer, run once by `run_create_verifier_case`.
/// Assert:  `matches!(result, Err(CreateVerifierErrorReturn::
///   UnsupportedStatementVersion { version: 2 }))`, the pattern naming
///   every field because `CreateVerifierErrorReturn` omits `PartialEq`.
/// Collaborators: as
///   `create_verifier_hands_the_consumer_the_concrete_the_selection_names`;
///   the refusal is decided from the selected concrete's declaration,
///   after the composition admission and before any construction.
#[test]
fn create_verifier_refuses_a_statement_version_the_declaration_does_not_list() {
    // Arrange
    let params = build_create_verifier_params(CreateVerifierParamsOverrides {
        statement_version: Some(2),
        ..Default::default()
    });
    let generate = build_generate_declaration(GenerateDeclarationOverrides::default());

    struct CaseConsumer<'a> {
        params: &'a CreateVerifierParams,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

        fn consume_create_verifier_case<P, C, D>(
            &self,
            _params: ConsumeCreateVerifierCaseParams,
            payload: ConsumeCreateVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    payload.pairing,
                    payload.chain,
                    payload.delivery_proof,
                    payload.encoder,
                    payload.generate,
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
        build_create_verifier_case_payload(&generate, Default::default()),
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

/// Contract: incompatible composition: the admission's `Err` →
///   `Err(CreateVerifierErrorReturn::Composition(error))`, `error` carried
///   whole, nothing constructed and the consumer not called.
/// Arrange: `build_create_verifier_deps` with `admit_verifier_composition`
///   `Some(refuse_verifier_composition)`, the block declaring
///   `refuse_verifier_composition` returning an `Incompatible` refusal
///   carrying a chain-member disagreement alone.
/// Act:     `create_verifier`, the single call inside the block's case
///   consumer, run once by `run_create_verifier_case`.
/// Assert:  `matches!(result, Err(CreateVerifierErrorReturn::Composition(
///   AdmitVerifierCompositionErrorReturn::Incompatible {
///   chain_member: Some(..), chain_forms: None, encoding: None,
///   verifier_form: None, contract_suite: None })))`, the pattern naming
///   every field because `CreateVerifierErrorReturn` omits `PartialEq`.
/// Collaborators: as
///   `create_verifier_hands_the_consumer_the_concrete_the_selection_names`,
///   with the composition admission overridden to the block's refusing
///   handle; the refusal is the family's flavor, returned before the
///   version is read and before any construction.
#[test]
fn create_verifier_carries_the_composition_refusal_unchanged() {
    // Arrange
    fn refuse_verifier_composition(
        _deps: &AdmitVerifierCompositionDeps,
        _params: AdmitVerifierCompositionParams,
        _payload: AdmitVerifierCompositionPayload<'_>,
    ) -> AdmitVerifierCompositionReturn {
        let _: AdmitVerifierCompositionFn = refuse_verifier_composition;
        Err(AdmitVerifierCompositionErrorReturn::Incompatible {
            chain_forms: None,
            chain_member: Some(build_chain_member_identifier_disagreement(
                ChainMemberIdentifierDisagreementOverrides::default(),
            )),
            encoding: None,
            verifier_form: None,
            contract_suite: None,
        })
    }

    let params = build_create_verifier_params(Default::default());
    let generate = build_generate_declaration(GenerateDeclarationOverrides::default());

    struct CaseConsumer<'a> {
        params: &'a CreateVerifierParams,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

        fn consume_create_verifier_case<P, C, D>(
            &self,
            _params: ConsumeCreateVerifierCaseParams,
            payload: ConsumeCreateVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    payload.pairing,
                    payload.chain,
                    payload.delivery_proof,
                    payload.encoder,
                    payload.generate,
                    MockIVerifierConsumer,
                    CreateVerifierDepsOverrides {
                        admit_verifier_composition: Some(refuse_verifier_composition),
                    },
                ),
                self.params.clone(),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let created = run_create_verifier_case(
        &build_create_verifier_case_deps(CaseConsumer { params: &params }, Default::default()),
        build_create_verifier_case_params(Default::default()),
        build_create_verifier_case_payload(&generate, Default::default()),
    );

    // Assert
    let Ok(success) = created else {
        panic!("the case runner admits each family's default selection")
    };
    assert!(matches!(
        success.output,
        Err(CreateVerifierErrorReturn::Composition(
            AdmitVerifierCompositionErrorReturn::Incompatible {
                chain_member: Some(..),
                chain_forms: None,
                encoding: None,
                verifier_form: None,
                contract_suite: None,
            }
        ))
    ));
    // create_verifier_carries_the_composition_refusal_unchanged
}
