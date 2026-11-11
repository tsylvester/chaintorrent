#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

#[cfg(test)]
use super::interface::{Blake3BaoCommitment, Blake3BaoCommitmentConstructorParams};
#[cfg(any(test, feature = "mocks"))]
use blake3::hazmat::{HasherExt, Mode, merge_subtrees_non_root};
#[cfg(any(test, feature = "mocks"))]
use blake3::{Hasher, keyed_hash};

#[cfg(test)]
#[derive(Default)]
pub struct Blake3BaoCommitmentConstructorParamsOverrides;

#[cfg(test)]
pub fn build_blake3_bao_commitment_constructor_params(
    _overrides: Blake3BaoCommitmentConstructorParamsOverrides,
) -> Blake3BaoCommitmentConstructorParams {
    Blake3BaoCommitmentConstructorParams
}

#[cfg(test)]
pub fn build_blake3_bao_commitment(
    overrides: Blake3BaoCommitmentConstructorParamsOverrides,
) -> Blake3BaoCommitment {
    let Ok(adapter) =
        Blake3BaoCommitment::try_new(build_blake3_bao_commitment_constructor_params(overrides));
    adapter
}

#[cfg(any(test, feature = "mocks"))]
pub fn expected_keyed_root(key: &[u8; 32], content: &[u8]) -> [u8; 32] {
    *keyed_hash(key, content).as_bytes()
}

#[cfg(any(test, feature = "mocks"))]
fn expected_mode<'a>(key: Option<&'a [u8; 32]>) -> Mode<'a> {
    match key {
        Some(key) => Mode::KeyedHash(key),
        None => Mode::Hash,
    }
}

#[cfg(any(test, feature = "mocks"))]
fn expected_left_subtree_length(length: u64) -> u64 {
    let mut left = 1024u64;
    while left * 2 < length {
        left *= 2;
    }
    left
}

#[cfg(any(test, feature = "mocks"))]
fn expected_leaf_cv(bytes: &[u8], offset: u64, key: Option<&[u8; 32]>) -> [u8; 32] {
    let mut hasher = match key {
        Some(key) => Hasher::new_keyed(key),
        None => Hasher::new(),
    };
    hasher.set_input_offset(offset);
    hasher.update(bytes);
    hasher.finalize_non_root()
}

#[cfg(any(test, feature = "mocks"))]
fn expected_cv(bytes: &[u8], offset: u64, key: Option<&[u8; 32]>) -> [u8; 32] {
    if bytes.len() <= 1024 {
        return expected_leaf_cv(bytes, offset, key);
    }
    let left_length = expected_left_subtree_length(
        u64::try_from(bytes.len()).expect("a subtree length fits a u64"),
    );
    let split = usize::try_from(left_length).expect("a subtree split fits a usize");
    let left = expected_cv(&bytes[..split], offset, key);
    let right = expected_cv(&bytes[split..], offset + left_length, key);
    merge_subtrees_non_root(&left, &right, expected_mode(key))
}

#[cfg(any(test, feature = "mocks"))]
pub fn expected_outboard(content: &[u8], key: Option<&[u8; 32]>) -> Vec<u8> {
    let mut outboard = u64::try_from(content.len())
        .expect("a content length fits a u64")
        .to_le_bytes()
        .to_vec();
    expected_write_subtree(content, 0, key, &mut outboard);
    outboard
}

#[cfg(any(test, feature = "mocks"))]
fn expected_write_subtree(
    bytes: &[u8],
    offset: u64,
    key: Option<&[u8; 32]>,
    outboard: &mut Vec<u8>,
) -> [u8; 32] {
    if bytes.len() <= 1024 {
        return expected_leaf_cv(bytes, offset, key);
    }
    let position = outboard.len();
    outboard.resize(position + 64, 0);
    let left_length = expected_left_subtree_length(
        u64::try_from(bytes.len()).expect("a subtree length fits a u64"),
    );
    let split = usize::try_from(left_length).expect("a subtree split fits a usize");
    let left = expected_write_subtree(&bytes[..split], offset, key, outboard);
    let right = expected_write_subtree(&bytes[split..], offset + left_length, key, outboard);
    outboard[position..position + 32].copy_from_slice(&left);
    outboard[position + 32..position + 64].copy_from_slice(&right);
    merge_subtrees_non_root(&left, &right, expected_mode(key))
}

#[cfg(any(test, feature = "mocks"))]
pub fn expected_path(content: &[u8], chunk_index: u64, key: Option<&[u8; 32]>) -> Vec<[u8; 32]> {
    let mut siblings = Vec::new();
    let mut bytes = content;
    let mut offset = 0u64;
    while bytes.len() > 1024 {
        let left_length = expected_left_subtree_length(
            u64::try_from(bytes.len()).expect("a subtree length fits a u64"),
        );
        let split = usize::try_from(left_length).expect("a subtree split fits a usize");
        if chunk_index * 1024 < offset + left_length {
            siblings.push(expected_cv(&bytes[split..], offset + left_length, key));
            bytes = &bytes[..split];
        } else {
            siblings.push(expected_cv(&bytes[..split], offset, key));
            offset += left_length;
            bytes = &bytes[split..];
        }
    }
    siblings
}
