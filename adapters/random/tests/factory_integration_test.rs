#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

// This integration test exercises the public surface of the crate.

use random::{
    AdmitRandomSelectionParamsOverrides, CreateRandomSourceParamsOverrides, FillBytesErrorReturn,
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RANDOM_SOURCE_KINDS, RandomSourceKind,
    admit_random_selection, build_admit_random_selection_deps, build_admit_random_selection_params,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, build_fill_bytes_payload, create_random_source,
};
use serde_json::{Map, Value};
use std::collections::HashSet;

/// Contract: own entry
///   `a_source_the_factory_constructs_for_each_declared_member_draws_the_requested_bytes_from_the_generator` —
///   `create_random_source` is called by a caller outside the crate with the
///   selection admitted for each member of `RANDOM_SOURCE_KINDS`, and
///   `fill_bytes` is called on the returned source for a length of 1024 →
///   `Ok` whose `bytes` hold exactly 1024 bytes, whose 32-byte segments are
///   pairwise distinct.
/// Boundary: the crate's public surface; the real chain
///   `admit_random_selection` → `create_random_source` → the member's
///   `try_new` → `IRandomSourceAdapter::fill_bytes` → the member's
///   generator.
/// Mocked:  nothing; each member's generator is its outer edge and runs
///   real, so this test proves the generator's output reaches the caller
///   and does not prove the generator's quality.
/// Arrange: for each member of `RANDOM_SOURCE_KINDS` in turn, the selection
///   admitted from the document `Value::Null`, the adapter returned by
///   `create_random_source` for it with
///   `build_create_random_source_deps(Default::default())` and
///   `build_create_random_source_payload(Default::default())` as its deps
///   and payload; `build_fill_bytes_payload` with the length override set
///   to 1024, which splits into segments of 32 bytes and differs from the
///   builder's default of 32.
/// Act:     `fill_bytes(FillBytesParams, payload)` on the returned adapter.
/// Assert:  the success arm is extracted with `expect`; the length of
///   `bytes.expose()` equals the literal 1024 written in the assertion; the
///   32-byte segments of `bytes.expose()` collected into a `HashSet` number
///   the literal 32 written in the assertion.
#[test]
fn a_source_from_the_factory_draws_random_bytes_through_the_family_trait() {
    for member in RANDOM_SOURCE_KINDS {
        // Arrange
        let selection = admit_random_selection(
            &build_admit_random_selection_deps(Default::default()),
            build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
                identifier: Some(*member),
            }),
            Value::Null,
        )
        .expect("a declared member admits a document supplying no setting")
        .selection;
        let adapter: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(selection),
            }),
            build_create_random_source_payload(Default::default()),
        )
        .expect("the admitted selection constructs its source")
        .adapter;
        let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(1024) });

        // Act
        let result = adapter.fill_bytes(FillBytesParams, payload);

        // Assert
        let success = result.expect("the member's source fills the buffer");
        assert_eq!(success.bytes.expose().len(), 1024);
        let segments: HashSet<&[u8]> = success.bytes.expose().chunks(32).collect();
        assert_eq!(segments.len(), 32);
    }
}
// a_source_from_the_factory_draws_random_bytes_through_the_family_trait

