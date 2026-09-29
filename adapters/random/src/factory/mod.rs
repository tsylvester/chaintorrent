mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::os::provides::{OsRandomSource, OsRandomSourceConstructorParams};
use interface::{
    CreateRandomSourceDeps, CreateRandomSourceParams, CreateRandomSourcePayload,
    CreateRandomSourceReturn, CreateRandomSourceSuccessReturn, RandomSourceKind,
};

pub fn create_random_source(
    _deps: &CreateRandomSourceDeps,
    _params: CreateRandomSourceParams,
    payload: CreateRandomSourcePayload,
) -> CreateRandomSourceReturn {
    match payload.kind {
        RandomSourceKind::OperatingSystem => {
            let Ok(source) = OsRandomSource::try_new(OsRandomSourceConstructorParams);
            Ok(CreateRandomSourceSuccessReturn {
                adapter: Box::new(source),
                declaration: OsRandomSource::DECLARATION,
            })
        }
    }
}
