use core::convert::Infallible;

use crate::factory::provides::{PairingConcrete, PairingDeclaration};

pub struct LookupPairingDeclarationDeps;

pub struct LookupPairingDeclarationParams {
    pub concrete: PairingConcrete,
}

pub struct LookupPairingDeclarationPayload;

pub struct LookupPairingDeclarationSuccessReturn {
    pub declaration: PairingDeclaration,
}

pub type LookupPairingDeclarationReturn = Result<LookupPairingDeclarationSuccessReturn, Infallible>;

pub type LookupPairingDeclarationFn = fn(
    &LookupPairingDeclarationDeps,
    LookupPairingDeclarationParams,
    LookupPairingDeclarationPayload,
) -> LookupPairingDeclarationReturn;
