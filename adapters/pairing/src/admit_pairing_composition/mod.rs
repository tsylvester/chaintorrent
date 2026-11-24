#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use crate::lookup_pairing_declaration::provides::{
    LookupPairingDeclarationDeps, LookupPairingDeclarationParams, LookupPairingDeclarationPayload,
};
use interface::{
    AdmitPairingCompositionDeps, AdmitPairingCompositionErrorReturn, AdmitPairingCompositionParams,
    AdmitPairingCompositionPayload, AdmitPairingCompositionReturn,
    AdmitPairingCompositionSuccessReturn, PairingDisagreement,
};

pub fn admit_pairing_composition(
    deps: &AdmitPairingCompositionDeps,
    params: AdmitPairingCompositionParams,
    _payload: AdmitPairingCompositionPayload,
) -> AdmitPairingCompositionReturn {
    let Ok(looked_up) = (deps.lookup_pairing_declaration)(
        &LookupPairingDeclarationDeps,
        LookupPairingDeclarationParams {
            concrete: params.identifier,
        },
        LookupPairingDeclarationPayload,
    );
    let mut disagreements = Vec::new();
    if looked_up.declaration.curve != params.required_curve {
        disagreements.push(PairingDisagreement::Curve {
            declared: looked_up.declaration.curve,
            required: params.required_curve,
        });
    }
    if looked_up.declaration.verifier_group_arithmetic != params.required_verifier_group_arithmetic
    {
        disagreements.push(PairingDisagreement::VerifierGroupArithmetic {
            declared: looked_up.declaration.verifier_group_arithmetic,
            required: params.required_verifier_group_arithmetic,
        });
    }
    if !params
        .required_precompile_encodings
        .contains(&looked_up.declaration.precompile_encoding)
    {
        disagreements.push(PairingDisagreement::PrecompileEncoding {
            declared: looked_up.declaration.precompile_encoding,
            required: params.required_precompile_encodings,
        });
    }
    if looked_up.declaration.target_group_encoding != params.required_target_group_encoding {
        disagreements.push(PairingDisagreement::TargetGroupEncoding {
            declared: looked_up.declaration.target_group_encoding,
            required: params.required_target_group_encoding,
        });
    }
    if disagreements.is_empty() {
        Ok(AdmitPairingCompositionSuccessReturn {
            identifier: params.identifier,
        })
    } else {
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: params.identifier,
            disagreements,
        })
    }
}
