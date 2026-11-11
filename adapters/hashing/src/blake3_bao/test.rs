#![allow(clippy::expect_used)]

use super::provides::{
    Blake3BaoCommitment, build_blake3_bao_commitment,
    build_blake3_bao_commitment_constructor_params, expected_keyed_root, expected_outboard,
    expected_path,
};
use crate::factory::provides::{
    AuthenticationPathOverrides, COMMITMENT_INTERFACE_VERSION, ChallengePayloadOverrides,
    ChunkOverrides, CommitErrorReturn, CommitmentKey, CommitmentSchemeIdentifier,
    ExtractPathErrorReturn, ExtractPathPayloadOverrides, ICommitmentAdapter, OutboardOverrides,
    REFERENCE_CONTENT_LENGTH, RespondErrorReturn, RespondPayloadOverrides,
    RootConstructorParamsOverrides, VerifyChunkErrorReturn, VerifyChunkPayloadOverrides,
    VerifyChunkSuccessReturn, VerifyErrorReturn, VerifySuccessReturn, build_authentication_path,
    build_challenge_params, build_challenge_payload, build_chunk, build_commit_params,
    build_commit_payload, build_extract_path_params, build_extract_path_payload, build_outboard,
    build_respond_params, build_respond_payload, build_root, build_verify_chunk_params,
    build_verify_chunk_payload, build_verify_params, build_verify_payload, reference_content,
    reference_key_bytes,
};
use blake3::hash;
use domain::{SecretConstructorParamsOverrides, build_secret};
use hex::decode;

/// Contract: `try_new`, admitted: any params → `Ok(Blake3BaoCommitment)`; the
///   error arm has no branch.
/// Arrange: `build_blake3_bao_commitment_constructor_params` with no override.
/// Act:     `Blake3BaoCommitment::try_new`.
/// Assert:  `result` matches `Ok(Blake3BaoCommitment)` by `assert!(matches!(…))`.
#[test]
fn try_new_returns_the_adapter() {
    // Arrange
    let params = build_blake3_bao_commitment_constructor_params(Default::default());

    // Act
    let result = Blake3BaoCommitment::try_new(params);

    // Assert
    assert!(matches!(result, Ok(Blake3BaoCommitment)));
}

