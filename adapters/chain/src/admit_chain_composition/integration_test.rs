#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_chain_composition;
use super::interface::*;
use super::mock::*;
use crate::factory::provides::{
    ChainFormsDeclarationOverrides, ChainFormsIdentifier, ChainMemberIdentifier,
    DECLARED_CHAIN_MEMBERS, build_chain_forms_declaration,
};

/// Contract: for each member of the declared set, a forms declaration
///   carrying `ChainFormsIdentifier::EvmV1`, the identifier that member's
///   declaration lists → `Ok(AdmitChainCompositionSuccessReturn)`.
/// Boundary: `admit_chain_composition` over
///   `ADMIT_CHAIN_COMPOSITION_DEPS`, which holds the real
///   `lookup_chain_declaration`.
/// Mocked:  none, the real lookup reading constants alone.
/// Arrange: per member the forms declaration built with `identifier`
///   `Some(ChainFormsIdentifier::EvmV1)`, an independent literal and not
///   read from the lookup.
/// Act:     `admit_chain_composition`.
/// Assert:  the result equals `Ok` of the built success return for every
///   member.
#[test]
fn every_declared_member_is_admitted_against_the_forms_identifier_it_lists() {
    for &identifier in DECLARED_CHAIN_MEMBERS {
        // Arrange
        let params = build_admit_chain_composition_params(AdmitChainCompositionParamsOverrides {
            identifier: Some(identifier),
        });
        let declaration = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
            identifier: Some(ChainFormsIdentifier::EvmV1),
            ..Default::default()
        });
        let payload = build_admit_chain_composition_payload(&declaration, Default::default());

        // Act
        let result = admit_chain_composition(&ADMIT_CHAIN_COMPOSITION_DEPS, params, payload);

        // Assert
        assert_eq!(
            result,
            Ok(build_admit_chain_composition_success_return(
                Default::default()
            ))
        );
    }
}
// every_declared_member_is_admitted_against_the_forms_identifier_it_lists

/// Contract: the identifier `ChainMemberIdentifier::Mock` with a forms
///   declaration carrying `ChainFormsIdentifier::EvmV1` →
///   `Ok(AdmitChainCompositionSuccessReturn)`.
/// Boundary: `admit_chain_composition` over
///   `ADMIT_CHAIN_COMPOSITION_DEPS`, which holds the real
///   `lookup_chain_declaration`.
/// Mocked:  none, the real lookup reading constants alone.
/// Arrange: the params built with `identifier`
///   `Some(ChainMemberIdentifier::Mock)` and the forms declaration built
///   with `identifier` `Some(ChainFormsIdentifier::EvmV1)`, an
///   independent literal.
/// Act:     `admit_chain_composition`.
/// Assert:  the result equals `Ok` of the built success return.
#[test]
fn the_mock_member_is_admitted_against_the_forms_identifier_it_lists() {
    // Arrange
    let params = build_admit_chain_composition_params(AdmitChainCompositionParamsOverrides {
        identifier: Some(ChainMemberIdentifier::Mock),
    });
    let declaration = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
        identifier: Some(ChainFormsIdentifier::EvmV1),
        ..Default::default()
    });
    let payload = build_admit_chain_composition_payload(&declaration, Default::default());

    // Act
    let result = admit_chain_composition(&ADMIT_CHAIN_COMPOSITION_DEPS, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_chain_composition_success_return(
            Default::default()
        ))
    );
}
// the_mock_member_is_admitted_against_the_forms_identifier_it_lists

// Private-surface enumeration: the own entries
//   `every_declared_member_is_admitted_against_the_forms_identifier_it_lists`
//   and `the_mock_member_is_admitted_against_the_forms_identifier_it_lists`,
//   each proven by the block of the same name above.
