#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use chain::ChainFormsIdentifierDisagreement;

use crate::generate::provides::EncodingIdentifierDisagreement;

use super::lookup_verifier_declaration::provides::{
    LookupVerifierDeclarationDeps, LookupVerifierDeclarationParams,
    LookupVerifierDeclarationPayload,
};
use interface::{
    AdmitVerifierCompositionDeps, AdmitVerifierCompositionErrorReturn, AdmitVerifierCompositionFn,
    AdmitVerifierCompositionParams, AdmitVerifierCompositionPayload,
    AdmitVerifierCompositionReturn, AdmitVerifierCompositionSuccessReturn,
    ChainMemberIdentifierDisagreement, ContractSuiteDisagreement, VerifierFormDisagreement,
};

pub fn admit_verifier_composition(
    deps: &AdmitVerifierCompositionDeps,
    params: AdmitVerifierCompositionParams,
    payload: AdmitVerifierCompositionPayload<'_>,
) -> AdmitVerifierCompositionReturn {
    let _: AdmitVerifierCompositionFn = admit_verifier_composition;
    let Ok(looked_up) = (deps.lookup_verifier_declaration)(
        &LookupVerifierDeclarationDeps,
        LookupVerifierDeclarationParams {
            identifier: params.identifier,
        },
        LookupVerifierDeclarationPayload,
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
    let chain_member = if looked_up.declaration.chain_member != payload.chain.identifier {
        Some(ChainMemberIdentifierDisagreement {
            required: looked_up.declaration.chain_member,
            declared: payload.chain.identifier,
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
    let verifier_form = if !looked_up
        .declaration
        .verifier_forms
        .contains(&payload.pairing.verifier_group_arithmetic)
    {
        Some(VerifierFormDisagreement {
            required: looked_up.declaration.verifier_forms,
            declared: payload.pairing.verifier_group_arithmetic,
        })
    } else {
        None
    };
    let contract_suite = if looked_up.declaration.contract_suite != payload.generate.contract_suite
    {
        Some(ContractSuiteDisagreement {
            required: looked_up.declaration.contract_suite,
            declared: payload.generate.contract_suite,
        })
    } else {
        None
    };
    if chain_forms.is_some()
        || chain_member.is_some()
        || encoding.is_some()
        || verifier_form.is_some()
        || contract_suite.is_some()
    {
        Err(AdmitVerifierCompositionErrorReturn::Incompatible {
            chain_forms,
            chain_member,
            encoding,
            verifier_form,
            contract_suite,
        })
    } else {
        Ok(AdmitVerifierCompositionSuccessReturn)
    }
}