/// Contract: own entry
///   `a_source_the_factory_constructs_draws_an_empty_buffer_for_a_zero_length` —
///   `create_random_source` is called by a caller outside the crate with the
///   selection admitted for each member of `RANDOM_SOURCE_KINDS`, and
///   `fill_bytes` is called on the returned source for a length of zero →
///   `Ok` whose `bytes` are empty.
/// Boundary: the crate's public surface; the real chain
///   `admit_random_selection` → `create_random_source` → the member's
///   `try_new` → `IRandomSourceAdapter::fill_bytes` → the member's
///   generator.
/// Mocked:  nothing; each member's generator is its outer edge and runs
///   real.
/// Arrange: for each member of `RANDOM_SOURCE_KINDS` in turn, the selection
///   admitted from the document `Value::Null`, the adapter returned by
///   `create_random_source` for it with
///   `build_create_random_source_deps(Default::default())` and
///   `build_create_random_source_payload(Default::default())` as its deps
///   and payload; `build_fill_bytes_payload` with the length override set
///   to 0, which differs from the builder's default of 32.
/// Act:     `fill_bytes(FillBytesParams, payload)` on the returned adapter.
/// Assert:  the success arm is extracted with `expect`; `bytes.expose()` is
///   empty.
#[test]
fn a_source_from_the_factory_draws_an_empty_buffer_for_a_zero_length() {
    for member in RANDOM_SOURCE_KINDS {
        // Arrange
        let selection = admit_random_selection(
            &build_admit_random_selection_deps(Default::default()),
            build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
                identifier: Some(*member),
            }),
            Value::Null,
        )
        .expect("a declared member admits a document supplying no setting")
        .selection;
        let adapter: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(selection),
            }),
            build_create_random_source_payload(Default::default()),
        )
        .expect("the admitted selection constructs its source")
        .adapter;
        let payload = build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(0) });

        // Act
        let result = adapter.fill_bytes(FillBytesParams, payload);

        // Assert
        let success = result.expect("the member's source fills the buffer");
        assert!(success.bytes.expose().is_empty());
    }
}
// a_source_from_the_factory_draws_an_empty_buffer_for_a_zero_length

/// Contract: carried entry
///   `every_declared_member_is_admitted_to_the_selection_naming_it` of
///   `random/admit_random_selection`, as this node restates it — the
///   selection admitted for each member of `RANDOM_SOURCE_KINDS`, from a
///   document that is `Value::Null` and from a document that is an empty
///   `Value::Object`, is the selection `create_random_source` constructs
///   that member's real source from, so two sources built from the two
///   admitted selections draw 32-byte buffers that differ.
/// Boundary: the crate's public surface; the real chain
///   `admit_random_selection` → `create_random_source` → the member's
///   `try_new` → `IRandomSourceAdapter::fill_bytes` → the member's
///   generator.
/// Mocked:  nothing; each member's generator is its outer edge and runs
///   real.
/// Arrange: for each member of `RANDOM_SOURCE_KINDS` in turn, two
///   selections admitted for it, one from the document `Value::Null` and
///   one from the document `Value::Object` over an empty `Map`, and the
///   adapter returned by `create_random_source` for each, a pair of fresh
///   sources whose first draws a mock concrete's fixed counter would make
///   equal; `build_fill_bytes_payload` at its default length of 32.
/// Act:     `fill_bytes(FillBytesParams, payload)` once on each of the two
///   adapters.
/// Assert:  both success arms are extracted with `expect`; the
///   `bytes.expose()` of the first adapter differs from the
///   `bytes.expose()` of the second under `assert_ne!`.
#[test]
fn a_selection_admitted_for_each_declared_member_constructs_the_source_that_member_names() {
    for member in RANDOM_SOURCE_KINDS {
        // Arrange
        let null_selection = admit_random_selection(
            &build_admit_random_selection_deps(Default::default()),
            build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
                identifier: Some(*member),
            }),
            Value::Null,
        )
        .expect("a declared member admits a document supplying no setting")
        .selection;
        let object_selection = admit_random_selection(
            &build_admit_random_selection_deps(Default::default()),
            build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
                identifier: Some(*member),
            }),
            Value::Object(Map::new()),
        )
        .expect("a declared member admits a document supplying no setting")
        .selection;
        let first: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(null_selection),
            }),
            build_create_random_source_payload(Default::default()),
        )
        .expect("the admitted selection constructs its source")
        .adapter;
        let second: Box<dyn IRandomSourceAdapter> = create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(object_selection),
            }),
            build_create_random_source_payload(Default::default()),
        )
        .expect("the admitted selection constructs its source")
        .adapter;

        // Act
        let first_result = first.fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(Default::default()),
        );
        let second_result = second.fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(Default::default()),
        );

        // Assert
        let first_success = first_result.expect("the member's source fills the buffer");
        let second_success = second_result.expect("the member's source fills the buffer");
        assert_ne!(first_success.bytes.expose(), second_success.bytes.expose());
    }
}
// a_selection_admitted_for_each_declared_member_constructs_the_source_that_member_names

