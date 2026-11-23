mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::RandomSourceKind;
#[cfg(any(test, feature = "mocks"))]
use crate::factory::provides::{MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION, RandomSourceDeclaration};
use crate::os::provides::OsRandomSource;
use interface::{
    LookupRandomDeclarationDeps, LookupRandomDeclarationParams, LookupRandomDeclarationPayload,
    LookupRandomDeclarationReturn, LookupRandomDeclarationSuccessReturn,
};

pub fn lookup_random_declaration(
    _deps: &LookupRandomDeclarationDeps,
    params: LookupRandomDeclarationParams,
    _payload: LookupRandomDeclarationPayload,
) -> LookupRandomDeclarationReturn {
    match params.identifier {
        RandomSourceKind::OperatingSystem => Ok(LookupRandomDeclarationSuccessReturn {
            declaration: OsRandomSource::DECLARATION,
        }),
        #[cfg(any(test, feature = "mocks"))]
        RandomSourceKind::Mock(_) => Ok(LookupRandomDeclarationSuccessReturn {
            declaration: RandomSourceDeclaration {
                source: params.identifier,
                ..MOCK_I_RANDOM_SOURCE_ADAPTER_DECLARATION
            },
        }),
    }
}
