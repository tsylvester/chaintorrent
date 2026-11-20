#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    ChainDeclarationOverrides, ChainFormsDeclarationOverrides, ChainMemberIdentifier,
    build_chain_declaration, build_chain_forms_declaration,
};
use encoding::{EncodingDeclarationOverrides, EncodingIdentifier, build_encoding_declaration};
use pairing::{PairingDeclarationOverrides, VerifierGroupArithmetic, build_pairing_declaration};

use crate::generate::provides::{
    EncodingIdentifierDisagreementOverrides, GenerateDeclarationOverrides,
    build_encoding_identifier_disagreement, build_generate_declaration,
};

use super::super::factory::provides::{VerifierDeclarationOverrides, build_verifier_declaration};
use super::super::lookup_verifier_declaration::provides::{
    LookupVerifierDeclarationDeps, LookupVerifierDeclarationFn, LookupVerifierDeclarationParams,
    LookupVerifierDeclarationPayload, LookupVerifierDeclarationReturn,
    LookupVerifierDeclarationSuccessReturnOverrides,
    build_lookup_verifier_declaration_success_return, mock_lookup_verifier_declaration,
};
use super::admit_verifier_composition;
use super::interface::*;
use super::mock::*;

/// Contract: a chain forms declaration at `ChainFormsIdentifier::EvmV1`,
///   a chain declaration carrying `ChainMemberIdentifier::Base`, an
///   encoder declaration carrying `EncodingIdentifier::EthereumAbiV1`, a
///   pairing declaration carrying `VerifierGroupArithmetic::
///   FirstGroupOnly`, and a generate declaration emitting for the
///   contract suite `evm`, the values the handle's declaration requires
///   or serves → `Ok(AdmitVerifierCompositionSuccessReturn)`.
/// Arrange: the encoder declaration built with `identifier`
///   `Some(EncodingIdentifier::EthereumAbiV1)`, the identifier stated
///   independently of the declaration the lookup handle returns so a
///   comparison that read the payload's identifiers back, or swapped the
///   required and declared sides, fails; every other declaration at its
///   defaults.
/// Act:    `admit_verifier_composition`.
/// Assert: the result equals `Ok(build_admit_verifier_composition_
///   success_return(Default::default()))`.
#[test]
fn declarations_at_every_required_value_are_admitted() {
    // Arrange
    let deps =
        build_admit_verifier_composition_deps(mock_lookup_verifier_declaration, Default::default());
    let params = build_admit_verifier_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(ChainFormsDeclarationOverrides::default());
    let chain = build_chain_declaration(Default::default());
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
        ..Default::default()
    });
    let pairing = build_pairing_declaration(Default::default());
    let generate = build_generate_declaration(Default::default());
    let payload = build_admit_verifier_composition_payload(
        &chain_forms,
        &chain,
        &encoder,
        &pairing,
        &generate,
        Default::default(),
    );

    // Act
    let result = admit_verifier_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_verifier_composition_success_return(
            Default::default()
        ))
    );
}
// declarations_at_every_required_value_are_admitted