/// Contract: `declaration`, declared: any adapter → `Blake3BaoCommitment::DECLARATION`.
/// Arrange: `build_blake3_bao_commitment` with no override.
/// Act:     `adapter.declaration()`.
/// Assert:  the identifier matches `CommitmentSchemeIdentifier::Blake3BaoV1`; the
///   chunk length equals 1024; the key length equals 32; the adapter version
///   equals 1; the interface version equals `COMMITMENT_INTERFACE_VERSION`, each
///   by `assert!`.
#[test]
fn declaration_returns_the_blake3_bao_declaration() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());

    // Act
    let declaration = adapter.declaration();

    // Assert
    assert!(declaration.identifier == CommitmentSchemeIdentifier::Blake3BaoV1);
    assert!(declaration.chunk_length == 1024);
    assert!(declaration.key_length == 32);
    assert!(declaration.adapter_version == 1);
    assert!(declaration.interface_version == COMMITMENT_INTERFACE_VERSION);
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 0, the empty object; `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_0_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(0);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 1, a single-byte object;
///   `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_1_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(1);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("48fc721fbbc172e0925fa27af1671de225ba927134802998b10a1568a188652b")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 1023, one byte short of the chunk
///   boundary; `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_1023_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(1023);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("15f8c1ae1049fe7e837186612c8ce732e66835841a4569b71e4ac3e3d3411b90")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 1024, exactly one chunk;
///   `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_1024_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(1024);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("f749c19181983b839cd97fe121cebaf076bc951e8c8e6d64accfedad5951ec22")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 1025, one byte past the chunk boundary,
///   the first length needing a parent node; `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_1025_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(1025);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("3613596275c4ea790774dedf20835b2daf86cacc892feef6ce720c121572f1f9")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 2048, two whole chunks;
///   `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_2048_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(2048);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("fed8b40d6095dc7c5061f9cd832fd192337473bd392bf6f6bbaf1261ea78f8fa")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 2049, three chunks with a one-byte
///   tail, so a wrong split yields another hash; `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_2049_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(2049);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("64770fa15a4bbe7770654c4ac68ed4f0e975ad6c85b5edb4d3db3b4b604e084e")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 3072, three whole chunks, an
///   unbalanced tree; `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_3072_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(3072);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("9748169f2aa70258d18cef6dc6b4b4511265e268e85f73dcbea6e34ab0341da1")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 3073, four chunks under three parent
///   nodes with a one-byte tail; `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_3073_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(3073);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose root is the content's Bao hash.
/// Arrange: `reference_content` of length 13312, thirteen chunks across a
///   deeper tree; `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the success arm's `root.as_bytes()` equals `hex::decode` of bao's
///   published hash vector for the length, by `assert_eq!`.
#[test]
fn commit_of_13312_bytes_returns_the_bao_hash_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(13312);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(
        success.root.as_bytes().as_slice(),
        decode("3e88d1dd20f426640077dcf82d6d4e18ee0062aa72f8ae547a0e65fcd36a0f06")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose outboard is bao's outboard encoding of the content.
/// Arrange: `reference_content` of length 1025 with `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the outboard's length equals 72; `blake3::hash` of the outboard's
///   bytes equals `hex::decode` of bao's published outboard digest for the
///   length, by `assert_eq!`.
#[test]
fn commit_of_1025_bytes_returns_the_bao_outboard_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(1025);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(success.outboard.bytes.len(), 72);
    assert_eq!(
        hash(&success.outboard.bytes).as_bytes().as_slice(),
        decode("025f630e00fbdcc023b970c9f1f21016c56c0a34e384e712c66428bf1c7999b9")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, unkeyed: the key is `Unkeyed` → `Ok(CommitSuccessReturn)`
///   whose outboard is bao's outboard encoding of the content.
/// Arrange: `reference_content` of length 3073 with `CommitmentKey::Unkeyed`.
/// Act:     `adapter.commit`.
/// Assert:  the outboard's length equals 200; `blake3::hash` of the outboard's
///   bytes equals `hex::decode` of bao's published outboard digest for the
///   length, by `assert_eq!`.
#[test]
fn commit_of_3073_bytes_returns_the_bao_outboard_test_vector() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(3073);
    let payload = build_commit_payload(&content, CommitmentKey::Unkeyed, Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the unkeyed commit is admitted");
    assert_eq!(success.outboard.bytes.len(), 200);
    assert_eq!(
        hash(&success.outboard.bytes).as_bytes().as_slice(),
        decode("fa89135a2c00ff1a073321ed79fd17b9fdd53c2d2549236cd8d796232a89dc83")
            .expect("the vector literal is valid hex")
            .as_slice(),
    );
}

/// Contract: `commit`, keyed: the key is `Keyed` and admits →
///   `Ok(CommitSuccessReturn)` with the keyed root and the keyed outboard.
/// Arrange: `build_secret` over `reference_key_bytes()`;
///   `reference_content(REFERENCE_CONTENT_LENGTH)`; `CommitmentKey::Keyed`
///   borrowing the secret.
/// Act:     `adapter.commit`.
/// Assert:  the root's `as_bytes()` equals `expected_keyed_root` over the
///   reference key and content; the outboard's bytes equal `expected_outboard`
///   over the content and the reference key and differ from `expected_outboard`
///   unkeyed, by `assert_eq!` and `assert!`.
#[test]
fn commit_under_a_key_returns_the_keyed_root_and_outboard() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let key = <[u8; 32]>::try_from(reference_key_bytes().as_slice())
        .expect("the reference key is 32 bytes");
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let payload = build_commit_payload(&content, CommitmentKey::Keyed(&secret), Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    let success = result.expect("the keyed commit is admitted");
    assert_eq!(
        success.root.as_bytes(),
        &expected_keyed_root(&key, &content)
    );
    assert_eq!(
        success.outboard.bytes,
        expected_outboard(&content, Some(&key))
    );
    assert!(success.outboard.bytes != expected_outboard(&content, None));
}

/// Contract: `commit`, key rejected: the key does not admit →
///   `Err(CommitErrorReturn::KeyLengthMismatch { expected, actual })`.
/// Arrange: `build_secret` over the first 31 bytes of `reference_key_bytes()`;
///   `CommitmentKey::Keyed` borrowing it.
/// Act:     `adapter.commit` over the reference content.
/// Assert:  `result.err()` equals
///   `Some(CommitErrorReturn::KeyLengthMismatch { expected: 32, actual: 31 })`
///   by `assert_eq!`.
#[test]
fn commit_under_a_key_of_the_wrong_length_returns_key_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let secret = build_secret(
        reference_key_bytes()[..31].to_vec(),
        SecretConstructorParamsOverrides,
    );
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let payload = build_commit_payload(&content, CommitmentKey::Keyed(&secret), Default::default());

    // Act
    let result = adapter.commit(build_commit_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CommitErrorReturn::KeyLengthMismatch {
            expected: 32,
            actual: 31,
        }),
    );
}

