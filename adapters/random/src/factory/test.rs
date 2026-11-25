#![allow(clippy::expect_used)]

use super::provides::{
    CreateRandomSourceParamsOverrides, MockIRandomSourceAdapterFailureMode,
    MockIRandomSourceConsumer, RANDOM_SOURCE_KINDS, RandomSourceKind,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_random_source_kind, create_random_source,
};

/// Contract: `create_random_source` is called with `params.kind` the
///   selection `build_random_source_kind` returns for a member of
///   `RANDOM_SOURCE_KINDS` → `Ok(CreateRandomSourceSuccessReturn { output })`
///   relaying the consumer's output, the factory handing the constructed
///   source to the consumer in `deps`.
/// Collaborators: the `try_new` of the concrete each member names, which
///   the factory constructs, runs for real; `MockIRandomSourceConsumer` in
///   `deps`, receiving each constructed source; fixtures
///   `build_create_random_source_params`, `build_create_random_source_deps`,
///   `build_create_random_source_payload`, and `build_random_source_kind`.
/// Arrange: the block's body written once and run over each member of
///   `RANDOM_SOURCE_KINDS`, `build_create_random_source_params` with the
///   kind override set to `build_random_source_kind(member,
///   Default::default())`, and `build_create_random_source_deps` with
///   `MockIRandomSourceConsumer` as its leading parameter.
/// Act:     `create_random_source(&deps, params,
///   build_create_random_source_payload(Default::default()))` once for each
///   member's selection.
/// Assert:  each success arm is extracted with `expect`; its `output` is
///   the `()` `MockIRandomSourceConsumer` returned, proving the consumer's
///   output is the factory's success field and no adapter is returned.
#[test]
fn every_member_of_the_declared_set_constructs_the_selected_source_and_returns_the_consumers_output()
 {
    for member in RANDOM_SOURCE_KINDS {
        // Arrange
        let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(build_random_source_kind(*member, Default::default())),
        });
        let deps = build_create_random_source_deps(MockIRandomSourceConsumer, Default::default());

        // Act
        let result = create_random_source(
            &deps,
            params,
            build_create_random_source_payload(Default::default()),
        );

        // Assert
        let success = result.expect("every member of the declared set constructs its source");
        assert_eq!(success.output, ());
    }
}
// every_member_of_the_declared_set_constructs_the_selected_source_and_returns_the_consumers_output

/// Contract: `params.kind` is `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)` →
///   `Ok(CreateRandomSourceSuccessReturn { output })` relaying the
///   consumer's output, the refusal being a draw behavior the integration
///   coverage proves, not a constructor failure.
/// Collaborators: `MockIRandomSourceAdapter::try_new`, the concrete the
///   factory constructs, runs for real; `MockIRandomSourceConsumer` in
///   `deps`; fixtures `build_create_random_source_params`,
///   `build_create_random_source_deps`, and
///   `build_create_random_source_payload`.
/// Arrange: `build_create_random_source_params` with the kind override set
///   to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, a mode that differs from the builder's default
///   `Succeeds`, and `build_create_random_source_deps` with
///   `MockIRandomSourceConsumer` as its leading parameter.
/// Act:     `create_random_source(&deps, params,
///   build_create_random_source_payload(Default::default()))`.
/// Assert:  the success arm is extracted with `expect`; its `output` is
///   the `()` `MockIRandomSourceConsumer` returned.
#[test]
fn create_random_source_returns_the_consumers_output_for_the_mock_selections_refused_mode() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
        )),
    });
    let deps = build_create_random_source_deps(MockIRandomSourceConsumer, Default::default());

    // Act
    let result = create_random_source(
        &deps,
        params,
        build_create_random_source_payload(Default::default()),
    );

    // Assert
    let success = result.expect("the refused-mode mock selection constructs its source");
    assert_eq!(success.output, ());
}
// create_random_source_returns_the_consumers_output_for_the_mock_selections_refused_mode

/// Contract: `params.kind` is `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::Succeeds)` →
///   `Ok(CreateRandomSourceSuccessReturn { output })` relaying the
///   consumer's output.
/// Collaborators: `MockIRandomSourceAdapter::try_new`, the concrete the
///   factory constructs, runs for real; `MockIRandomSourceConsumer` in
///   `deps`; fixtures `build_create_random_source_params`,
///   `build_create_random_source_deps`, and
///   `build_create_random_source_payload`.
/// Arrange: `build_create_random_source_params` with the kind override set
///   to `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   Succeeds)`, and `build_create_random_source_deps` with
///   `MockIRandomSourceConsumer` as its leading parameter.
/// Act:     `create_random_source(&deps, params,
///   build_create_random_source_payload(Default::default()))`.
/// Assert:  the success arm is extracted with `expect`; its `output` is
///   the `()` `MockIRandomSourceConsumer` returned.
#[test]
fn create_random_source_returns_the_consumers_output_for_the_mock_selections_succeeds_mode() {
    // Arrange
    let params = build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::Mock(
            MockIRandomSourceAdapterFailureMode::Succeeds,
        )),
    });
    let deps = build_create_random_source_deps(MockIRandomSourceConsumer, Default::default());

    // Act
    let result = create_random_source(
        &deps,
        params,
        build_create_random_source_payload(Default::default()),
    );

    // Assert
    let success = result.expect("the succeeds-mode mock selection constructs its source");
    assert_eq!(success.output, ());
}
// create_random_source_returns_the_consumers_output_for_the_mock_selections_succeeds_mode
