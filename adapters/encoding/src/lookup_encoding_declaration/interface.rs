use crate::factory::provides::{EncodingConcrete, EncodingDeclaration};
use core::convert::Infallible;

pub struct LookupEncodingDeclarationDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupEncodingDeclarationParams {
    pub concrete: EncodingConcrete,
}

pub struct LookupEncodingDeclarationPayload;

pub struct LookupEncodingDeclarationSuccessReturn {
    pub declaration: EncodingDeclaration,
}

pub type LookupEncodingDeclarationReturn =
    Result<LookupEncodingDeclarationSuccessReturn, Infallible>;

pub type LookupEncodingDeclarationFn = fn(
    &LookupEncodingDeclarationDeps,
    LookupEncodingDeclarationParams,
    LookupEncodingDeclarationPayload,
) -> LookupEncodingDeclarationReturn;
