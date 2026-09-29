mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    ASSET_COORDINATE_SEPARATOR, AssetIdentity, AssetIdentityConstructorParams,
    AssetIdentityTryNewErrorReturn, AssetIdentityTryNewReturn,
};

impl AssetIdentity {
    pub fn try_new(params: AssetIdentityConstructorParams) -> AssetIdentityTryNewReturn {
        if params.name.is_empty() {
            return Err(AssetIdentityTryNewErrorReturn::EmptyName);
        }
        if let Some((index, byte)) = params
            .name
            .bytes()
            .enumerate()
            .find(|(_, byte)| !byte.is_ascii_graphic())
        {
            return Err(
                AssetIdentityTryNewErrorReturn::NameByteOutsideVisibleAscii { index, byte },
            );
        }
        if params.version.is_empty() {
            return Err(AssetIdentityTryNewErrorReturn::EmptyVersion);
        }
        if let Some((index, byte)) = params
            .version
            .bytes()
            .enumerate()
            .find(|(_, byte)| !byte.is_ascii_graphic() || *byte == ASSET_COORDINATE_SEPARATOR)
        {
            return Err(if byte == ASSET_COORDINATE_SEPARATOR {
                AssetIdentityTryNewErrorReturn::VersionContainsSeparator { index }
            } else {
                AssetIdentityTryNewErrorReturn::VersionByteOutsideVisibleAscii { index, byte }
            });
        }
        Ok(AssetIdentity {
            name: params.name,
            version: params.version,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