/// Contract: `verify`, verified: every check passes → `Ok(VerifySuccessReturn)`.
/// Arrange: the reference content; `build_outboard` with the bytes
///   `expected_outboard` over it unkeyed; `build_root` with the bytes of the
///   3073-byte hash literal.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result` matches `Ok(VerifySuccessReturn)` by `assert!(matches!(…))`.
#[test]
fn verify_accepts_content_matching_its_root_and_outboard() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(&content, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert!(matches!(result, Ok(VerifySuccessReturn)));
}

/// Contract: `verify`, chunk: a leaf's computed chaining value differs from the
///   one its parent recorded → `Err(VerifyErrorReturn::ChunkMismatch)` naming
///   the first changed chunk.
/// Arrange: the accepting arrangement with one byte of chunk 2 inverted.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result.err()` equals
///   `Some(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 })` by `assert_eq!`.
#[test]
fn verify_returns_chunk_mismatch_for_the_first_changed_chunk() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let mut content = reference_content(REFERENCE_CONTENT_LENGTH);
    content[2048] ^= 0xff;
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(
            &reference_content(REFERENCE_CONTENT_LENGTH),
            None,
        )),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyErrorReturn::ChunkMismatch { chunk_index: 2 }),
    );
}

/// Contract: `verify`, chunk: a leaf's computed chaining value differs from the
///   one its parent recorded → `Err(VerifyErrorReturn::ChunkMismatch)` naming
///   the first changed chunk.
/// Arrange: the accepting arrangement with chunks 0 and 1 exchanged, so the
///   first leaf differs.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result.err()` equals
///   `Some(VerifyErrorReturn::ChunkMismatch { chunk_index: 0 })` by `assert_eq!`.
#[test]
fn verify_returns_chunk_mismatch_when_two_chunks_are_swapped() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let mut content = reference_content(REFERENCE_CONTENT_LENGTH);
    for index in 0..1024 {
        content.swap(index, index + 1024);
    }
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(
            &reference_content(REFERENCE_CONTENT_LENGTH),
            None,
        )),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyErrorReturn::ChunkMismatch { chunk_index: 0 }),
    );
}

/// Contract: `verify`, root: the computed root differs from `payload.root` →
///   `Err(VerifyErrorReturn::RootMismatch)`.
/// Arrange: the accepting arrangement with the root built from the 3072-byte
///   hash literal, a different object's root.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result.err()` equals `Some(VerifyErrorReturn::RootMismatch)` by
///   `assert_eq!`.
#[test]
fn verify_returns_root_mismatch_for_another_root() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(&content, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("9748169f2aa70258d18cef6dc6b4b4511265e268e85f73dcbea6e34ab0341da1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(result.err(), Some(VerifyErrorReturn::RootMismatch));
}

/// Contract: `verify`, parent: a parent's two recorded children merge to a
///   chaining value different from the one its parent recorded →
///   `Err(VerifyErrorReturn::ParentMismatch)` naming that subtree.
/// Arrange: the accepting arrangement with one byte of the third parent node,
///   the right subtree's, inverted.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result.err()` equals
///   `Some(VerifyErrorReturn::ParentMismatch { offset: 2048, length: 1025 })`
///   by `assert_eq!`.
#[test]
fn verify_returns_parent_mismatch_for_a_changed_parent_node() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mut bytes = expected_outboard(&content, None);
    bytes[136] ^= 0xff;
    let outboard = build_outboard(OutboardOverrides { bytes: Some(bytes) });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyErrorReturn::ParentMismatch {
            offset: 2048,
            length: 1025,
        }),
    );
}

