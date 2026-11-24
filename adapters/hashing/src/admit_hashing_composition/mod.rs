mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::lookup_hashing_declaration::provides::{
    LookupHashingDeclarationDeps, LookupHashingDeclarationParams, LookupHashingDeclarationPayload,
};
use interface::{
    AdmitHashingCompositionDeps, AdmitHashingCompositionErrorReturn, AdmitHashingCompositionParams,
    AdmitHashingCompositionPayload, AdmitHashingCompositionReturn,
    AdmitHashingCompositionSuccessReturn,
};

pub fn admit_hashing_composition(
    deps: &AdmitHashingCompositionDeps,
    params: AdmitHashingCompositionParams,
    _payload: AdmitHashingCompositionPayload,
) -> AdmitHashingCompositionReturn {
    let Ok(looked_up) = (deps.lookup_hashing_declaration)(
        &LookupHashingDeclarationDeps,
        LookupHashingDeclarationParams {
            concrete: params.concrete,
        },
        LookupHashingDeclarationPayload,
    );
    if looked_up.declaration.identifier == params.identifier {
        Ok(AdmitHashingCompositionSuccessReturn)
    } else {
        Err(AdmitHashingCompositionErrorReturn::Incompatible {
            concrete: params.concrete,
            declared: looked_up.declaration.identifier,
            required: params.identifier,
        })
    }
}
