use super::provides::{
    LookupHashingDeclarationParamsOverrides, build_lookup_hashing_declaration_deps,
    build_lookup_hashing_declaration_params, build_lookup_hashing_declaration_payload,
    lookup_hashing_declaration,
};

use crate::factory::provides::{
    COMMITMENT_INTERFACE_VERSION, CommitmentConcrete, CommitmentSchemeIdentifier,
    MockICommitmentAdapterFailureMode,
};

/// Contract: `lookup_hashing_declaration`, member `Blake3Bao`: `params.concrete`
///   is `CommitmentConcrete::Blake3Bao` → `Ok` whose declaration is the real
///   member's declaration.
/// Arrange: `build_lookup_hashing_declaration_params` with the `concrete`
///   override `CommitmentConcrete::Blake3Bao`; the real member's chunk length
///   1024 differs from the mock constant's 64, so a lookup answering the mock
///   constant fails.
/// Act:     `lookup_hashing_declaration(&deps, params, payload)`.
/// Assert:  the success arm binds through the irrefutable pattern
///   `let Ok(success) = result;`; `declaration.concrete` equals
///   `CommitmentConcrete::Blake3Bao`; `declaration.identifier` equals
///   `CommitmentSchemeIdentifier::Blake3BaoV1`; `declaration.chunk_length`
///   equals the literal `1024`; `declaration.key_length` equals the literal
///   `32`; `declaration.adapter_version` equals the literal `1`;
///   `declaration.interface_version` equals `COMMITMENT_INTERFACE_VERSION`.
#[test]
fn lookup_returns_the_blake3_bao_declaration() {
    // Arrange
    let deps = build_lookup_hashing_declaration_deps(Default::default());
    let params = build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
        concrete: Some(CommitmentConcrete::Blake3Bao),
    });
    let payload = build_lookup_hashing_declaration_payload(Default::default());

    // Act
    let result = lookup_hashing_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(success.declaration.concrete, CommitmentConcrete::Blake3Bao);
    assert_eq!(
        success.declaration.identifier,
        CommitmentSchemeIdentifier::Blake3BaoV1
    );
    assert_eq!(success.declaration.chunk_length, 1024);
    assert_eq!(success.declaration.key_length, 32);
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(
        success.declaration.interface_version,
        COMMITMENT_INTERFACE_VERSION
    );
}

/// Contract: `lookup_hashing_declaration`, member `Mock`: `params.concrete` is
///   `CommitmentConcrete::Mock(failure_mode)` → `Ok` whose declaration is the
///   mock constant's declaration with `concrete` the selected member.
/// Arrange: `build_lookup_hashing_declaration_params` with the `concrete`
///   override
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`,
///   a mode other than the `NoFailure` the constant holds, so a lookup that
///   returns the constant unsubstituted fails.
/// Act:     `lookup_hashing_declaration(&deps, params, payload)`.
/// Assert:  `declaration.concrete` equals
///   `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)`;
///   `declaration.identifier` equals `CommitmentSchemeIdentifier::Blake3BaoV1`;
///   `declaration.chunk_length` equals the literal `64`;
///   `declaration.key_length` equals the literal `32`;
///   `declaration.adapter_version` equals the literal `1`;
///   `declaration.interface_version` equals `COMMITMENT_INTERFACE_VERSION`.
#[test]
fn lookup_returns_the_mock_declaration_naming_the_selected_failure_mode() {
    // Arrange
    let deps = build_lookup_hashing_declaration_deps(Default::default());
    let params = build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
        concrete: Some(CommitmentConcrete::Mock(
            MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable,
        )),
    });
    let payload = build_lookup_hashing_declaration_payload(Default::default());

    // Act
    let result = lookup_hashing_declaration(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert_eq!(
        success.declaration.concrete,
        CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::ContentLengthUnrepresentable)
    );
    assert_eq!(
        success.declaration.identifier,
        CommitmentSchemeIdentifier::Blake3BaoV1
    );
    assert_eq!(success.declaration.chunk_length, 64);
    assert_eq!(success.declaration.key_length, 32);
    assert_eq!(success.declaration.adapter_version, 1);
    assert_eq!(
        success.declaration.interface_version,
        COMMITMENT_INTERFACE_VERSION
    );
}