/// Contract: `verify`, content length: the declared length differs from
///   `payload.content.len()` → `Err(VerifyErrorReturn::ContentLengthMismatch)`.
/// Arrange: the accepting arrangement with content of 3072 bytes against the
///   3073-byte outboard.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result.err()` equals
///   `Some(VerifyErrorReturn::ContentLengthMismatch { declared: 3073, actual: 3072 })`
///   by `assert_eq!`.
#[test]
fn verify_returns_content_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(3072);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(
            &reference_content(REFERENCE_CONTENT_LENGTH),
            None,
        )),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyErrorReturn::ContentLengthMismatch {
            declared: 3073,
            actual: 3072,
        }),
    );
}

/// Contract: `verify`, outboard truncated: the outboard holds fewer than
///   `OUTBOARD_HEADER_LENGTH` bytes → `Err(VerifyErrorReturn::OutboardTruncated)`.
/// Arrange: the accepting arrangement with an outboard of 4 bytes.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result.err()` equals
///   `Some(VerifyErrorReturn::OutboardTruncated { length: 4 })` by `assert_eq!`.
#[test]
fn verify_returns_outboard_truncated() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(vec![0; 4]),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyErrorReturn::OutboardTruncated { length: 4 }),
    );
}

/// Contract: `verify`, outboard length: the outboard's length differs from the
///   expected outboard length → `Err(VerifyErrorReturn::OutboardLengthMismatch)`.
/// Arrange: the accepting arrangement with the outboard's last 64 bytes, the
///   third parent node, removed.
/// Act:     `adapter.verify` with `CommitmentKey::Unkeyed`.
/// Assert:  `result.err()` equals
///   `Some(VerifyErrorReturn::OutboardLengthMismatch { expected: 200, actual: 136 })`
///   by `assert_eq!`.
#[test]
fn verify_returns_outboard_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mut bytes = expected_outboard(&content, None);
    bytes.truncate(136);
    let outboard = build_outboard(OutboardOverrides { bytes: Some(bytes) });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Unkeyed,
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyErrorReturn::OutboardLengthMismatch {
            expected: 200,
            actual: 136,
        }),
    );
}

/// Contract: `verify`, key rejected: the key does not admit →
///   `Err(VerifyErrorReturn::KeyLengthMismatch { expected, actual })`.
/// Arrange: the accepting arrangement with `CommitmentKey::Keyed` over the
///   first 31 bytes of `reference_key_bytes()`.
/// Act:     `adapter.verify`.
/// Assert:  `result.err()` equals
///   `Some(VerifyErrorReturn::KeyLengthMismatch { expected: 32, actual: 31 })`
///   by `assert_eq!`.
#[test]
fn verify_returns_key_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(&content, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let secret = build_secret(
        reference_key_bytes()[..31].to_vec(),
        SecretConstructorParamsOverrides,
    );
    let payload = build_verify_payload(
        &content,
        &outboard,
        &root,
        CommitmentKey::Keyed(&secret),
        Default::default(),
    );

    // Act
    let result = adapter.verify(build_verify_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyErrorReturn::KeyLengthMismatch {
            expected: 32,
            actual: 31,
        }),
    );
}

/// Contract: `extract_path`, extracted: every check passes →
///   `Ok(ExtractPathSuccessReturn)` with the sibling values from the root down
///   to the chunk.
/// Arrange: `build_outboard` with `expected_outboard` over the reference
///   content unkeyed; `build_extract_path_payload` with the chunk index
///   override 2, a chunk in the right subtree, so a descent that always goes
///   left fails.
/// Act:     `adapter.extract_path`.
/// Assert:  the success arm's `path.siblings` equals `expected_path` over the
///   reference content, index 2, unkeyed, by `assert_eq!`.
#[test]
fn extract_path_returns_the_siblings_of_the_chunk() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(&content, None)),
    });
    let payload = build_extract_path_payload(
        &outboard,
        ExtractPathPayloadOverrides {
            chunk_index: Some(2),
        },
    );

    // Act
    let result = adapter.extract_path(build_extract_path_params(Default::default()), payload);

    // Assert
    let success = result.expect("the extraction is admitted");
    assert_eq!(success.path.siblings, expected_path(&content, 2, None));
}

