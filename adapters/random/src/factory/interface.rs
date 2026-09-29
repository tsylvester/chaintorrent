use crate::os::provides::OsRandomSourceFillBytesErrorReturn;
use core::convert::Infallible;
use domain::Secret;

pub const RANDOM_SOURCE_INTERFACE_VERSION: u32 = 1;

pub enum RandomSourceKind {
    OperatingSystem,
}

pub struct RandomSourceDeclaration {
    pub source: RandomSourceKind,
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct FillBytesParams;

pub struct FillBytesPayload {
    pub length: usize,
}

pub struct FillBytesSuccessReturn {
    pub bytes: Secret<Vec<u8>>,
}

pub enum FillBytesErrorReturn {
    OperatingSystem(OsRandomSourceFillBytesErrorReturn),
}

pub type FillBytesReturn = Result<FillBytesSuccessReturn, FillBytesErrorReturn>;

pub trait IRandomSourceAdapter {
    fn fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn;
}

pub struct CreateRandomSourceDeps;

pub struct CreateRandomSourceParams;

pub struct CreateRandomSourcePayload {
    pub kind: RandomSourceKind,
}

pub struct CreateRandomSourceSuccessReturn {
    pub adapter: Box<dyn IRandomSourceAdapter>,
    pub declaration: RandomSourceDeclaration,
}

pub enum CreateRandomSourceErrorReturn {
    OperatingSystem(Infallible),
}

pub type CreateRandomSourceReturn =
    Result<CreateRandomSourceSuccessReturn, CreateRandomSourceErrorReturn>;

pub type CreateRandomSourceFn = fn(
    &CreateRandomSourceDeps,
    CreateRandomSourceParams,
    CreateRandomSourcePayload,
) -> CreateRandomSourceReturn;
