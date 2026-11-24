use crate::factory::provides::{
    PAIRING_INTERFACE_VERSION, PairingConcrete, PairingCurve, PairingDeclaration,
    PrecompileEncoding, TargetGroupEncodingIdentifier, VerifierGroupArithmetic,
};
use core::convert::Infallible;

/// The inverse of three in the scalar field, the multiple by which the
/// library's reduced pairing exceeds the identifier's exact value.
pub struct Bls12381Halo2curvesPairing {
    pub(super) reduced_pairing_correction: halo2curves::bls12381::Fr,
}

pub struct Bls12381Halo2curvesPairingConstructorParams;

pub type Bls12381Halo2curvesPairingTryNewReturn = Result<Bls12381Halo2curvesPairing, Infallible>;

#[derive(Clone)]
pub struct Bls12381Halo2curvesScalar {
    pub(super) value: halo2curves::bls12381::Fr,
}

#[derive(Clone)]
pub struct Bls12381Halo2curvesG1 {
    pub(super) value: halo2curves::bls12381::G1Affine,
}

#[derive(Clone)]
pub struct Bls12381Halo2curvesG2 {
    pub(super) value: halo2curves::bls12381::G2Affine,
}

pub struct Bls12381Halo2curvesGt {
    pub(super) value: halo2curves::bls12381::Gt,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Bls12381Halo2curvesEncodedG1 {
    pub(super) bytes: [u8; 128],
}

#[derive(Clone, PartialEq, Eq)]
pub struct Bls12381Halo2curvesEncodedG2 {
    pub(super) bytes: [u8; 256],
}

pub struct Bls12381Halo2curvesEncodedScalar {
    pub(super) bytes: [u8; 32],
}

pub struct Bls12381Halo2curvesEncodedGt {
    pub(super) bytes: [u8; 576],
}

impl Bls12381Halo2curvesPairing {
    pub const DECLARATION: PairingDeclaration = PairingDeclaration {
        concrete: PairingConcrete::Bls12381Halo2curves,
        curve: PairingCurve::Bls12381,
        verifier_group_arithmetic: VerifierGroupArithmetic::BothGroups,
        precompile_encoding: PrecompileEncoding::Eip2537,
        target_group_encoding: TargetGroupEncodingIdentifier::Bls12381V1,
        adapter_version: 1,
        interface_version: PAIRING_INTERFACE_VERSION,
    };
}
