#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::lookup_chain_declaration::provides::{
    LookupChainDeclarationDeps, LookupChainDeclarationParams, LookupChainDeclarationPayload,
};
use interface::{
    AdmitChainCompositionDeps, AdmitChainCompositionErrorReturn, AdmitChainCompositionFn,
    AdmitChainCompositionParams, AdmitChainCompositionPayload, AdmitChainCompositionReturn,
    AdmitChainCompositionSuccessReturn, ChainFormsIdentifierDisagreement,
};

pub fn admit_chain_composition(
    deps: &AdmitChainCompositionDeps,
    params: AdmitChainCompositionParams,
    payload: AdmitChainCompositionPayload<'_>,
) -> AdmitChainCompositionReturn {
    let _: AdmitChainCompositionFn = admit_chain_composition;
    let Ok(looked_up) = (deps.lookup_chain_declaration)(
        &LookupChainDeclarationDeps,
        LookupChainDeclarationParams {
            identifier: params.identifier,
        },
        LookupChainDeclarationPayload,
    );
    if looked_up.declaration.forms_identifier != payload.forms.identifier {
        return Err(AdmitChainCompositionErrorReturn::Incompatible {
            forms: ChainFormsIdentifierDisagreement {
                required: looked_up.declaration.forms_identifier,
                declared: payload.forms.identifier,
            },
        });
    }
    Ok(AdmitChainCompositionSuccessReturn)
}
