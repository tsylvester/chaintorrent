use crate::factory::provides::{IdentityScope, KemConcrete};
use crate::lookup_kem_declaration::provides::LookupKemDeclarationFn;

pub struct AdmitKemCompositionDeps {
    pub lookup_kem_declaration: LookupKemDeclarationFn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitKemCompositionParams {
    pub concrete: KemConcrete,
    pub scope: IdentityScope,
}

pub struct AdmitKemCompositionPayload;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmitKemCompositionSuccessReturn {
    pub concrete: KemConcrete,
    pub scope: IdentityScope,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitKemCompositionErrorReturn {
    Incompatible {
        concrete: KemConcrete,
        declared: &'static [IdentityScope],
        required: IdentityScope,
    },
}

pub type AdmitKemCompositionReturn =
    Result<AdmitKemCompositionSuccessReturn, AdmitKemCompositionErrorReturn>;

pub type AdmitKemCompositionFn = fn(
    &AdmitKemCompositionDeps,
    AdmitKemCompositionParams,
    AdmitKemCompositionPayload,
) -> AdmitKemCompositionReturn;
