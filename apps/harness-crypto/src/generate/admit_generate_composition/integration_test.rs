#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{ChainFormsDeclarationOverrides, ChainFormsIdentifier, build_chain_forms_declaration};
use encoding::{EncodingDeclarationOverrides, EncodingIdentifier, build_encoding_declaration};

use super::super::factory::provides::{GENERATE_CONCRETES, GenerateConcreteKind};
use super::super::lookup_generate_declaration::provides::{
    LookupGenerateDeclarationDeps, LookupGenerateDeclarationParams,
    LookupGenerateDeclarationPayload, lookup_generate_declaration,
};
use super::admit_generate_composition;
use super::interface::*;
use super::mock::*;

/// Contract: for each member of the declared set, a chain forms
///   declaration carrying the member's `chain_forms_identifier` and an
///   encoder declaration carrying its `encoding_identifier` →
///   `Ok(AdmitGenerateCompositionSuccessReturn)`.
/// Arrange: per member of `GENERATE_CONCRETES`, the member's declaration
///   read through the real `lookup_generate_declaration`, the chain forms
///   declaration built with `identifier` the declaration's
///   `chain_forms_identifier`, the encoder declaration built with
///   `identifier` the declaration's `encoding_identifier`, the deps built
///   with the real `lookup_generate_declaration` as the handle, and the
///   params naming the member; the body names no concrete.
/// Act:    `admit_generate_composition`, once per member.
/// Assert: the result equals `Ok(build_admit_generate_composition_
///   success_return(Default::default()))` for every member.
/// Boundary: `admit_generate_composition` over the real
///   `lookup_generate_declaration`.
/// Mocked:  nothing, the lookup reading constants alone.
#[test]
fn every_declared_member_is_admitted_against_the_identifiers_it_requires() {
    for &identifier in GENERATE_CONCRETES {
        // Arrange
        let deps = build_admit_generate_composition_deps(AdmitGenerateCompositionDepsOverrides {
            lookup_generate_declaration: Some(lookup_generate_declaration),
        });
        let params =
            build_admit_generate_composition_params(AdmitGenerateCompositionParamsOverrides {
                identifier: Some(identifier),
            });
        let Ok(looked_up) = lookup_generate_declaration(
            &LookupGenerateDeclarationDeps,
            LookupGenerateDeclarationParams { identifier },
            LookupGenerateDeclarationPayload,
        );
        let chain_forms = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
            identifier: Some(looked_up.declaration.chain_forms_identifier),
            ..Default::default()
        });
        let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
            identifier: Some(looked_up.declaration.encoding_identifier),
            ..Default::default()
        });
        let payload =
            build_admit_generate_composition_payload(&chain_forms, &encoder, Default::default());

        // Act
        let result = admit_generate_composition(&deps, params, payload);

        // Assert
        assert_eq!(
            result,
            Ok(build_admit_generate_composition_success_return(
                Default::default()
            ))
        );
    }
}
// every_declared_member_is_admitted_against_the_identifiers_it_requires

/// Contract: the identifier `GenerateConcreteKind::Solidity` with an
///   encoder declaring `EncodingIdentifier::Mock` → `Err(Incompatible)`
///   holding `required` `EncodingIdentifier::EthereumAbiV1` and
///   `declared` `EncodingIdentifier::Mock`; the identifier
///   `GenerateConcreteKind::Mock` with an encoder declaring
///   `EncodingIdentifier::EthereumAbiV1` → `Err(Incompatible)` holding
///   `required` `EncodingIdentifier::Mock` and `declared`
///   `EncodingIdentifier::EthereumAbiV1`; the identifier
///   `GenerateConcreteKind::Mock` with an encoder declaring
///   `EncodingIdentifier::Mock` → `Ok`; each with no chain forms
///   disagreement.
/// Arrange: the three pairs of member identifier and encoder identifier
///   below, each with the chain forms declaration carrying
///   `ChainFormsIdentifier::EvmV1`, the encoder declaration built with
///   `identifier` the pair's value, the deps built with the real
///   `lookup_generate_declaration` as the handle, and the params naming
///   the pair's member.
/// Act:    `admit_generate_composition`, once per pair.
/// Assert: each result equals the whole expected result the contract
///   states, the required identifiers stated as independent literals and
///   not read from the lookup.
/// Boundary: `admit_generate_composition` over the real
///   `lookup_generate_declaration`.
/// Mocked:  nothing, the lookup reading constants alone.
#[test]
fn the_requirement_is_read_from_the_member_the_identifier_names() {
    let cases = [
        (
            GenerateConcreteKind::Solidity,
            EncodingIdentifier::Mock,
            Err(AdmitGenerateCompositionErrorReturn::Incompatible {
                chain_forms: None,
                encoding: Some(build_encoding_identifier_disagreement(
                    EncodingIdentifierDisagreementOverrides {
                        required: Some(EncodingIdentifier::EthereumAbiV1),
                        declared: Some(EncodingIdentifier::Mock),
                    },
                )),
            }),
        ),
        (
            GenerateConcreteKind::Mock,
            EncodingIdentifier::EthereumAbiV1,
            Err(AdmitGenerateCompositionErrorReturn::Incompatible {
                chain_forms: None,
                encoding: Some(build_encoding_identifier_disagreement(
                    EncodingIdentifierDisagreementOverrides {
                        required: Some(EncodingIdentifier::Mock),
                        declared: Some(EncodingIdentifier::EthereumAbiV1),
                    },
                )),
            }),
        ),
        (
            GenerateConcreteKind::Mock,
            EncodingIdentifier::Mock,
            Ok(build_admit_generate_composition_success_return(
                Default::default(),
            )),
        ),
    ];
    for (identifier, encoding_identifier, expected) in cases {
        // Arrange
        let deps = build_admit_generate_composition_deps(AdmitGenerateCompositionDepsOverrides {
            lookup_generate_declaration: Some(lookup_generate_declaration),
        });
        let params =
            build_admit_generate_composition_params(AdmitGenerateCompositionParamsOverrides {
                identifier: Some(identifier),
            });
        let chain_forms = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
            identifier: Some(ChainFormsIdentifier::EvmV1),
            ..Default::default()
        });
        let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
            identifier: Some(encoding_identifier),
            ..Default::default()
        });
        let payload =
            build_admit_generate_composition_payload(&chain_forms, &encoder, Default::default());

        // Act
        let result = admit_generate_composition(&deps, params, payload);

        // Assert
        assert_eq!(result, expected);
    }
}
// the_requirement_is_read_from_the_member_the_identifier_names

// Private-surface enumeration: the own entries
//   `every_declared_member_is_admitted_against_the_identifiers_it_requires`
//   and `the_requirement_is_read_from_the_member_the_identifier_names`,
//   each proven by the block of the same name above, and the absorbed
//   `lookup_generate_declaration` entry `Own entries`, proven by
//   `the_requirement_is_read_from_the_member_the_identifier_names`.