/// Contract: `extract_path`, index: `payload.chunk_index` is not less than the
///   chunk count → `Err(ExtractPathErrorReturn::ChunkIndexOutOfRange)`.
/// Arrange: the accepting arrangement with the chunk index override 4, one past
///   the last of four chunks.
/// Act:     `adapter.extract_path`.
/// Assert:  `result.err()` equals
///   `Some(ExtractPathErrorReturn::ChunkIndexOutOfRange { chunk_index: 4, chunk_count: 4 })`
///   by `assert_eq!`.
#[test]
fn extract_path_returns_chunk_index_out_of_range() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(&content, None)),
    });
    let payload = build_extract_path_payload(
        &outboard,
        ExtractPathPayloadOverrides {
            chunk_index: Some(4),
        },
    );

    // Act
    let result = adapter.extract_path(build_extract_path_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(ExtractPathErrorReturn::ChunkIndexOutOfRange {
            chunk_index: 4,
            chunk_count: 4,
        }),
    );
}

/// Contract: `extract_path`, outboard truncated: the outboard holds fewer than
///   `OUTBOARD_HEADER_LENGTH` bytes →
///   `Err(ExtractPathErrorReturn::OutboardTruncated)`.
/// Arrange: the accepting arrangement with an outboard of 4 bytes.
/// Act:     `adapter.extract_path`.
/// Assert:  `result.err()` equals
///   `Some(ExtractPathErrorReturn::OutboardTruncated { length: 4 })` by
///   `assert_eq!`.
#[test]
fn extract_path_returns_outboard_truncated() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(vec![0; 4]),
    });
    let payload = build_extract_path_payload(&outboard, Default::default());

    // Act
    let result = adapter.extract_path(build_extract_path_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(ExtractPathErrorReturn::OutboardTruncated { length: 4 }),
    );
}

/// Contract: `extract_path`, outboard length: the outboard's length differs
///   from the expected outboard length →
///   `Err(ExtractPathErrorReturn::OutboardLengthMismatch)`.
/// Arrange: the accepting arrangement with the outboard's last 64 bytes removed.
/// Act:     `adapter.extract_path`.
/// Assert:  `result.err()` equals
///   `Some(ExtractPathErrorReturn::OutboardLengthMismatch { expected: 200, actual: 136 })`
///   by `assert_eq!`.
#[test]
fn extract_path_returns_outboard_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mut bytes = expected_outboard(&content, None);
    bytes.truncate(136);
    let outboard = build_outboard(OutboardOverrides { bytes: Some(bytes) });
    let payload = build_extract_path_payload(&outboard, Default::default());

    // Act
    let result = adapter.extract_path(build_extract_path_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(ExtractPathErrorReturn::OutboardLengthMismatch {
            expected: 200,
            actual: 136,
        }),
    );
}