/// Contract: a chain declaration carrying `ChainMemberIdentifier::Mock`
///   while the handle's declaration requires `ChainMemberIdentifier::
///   Base`, every other declaration at a required value →
///   `Err(Incompatible)` holding the chain member disagreement and no
///   other.
/// Arrange: the chain declaration built with `identifier`
///   `Some(ChainMemberIdentifier::Mock)` and the encoder declaration at
///   `EthereumAbiV1`, so a refusal that names the declared identifier as
///   the required one, that also names another disagreement, or that
///   succeeds, fails.
/// Act:    `admit_verifier_composition`.
/// Assert: the result equals `Err(AdmitVerifierCompositionErrorReturn::
///   Incompatible { chain_forms: None, chain_member: Some(
///   build_chain_member_identifier_disagreement(
///   ChainMemberIdentifierDisagreementOverrides { required:
///   Some(ChainMemberIdentifier::Base), declared:
///   Some(ChainMemberIdentifier::Mock) })), encoding: None,
///   verifier_form: None, contract_suite: None })`.
#[test]
fn a_chain_member_other_than_the_required_one_is_refused_naming_both_identifiers() {
    // Arrange
    let deps =
        build_admit_verifier_composition_deps(mock_lookup_verifier_declaration, Default::default());
    let params = build_admit_verifier_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(Default::default());
    let chain = build_chain_declaration(ChainDeclarationOverrides {
        identifier: Some(ChainMemberIdentifier::Mock),
        ..Default::default()
    });
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
        ..Default::default()
    });
    let pairing = build_pairing_declaration(Default::default());
    let generate = build_generate_declaration(Default::default());
    let payload = build_admit_verifier_composition_payload(
        &chain_forms,
        &chain,
        &encoder,
        &pairing,
        &generate,
        Default::default(),
    );

    // Act
    let result = admit_verifier_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitVerifierCompositionErrorReturn::Incompatible {
            chain_forms: None,
            chain_member: Some(build_chain_member_identifier_disagreement(
                ChainMemberIdentifierDisagreementOverrides {
                    required: Some(ChainMemberIdentifier::Base),
                    declared: Some(ChainMemberIdentifier::Mock),
                },
            )),
            encoding: None,
            verifier_form: None,
            contract_suite: None,
        })
    );
}
// a_chain_member_other_than_the_required_one_is_refused_naming_both_identifiers

/// Contract: an encoder declaration carrying `EncodingIdentifier::Mock`
///   while the handle's declaration requires `EncodingIdentifier::
///   EthereumAbiV1`, every other declaration at a required value →
///   `Err(Incompatible)` holding the encoding disagreement and no other.
/// Arrange: the encoder declaration built with `identifier`
///   `Some(EncodingIdentifier::Mock)`, so a refusal that names the
///   declared identifier as the required one, that also names another
///   disagreement, or that succeeds, fails.
/// Act:    `admit_verifier_composition`.
/// Assert: the result equals `Err(AdmitVerifierCompositionErrorReturn::
///   Incompatible { chain_forms: None, chain_member: None, encoding:
///   Some(build_encoding_identifier_disagreement(
///   EncodingIdentifierDisagreementOverrides { required:
///   Some(EncodingIdentifier::EthereumAbiV1), declared:
///   Some(EncodingIdentifier::Mock) })), verifier_form: None,
///   contract_suite: None })`.
#[test]
fn an_encoder_declaring_another_encoding_identifier_is_refused_naming_both_identifiers() {
    // Arrange
    let deps =
        build_admit_verifier_composition_deps(mock_lookup_verifier_declaration, Default::default());
    let params = build_admit_verifier_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(Default::default());
    let chain = build_chain_declaration(Default::default());
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::Mock),
        ..Default::default()
    });
    let pairing = build_pairing_declaration(Default::default());
    let generate = build_generate_declaration(Default::default());
    let payload = build_admit_verifier_composition_payload(
        &chain_forms,
        &chain,
        &encoder,
        &pairing,
        &generate,
        Default::default(),
    );

    // Act
    let result = admit_verifier_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitVerifierCompositionErrorReturn::Incompatible {
            chain_forms: None,
            chain_member: None,
            encoding: Some(build_encoding_identifier_disagreement(
                EncodingIdentifierDisagreementOverrides {
                    required: Some(EncodingIdentifier::EthereumAbiV1),
                    declared: Some(EncodingIdentifier::Mock),
                },
            )),
            verifier_form: None,
            contract_suite: None,
        })
    );
}
// an_encoder_declaring_another_encoding_identifier_is_refused_naming_both_identifiers

