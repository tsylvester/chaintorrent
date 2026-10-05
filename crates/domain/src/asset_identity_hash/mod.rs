mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParams,
    AssetIdentityHashTryNewErrorReturn, AssetIdentityHashTryNewReturn,
};

impl AssetIdentityHash {
    pub fn try_new(params: AssetIdentityHashConstructorParams) -> AssetIdentityHashTryNewReturn {
        if params.bytes.iter().all(|byte| *byte == 0) {
            return Err(AssetIdentityHashTryNewErrorReturn::AllZero);
        }
        Ok(AssetIdentityHash {
            bytes: params.bytes,
        })
    }

    pub fn as_bytes(&self) -> &[u8; ASSET_IDENTITY_HASH_LENGTH] {
        &self.bytes
    }
}
