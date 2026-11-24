#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use hashing::{
    COMMITMENT_CONCRETES, CommitmentConcreteOverrides, CommitmentKey, CommitmentSchemeIdentifier,
    CreateCommitmentDepsOverrides, CreateCommitmentParamsOverrides, ICommitmentAdapter,
    LookupHashingDeclarationParamsOverrides, REFERENCE_CONTENT_LENGTH, admit_hashing_composition,
    build_commit_params, build_commit_payload, build_commitment_concrete,
    build_create_commitment_deps, build_create_commitment_params, build_create_commitment_payload,
    build_lookup_hashing_declaration_deps, build_lookup_hashing_declaration_params,
    build_lookup_hashing_declaration_payload, create_commitment, expected_outboard,
    lookup_hashing_declaration, reference_content,
};
use hex::decode;

/// Contract: the own entry of this name: `create_commitment` is called
///   with the selection of a member of `COMMITMENT_CONCRETES` whose
///   declaration the real lookup returns with the identifier
///   `CommitmentSchemeIdentifier::Blake3BaoV1`, that identifier, and
///   deps holding the real admission and lookup, and the adapter commits
///   the reference content unkeyed → `Ok(CommitSuccessReturn { root, outboard })`
///   with `root.as_bytes()` equal to bao's published 3073-byte hash
///   `5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1`
///   and `outboard.bytes` equal to `expected_outboard` over the
///   reference content unkeyed.
/// Arrange: for each case, the adapter from `create_commitment` over
///   `build_create_commitment_deps` overriding `admit_hashing_composition`
///   and `lookup_hashing_declaration` with the real functions,
///   `build_create_commitment_params` overriding `concrete` and
///   `identifier` with the case's selection and its looked-up identifier,
///   and `build_create_commitment_payload`, bound through `expect`;
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
///   adapter's `commit`; the route `create_commitment`,
///   `admit_hashing_composition`, `lookup_hashing_declaration`, the
///   selected concrete's constructor, and its `commit`.
/// Mocked:  nothing; `blake3` and `bao` run for real.
#[test]
fn every_concrete_declaring_the_blake3_bao_identifier_commits_the_reference_content_to_the_bao_hash_and_outboard()
 {
    // Arrange
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    for member in COMMITMENT_CONCRETES {
        let concrete = build_commitment_concrete(*member, CommitmentConcreteOverrides);
        let declaration = {
            let Ok(looked_up) = lookup_hashing_declaration(
                &build_lookup_hashing_declaration_deps(Default::default()),
                build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                    concrete: Some(concrete),
                }),
                build_lookup_hashing_declaration_payload(Default::default()),
            );
            looked_up.declaration
        };
        if declaration.identifier != CommitmentSchemeIdentifier::Blake3BaoV1 {
            continue;
        }
        let deps = build_create_commitment_deps(CreateCommitmentDepsOverrides {
            admit_hashing_composition: Some(admit_hashing_composition),
            lookup_hashing_declaration: Some(lookup_hashing_declaration),
        });
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
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
