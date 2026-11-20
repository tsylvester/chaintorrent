#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    ChainDeclarationOverrides, ChainFormsDeclarationOverrides, ChainMemberIdentifier,
    build_chain_declaration, build_chain_forms_declaration,
};
use encoding::{EncodingDeclarationOverrides, EncodingIdentifier, build_encoding_declaration};
use pairing::{PairingDeclarationOverrides, VerifierGroupArithmetic, build_pairing_declaration};

use crate::generate::provides::{GenerateDeclarationOverrides, build_generate_declaration};

use super::super::factory::provides::{VERIFIER_CONCRETES, VerifierConcreteKind};
use super::admit_verifier_composition;
use super::interface::*;
use super::mock::*;

/// Contract: for each member of the declared set and each verifier
///   form, a chain forms declaration at `ChainFormsIdentifier::EvmV1`,
///   a chain declaration carrying `ChainMemberIdentifier::Base`, an
///   encoder declaration carrying `EncodingIdentifier::EthereumAbiV1`,
///   a pairing declaration carrying the form, and a generate
///   declaration emitting for `evm` →
///   `Ok(AdmitVerifierCompositionSuccessReturn)`.
/// Boundary: `admit_verifier_composition` over
///   `ADMIT_VERIFIER_COMPOSITION_DEPS`, which holds the real
///   `lookup_verifier_declaration`.
/// Mocked:  none, the lookup reading constants alone.
/// Arrange: per member of `VERIFIER_CONCRETES` and per verifier form,
///   the encoder declaration built with `identifier`
///   `Some(EncodingIdentifier::EthereumAbiV1)`, the pairing declaration
///   built with `verifier_group_arithmetic` the form, and the other
///   declarations at their defaults, the required values stated as
///   independent literals and not read from the lookup, so a branch
///   that served one form only, or that read another member's
///   requirements, fails; the body names no concrete.
/// Act:    `admit_verifier_composition`, once per member and form.
/// Assert: the result equals `Ok(build_admit_verifier_composition_
///   success_return(Default::default()))` for every member and form.
#[test]
fn every_declared_member_is_admitted_against_the_values_it_requires() {
    for &identifier in VERIFIER_CONCRETES {
        for verifier_form in [
            VerifierGroupArithmetic::FirstGroupOnly,
            VerifierGroupArithmetic::BothGroups,
        ] {
            // Arrange
            let params =
                build_admit_verifier_composition_params(AdmitVerifierCompositionParamsOverrides {
                    identifier: Some(identifier),
                });
            let chain_forms =
                build_chain_forms_declaration(ChainFormsDeclarationOverrides::default());
            let chain = build_chain_declaration(Default::default());
            let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
                identifier: Some(EncodingIdentifier::EthereumAbiV1),
                ..Default::default()
            });
            let pairing = build_pairing_declaration(PairingDeclarationOverrides {
                verifier_group_arithmetic: Some(verifier_form),
                ..Default::default()
            });
            let generate = build_generate_declaration(GenerateDeclarationOverrides::default());
            let payload = build_admit_verifier_composition_payload(
                &chain_forms,
                &chain,
                &encoder,
                &pairing,
                &generate,
                Default::default(),
            );

            // Act
            let result =
                admit_verifier_composition(&ADMIT_VERIFIER_COMPOSITION_DEPS, params, payload);

            // Assert
            assert_eq!(
                result,
                Ok(build_admit_verifier_composition_success_return(
                    Default::default()
                ))
            );
        }
    }
}
// every_declared_member_is_admitted_against_the_values_it_requires

/// Contract: the identifier `VerifierConcreteKind::Evm` with a chain
///   declaration carrying `ChainMemberIdentifier::Mock` →
///   `Err(Incompatible)` holding `required` `ChainMemberIdentifier::
///   Base` and `declared` `ChainMemberIdentifier::Mock`; the identifier
///   `VerifierConcreteKind::Mock` with a chain declaration carrying
///   `ChainMemberIdentifier::Base` → `Err(Incompatible)` holding
///   `required` `ChainMemberIdentifier::Mock` and `declared`
///   `ChainMemberIdentifier::Base`; the identifier `VerifierConcreteKind
///   ::Mock` with a chain declaration carrying `ChainMemberIdentifier::
///   Mock` → `Ok`; each with no other disagreement.
/// Boundary: `admit_verifier_composition` over
///   `ADMIT_VERIFIER_COMPOSITION_DEPS`, which holds the real
///   `lookup_verifier_declaration`.
/// Mocked:  none, the lookup reading constants alone.
/// Arrange: the three pairs of member identifier, declared chain
///   member, and encoder identifier below, the encoder declaration
///   built with `identifier` the pair's third value — the named
///   member's own required encoding identifier, stated as an
///   independent literal, so no encoding disagreement appears beside
///   the chain member one — the chain declaration built with
///   `identifier` the pair's second value, and the other declarations
///   at their defaults.
/// Act:    `admit_verifier_composition`, once per pair.
/// Assert: each result equals the whole expected result the contract
///   states, the required identifiers stated as the independent
///   literals above and not read from the lookup.
#[test]
fn the_requirement_is_read_from_the_member_the_identifier_names() {
    let cases = [
        (
            VerifierConcreteKind::Evm,
            ChainMemberIdentifier::Mock,
            EncodingIdentifier::EthereumAbiV1,
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
            }),
        ),
        (
            VerifierConcreteKind::Mock,
            ChainMemberIdentifier::Base,
            EncodingIdentifier::Mock,
            Err(AdmitVerifierCompositionErrorReturn::Incompatible {
                chain_forms: None,
                chain_member: Some(build_chain_member_identifier_disagreement(
                    ChainMemberIdentifierDisagreementOverrides {
                        required: Some(ChainMemberIdentifier::Mock),
                        declared: Some(ChainMemberIdentifier::Base),
                    },
                )),
                encoding: None,
                verifier_form: None,
                contract_suite: None,
            }),
        ),
        (
            VerifierConcreteKind::Mock,
            ChainMemberIdentifier::Mock,
            EncodingIdentifier::Mock,
            Ok(build_admit_verifier_composition_success_return(
                Default::default(),
            )),
        ),
    ];
    for (identifier, chain_member, encoding_identifier, expected) in cases {
        // Arrange
        let params =
            build_admit_verifier_composition_params(AdmitVerifierCompositionParamsOverrides {
                identifier: Some(identifier),
            });
        let chain_forms = build_chain_forms_declaration(ChainFormsDeclarationOverrides::default());
        let chain = build_chain_declaration(ChainDeclarationOverrides {
            identifier: Some(chain_member),
            ..Default::default()
        });
        let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
            identifier: Some(encoding_identifier),
            ..Default::default()
        });
        let pairing = build_pairing_declaration(Default::default());
        let generate = build_generate_declaration(GenerateDeclarationOverrides::default());
        let payload = build_admit_verifier_composition_payload(
            &chain_forms,
            &chain,
            &encoder,
            &pairing,
            &generate,
            Default::default(),
        );

        // Act
        let result = admit_verifier_composition(&ADMIT_VERIFIER_COMPOSITION_DEPS, params, payload);

        // Assert
        assert_eq!(result, expected);
    }
}
// the_requirement_is_read_from_the_member_the_identifier_names

// Private-surface enumeration: the own entries
//   `every_declared_member_is_admitted_against_the_values_it_requires`
//   and `the_requirement_is_read_from_the_member_the_identifier_names`,
//   each proven by the block of the same name above.