/// Contract: a pairing declaration carrying `VerifierGroupArithmetic::
///   BothGroups` while the handle's declaration serves `FirstGroupOnly`
///   alone, every other declaration at a required value →
///   `Err(Incompatible)` holding the verifier form disagreement and no
///   other.
/// Arrange: the test's own function `lookup_serving_first_group_only`
///   typed `LookupVerifierDeclarationFn`, composed from
///   `build_lookup_verifier_declaration_success_return` over
///   `LookupVerifierDeclarationSuccessReturnOverrides` holding
///   `build_verifier_declaration` with `verifier_forms`
///   `Some(&[VerifierGroupArithmetic::FirstGroupOnly])`, handed as the
///   handle; the pairing declaration built with
///   `verifier_group_arithmetic` `Some(VerifierGroupArithmetic::
///   BothGroups)`; the encoder declaration at `EthereumAbiV1`.
/// Act:    `admit_verifier_composition`.
/// Assert: the result equals `Err(AdmitVerifierCompositionErrorReturn::
///   Incompatible { chain_forms: None, chain_member: None, encoding:
///   None, verifier_form: Some(build_verifier_form_disagreement(
///   VerifierFormDisagreementOverrides { required:
///   Some(&[VerifierGroupArithmetic::FirstGroupOnly]), declared:
///   Some(VerifierGroupArithmetic::BothGroups) })), contract_suite:
///   None })`.
#[test]
fn a_pairing_form_the_member_does_not_serve_is_refused_naming_the_served_forms_and_the_declared_form()
 {
    // Arrange
    fn lookup_serving_first_group_only(
        _deps: &LookupVerifierDeclarationDeps,
        _params: LookupVerifierDeclarationParams,
        _payload: LookupVerifierDeclarationPayload,
    ) -> LookupVerifierDeclarationReturn {
        let _: LookupVerifierDeclarationFn = lookup_serving_first_group_only;
        Ok(build_lookup_verifier_declaration_success_return(
            LookupVerifierDeclarationSuccessReturnOverrides {
                declaration: Some(build_verifier_declaration(VerifierDeclarationOverrides {
                    verifier_forms: Some(&[VerifierGroupArithmetic::FirstGroupOnly]),
                    ..Default::default()
                })),
            },
        ))
    }

    let deps =
        build_admit_verifier_composition_deps(lookup_serving_first_group_only, Default::default());
    let params = build_admit_verifier_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(Default::default());
    let chain = build_chain_declaration(Default::default());
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
        ..Default::default()
    });
    let pairing = build_pairing_declaration(PairingDeclarationOverrides {
        verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        ..Default::default()
    });
    let generate = build_generate_declaration(Default::default());
    let payload = build_admit_verifier_composition_payload(
        &chain_forms,
        &chain,
        &encoder,
        &pairing,
        &generate,
        Default::default(),
    );

    // Act
    let result = admit_verifier_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitVerifierCompositionErrorReturn::Incompatible {
            chain_forms: None,
            chain_member: None,
            encoding: None,
            verifier_form: Some(build_verifier_form_disagreement(
                VerifierFormDisagreementOverrides {
                    required: Some(&[VerifierGroupArithmetic::FirstGroupOnly]),
                    declared: Some(VerifierGroupArithmetic::BothGroups),
                },
            )),
            contract_suite: None,
        })
    );
}
// a_pairing_form_the_member_does_not_serve_is_refused_naming_the_served_forms_and_the_declared_form