/// Contract: `verify_chunk`, verified: every check passes →
///   `Ok(VerifyChunkSuccessReturn)`.
/// Arrange: `build_chunk` with bytes 2048 through 3071 of the reference
///   content; `build_authentication_path` with `expected_path` at index 2
///   unkeyed; `build_root` with the 3073-byte hash literal;
///   `build_verify_chunk_payload` with the chunk index override 2 and
///   `CommitmentKey::Unkeyed`.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result` matches `Ok(VerifyChunkSuccessReturn)` by
///   `assert!(matches!(…))`.
#[test]
fn verify_chunk_accepts_a_chunk_with_its_path() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3072].to_vec()),
    });
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(expected_path(&content, 2, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Unkeyed,
        VerifyChunkPayloadOverrides {
            chunk_index: Some(2),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert!(matches!(result, Ok(VerifyChunkSuccessReturn)));
}

/// Contract: `verify_chunk`, verified: every check passes under the key →
///   `Ok(VerifyChunkSuccessReturn)`.
/// Arrange: `build_secret` over `reference_key_bytes()`; `build_chunk` with
///   bytes 2048 through 3071 of the reference content;
///   `build_authentication_path` with `expected_path` at index 2 keyed;
///   `build_root` with `expected_keyed_root`; `build_verify_chunk_payload` with
///   the chunk index override 2 and `CommitmentKey::Keyed`.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result` matches `Ok(VerifyChunkSuccessReturn)` by
///   `assert!(matches!(…))`.
#[test]
fn verify_chunk_accepts_a_keyed_chunk_with_its_keyed_path() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let secret = build_secret(reference_key_bytes(), SecretConstructorParamsOverrides);
    let key = <[u8; 32]>::try_from(reference_key_bytes().as_slice())
        .expect("the reference key is 32 bytes");
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3072].to_vec()),
    });
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(expected_path(&content, 2, Some(&key))),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(expected_keyed_root(&key, &content)),
    });
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Keyed(&secret),
        VerifyChunkPayloadOverrides {
            chunk_index: Some(2),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert!(matches!(result, Ok(VerifyChunkSuccessReturn)));
}

/// Contract: `verify_chunk`, root: the folded root differs from `payload.root` →
///   `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: the unkeyed accepting arrangement with one byte of the chunk
///   inverted.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result.err()` equals `Some(VerifyChunkErrorReturn::RootMismatch)`
///   by `assert_eq!`.
#[test]
fn verify_chunk_returns_root_mismatch_for_a_changed_chunk() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let mut bytes = content[2048..3072].to_vec();
    bytes[0] ^= 0xff;
    let chunk = build_chunk(ChunkOverrides { bytes: Some(bytes) });
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(expected_path(&content, 2, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Unkeyed,
        VerifyChunkPayloadOverrides {
            chunk_index: Some(2),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert_eq!(result.err(), Some(VerifyChunkErrorReturn::RootMismatch));
}

/// Contract: `verify_chunk`, root: the folded root differs from `payload.root` →
///   `Err(VerifyChunkErrorReturn::RootMismatch)`.
/// Arrange: the unkeyed accepting arrangement with one byte of the first
///   sibling inverted.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result.err()` equals `Some(VerifyChunkErrorReturn::RootMismatch)`
///   by `assert_eq!`.
#[test]
fn verify_chunk_returns_root_mismatch_for_a_changed_sibling() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3072].to_vec()),
    });
    let mut siblings = expected_path(&content, 2, None);
    siblings[0][0] ^= 0xff;
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(siblings),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Unkeyed,
        VerifyChunkPayloadOverrides {
            chunk_index: Some(2),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert_eq!(result.err(), Some(VerifyChunkErrorReturn::RootMismatch));
}

/// Contract: `verify_chunk`, chunk length: the chunk's length differs from the
///   expected length of that chunk →
///   `Err(VerifyChunkErrorReturn::ChunkLengthMismatch)`.
/// Arrange: the unkeyed accepting arrangement with 1023 of the chunk's bytes.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result.err()` equals
///   `Some(VerifyChunkErrorReturn::ChunkLengthMismatch { expected: 1024, actual: 1023 })`
///   by `assert_eq!`.
#[test]
fn verify_chunk_returns_chunk_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3071].to_vec()),
    });
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(expected_path(&content, 2, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Unkeyed,
        VerifyChunkPayloadOverrides {
            chunk_index: Some(2),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyChunkErrorReturn::ChunkLengthMismatch {
            expected: 1024,
            actual: 1023,
        }),
    );
}

/// Contract: `verify_chunk`, path length: the path's length differs from the
///   number of splits on the route →
///   `Err(VerifyChunkErrorReturn::PathLengthMismatch)`.
/// Arrange: the unkeyed accepting arrangement with the path's last sibling
///   removed.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result.err()` equals
///   `Some(VerifyChunkErrorReturn::PathLengthMismatch { expected: 2, actual: 1 })`
///   by `assert_eq!`.
#[test]
fn verify_chunk_returns_path_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3072].to_vec()),
    });
    let mut siblings = expected_path(&content, 2, None);
    siblings.pop();
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(siblings),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Unkeyed,
        VerifyChunkPayloadOverrides {
            chunk_index: Some(2),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyChunkErrorReturn::PathLengthMismatch {
            expected: 2,
            actual: 1,
        }),
    );
}

