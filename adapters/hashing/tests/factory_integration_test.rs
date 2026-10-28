#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use hashing::{
    CommitmentKey, CommitmentSchemeIdentifier, CreateCommitmentParamsOverrides,
    DECLARED_COMMITMENT_SELECTIONS, ICommitmentAdapter, REFERENCE_CONTENT_LENGTH,
    build_commit_params, build_commit_payload, build_create_commitment_deps,
    build_create_commitment_params, build_create_commitment_payload, create_commitment,
    expected_outboard, reference_content,
};
use hex::decode;

/// Contract: `create_commitment` is called with the selection of a
///   concrete that declares `CommitmentSchemeIdentifier::Blake3BaoV1`
///   and that identifier, and the adapter commits the reference content
///   unkeyed → `Ok(CommitSuccessReturn { root, outboard })` with
///   `root.as_bytes()` equal to bao's published 3073-byte hash
///   `5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1`
///   and `outboard.bytes` equal to `expected_outboard` over the
///   reference content unkeyed.
/// Arrange: for each case, the adapter from `create_commitment` over
///   `build_create_commitment_deps`,
///   `build_create_commitment_params` overriding `concrete` and
///   `identifier` with the case's selection and identifier, and
///   `build_create_commitment_payload`, bound through `expect`;
///   `build_commit_payload` over `reference_content(REFERENCE_CONTENT_LENGTH)`
///   with `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit(build_commit_params(Default::default()), payload)`
///   for the case.
/// Assert:  the success arm is bound through `expect`;
///   `root.as_bytes()` equals `hex::decode` of the published hash
///   literal by `assert_eq!`; `outboard.bytes` equals `expected_outboard`
///   over the reference content with no key by `assert_eq!`, the
///   expectations a published vector and an independent oracle and not
///   read from the arrangement.
/// Boundary: the public call `create_commitment` and the returned
///   adapter's `commit`; the route `create_commitment`, the selected
///   concrete's constructor, and its `commit`.
/// Mocked:  nothing; `blake3` and `bao` run for real.
#[test]
fn every_concrete_declaring_the_blake3_bao_identifier_commits_the_reference_content_to_the_bao_hash_and_outboard()
 {
    // Arrange
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    for selection in DECLARED_COMMITMENT_SELECTIONS
        .iter()
        .filter(|selection| selection.identifier == CommitmentSchemeIdentifier::Blake3BaoV1)
    {
        let deps = build_create_commitment_deps(Default::default());
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(selection.concrete),
            identifier: Some(selection.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the adapter");
        let adapter: &dyn ICommitmentAdapter = &*success.adapter;
        let commit_payload =
            build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

        // Act
        let result = adapter.commit(build_commit_params(Default::default()), commit_payload);

        // Assert
        let success = result.expect("the unkeyed commit is admitted");
        assert_eq!(
            success.root.as_bytes().as_slice(),
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .as_slice(),
        );
        assert_eq!(success.outboard.bytes, expected_outboard(&content, None));
    }
}
