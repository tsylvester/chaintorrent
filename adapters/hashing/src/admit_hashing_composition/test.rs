use super::provides::{
    AdmitHashingCompositionParamsOverrides, AdmitHashingCompositionSuccessReturn,
    admit_hashing_composition, build_admit_hashing_composition_deps,
    build_admit_hashing_composition_params, build_admit_hashing_composition_payload,
};

use crate::factory::provides::{
    CommitmentConcrete, CommitmentSchemeIdentifier, MockICommitmentAdapterFailureMode,
};

/// Contract: `admit_hashing_composition`: the declaration the lookup returns
///   holds an `identifier` equal to `params.identifier` →
///   `Ok(AdmitHashingCompositionSuccessReturn)`.
/// Arrange: `build_admit_hashing_composition_deps` with no override, so the
///   lookup is `mock_lookup_hashing_declaration`, whose declaration's
///   identifier is the real `CommitmentSchemeIdentifier::Blake3BaoV1`;
///   `build_admit_hashing_composition_params` with the `identifier` override
///   `CommitmentSchemeIdentifier::Blake3BaoV1`, stated as an independent
///   literal.
/// Act:     `admit_hashing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Ok(AdmitHashingCompositionSuccessReturn)`.
#[test]
fn admit_returns_ok_when_the_required_identifier_equals_the_declared_identifier() {
    // Arrange
    let deps = build_admit_hashing_composition_deps(Default::default());
    let params = build_admit_hashing_composition_params(AdmitHashingCompositionParamsOverrides {
        identifier: Some(CommitmentSchemeIdentifier::Blake3BaoV1),
        ..Default::default()
    });
    let payload = build_admit_hashing_composition_payload(Default::default());

    // Act
    let result = admit_hashing_composition(&deps, params, payload);

    // Assert
    assert_eq!(result, Ok(AdmitHashingCompositionSuccessReturn));
}

/// Contract: `admit_hashing_composition`, member `Mock`: `params.concrete` is
///   `CommitmentConcrete::Mock(failure_mode)` and the declaration the lookup
///   returns holds an `identifier` equal to `params.identifier` →
///   `Ok(AdmitHashingCompositionSuccessReturn)`.
/// Arrange: `build_admit_hashing_composition_deps` with no override;
///   `build_admit_hashing_composition_params` with the `concrete` override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`,
///   a member other than the real member and a mode other than the `NoFailure`
///   the family's mock constant holds, and the `identifier` override
///   `CommitmentSchemeIdentifier::Blake3BaoV1`, stated as an independent
///   literal.
/// Act:     `admit_hashing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Ok(AdmitHashingCompositionSuccessReturn)`.
#[test]
fn admit_returns_ok_for_the_mock_member_naming_the_required_identifier() {
    // Arrange
    let deps = build_admit_hashing_composition_deps(Default::default());
    let params = build_admit_hashing_composition_params(AdmitHashingCompositionParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable,
        )),
        identifier: Some(CommitmentSchemeIdentifier::Blake3BaoV1),
    });
    let payload = build_admit_hashing_composition_payload(Default::default());

    // Act
    let result = admit_hashing_composition(&deps, params, payload);

    // Assert
    assert_eq!(result, Ok(AdmitHashingCompositionSuccessReturn));
}