/// Contract: `verify_chunk`, index: `payload.chunk_index` is not less than the
///   chunk count → `Err(VerifyChunkErrorReturn::ChunkIndexOutOfRange)`.
/// Arrange: the unkeyed accepting arrangement with the chunk index override 4.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result.err()` equals
///   `Some(VerifyChunkErrorReturn::ChunkIndexOutOfRange { chunk_index: 4, chunk_count: 4 })`
///   by `assert_eq!`.
#[test]
fn verify_chunk_returns_chunk_index_out_of_range() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3072].to_vec()),
    });
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(expected_path(&content, 2, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Unkeyed,
        VerifyChunkPayloadOverrides {
            chunk_index: Some(4),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyChunkErrorReturn::ChunkIndexOutOfRange {
            chunk_index: 4,
            chunk_count: 4,
        }),
    );
}

/// Contract: `verify_chunk`, key rejected: the key does not admit →
///   `Err(VerifyChunkErrorReturn::KeyLengthMismatch { expected, actual })`.
/// Arrange: the unkeyed accepting arrangement with `CommitmentKey::Keyed` over
///   the first 31 bytes of `reference_key_bytes()`.
/// Act:     `adapter.verify_chunk`.
/// Assert:  `result.err()` equals
///   `Some(VerifyChunkErrorReturn::KeyLengthMismatch { expected: 32, actual: 31 })`
///   by `assert_eq!`.
#[test]
fn verify_chunk_returns_key_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let chunk = build_chunk(ChunkOverrides {
        bytes: Some(content[2048..3072].to_vec()),
    });
    let path = build_authentication_path(AuthenticationPathOverrides {
        siblings: Some(expected_path(&content, 2, None)),
    });
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some(
            decode("5ba075072daba2470558a171e3769fba057dc3f12375c60892bdbe73348d9fd1")
                .expect("the vector literal is valid hex")
                .try_into()
                .expect("the hash literal is 32 bytes"),
        ),
    });
    let secret = build_secret(
        reference_key_bytes()[..31].to_vec(),
        SecretConstructorParamsOverrides,
    );
    let payload = build_verify_chunk_payload(
        &root,
        &chunk,
        &path,
        CommitmentKey::Keyed(&secret),
        VerifyChunkPayloadOverrides {
            chunk_index: Some(2),
            ..Default::default()
        },
    );

    // Act
    let result = adapter.verify_chunk(build_verify_chunk_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(VerifyChunkErrorReturn::KeyLengthMismatch {
            expected: 32,
            actual: 31,
        }),
    );
}

/// Contract: `challenge`, drawn: any payload → `Ok(ChallengeSuccessReturn)`
///   with the uniform value modulo the chunk count.
/// Arrange: `build_challenge_payload` with the uniform override the
///   little-endian bytes of 6, so the index is neither the value nor zero.
/// Act:     `adapter.challenge`.
/// Assert:  the success arm's `chunk_index` equals 2 by `assert_eq!`.
#[test]
fn challenge_maps_uniform_bytes_to_a_chunk_index() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let payload = build_challenge_payload(ChallengePayloadOverrides {
        uniform: Some(6u64.to_le_bytes()),
        ..Default::default()
    });

    // Act
    let result = adapter.challenge(build_challenge_params(Default::default()), payload);

    // Assert
    let success = result.expect("the challenge always succeeds");
    assert_eq!(success.chunk_index, 2);
}

/// Contract: `challenge`, drawn: content length zero → one chunk, index 0.
/// Arrange: `build_challenge_payload` with the uniform override the
///   little-endian bytes of 6 and the content length override 0.
/// Act:     `adapter.challenge`.
/// Assert:  the success arm's `chunk_index` equals 0 by `assert_eq!`.
#[test]
fn challenge_of_empty_content_returns_index_zero() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let payload = build_challenge_payload(ChallengePayloadOverrides {
        uniform: Some(6u64.to_le_bytes()),
        content_length: Some(0),
    });

    // Act
    let result = adapter.challenge(build_challenge_params(Default::default()), payload);

    // Assert
    let success = result.expect("the challenge always succeeds");
    assert_eq!(success.chunk_index, 0);
}