/// Contract: carried entry
///   `the_mock_member_is_admitted_to_the_selection_carrying_its_failure_mode` of
///   `random/admit_random_selection`, as this node restates it — the
///   selection admitted for
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)` constructs a source whose `fill_bytes` returns
///   `Err(FillBytesErrorReturn::MockIRandomSourceAdapter)`, and the
///   selection admitted for `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::Succeeds)` constructs a source
///   whose `fill_bytes` returns the bytes requested.
/// Boundary: the crate's public surface; the real chain
///   `admit_random_selection` → `create_random_source` → the mock
///   selection's `try_new` → `IRandomSourceAdapter::fill_bytes`.
/// Mocked:  nothing; the mock concrete is the family's own.
/// Arrange: the selection admitted for `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)` from the
///   document `Value::Null`, and the selection admitted for
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   Succeeds)` from the document `Value::Object` over an empty `Map`, the
///   two modes differing so a factory that fixes one mode fails; the
///   adapter returned by `create_random_source` for each;
///   `build_fill_bytes_payload` with the length override set to 48, which
///   differs from the builder's default of 32.
/// Act:     `fill_bytes(FillBytesParams, payload)` on each of the two
///   adapters.
/// Assert:  the result of the `FillBytesRefused` adapter has an `err()`
///   equal to `Some(FillBytesErrorReturn::MockIRandomSourceAdapter)` under
///   `assert_eq!`; the `Succeeds` adapter's success arm is extracted with
///   `expect` and the length of its `bytes.expose()` equals the literal 48
///   written in the assertion.
#[test]
fn a_mock_selection_admitted_with_its_failure_mode_constructs_the_source_failing_in_that_mode() {
    // Arrange
    let refused_selection = admit_random_selection(
        &build_admit_random_selection_deps(Default::default()),
        build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
            identifier: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
            )),
        }),
        Value::Null,
    )
    .expect("the mock member admits a document supplying no setting")
    .selection;
    let succeeds_selection = admit_random_selection(
        &build_admit_random_selection_deps(Default::default()),
        build_admit_random_selection_params(AdmitRandomSelectionParamsOverrides {
            identifier: Some(RandomSourceKind::Mock(
                MockIRandomSourceAdapterFailureMode::Succeeds,
            )),
        }),
        Value::Object(Map::new()),
    )
    .expect("the mock member admits a document supplying no setting")
    .selection;
    let refused: Box<dyn IRandomSourceAdapter> = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(refused_selection),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the admitted mock selection constructs its source")
    .adapter;
    let succeeds: Box<dyn IRandomSourceAdapter> = create_random_source(
        &build_create_random_source_deps(Default::default()),
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(succeeds_selection),
        }),
        build_create_random_source_payload(Default::default()),
    )
    .expect("the admitted mock selection constructs its source")
    .adapter;

    // Act
    let refused_result = refused.fill_bytes(
        FillBytesParams,
        build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) }),
    );
    let succeeds_result = succeeds.fill_bytes(
        FillBytesParams,
        build_fill_bytes_payload(FillBytesPayloadOverrides { length: Some(48) }),
    );

    // Assert
    assert_eq!(
        refused_result.err(),
        Some(FillBytesErrorReturn::MockIRandomSourceAdapter)
    );
    let succeeds_success = succeeds_result.expect("the succeeds-mode source fills the buffer");
    assert_eq!(succeeds_success.bytes.expose().len(), 48);
}
// a_mock_selection_admitted_with_its_failure_mode_constructs_the_source_failing_in_that_mode
