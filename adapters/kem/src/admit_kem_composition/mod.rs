mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::lookup_kem_declaration::provides::{
    LookupKemDeclarationDeps, LookupKemDeclarationParams, LookupKemDeclarationPayload,
};
use interface::{
    AdmitKemCompositionDeps, AdmitKemCompositionErrorReturn, AdmitKemCompositionParams,
    AdmitKemCompositionPayload, AdmitKemCompositionReturn, AdmitKemCompositionSuccessReturn,
};

pub fn admit_kem_composition(
    deps: &AdmitKemCompositionDeps,
    params: AdmitKemCompositionParams,
    _payload: AdmitKemCompositionPayload,
) -> AdmitKemCompositionReturn {
    let declaration = match (deps.lookup_kem_declaration)(
        &LookupKemDeclarationDeps,
        LookupKemDeclarationParams {
            concrete: params.concrete,
        },
        LookupKemDeclarationPayload,
    ) {
        Ok(success) => success.declaration,
        Err(infallible) => match infallible {},
    };
    if declaration.identity_scopes.contains(&params.scope) {
        Ok(AdmitKemCompositionSuccessReturn {
            concrete: params.concrete,
            scope: params.scope,
        })
    } else {
        Err(AdmitKemCompositionErrorReturn::Incompatible {
            concrete: params.concrete,
            declared: declaration.identity_scopes,
            required: params.scope,
        })
    }
}