/// Contract: `respond`, answered: every check passes →
///   `Ok(RespondSuccessReturn)` with the chunk's bytes and its path.
/// Arrange: the reference content; `build_outboard` with `expected_outboard`
///   over it unkeyed; `build_respond_payload` with the chunk index override 3,
///   the short final chunk, so a response of a full chunk fails.
/// Act:     `adapter.respond`.
/// Assert:  the success arm's `chunk.bytes` equals byte 3072 of the reference
///   content alone and `path.siblings` equals `expected_path` at index 3
///   unkeyed, by `assert_eq!`.
#[test]
fn respond_returns_the_chunk_and_its_path() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(&content, None)),
    });
    let payload = build_respond_payload(
        &content,
        &outboard,
        RespondPayloadOverrides {
            chunk_index: Some(3),
        },
    );

    // Act
    let result = adapter.respond(build_respond_params(Default::default()), payload);

    // Assert
    let success = result.expect("the response is admitted");
    assert_eq!(success.chunk.bytes, content[3072..3073].to_vec());
    assert_eq!(success.path.siblings, expected_path(&content, 3, None));
}

/// Contract: `respond`, content length: the declared length differs from
///   `payload.content.len()` → `Err(RespondErrorReturn::ContentLengthMismatch)`.
/// Arrange: the accepting arrangement with content of 3072 bytes.
/// Act:     `adapter.respond`.
/// Assert:  `result.err()` equals
///   `Some(RespondErrorReturn::ContentLengthMismatch { declared: 3073, actual: 3072 })`
///   by `assert_eq!`.
#[test]
fn respond_returns_content_length_mismatch() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(3072);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(
            &reference_content(REFERENCE_CONTENT_LENGTH),
            None,
        )),
    });
    let payload = build_respond_payload(&content, &outboard, Default::default());

    // Act
    let result = adapter.respond(build_respond_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(RespondErrorReturn::ContentLengthMismatch {
            declared: 3073,
            actual: 3072,
        }),
    );
}

/// Contract: `respond`, index: `payload.chunk_index` is not less than the chunk
///   count → `Err(RespondErrorReturn::ChunkIndexOutOfRange)`.
/// Arrange: the accepting arrangement with the chunk index override 4.
/// Act:     `adapter.respond`.
/// Assert:  `result.err()` equals
///   `Some(RespondErrorReturn::ChunkIndexOutOfRange { chunk_index: 4, chunk_count: 4 })`
///   by `assert_eq!`.
#[test]
fn respond_returns_chunk_index_out_of_range() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(expected_outboard(&content, None)),
    });
    let payload = build_respond_payload(
        &content,
        &outboard,
        RespondPayloadOverrides {
            chunk_index: Some(4),
        },
    );

    // Act
    let result = adapter.respond(build_respond_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(RespondErrorReturn::ChunkIndexOutOfRange {
            chunk_index: 4,
            chunk_count: 4,
        }),
    );
}

/// Contract: `respond`, outboard truncated: the outboard holds fewer than
///   `OUTBOARD_HEADER_LENGTH` bytes →
///   `Err(RespondErrorReturn::OutboardTruncated)`.
/// Arrange: the accepting arrangement with an outboard of 4 bytes.
/// Act:     `adapter.respond`.
/// Assert:  `result.err()` equals
///   `Some(RespondErrorReturn::OutboardTruncated { length: 4 })` by `assert_eq!`.
#[test]
fn respond_returns_outboard_truncated() {
    // Arrange
    let adapter = build_blake3_bao_commitment(Default::default());
    let content = reference_content(REFERENCE_CONTENT_LENGTH);
    let outboard = build_outboard(OutboardOverrides {
        bytes: Some(vec![0; 4]),
    });
    let payload = build_respond_payload(&content, &outboard, Default::default());

    // Act
    let result = adapter.respond(build_respond_params(Default::default()), payload);

    // Assert
    assert_eq!(
        result.err(),
        Some(RespondErrorReturn::OutboardTruncated { length: 4 }),
    );
}
