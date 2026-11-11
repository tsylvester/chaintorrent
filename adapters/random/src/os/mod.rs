mod interface;
#[cfg(test)]
mod mock;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    FillBytesErrorReturn, FillBytesParams, FillBytesPayload, FillBytesReturn,
    FillBytesSuccessReturn, IRandomSourceAdapter, RANDOM_SOURCE_INTERFACE_VERSION,
    RandomSourceDeclaration, RandomSourceKind,
};
use domain::{Secret, SecretConstructorParams};
use interface::{
    OsRandomSource, OsRandomSourceConstructorParams, OsRandomSourceFillBytesErrorReturn,
    OsRandomSourceTryNewReturn,
};

impl OsRandomSource {
    pub const DECLARATION: RandomSourceDeclaration = RandomSourceDeclaration {
        source: RandomSourceKind::OperatingSystem,
        adapter_version: 1,
        interface_version: RANDOM_SOURCE_INTERFACE_VERSION,
    };

    pub fn try_new(_params: OsRandomSourceConstructorParams) -> OsRandomSourceTryNewReturn {
        Ok(OsRandomSource)
    }
}

impl IRandomSourceAdapter for OsRandomSource {
    fn declaration(&self) -> RandomSourceDeclaration {
        Self::DECLARATION
    }

    fn fill_bytes(&self, _params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn {
        let mut buffer = vec![0u8; payload.length];
        let result = getrandom::fill(&mut buffer);
        let Ok(bytes) = Secret::try_new(SecretConstructorParams { value: buffer });
        match result {
            Ok(()) => Ok(FillBytesSuccessReturn { bytes }),
            Err(error) => Err(FillBytesErrorReturn::OperatingSystem(
                OsRandomSourceFillBytesErrorReturn::OperatingSystem(error),
            )),
        }
    }
}
