#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

#[cfg(test)]
use super::interface::{Blake3KeyedKdf, Blake3KeyedKdfConstructorParams};
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::{DerivationPurpose, reference_key_material};
#[cfg(any(test, feature = "mocks"))]
use blake3::Hasher;

#[cfg(test)]
#[derive(Default)]
pub struct Blake3KeyedKdfConstructorParamsOverrides;

#[cfg(test)]
pub fn build_blake3_keyed_kdf_constructor_params(
    _overrides: Blake3KeyedKdfConstructorParamsOverrides,
) -> Blake3KeyedKdfConstructorParams {
    Blake3KeyedKdfConstructorParams
}

#[cfg(test)]
pub fn build_blake3_keyed_kdf(
    overrides: Blake3KeyedKdfConstructorParamsOverrides,
) -> Blake3KeyedKdf {
    let Ok(adapter) = Blake3KeyedKdf::try_new(build_blake3_keyed_kdf_constructor_params(overrides));
    adapter
}

#[cfg(any(test, feature = "mocks"))]
pub fn expected_derived_key(purpose: DerivationPurpose, context: &[u8], length: usize) -> Vec<u8> {
    let context_string = match purpose {
        DerivationPurpose::WrappingKey => "ChainTorrent v1 wrapping-key",
        DerivationPurpose::PublisherRoot => "ChainTorrent v1 publisher-root",
        DerivationPurpose::AssetRoot => "ChainTorrent v1 asset-root",
        DerivationPurpose::MasterScalar => "ChainTorrent v1 master-scalar",
        DerivationPurpose::IdentityBases => "ChainTorrent v1 identity-bases",
        DerivationPurpose::CapsuleRandomness => "ChainTorrent v1 capsule-randomness",
        DerivationPurpose::PieceGroupKey => "ChainTorrent v1 piece-group-key",
        DerivationPurpose::PlaintextRootKey => "ChainTorrent v1 plaintext-root",
    };
    let key_material = reference_key_material().expose();
    let prefix = u64::try_from(key_material.len())
        .expect("a usize fits a u64 on every supported target")
        .to_be_bytes();
    let mut input = Vec::with_capacity(prefix.len() + key_material.len() + context.len());
    input.extend_from_slice(&prefix);
    input.extend_from_slice(key_material);
    input.extend_from_slice(context);
    let mut hasher = Hasher::new_derive_key(context_string);
    hasher.update(&input);
    let mut output = vec![0u8; length];
    hasher.finalize_xof().fill(&mut output);
    output
}
