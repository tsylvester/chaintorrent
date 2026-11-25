#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::os::provides::{OsRandomSource, OsRandomSourceConstructorParams};
use interface::{
    ConsumeRandomSourceParams, ConsumeRandomSourcePayload, CreateRandomSourceDeps,
    CreateRandomSourceParams, CreateRandomSourcePayload, CreateRandomSourceReturn,
    CreateRandomSourceSuccessReturn, IRandomSourceConsumer, RandomSourceKind,
};
#[cfg(any(test, feature = "mocks"))]
use mock::{MockIRandomSourceAdapter, MockIRandomSourceAdapterConstructorParams};

pub fn create_random_source<C: IRandomSourceConsumer>(
    deps: &CreateRandomSourceDeps<C>,
    params: CreateRandomSourceParams,
    _payload: CreateRandomSourcePayload,
) -> CreateRandomSourceReturn<C::Output> {
    match params.kind {
        RandomSourceKind::OperatingSystem => {
            let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
            Ok(CreateRandomSourceSuccessReturn {
                output: deps.consumer.consume_random_source(
                    ConsumeRandomSourceParams,
                    ConsumeRandomSourcePayload { source },
                ),
            })
        }
        #[cfg(any(test, feature = "mocks"))]
        RandomSourceKind::Mock(mode) => {
            let Ok(source) =
                MockIRandomSourceAdapter::try_new(MockIRandomSourceAdapterConstructorParams {
                    failure_mode: mode,
                });
            Ok(CreateRandomSourceSuccessReturn {
                output: deps.consumer.consume_random_source(
                    ConsumeRandomSourceParams,
                    ConsumeRandomSourcePayload { source },
                ),
            })
        }
    }
}
