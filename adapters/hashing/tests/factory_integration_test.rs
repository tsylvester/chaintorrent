#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use hashing::{
    COMMITMENT_CONCRETES, CommitReturn, CommitmentConcreteOverrides, CommitmentKey,
    CommitmentSchemeIdentifier, ConsumeCommitmentParams, ConsumeCommitmentPayload,
    CreateCommitmentDepsOverrides, CreateCommitmentParamsOverrides, ICommitmentAdapter,
    ICommitmentConsumer, LookupHashingDeclarationParamsOverrides, REFERENCE_CONTENT_LENGTH,
    admit_hashing_composition, build_commit_params, build_commit_payload,
    build_commitment_concrete, build_create_commitment_deps, build_create_commitment_params,
    build_create_commitment_payload, build_lookup_hashing_declaration_deps,
    build_lookup_hashing_declaration_params, build_lookup_hashing_declaration_payload,
    create_commitment, expected_outboard, lookup_hashing_declaration, reference_content,
};
use hex::decode;

/// Contract: the own entry of this name: `create_commitment` is called
///   with the selection of a member of `COMMITMENT_CONCRETES` whose
///   declaration the real lookup returns with the identifier
///   `CommitmentSchemeIdentifier::Blake3BaoV1`, that identifier, and
///   deps holding the real admission and lookup →
///   `Ok(CreateCommitmentSuccessReturn { output })` whose `output` is the
///   `CommitReturn` the consumer observes on the adapter it receives:
///   `Ok(CommitSuccessReturn { root, outboard })` with `root.as_bytes()`
///   equal to bao's published 3073-byte hash
///   `5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1`
///   and `outboard.bytes` equal to `expected_outboard` over the
///   reference content unkeyed.
/// Arrange: for each case, `build_create_commitment_params` overriding
///   `concrete` and `identifier` with the case's selection and its
///   looked-up identifier, and `build_create_commitment_payload`; deps
///   from `build_create_commitment_deps` with the block's `Consumer`
///   leading and `admit_hashing_composition` and
///   `lookup_hashing_declaration` overridden with the real functions;
///   the block's `Consumer`, whose `consume_commitment` calls
///   `commit(build_commit_params(Default::default()), payload)` on the
///   adapter it receives over `build_commit_payload` of
///   `reference_content(REFERENCE_CONTENT_LENGTH)` with
///   `CommitmentKey::Unkeyed` and returns the `CommitReturn` it observes
///   as its `Output`.
/// Act:     `create_commitment(&deps, params, payload)` for the case.
/// Assert:  the success arm is bound through `expect`; the
///   `CommitSuccessReturn` the `output` observes is bound through
///   `expect`; `root.as_bytes()` equals `hex::decode` of the published
///   hash literal by `assert_eq!`; `outboard.bytes` equals
///   `expected_outboard` over the reference content with no key by
///   `assert_eq!`, the expectations a published vector and an
///   independent oracle and not read from the arrangement.
/// Boundary: the public call `create_commitment` and
///   `ICommitmentConsumer::consume_commitment` calling `commit` on the
///   adapter it receives; the route `create_commitment`,
///   `admit_hashing_composition`, `lookup_hashing_declaration`, the
///   selected concrete's constructor, `consume_commitment`, and
///   `commit`.
/// Mocked:  nothing; `blake3` and `bao` run for real.
#[test]
fn every_concrete_declaring_the_blake3_bao_identifier_commits_the_reference_content_to_the_bao_hash_and_outboard()
 {
    struct Consumer;

    impl ICommitmentConsumer for Consumer {
        type Output = CommitReturn;

        fn consume_commitment<A: ICommitmentAdapter>(
            &self,
            _params: ConsumeCommitmentParams,
            payload: ConsumeCommitmentPayload<A>,
        ) -> Self::Output {
            let content = reference_content(REFERENCE_CONTENT_LENGTH);
            payload.adapter.commit(
                build_commit_params(Default::default()),
                build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default()),
            )
        }
    }

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
        let deps = build_create_commitment_deps(
            Consumer,
            CreateCommitmentDepsOverrides {
                admit_hashing_composition: Some(admit_hashing_composition),
                lookup_hashing_declaration: Some(lookup_hashing_declaration),
            },
        );
        let params = build_create_commitment_params(CreateCommitmentParamsOverrides {
            concrete: Some(concrete),
            identifier: Some(declaration.identifier),
        });
        let payload = build_create_commitment_payload(Default::default());

        // Act
        let success = create_commitment(&deps, params, payload)
            .expect("an admitted selection returns the consumer's output");

        // Assert
        let committed = success.output.expect("the unkeyed commit is admitted");
        assert_eq!(
            committed.root.as_bytes().as_slice(),
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .as_slice(),
        );
        assert_eq!(committed.outboard.bytes, expected_outboard(&content, None));
    }
}
