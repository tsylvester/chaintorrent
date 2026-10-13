#![allow(clippy::expect_used)]

#[cfg(test)]
use super::interface::{Keccak256HashToScalar, Keccak256HashToScalarConstructorParams};
#[cfg(any(test, feature = "mocks"))]
use crate::domain_tag::provides::DomainTag;
#[cfg(any(test, feature = "mocks"))]
use sha3::{Digest, Keccak256};

#[cfg(test)]
#[derive(Default)]
pub struct Keccak256HashToScalarConstructorParamsOverrides;

#[cfg(test)]
pub fn build_keccak256_hash_to_scalar_constructor_params(
    _overrides: Keccak256HashToScalarConstructorParamsOverrides,
) -> Keccak256HashToScalarConstructorParams {
    Keccak256HashToScalarConstructorParams
}

#[cfg(test)]
pub fn build_keccak256_hash_to_scalar(
    overrides: Keccak256HashToScalarConstructorParamsOverrides,
) -> Keccak256HashToScalar {
    let Ok(adapter) = Keccak256HashToScalar::try_new(
        build_keccak256_hash_to_scalar_constructor_params(overrides),
    );
    adapter
}

#[cfg(any(test, feature = "mocks"))]
pub const REFERENCE_TAG: &[u8] = b"ChainTorrent v1 reference tag";
#[cfg(any(test, feature = "mocks"))]
pub const REFERENCE_MESSAGE: &[u8] = b"reference message";
#[cfg(any(test, feature = "mocks"))]
pub const EMPTY_MESSAGE: &[u8] = b"";
#[cfg(any(test, feature = "mocks"))]
pub const OTHER_TAG: &[u8] = b"ChainTorrent v1 other tag";
#[cfg(any(test, feature = "mocks"))]
pub const BINDING_LONGER_TAG: &[u8] = b"binding-tag-abcd";
#[cfg(any(test, feature = "mocks"))]
pub const BINDING_SHORTER_MESSAGE: &[u8] = b"ef";

#[cfg(any(test, feature = "mocks"))]
pub fn expected_uniform_bytes(tag: &DomainTag, message: &[u8], uniform_length: usize) -> Vec<u8> {
    let prefix = u8::try_from(tag.as_bytes().len()).expect("a domain tag fits the one-byte prefix");
    let input = [&[prefix][..], tag.as_bytes(), message].concat();
    let digest = Keccak256::digest(&input);
    let mut uniform = vec![
        0u8;
        uniform_length
            .checked_sub(32)
            .expect("the sampling bound holds the digest")
    ];
    uniform.extend_from_slice(&digest);
    uniform
}
