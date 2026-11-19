#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use super::lookup_generate_declaration::provides::{
    LookupGenerateDeclarationDeps, LookupGenerateDeclarationParams,
    LookupGenerateDeclarationPayload,
};
use interface::{
    AdmitGenerateCompositionDeps, AdmitGenerateCompositionErrorReturn, AdmitGenerateCompositionFn,
    AdmitGenerateCompositionParams, AdmitGenerateCompositionPayload,
    AdmitGenerateCompositionReturn, AdmitGenerateCompositionSuccessReturn,
    ChainFormsIdentifierDisagreement, EncodingIdentifierDisagreement,
};

pub fn admit_generate_composition(
    deps: &AdmitGenerateCompositionDeps,
    params: AdmitGenerateCompositionParams,
    payload: AdmitGenerateCompositionPayload<'_>,
) -> AdmitGenerateCompositionReturn {
    let _: AdmitGenerateCompositionFn = admit_generate_composition;
    let Ok(looked_up) = (deps.lookup_generate_declaration)(
        &LookupGenerateDeclarationDeps,
        LookupGenerateDeclarationParams {
            identifier: params.identifier,
        },
        LookupGenerateDeclarationPayload,
    );
    let chain_forms =
        if looked_up.declaration.chain_forms_identifier != payload.chain_forms.identifier {
            Some(ChainFormsIdentifierDisagreement {
                required: looked_up.declaration.chain_forms_identifier,
                declared: payload.chain_forms.identifier,
            })
        } else {
            None
        };
    let encoding = if looked_up.declaration.encoding_identifier != payload.encoder.identifier {
        Some(EncodingIdentifierDisagreement {
            required: looked_up.declaration.encoding_identifier,
            declared: payload.encoder.identifier,
        })
    } else {
        None
    };
    if chain_forms.is_some() || encoding.is_some() {
        Err(AdmitGenerateCompositionErrorReturn::Incompatible {
            chain_forms,
            encoding,
        })
    } else {
        Ok(AdmitGenerateCompositionSuccessReturn)
    }
}
