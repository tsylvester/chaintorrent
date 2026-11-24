use crate::factory::provides::{
    PairingConcrete, PairingCurve, PrecompileEncoding, TargetGroupEncodingIdentifier,
    VerifierGroupArithmetic,
};
use crate::lookup_pairing_declaration::provides::LookupPairingDeclarationFn;

pub struct AdmitPairingCompositionDeps {
    pub lookup_pairing_declaration: LookupPairingDeclarationFn,
}

pub struct AdmitPairingCompositionParams {
    pub identifier: PairingConcrete,
    pub required_curve: PairingCurve,
    pub required_verifier_group_arithmetic: VerifierGroupArithmetic,
    pub required_precompile_encodings: Vec<PrecompileEncoding>,
    pub required_target_group_encoding: TargetGroupEncodingIdentifier,
}

pub struct AdmitPairingCompositionPayload;

#[derive(Debug, PartialEq, Eq)]
pub struct AdmitPairingCompositionSuccessReturn {
    pub identifier: PairingConcrete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingDisagreement {
    Curve {
        declared: PairingCurve,
        required: PairingCurve,
    },
    VerifierGroupArithmetic {
        declared: VerifierGroupArithmetic,
        required: VerifierGroupArithmetic,
    },
    PrecompileEncoding {
        declared: PrecompileEncoding,
        required: Vec<PrecompileEncoding>,
    },
    TargetGroupEncoding {
        declared: TargetGroupEncodingIdentifier,
        required: TargetGroupEncodingIdentifier,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum AdmitPairingCompositionErrorReturn {
    Incompatible {
        identifier: PairingConcrete,
        disagreements: Vec<PairingDisagreement>,
    },
}

pub type AdmitPairingCompositionReturn =
    Result<AdmitPairingCompositionSuccessReturn, AdmitPairingCompositionErrorReturn>;

pub type AdmitPairingCompositionFn = fn(
    &AdmitPairingCompositionDeps,
    AdmitPairingCompositionParams,
    AdmitPairingCompositionPayload,
) -> AdmitPairingCompositionReturn;
