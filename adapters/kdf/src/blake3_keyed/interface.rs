use crate::factory::provides::{KDF_INTERFACE_VERSION, KdfConcrete, KdfDeclaration, KdfIdentifier};
use core::convert::Infallible;

pub struct Blake3KeyedKdf;

pub struct Blake3KeyedKdfConstructorParams;

pub type Blake3KeyedKdfTryNewReturn = Result<Blake3KeyedKdf, Infallible>;

pub const BLAKE3_KEYED_WRAPPING_KEY_CONTEXT: &str = "ChainTorrent v1 wrapping-key";
pub const BLAKE3_KEYED_PUBLISHER_ROOT_CONTEXT: &str = "ChainTorrent v1 publisher-root";
pub const BLAKE3_KEYED_ASSET_ROOT_CONTEXT: &str = "ChainTorrent v1 asset-root";
pub const BLAKE3_KEYED_MASTER_SCALAR_CONTEXT: &str = "ChainTorrent v1 master-scalar";
pub const BLAKE3_KEYED_IDENTITY_BASES_CONTEXT: &str = "ChainTorrent v1 identity-bases";
pub const BLAKE3_KEYED_CAPSULE_RANDOMNESS_CONTEXT: &str = "ChainTorrent v1 capsule-randomness";
pub const BLAKE3_KEYED_PIECE_GROUP_KEY_CONTEXT: &str = "ChainTorrent v1 piece-group-key";
pub const BLAKE3_KEYED_PLAINTEXT_ROOT_KEY_CONTEXT: &str = "ChainTorrent v1 plaintext-root";

#[derive(Debug, PartialEq, Eq)]
pub enum Blake3KeyedKdfDeriveKeyErrorReturn {
    KeyMaterialLengthExceedsPrefix { length: usize },
}

impl Blake3KeyedKdf {
    pub const DECLARATION: KdfDeclaration = KdfDeclaration {
        concrete: KdfConcrete::Blake3Keyed,
        identifier: KdfIdentifier::Blake3KeyedV1,
        adapter_version: 1,
        interface_version: KDF_INTERFACE_VERSION,
    };
}
