pub const ASSET_IDENTITY_HASH_LENGTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetIdentityHash {
    pub(super) bytes: [u8; ASSET_IDENTITY_HASH_LENGTH],
}

pub struct AssetIdentityHashConstructorParams {
    pub bytes: [u8; ASSET_IDENTITY_HASH_LENGTH],
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssetIdentityHashTryNewErrorReturn {
    AllZero,
}

pub type AssetIdentityHashTryNewReturn =
    Result<AssetIdentityHash, AssetIdentityHashTryNewErrorReturn>;
