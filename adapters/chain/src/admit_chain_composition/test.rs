#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_chain_composition;
use super::mock::*;
use crate::factory::provides::{
    ChainFormsDeclarationOverrides, ChainFormsIdentifier, build_chain_forms_declaration,
};
use crate::lookup_chain_declaration::provides::mock_lookup_chain_declaration;

/// Contract: a forms declaration carrying `ChainFormsIdentifier::EvmV1`,
///   the identifier the declaration the handle returns lists →
///   `Ok(AdmitChainCompositionSuccessReturn)`.
/// Arrange: the deps handed `mock_lookup_chain_declaration`, which
///   returns the declaration `build_chain_declaration` builds at its
///   defaults listing `ChainFormsIdentifier::EvmV1`; the forms
///   declaration built with `identifier`
///   `Some(ChainFormsIdentifier::EvmV1)`, an independent literal.
/// Act:     `admit_chain_composition`.
/// Assert:  the result equals `Ok` of the built success return.
#[test]
fn a_forms_declaration_at_the_identifier_the_member_lists_is_admitted() {
    // Arrange
    let deps =
        build_admit_chain_composition_deps(mock_lookup_chain_declaration, Default::default());
    let params = build_admit_chain_composition_params(Default::default());
    let declaration = build_chain_forms_declaration(ChainFormsDeclarationOverrides {
        identifier: Some(ChainFormsIdentifier::EvmV1),
        ..Default::default()
    });
    let payload = build_admit_chain_composition_payload(&declaration, Default::default());

    // Act
    let result = admit_chain_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_chain_composition_success_return(
            Default::default()
        ))
    );
}
// a_forms_declaration_at_the_identifier_the_member_lists_is_admitted
