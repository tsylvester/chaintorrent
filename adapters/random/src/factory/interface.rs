use crate::os::provides::OsRandomSourceFillBytesErrorReturn;
use core::convert::Infallible;
use domain::Secret;

pub const RANDOM_SOURCE_INTERFACE_VERSION: u32 = 1;

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy)]
pub enum MockIRandomSourceAdapterFailureMode {
    Succeeds,
    FillBytesRefused,
}

pub enum RandomSourceKind {
    OperatingSystem,
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIRandomSourceAdapterFailureMode),
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

#[derive(Debug, PartialEq, Eq)]
pub enum FillBytesErrorReturn {
    OperatingSystem(OsRandomSourceFillBytesErrorReturn),
    #[cfg(any(test, feature = "mocks"))]
    MockIRandomSourceAdapter,
}

pub type FillBytesReturn = Result<FillBytesSuccessReturn, FillBytesErrorReturn>;

pub trait IRandomSourceAdapter {
    fn declaration(&self) -> RandomSourceDeclaration;
    fn fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn;
}

pub struct CreateRandomSourceDeps;

pub struct CreateRandomSourceParams {
    pub kind: RandomSourceKind,
}

pub struct CreateRandomSourcePayload;

pub struct CreateRandomSourceSuccessReturn {
    pub adapter: Box<dyn IRandomSourceAdapter>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateRandomSourceErrorReturn {
    OperatingSystem(Infallible),
    #[cfg(any(test, feature = "mocks"))]
    MockIRandomSourceAdapter(Infallible),
}

pub type CreateRandomSourceReturn =
    Result<CreateRandomSourceSuccessReturn, CreateRandomSourceErrorReturn>;

pub type CreateRandomSourceFn = fn(
    &CreateRandomSourceDeps,
    CreateRandomSourceParams,
    CreateRandomSourcePayload,
) -> CreateRandomSourceReturn;
