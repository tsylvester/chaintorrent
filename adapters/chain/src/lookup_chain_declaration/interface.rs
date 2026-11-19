use core::convert::Infallible;

use crate::factory::provides::{ChainDeclaration, ChainMemberIdentifier};

pub struct LookupChainDeclarationDeps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupChainDeclarationParams {
    pub identifier: ChainMemberIdentifier,
}

pub struct LookupChainDeclarationPayload;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupChainDeclarationSuccessReturn {
    pub declaration: ChainDeclaration,
}

pub type LookupChainDeclarationReturn = Result<LookupChainDeclarationSuccessReturn, Infallible>;

pub type LookupChainDeclarationFn = fn(
    &LookupChainDeclarationDeps,
    LookupChainDeclarationParams,
    LookupChainDeclarationPayload,
) -> LookupChainDeclarationReturn;