/// Contract: a generate declaration emitting for the contract suite
///   `other` while the handle's declaration verifies against `evm`,
///   every other declaration at a required value → `Err(Incompatible)`
///   holding the contract suite disagreement and no other.
/// Arrange: the generate declaration built with `contract_suite`
///   `Some("other")` and the encoder declaration at `EthereumAbiV1`, so
///   a refusal that names the declared suite as the required one, that
///   also names another disagreement, or that succeeds, fails.
/// Act:    `admit_verifier_composition`.
/// Assert: the result equals `Err(AdmitVerifierCompositionErrorReturn::
///   Incompatible { chain_forms: None, chain_member: None, encoding:
///   None, verifier_form: None, contract_suite: Some(
///   build_contract_suite_disagreement(
///   ContractSuiteDisagreementOverrides { required: Some("evm"),
///   declared: Some("other") })) })`.
#[test]
fn a_generate_member_emitting_for_another_contract_suite_is_refused_naming_both_suites() {
    // Arrange
    let deps =
        build_admit_verifier_composition_deps(mock_lookup_verifier_declaration, Default::default());
    let params = build_admit_verifier_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(Default::default());
    let chain = build_chain_declaration(Default::default());
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
        ..Default::default()
    });
    let pairing = build_pairing_declaration(Default::default());
    let generate = build_generate_declaration(GenerateDeclarationOverrides {
        contract_suite: Some("other"),
        ..Default::default()
    });
    let payload = build_admit_verifier_composition_payload(
        &chain_forms,
        &chain,
        &encoder,
        &pairing,
        &generate,
        Default::default(),
    );

    // Act
    let result = admit_verifier_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitVerifierCompositionErrorReturn::Incompatible {
            chain_forms: None,
            chain_member: None,
            encoding: None,
            verifier_form: None,
            contract_suite: Some(build_contract_suite_disagreement(
                ContractSuiteDisagreementOverrides {
                    required: Some("evm"),
                    declared: Some("other"),
                },
            )),
        })
    );
}
// a_generate_member_emitting_for_another_contract_suite_is_refused_naming_both_suites

/// Contract: a chain declaration carrying `ChainMemberIdentifier::Mock`,
///   an encoder declaration carrying `EncodingIdentifier::Mock`, and a
///   generate declaration emitting for `other` at once →
///   `Err(Incompatible)` holding the chain member, encoding, and
///   contract suite disagreements together and neither of the other two.
/// Arrange: those three declarations built with the three overrides and
///   the chain forms and pairing declarations at their defaults, so a
///   refusal that drops any disagreement, that names a met requirement,
///   or that succeeds, fails.
/// Act:    `admit_verifier_composition`.
/// Assert: the result equals `Err(AdmitVerifierCompositionErrorReturn::
///   Incompatible { chain_forms: None, chain_member: Some(…), encoding:
///   Some(…), verifier_form: None, contract_suite: Some(…) })`, each
///   `Some` the whole disagreement the three blocks above state.
#[test]
fn every_unmet_requirement_is_named_in_one_refusal() {
    // Arrange
    let deps =
        build_admit_verifier_composition_deps(mock_lookup_verifier_declaration, Default::default());
    let params = build_admit_verifier_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(Default::default());
    let chain = build_chain_declaration(ChainDeclarationOverrides {
        identifier: Some(ChainMemberIdentifier::Mock),
        ..Default::default()
    });
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::Mock),
        ..Default::default()
    });
    let pairing = build_pairing_declaration(Default::default());
    let generate = build_generate_declaration(GenerateDeclarationOverrides {
        contract_suite: Some("other"),
        ..Default::default()
    });
    let payload = build_admit_verifier_composition_payload(
        &chain_forms,
        &chain,
        &encoder,
        &pairing,
        &generate,
        Default::default(),
    );

    // Act
    let result = admit_verifier_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitVerifierCompositionErrorReturn::Incompatible {
            chain_forms: None,
            chain_member: Some(build_chain_member_identifier_disagreement(
                ChainMemberIdentifierDisagreementOverrides {
                    required: Some(ChainMemberIdentifier::Base),
                    declared: Some(ChainMemberIdentifier::Mock),
                },
            )),
            encoding: Some(build_encoding_identifier_disagreement(
                EncodingIdentifierDisagreementOverrides {
                    required: Some(EncodingIdentifier::EthereumAbiV1),
                    declared: Some(EncodingIdentifier::Mock),
                },
            )),
            verifier_form: None,
            contract_suite: Some(build_contract_suite_disagreement(
                ContractSuiteDisagreementOverrides {
                    required: Some("evm"),
                    declared: Some("other"),
                },
            )),
        })
    );
}
// every_unmet_requirement_is_named_in_one_refusal
