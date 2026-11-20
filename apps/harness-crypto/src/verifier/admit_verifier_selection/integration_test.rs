#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use serde_json;

use super::super::factory::provides::{
    VERIFIER_CONCRETES, VerifierConcreteOverrides, build_verifier_concrete,
};
use super::admit_verifier_selection;
use super::interface::*;
use super::mock::*;

/// Contract: each member of `VERIFIER_CONCRETES` with a payload holding no
///   key → `Ok` whose selection equals `build_verifier_concrete` for that
///   member.
/// Arrange: for each identifier of `VERIFIER_CONCRETES`, the deps
///   `&ADMIT_VERIFIER_SELECTION_DEPS` and the params from their builder
///   naming the identifier, and the payload `serde_json::json!({})`.
/// Act:    `admit_verifier_selection`, once per identifier.
/// Assert: the result equals `Ok` of the success return built over the
///   selection `build_verifier_concrete` returns for that member.
/// Boundary: `admit_verifier_selection` over
///   `ADMIT_VERIFIER_SELECTION_DEPS`, the document guard, the real
///   `lookup_verifier_declaration`, and the real
///   `admit_evm_verifier_settings`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn every_declared_member_is_selected_at_its_declared_defaults_by_the_real_chain() {
    for &identifier in VERIFIER_CONCRETES {
        // Arrange
        let deps = &ADMIT_VERIFIER_SELECTION_DEPS;
        let params = build_admit_verifier_selection_params(AdmitVerifierSelectionParamsOverrides {
            identifier: Some(identifier),
        });
        let payload = serde_json::json!({});

        // Act
        let result = admit_verifier_selection(deps, params, payload);

        // Assert
        assert_eq!(
            result,
            Ok(build_admit_verifier_selection_success_return(
                AdmitVerifierSelectionSuccessReturnOverrides {
                    selection: Some(build_verifier_concrete(VerifierConcreteOverrides {
                        kind: Some(identifier),
                        ..Default::default()
                    })),
                },
            ))
        );
    }
}
// every_declared_member_is_selected_at_its_declared_defaults_by_the_real_chain
