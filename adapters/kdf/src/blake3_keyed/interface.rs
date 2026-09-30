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

pub enum Blake3KeyedKdfDeriveKeyErrorReturn {
    KeyMaterialLengthExceedsPrefix { length: usize },
}
