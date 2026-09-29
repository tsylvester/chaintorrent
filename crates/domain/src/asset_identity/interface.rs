pub const ASSET_COORDINATE_SEPARATOR: u8 = b'@';

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetIdentity {
    pub(super) name: String,
    pub(super) version: String,
}

pub struct AssetIdentityConstructorParams {
    pub name: String,
    pub version: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssetIdentityTryNewErrorReturn {
    EmptyName,
    NameByteOutsideVisibleAscii { index: usize, byte: u8 },
    EmptyVersion,
    VersionByteOutsideVisibleAscii { index: usize, byte: u8 },
    VersionContainsSeparator { index: usize },
}

pub type AssetIdentityTryNewReturn = Result<AssetIdentity, AssetIdentityTryNewErrorReturn>;
