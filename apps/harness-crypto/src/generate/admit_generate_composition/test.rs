#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{ChainFormsDeclarationOverrides, ChainFormsIdentifier, build_chain_forms_declaration};
use encoding::{EncodingDeclarationOverrides, EncodingIdentifier, build_encoding_declaration};

use super::admit_generate_composition;
use super::interface::*;
use super::mock::*;

/// Contract: a chain forms declaration carrying `ChainFormsIdentifier::
///   EvmV1` and an encoder declaration carrying `EncodingIdentifier::
///   EthereumAbiV1`, the identifiers the handle's declaration requires →
///   `Ok(AdmitGenerateCompositionSuccessReturn)`.
/// Arrange: the encoder declaration built with `identifier`
///   `Some(EncodingIdentifier::EthereumAbiV1)` and the chain forms
///   declaration built with `identifier` `Some(ChainFormsIdentifier::
///   EvmV1)`, each identifier stated independently of the declaration the
///   lookup handle returns, so a comparison that read the payload's
///   identifiers back, or swapped the required and declared sides, fails.
/// Act:    `admit_generate_composition`.
/// Assert: the result equals `Ok(build_admit_generate_composition_
///   success_return(Default::default()))`.
#[test]
fn a_chain_forms_declaration_and_an_encoder_declaration_at_the_required_identifiers_are_admitted() {
    // Arrange
    let deps = build_admit_generate_composition_deps(Default::default());
    let params = build_admit_generate_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
        identifier: Some(ChainFormsIdentifier::EvmV1),
        ..Default::default()
    });
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::EthereumAbiV1),
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
// a_chain_forms_declaration_and_an_encoder_declaration_at_the_required_identifiers_are_admitted

/// Contract: a chain forms declaration at the required identifier and an
///   encoder declaration carrying `EncodingIdentifier::Mock` while the
///   handle's declaration requires `EncodingIdentifier::EthereumAbiV1` →
///   `Err(Incompatible)` holding the encoding disagreement and no chain
///   forms disagreement.
/// Arrange: the encoder declaration built with `identifier`
///   `Some(EncodingIdentifier::Mock)` and the chain forms declaration
///   built with `identifier` `Some(ChainFormsIdentifier::EvmV1)`, so a
///   refusal that names the declared identifier as the required one, that
///   also names a chain forms disagreement, or that succeeds, fails.
/// Act:    `admit_generate_composition`.
/// Assert: the result equals `Err(AdmitGenerateCompositionErrorReturn::
///   Incompatible { chain_forms: None, encoding: Some(
///   build_encoding_identifier_disagreement(EncodingIdentifierDisagreement
///   Overrides { required: Some(EncodingIdentifier::EthereumAbiV1),
///   declared: Some(EncodingIdentifier::Mock) })) })`.
#[test]
fn an_encoder_declaring_another_encoding_identifier_is_refused_naming_both_identifiers() {
    // Arrange
    let deps = build_admit_generate_composition_deps(Default::default());
    let params = build_admit_generate_composition_params(Default::default());
    let chain_forms = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
        identifier: Some(ChainFormsIdentifier::EvmV1),
        ..Default::default()
    });
    let encoder = build_encoding_declaration(EncodingDeclarationOverrides {
        identifier: Some(EncodingIdentifier::Mock),
        ..Default::default()
    });
    let payload =
        build_admit_generate_composition_payload(&chain_forms, &encoder, Default::default());

    // Act
    let result = admit_generate_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateCompositionErrorReturn::Incompatible {
            chain_forms: None,
            encoding: Some(build_encoding_identifier_disagreement(
                EncodingIdentifierDisagreementOverrides {
                    required: Some(EncodingIdentifier::EthereumAbiV1),
                    declared: Some(EncodingIdentifier::Mock),
                },
            )),
        })
    );
}
// an_encoder_declaring_another_encoding_identifier_is_refused_naming_both_identifiers
