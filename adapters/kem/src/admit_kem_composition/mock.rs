#![allow(dead_code, clippy::too_many_arguments)]

use super::interface::{
    AdmitKemCompositionDeps, AdmitKemCompositionFn, AdmitKemCompositionParams,
    AdmitKemCompositionPayload, AdmitKemCompositionReturn, AdmitKemCompositionSuccessReturn,
};
use crate::factory::provides::{
    IdentityScope, KemConcrete, KemConcreteOverrides, build_kem_concrete,
};
use crate::lookup_kem_declaration::provides::{
    LookupKemDeclarationFn, mock_lookup_kem_declaration,
};

#[derive(Default)]
pub struct AdmitKemCompositionDepsOverrides {
    pub lookup_kem_declaration: Option<LookupKemDeclarationFn>,
}

pub fn build_admit_kem_composition_deps(
    overrides: AdmitKemCompositionDepsOverrides,
) -> AdmitKemCompositionDeps {
    AdmitKemCompositionDeps {
        lookup_kem_declaration: overrides
            .lookup_kem_declaration
            .unwrap_or(mock_lookup_kem_declaration),
    }
}

#[derive(Default)]
pub struct AdmitKemCompositionParamsOverrides {
    pub concrete: Option<KemConcrete>,
    pub scope: Option<IdentityScope>,
}

pub fn build_admit_kem_composition_params(
    overrides: AdmitKemCompositionParamsOverrides,
) -> AdmitKemCompositionParams {
    AdmitKemCompositionParams {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_kem_concrete(KemConcrete::Bb1DepthOne, KemConcreteOverrides)),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

#[derive(Default)]
pub struct AdmitKemCompositionPayloadOverrides;

pub fn build_admit_kem_composition_payload(
    _overrides: AdmitKemCompositionPayloadOverrides,
) -> AdmitKemCompositionPayload {
    AdmitKemCompositionPayload
}

#[derive(Default)]
pub struct AdmitKemCompositionSuccessReturnOverrides {
    pub concrete: Option<KemConcrete>,
    pub scope: Option<IdentityScope>,
}

pub fn build_admit_kem_composition_success_return(
    overrides: AdmitKemCompositionSuccessReturnOverrides,
) -> AdmitKemCompositionSuccessReturn {
    AdmitKemCompositionSuccessReturn {
        concrete: overrides
            .concrete
            .unwrap_or_else(|| build_kem_concrete(KemConcrete::Bb1DepthOne, KemConcreteOverrides)),
        scope: overrides.scope.unwrap_or(IdentityScope::Entitlement),
    }
}

pub fn mock_admit_kem_composition(
    _deps: &AdmitKemCompositionDeps,
    _params: AdmitKemCompositionParams,
    _payload: AdmitKemCompositionPayload,
) -> AdmitKemCompositionReturn {
    let _: AdmitKemCompositionFn = mock_admit_kem_composition;
    Ok(build_admit_kem_composition_success_return(
        Default::default(),
    ))
}
