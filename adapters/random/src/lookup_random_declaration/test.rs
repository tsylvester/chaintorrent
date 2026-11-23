use super::interface::*;
use super::lookup_random_declaration;
use super::mock::*;
use crate::factory::provides::{
    MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_INTERFACE_VERSION,
    RandomSourceDeclarationOverrides, RandomSourceKind, build_random_source_declaration,
};

/// Contract: the identifier `RandomSourceKind::OperatingSystem` → `Ok` holding
///   the operating-system member's declaration.
/// Arrange: the identifier `OperatingSystem`, a member whose declaration names
///   itself as the source, so a branch that returned the mock concrete's
///   declaration, whose `source` is a `Mock` kind, fails.
/// Act:     `lookup_random_declaration`.
/// Assert:  the result equals `Ok` holding the success return built with the
///   declaration `build_random_source_declaration` returns for the overrides
///   `source` `RandomSourceKind::OperatingSystem`, `adapter_version` the
///   literal 1, and `interface_version` `RANDOM_SOURCE_INTERFACE_VERSION`, and
///   the `Ok` arm's declaration equals that built declaration.
#[test]
fn the_operating_system_source_returns_the_declaration_its_member_owns() {
    // Arrange
    let deps = build_lookup_random_declaration_deps(Default::default());
    let params = build_lookup_random_declaration_params(LookupRandomDeclarationParamsOverrides {
        identifier: Some(RandomSourceKind::OperatingSystem),
    });
    let payload = build_lookup_random_declaration_payload(Default::default());

    // Act
    let result: LookupRandomDeclarationReturn = lookup_random_declaration(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_lookup_random_declaration_success_return(
            LookupRandomDeclarationSuccessReturnOverrides {
                declaration: Some(build_random_source_declaration(
                    RandomSourceDeclarationOverrides {
                        source: Some(RandomSourceKind::OperatingSystem),
                        adapter_version: Some(1),
                        interface_version: Some(RANDOM_SOURCE_INTERFACE_VERSION),
                    }
                )),
            }
        ))
    );
    let Ok(success) = result;
    assert_eq!(
        success.declaration,
        build_random_source_declaration(RandomSourceDeclarationOverrides {
            source: Some(RandomSourceKind::OperatingSystem),
            adapter_version: Some(1),
            interface_version: Some(RANDOM_SOURCE_INTERFACE_VERSION),
        })
    );
}
// the_operating_system_source_returns_the_declaration_its_member_owns

/// Contract: the identifier `RandomSourceKind::Mock(mode)` → `Ok` holding a
///   declaration whose `source` is that identifier and whose versions are the
///   mock concrete's.
/// Arrange: the identifier
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`,
///   a mode that differs from the `Succeeds` mode the mock concrete's constant
///   holds, so a branch that returned the constant unchanged, or the
///   operating-system member's declaration, fails.
/// Act:     `lookup_random_declaration`.
/// Assert:  the result equals `Ok` holding the success return built with the
///   declaration `build_random_source_declaration` returns for the overrides
///   `source`
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`,
///   `adapter_version` the literal 1, and `interface_version`
///   `RANDOM_SOURCE_INTERFACE_VERSION`, and the `Ok` arm's declaration equals
///   that built declaration.
#[test]
fn the_mock_source_returns_a_declaration_naming_the_identifier_it_was_given() {
    // Arrange
    let deps = build_lookup_random_declaration_deps(Default::default());
    let params = build_lookup_random_declaration_params(LookupRandomDeclarationParamsOverrides {
        identifier: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let payload = build_lookup_random_declaration_payload(Default::default());

    // Act
    let result: LookupRandomDeclarationReturn = lookup_random_declaration(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_lookup_random_declaration_success_return(
            LookupRandomDeclarationSuccessReturnOverrides {
                declaration: Some(build_random_source_declaration(
                    RandomSourceDeclarationOverrides {
                        source: Some(RandomSourceKind::Mock(
                            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                        )),
                        adapter_version: Some(1),
                        interface_version: Some(RANDOM_SOURCE_INTERFACE_VERSION),
                    }
                )),
            }
        ))
    );
    let Ok(success) = result;
    assert_eq!(
        success.declaration,
        build_random_source_declaration(RandomSourceDeclarationOverrides {
            source: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
            )),
            adapter_version: Some(1),
            interface_version: Some(RANDOM_SOURCE_INTERFACE_VERSION),
        })
    );
}
// the_mock_source_returns_a_declaration_naming_the_identifier_it_was_given
