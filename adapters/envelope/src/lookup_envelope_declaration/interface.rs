use core::convert::Infallible;

use crate::factory::provides::{KeyAgreementConcrete, KeyAgreementDeclaration};

pub struct LookupEnvelopeDeclarationDeps;

pub struct LookupEnvelopeDeclarationParams {
    pub concrete: KeyAgreementConcrete,
}

pub struct LookupEnvelopeDeclarationPayload;

pub struct LookupEnvelopeDeclarationSuccessReturn {
    pub declaration: KeyAgreementDeclaration,
}

pub type LookupEnvelopeDeclarationReturn =
    Result<LookupEnvelopeDeclarationSuccessReturn, Infallible>;

pub type LookupEnvelopeDeclarationFn = fn(
    &LookupEnvelopeDeclarationDeps,
    LookupEnvelopeDeclarationParams,
    LookupEnvelopeDeclarationPayload,
) -> LookupEnvelopeDeclarationReturn;
