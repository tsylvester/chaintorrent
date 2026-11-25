use crate::os::provides::OsRandomSourceFillBytesErrorReturn;
use core::convert::Infallible;
use domain::Secret;
#[cfg(any(test, feature = "mocks"))]
use serde::Serialize;

pub const RANDOM_SOURCE_INTERFACE_VERSION: u32 = 1;

#[cfg(any(test, feature = "mocks"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum MockIRandomSourceAdapterFailureMode {
    Succeeds,
    FillBytesRefused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RandomSourceKind {
    OperatingSystem,
    #[cfg(any(test, feature = "mocks"))]
    Mock(MockIRandomSourceAdapterFailureMode),
}

impl RandomSourceKind {
    pub const fn name(&self) -> &'static str {
        match self {
            RandomSourceKind::OperatingSystem => "operating_system",
            #[cfg(any(test, feature = "mocks"))]
            RandomSourceKind::Mock(_) => "mock",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    fn fill_bytes(&self, params: FillBytesParams, payload: FillBytesPayload) -> FillBytesReturn;
}

pub struct ConsumeRandomSourceParams;

pub struct ConsumeRandomSourcePayload<K: IRandomSourceAdapter> {
    pub source: K,
}

pub trait IRandomSourceConsumer {
    type Output;

    fn consume_random_source<K: IRandomSourceAdapter>(
        &self,
        params: ConsumeRandomSourceParams,
        payload: ConsumeRandomSourcePayload<K>,
    ) -> Self::Output;
}

pub struct CreateRandomSourceDeps<C: IRandomSourceConsumer> {
    pub consumer: C,
}

pub struct CreateRandomSourceParams {
    pub kind: RandomSourceKind,
}

pub struct CreateRandomSourcePayload;

pub struct CreateRandomSourceSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateRandomSourceErrorReturn {
    OperatingSystem(Infallible),
    #[cfg(any(test, feature = "mocks"))]
    MockIRandomSourceAdapter(Infallible),
}

pub type CreateRandomSourceReturn<O> =
    Result<CreateRandomSourceSuccessReturn<O>, CreateRandomSourceErrorReturn>;

pub type CreateRandomSourceFn<C> =
    fn(
        &CreateRandomSourceDeps<C>,
        CreateRandomSourceParams,
        CreateRandomSourcePayload,
    ) -> CreateRandomSourceReturn<<C as IRandomSourceConsumer>::Output>;

pub const RANDOM_SOURCE_KINDS: &[RandomSourceKind] = &[RandomSourceKind::OperatingSystem];
