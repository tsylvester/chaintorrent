mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

#[cfg(any(test, feature = "mocks"))]
use crate::factory::mock::{MockIRandomSourceAdapter, MockIRandomSourceAdapterConstructorParams};
use crate::os::provides::{OsRandomSource, OsRandomSourceConstructorParams};
use interface::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    CreateRandomSourceReturn, CreateRandomSourceSuccessReturn, RandomSourceKind,
};

pub fn create_random_source(
    _deps: &CreateRandomSourceDeps,
    params: CreateRandomSourceParams,
    _payload: CreateRandomSourcePayload,
) -> CreateRandomSourceReturn {
    match params.kind {
        RandomSourceKind::OperatingSystem => {
            let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
            Ok(CreateRandomSourceSuccessReturn {
                adapter: Box::new(source),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        RandomSourceKind::Mock(mode) => {
            let Ok(source) =
                MockIRandomSourceAdapter::try_new(MockIRandomSourceAdapterConstructorParams {
                    failure_mode: mode,
                });
            Ok(CreateRandomSourceSuccessReturn {
                adapter: Box::new(source),
            })
        }
    }
}
