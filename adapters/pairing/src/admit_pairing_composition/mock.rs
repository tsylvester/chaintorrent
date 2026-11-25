#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    AdmitPairingCompositionDeps, AdmitPairingCompositionParams, AdmitPairingCompositionPayload,
    AdmitPairingCompositionReturn, AdmitPairingCompositionSuccessReturn,
};
use crate::factory::provides::{
    PairingConcrete, PairingConcreteOverrides, PairingCurve, PairingDeclarationOverrides,
    PrecompileEncoding, TargetGroupEncodingIdentifier, VerifierGroupArithmetic,
    build_pairing_concrete, build_pairing_declaration,
};
use crate::lookup_pairing_declaration::provides::LookupPairingDeclarationFn;

#[derive(Default)]
pub struct AdmitPairingCompositionDepsOverrides;

pub fn build_admit_pairing_composition_deps(
    lookup_pairing_declaration: LookupPairingDeclarationFn,
    _overrides: AdmitPairingCompositionDepsOverrides,
) -> AdmitPairingCompositionDeps {
    AdmitPairingCompositionDeps {
        lookup_pairing_declaration,
    }
}

#[derive(Default)]
pub struct AdmitPairingCompositionParamsOverrides {
    pub identifier: Option<PairingConcrete>,
    pub required_curve: Option<PairingCurve>,
    pub required_verifier_group_arithmetic: Option<VerifierGroupArithmetic>,
    pub required_precompile_encodings: Option<Vec<PrecompileEncoding>>,
    pub required_target_group_encoding: Option<TargetGroupEncodingIdentifier>,
}

pub fn build_admit_pairing_composition_params(
    overrides: AdmitPairingCompositionParamsOverrides,
) -> AdmitPairingCompositionParams {
    let declaration = build_pairing_declaration(PairingDeclarationOverrides::default());
    AdmitPairingCompositionParams {
        identifier: overrides
            .identifier
            .unwrap_or(PairingConcrete::Bn254Arkworks),
        required_curve: overrides.required_curve.unwrap_or(declaration.curve),
        required_verifier_group_arithmetic: overrides
            .required_verifier_group_arithmetic
            .unwrap_or(declaration.verifier_group_arithmetic),
        required_precompile_encodings: overrides
            .required_precompile_encodings
            .unwrap_or_else(|| vec![declaration.precompile_encoding]),
        required_target_group_encoding: overrides
            .required_target_group_encoding
            .unwrap_or(declaration.target_group_encoding),
    }
}

#[derive(Default)]
pub struct AdmitPairingCompositionPayloadOverrides;

pub fn build_admit_pairing_composition_payload(
    _overrides: AdmitPairingCompositionPayloadOverrides,
) -> AdmitPairingCompositionPayload {
    AdmitPairingCompositionPayload
}

#[derive(Default)]
pub struct AdmitPairingCompositionSuccessReturnOverrides {
    pub identifier: Option<PairingConcrete>,
}

pub fn build_admit_pairing_composition_success_return(
    overrides: AdmitPairingCompositionSuccessReturnOverrides,
) -> AdmitPairingCompositionSuccessReturn {
    AdmitPairingCompositionSuccessReturn {
        identifier: overrides.identifier.unwrap_or_else(|| {
            build_pairing_concrete(PairingConcrete::Bn254Arkworks, PairingConcreteOverrides)
        }),
    }
}

pub fn mock_admit_pairing_composition(
    _deps: &AdmitPairingCompositionDeps,
    _params: AdmitPairingCompositionParams,
    _payload: AdmitPairingCompositionPayload,
) -> AdmitPairingCompositionReturn {
    Ok(build_admit_pairing_composition_success_return(
        Default::default(),
    ))
}
// it is the production function type and nothing else:
// `let f: AdmitPairingCompositionFn = mock_admit_pairing_composition;` holds
