#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    AddG1Params, AddG1Payload, AddG1Return, AddG1SuccessReturn, AddG2Params, AddG2Payload,
    AddG2Return, AddG2SuccessReturn, AddScalarParams, AddScalarPayload, AddScalarReturn,
    AddScalarSuccessReturn, ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps,
    CreatePairingParams, CreatePairingPayload, CreatePairingReturn, CreatePairingSuccessReturn,
    DecodeG1ErrorReturn, DecodeG1Params, DecodeG1Return, DecodeG1SuccessReturn,
    DecodeG2ErrorReturn, DecodeG2Params, DecodeG2Return, DecodeG2SuccessReturn,
    DecodeScalarErrorReturn, DecodeScalarParams, DecodeScalarReturn, DecodeScalarSuccessReturn,
    EncodeG1Params, EncodeG1Payload, EncodeG1Return, EncodeG1SuccessReturn, EncodeG2Params,
    EncodeG2Payload, EncodeG2Return, EncodeG2SuccessReturn, EncodeGtParams, EncodeGtPayload,
    EncodeGtReturn, EncodeGtSuccessReturn, EncodeScalarParams, EncodeScalarPayload,
    EncodeScalarReturn, EncodeScalarSuccessReturn, G1GeneratorParams, G1GeneratorPayload,
    G1GeneratorReturn, G1GeneratorSuccessReturn, G1OutsideSubgroupEncodingErrorReturn,
    G1OutsideSubgroupEncodingParams, G1OutsideSubgroupEncodingPayload,
    G1OutsideSubgroupEncodingReturn, G1OutsideSubgroupEncodingSuccessReturn, G2GeneratorParams,
    G2GeneratorPayload, G2GeneratorReturn, G2GeneratorSuccessReturn,
    G2OutsideSubgroupEncodingErrorReturn, G2OutsideSubgroupEncodingParams,
    G2OutsideSubgroupEncodingPayload, G2OutsideSubgroupEncodingReturn,
    G2OutsideSubgroupEncodingSuccessReturn, IPairingAdapter, IPairingArithmetic, IPairingConsumer,
    IPairingReference, ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1Payload,
    IsIdentityG1Return, IsIdentityG1SuccessReturn, IsIdentityG2Params, IsIdentityG2Payload,
    IsIdentityG2Return, IsIdentityG2SuccessReturn, MockIPairingAdapterFailureMode, MsmG1Params,
    MsmG1Payload, MsmG1Return, MsmG1SuccessReturn, MsmG1Term, MsmG2Params, MsmG2Payload,
    MsmG2Return, MsmG2SuccessReturn, MsmG2Term, MulG1Params, MulG1Payload, MulG1Return,
    MulG1SuccessReturn, MulG2Params, MulG2Payload, MulG2Return, MulG2SuccessReturn,
    MulScalarParams, MulScalarPayload, MulScalarReturn, MulScalarSuccessReturn, NegG1Params,
    NegG1Payload, NegG1Return, NegG1SuccessReturn, NegG2Params, NegG2Payload, NegG2Return,
    NegG2SuccessReturn, NegScalarParams, NegScalarPayload, NegScalarReturn, NegScalarSuccessReturn,
    PAIRING_INTERFACE_VERSION, PairingConcrete, PairingCurve, PairingDeclaration,
    PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductIsOneReturn,
    PairingProductIsOneSuccessReturn, PairingProductParams, PairingProductPayload,
    PairingProductReturn, PairingProductSuccessReturn, PairingProductTerm, PrecompileEncoding,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    SampleUniformScalarReturn, SampleUniformScalarSuccessReturn, ScalarFieldOrderParams,
    ScalarFieldOrderPayload, ScalarFieldOrderReturn, ScalarFieldOrderSuccessReturn,
    TargetGroupEncodingIdentifier, VerifierGroupArithmetic,
};
use core::cell::Cell;
use core::convert::Infallible;
use domain::{Secret, SecretConstructorParams, SecretConstructorParamsOverrides, build_secret};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Default)]
pub struct PairingDeclarationOverrides {
    pub curve: Option<PairingCurve>,
    pub verifier_group_arithmetic: Option<VerifierGroupArithmetic>,
    pub precompile_encoding: Option<PrecompileEncoding>,
    pub target_group_encoding: Option<TargetGroupEncodingIdentifier>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_pairing_declaration(overrides: PairingDeclarationOverrides) -> PairingDeclaration {
    PairingDeclaration {
        curve: overrides.curve.unwrap_or(PairingCurve::Bn254),
        verifier_group_arithmetic: overrides
            .verifier_group_arithmetic
            .unwrap_or(VerifierGroupArithmetic::FirstGroupOnly),
        precompile_encoding: overrides
            .precompile_encoding
            .unwrap_or(PrecompileEncoding::Eip196Eip197),
        target_group_encoding: overrides
            .target_group_encoding
            .unwrap_or(TargetGroupEncodingIdentifier::Bn254V1),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(PAIRING_INTERFACE_VERSION),
    }
}

fn default_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    generator.point
}

fn default_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    generator.point
}

fn default_scalar_secret<P: IPairingAdapter>() -> Secret<P::Scalar> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(
                vec![1u8; P::Scalar::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            ),
        },
    )
    .expect("a uniform input of UNIFORM_BYTES_LENGTH samples a scalar")
    .scalar
}

fn default_scalar<P: IPairingAdapter>() -> P::Scalar {
    default_scalar_secret::<P>().expose().clone()
}

fn default_encoded_g1<P: IPairingAdapter>(pairing: &P) -> P::EncodedG1 {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: default_g1(pairing),
        },
    );
    encoded.bytes
}

fn default_encoded_g2<P: IPairingAdapter>(pairing: &P) -> P::EncodedG2 {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: default_g2(pairing),
        },
    );
    encoded.bytes
}

fn default_encoded_scalar<P: IPairingAdapter>(pairing: &P) -> Secret<P::EncodedScalar> {
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: default_scalar::<P>(),
        },
    );
    encoded.bytes
}

fn default_gt<P: IPairingArithmetic>(pairing: &P) -> P::Gt {
    let Ok(product) = pairing.pairing_product(
        PairingProductParams,
        PairingProductPayload {
            terms: vec![PairingProductTerm {
                g1: default_g1(pairing),
                g2: default_g2(pairing),
            }],
        },
    );
    product.product
}

fn default_encoded_gt<P: IPairingArithmetic>(pairing: &P) -> Secret<P::EncodedGt> {
    let Ok(encoded) = pairing.encode_gt(
        EncodeGtParams,
        EncodeGtPayload {
            value: default_gt(pairing),
        },
    );
    encoded.bytes
}

pub struct G1GeneratorSuccessReturnOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for G1GeneratorSuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_g1_generator_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: G1GeneratorSuccessReturnOverrides<P::G1>,
) -> G1GeneratorSuccessReturn<P::G1> {
    G1GeneratorSuccessReturn {
        point: overrides.point.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct G2GeneratorSuccessReturnOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for G2GeneratorSuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_g2_generator_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: G2GeneratorSuccessReturnOverrides<P::G2>,
) -> G2GeneratorSuccessReturn<P::G2> {
    G2GeneratorSuccessReturn {
        point: overrides.point.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct AddG1PayloadOverrides<G> {
    pub left: Option<G>,
    pub right: Option<G>,
}

impl<G> Default for AddG1PayloadOverrides<G> {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

pub fn build_add_g1_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: AddG1PayloadOverrides<P::G1>,
) -> AddG1Payload<P::G1> {
    AddG1Payload {
        left: overrides.left.unwrap_or_else(|| default_g1(pairing)),
        right: overrides.right.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct AddG1SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

impl<G> Default for AddG1SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { sum: None }
    }
}

pub fn build_add_g1_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: AddG1SuccessReturnOverrides<P::G1>,
) -> AddG1SuccessReturn<P::G1> {
    AddG1SuccessReturn {
        sum: overrides.sum.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct AddG2PayloadOverrides<G> {
    pub left: Option<G>,
    pub right: Option<G>,
}

impl<G> Default for AddG2PayloadOverrides<G> {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

pub fn build_add_g2_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: AddG2PayloadOverrides<P::G2>,
) -> AddG2Payload<P::G2> {
    AddG2Payload {
        left: overrides.left.unwrap_or_else(|| default_g2(pairing)),
        right: overrides.right.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct AddG2SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

impl<G> Default for AddG2SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { sum: None }
    }
}

pub fn build_add_g2_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: AddG2SuccessReturnOverrides<P::G2>,
) -> AddG2SuccessReturn<P::G2> {
    AddG2SuccessReturn {
        sum: overrides.sum.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct MulG1PayloadOverrides<G, S> {
    pub point: Option<G>,
    pub scalar: Option<S>,
}

impl<G, S> Default for MulG1PayloadOverrides<G, S> {
    fn default() -> Self {
        Self {
            point: None,
            scalar: None,
        }
    }
}

pub fn build_mul_g1_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: MulG1PayloadOverrides<P::G1, P::Scalar>,
) -> MulG1Payload<P::G1, P::Scalar> {
    MulG1Payload {
        point: overrides.point.unwrap_or_else(|| default_g1(pairing)),
        scalar: overrides.scalar.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct MulG1SuccessReturnOverrides<G> {
    pub product: Option<G>,
}

impl<G> Default for MulG1SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { product: None }
    }
}

pub fn build_mul_g1_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: MulG1SuccessReturnOverrides<P::G1>,
) -> MulG1SuccessReturn<P::G1> {
    MulG1SuccessReturn {
        product: overrides.product.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct MulG2PayloadOverrides<G, S> {
    pub point: Option<G>,
    pub scalar: Option<S>,
}

impl<G, S> Default for MulG2PayloadOverrides<G, S> {
    fn default() -> Self {
        Self {
            point: None,
            scalar: None,
        }
    }
}

pub fn build_mul_g2_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: MulG2PayloadOverrides<P::G2, P::Scalar>,
) -> MulG2Payload<P::G2, P::Scalar> {
    MulG2Payload {
        point: overrides.point.unwrap_or_else(|| default_g2(pairing)),
        scalar: overrides.scalar.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct MulG2SuccessReturnOverrides<G> {
    pub product: Option<G>,
}

impl<G> Default for MulG2SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { product: None }
    }
}

pub fn build_mul_g2_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: MulG2SuccessReturnOverrides<P::G2>,
) -> MulG2SuccessReturn<P::G2> {
    MulG2SuccessReturn {
        product: overrides.product.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct MsmG1TermOverrides<G, S> {
    pub base: Option<G>,
    pub scalar: Option<S>,
}

impl<G, S> Default for MsmG1TermOverrides<G, S> {
    fn default() -> Self {
        Self {
            base: None,
            scalar: None,
        }
    }
}

pub fn build_msm_g1_term<P: IPairingAdapter>(
    pairing: &P,
    overrides: MsmG1TermOverrides<P::G1, P::Scalar>,
) -> MsmG1Term<P::G1, P::Scalar> {
    MsmG1Term {
        base: overrides.base.unwrap_or_else(|| default_g1(pairing)),
        scalar: overrides.scalar.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct MsmG1PayloadOverrides<G, S> {
    pub terms: Option<Vec<MsmG1Term<G, S>>>,
}

impl<G, S> Default for MsmG1PayloadOverrides<G, S> {
    fn default() -> Self {
        Self { terms: None }
    }
}

pub fn build_msm_g1_payload<P: IPairingAdapter>(
    _pairing: &P,
    overrides: MsmG1PayloadOverrides<P::G1, P::Scalar>,
) -> MsmG1Payload<P::G1, P::Scalar> {
    MsmG1Payload {
        terms: overrides.terms.unwrap_or_default(),
    }
}

pub struct MsmG1SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

impl<G> Default for MsmG1SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { sum: None }
    }
}

pub fn build_msm_g1_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: MsmG1SuccessReturnOverrides<P::G1>,
) -> MsmG1SuccessReturn<P::G1> {
    MsmG1SuccessReturn {
        sum: overrides.sum.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct MsmG2TermOverrides<G, S> {
    pub base: Option<G>,
    pub scalar: Option<S>,
}

impl<G, S> Default for MsmG2TermOverrides<G, S> {
    fn default() -> Self {
        Self {
            base: None,
            scalar: None,
        }
    }
}

pub fn build_msm_g2_term<P: IPairingAdapter>(
    pairing: &P,
    overrides: MsmG2TermOverrides<P::G2, P::Scalar>,
) -> MsmG2Term<P::G2, P::Scalar> {
    MsmG2Term {
        base: overrides.base.unwrap_or_else(|| default_g2(pairing)),
        scalar: overrides.scalar.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct MsmG2PayloadOverrides<G, S> {
    pub terms: Option<Vec<MsmG2Term<G, S>>>,
}

impl<G, S> Default for MsmG2PayloadOverrides<G, S> {
    fn default() -> Self {
        Self { terms: None }
    }
}

pub fn build_msm_g2_payload<P: IPairingAdapter>(
    _pairing: &P,
    overrides: MsmG2PayloadOverrides<P::G2, P::Scalar>,
) -> MsmG2Payload<P::G2, P::Scalar> {
    MsmG2Payload {
        terms: overrides.terms.unwrap_or_default(),
    }
}

pub struct MsmG2SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

impl<G> Default for MsmG2SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { sum: None }
    }
}

pub fn build_msm_g2_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: MsmG2SuccessReturnOverrides<P::G2>,
) -> MsmG2SuccessReturn<P::G2> {
    MsmG2SuccessReturn {
        sum: overrides.sum.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct PairingProductTermOverrides<G1, G2> {
    pub g1: Option<G1>,
    pub g2: Option<G2>,
}

impl<G1, G2> Default for PairingProductTermOverrides<G1, G2> {
    fn default() -> Self {
        Self { g1: None, g2: None }
    }
}

pub fn build_pairing_product_term<P: IPairingAdapter>(
    pairing: &P,
    overrides: PairingProductTermOverrides<P::G1, P::G2>,
) -> PairingProductTerm<P::G1, P::G2> {
    PairingProductTerm {
        g1: overrides.g1.unwrap_or_else(|| default_g1(pairing)),
        g2: overrides.g2.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct PairingProductIsOnePayloadOverrides<G1, G2> {
    pub terms: Option<Vec<PairingProductTerm<G1, G2>>>,
}

impl<G1, G2> Default for PairingProductIsOnePayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { terms: None }
    }
}

pub fn build_pairing_product_is_one_payload<P: IPairingAdapter>(
    _pairing: &P,
    overrides: PairingProductIsOnePayloadOverrides<P::G1, P::G2>,
) -> PairingProductIsOnePayload<P::G1, P::G2> {
    PairingProductIsOnePayload {
        terms: overrides.terms.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct PairingProductIsOneSuccessReturnOverrides {
    pub is_one: Option<bool>,
}

pub fn build_pairing_product_is_one_success_return(
    overrides: PairingProductIsOneSuccessReturnOverrides,
) -> PairingProductIsOneSuccessReturn {
    PairingProductIsOneSuccessReturn {
        is_one: overrides.is_one.unwrap_or(true),
    }
}

pub struct DecodeG1SuccessReturnOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for DecodeG1SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_decode_g1_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: DecodeG1SuccessReturnOverrides<P::G1>,
) -> DecodeG1SuccessReturn<P::G1> {
    DecodeG1SuccessReturn {
        point: overrides.point.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct DecodeG2SuccessReturnOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for DecodeG2SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_decode_g2_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: DecodeG2SuccessReturnOverrides<P::G2>,
) -> DecodeG2SuccessReturn<P::G2> {
    DecodeG2SuccessReturn {
        point: overrides.point.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct DecodeScalarSuccessReturnOverrides<S> {
    pub scalar: Option<S>,
}

impl<S> Default for DecodeScalarSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { scalar: None }
    }
}

pub fn build_decode_scalar_success_return<P: IPairingAdapter>(
    _pairing: &P,
    overrides: DecodeScalarSuccessReturnOverrides<P::Scalar>,
) -> DecodeScalarSuccessReturn<P::Scalar> {
    DecodeScalarSuccessReturn {
        scalar: overrides.scalar.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct EncodeG1PayloadOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for EncodeG1PayloadOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_encode_g1_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: EncodeG1PayloadOverrides<P::G1>,
) -> EncodeG1Payload<P::G1> {
    EncodeG1Payload {
        point: overrides.point.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct EncodeG1SuccessReturnOverrides<E> {
    pub bytes: Option<E>,
}

impl<E> Default for EncodeG1SuccessReturnOverrides<E> {
    fn default() -> Self {
        Self { bytes: None }
    }
}

pub fn build_encode_g1_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: EncodeG1SuccessReturnOverrides<P::EncodedG1>,
) -> EncodeG1SuccessReturn<P::EncodedG1> {
    EncodeG1SuccessReturn {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| default_encoded_g1(pairing)),
    }
}

pub struct EncodeG2PayloadOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for EncodeG2PayloadOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_encode_g2_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: EncodeG2PayloadOverrides<P::G2>,
) -> EncodeG2Payload<P::G2> {
    EncodeG2Payload {
        point: overrides.point.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct EncodeG2SuccessReturnOverrides<E> {
    pub bytes: Option<E>,
}

impl<E> Default for EncodeG2SuccessReturnOverrides<E> {
    fn default() -> Self {
        Self { bytes: None }
    }
}

pub fn build_encode_g2_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: EncodeG2SuccessReturnOverrides<P::EncodedG2>,
) -> EncodeG2SuccessReturn<P::EncodedG2> {
    EncodeG2SuccessReturn {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| default_encoded_g2(pairing)),
    }
}

pub struct EncodeScalarPayloadOverrides<S> {
    pub scalar: Option<S>,
}

impl<S> Default for EncodeScalarPayloadOverrides<S> {
    fn default() -> Self {
        Self { scalar: None }
    }
}

pub fn build_encode_scalar_payload<P: IPairingAdapter>(
    _pairing: &P,
    overrides: EncodeScalarPayloadOverrides<P::Scalar>,
) -> EncodeScalarPayload<P::Scalar> {
    EncodeScalarPayload {
        scalar: overrides.scalar.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct EncodeScalarSuccessReturnOverrides<E: Zeroize> {
    pub bytes: Option<Secret<E>>,
}

impl<E: Zeroize> Default for EncodeScalarSuccessReturnOverrides<E> {
    fn default() -> Self {
        Self { bytes: None }
    }
}

pub fn build_encode_scalar_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: EncodeScalarSuccessReturnOverrides<P::EncodedScalar>,
) -> EncodeScalarSuccessReturn<P::EncodedScalar> {
    EncodeScalarSuccessReturn {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| default_encoded_scalar(pairing)),
    }
}

pub struct AddScalarPayloadOverrides<S> {
    pub left: Option<S>,
    pub right: Option<S>,
}

impl<S> Default for AddScalarPayloadOverrides<S> {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

pub fn build_add_scalar_payload<P: IPairingArithmetic>(
    _pairing: &P,
    overrides: AddScalarPayloadOverrides<P::Scalar>,
) -> AddScalarPayload<P::Scalar> {
    AddScalarPayload {
        left: overrides.left.unwrap_or_else(default_scalar::<P>),
        right: overrides.right.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct AddScalarSuccessReturnOverrides<S> {
    pub sum: Option<S>,
}

impl<S> Default for AddScalarSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { sum: None }
    }
}

pub fn build_add_scalar_success_return<P: IPairingArithmetic>(
    _pairing: &P,
    overrides: AddScalarSuccessReturnOverrides<P::Scalar>,
) -> AddScalarSuccessReturn<P::Scalar> {
    AddScalarSuccessReturn {
        sum: overrides.sum.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct MulScalarPayloadOverrides<S> {
    pub left: Option<S>,
    pub right: Option<S>,
}

impl<S> Default for MulScalarPayloadOverrides<S> {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

pub fn build_mul_scalar_payload<P: IPairingArithmetic>(
    _pairing: &P,
    overrides: MulScalarPayloadOverrides<P::Scalar>,
) -> MulScalarPayload<P::Scalar> {
    MulScalarPayload {
        left: overrides.left.unwrap_or_else(default_scalar::<P>),
        right: overrides.right.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct MulScalarSuccessReturnOverrides<S> {
    pub product: Option<S>,
}

impl<S> Default for MulScalarSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { product: None }
    }
}

pub fn build_mul_scalar_success_return<P: IPairingArithmetic>(
    _pairing: &P,
    overrides: MulScalarSuccessReturnOverrides<P::Scalar>,
) -> MulScalarSuccessReturn<P::Scalar> {
    MulScalarSuccessReturn {
        product: overrides.product.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct NegScalarPayloadOverrides<S> {
    pub scalar: Option<S>,
}

impl<S> Default for NegScalarPayloadOverrides<S> {
    fn default() -> Self {
        Self { scalar: None }
    }
}

pub fn build_neg_scalar_payload<P: IPairingArithmetic>(
    _pairing: &P,
    overrides: NegScalarPayloadOverrides<P::Scalar>,
) -> NegScalarPayload<P::Scalar> {
    NegScalarPayload {
        scalar: overrides.scalar.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct NegScalarSuccessReturnOverrides<S> {
    pub negation: Option<S>,
}

impl<S> Default for NegScalarSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { negation: None }
    }
}

pub fn build_neg_scalar_success_return<P: IPairingArithmetic>(
    _pairing: &P,
    overrides: NegScalarSuccessReturnOverrides<P::Scalar>,
) -> NegScalarSuccessReturn<P::Scalar> {
    NegScalarSuccessReturn {
        negation: overrides.negation.unwrap_or_else(default_scalar::<P>),
    }
}

pub struct NegG1PayloadOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for NegG1PayloadOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_neg_g1_payload<P: IPairingArithmetic>(
    pairing: &P,
    overrides: NegG1PayloadOverrides<P::G1>,
) -> NegG1Payload<P::G1> {
    NegG1Payload {
        point: overrides.point.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct NegG1SuccessReturnOverrides<G> {
    pub negation: Option<G>,
}

impl<G> Default for NegG1SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { negation: None }
    }
}

pub fn build_neg_g1_success_return<P: IPairingArithmetic>(
    pairing: &P,
    overrides: NegG1SuccessReturnOverrides<P::G1>,
) -> NegG1SuccessReturn<P::G1> {
    NegG1SuccessReturn {
        negation: overrides.negation.unwrap_or_else(|| default_g1(pairing)),
    }
}

pub struct NegG2PayloadOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for NegG2PayloadOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_neg_g2_payload<P: IPairingArithmetic>(
    pairing: &P,
    overrides: NegG2PayloadOverrides<P::G2>,
) -> NegG2Payload<P::G2> {
    NegG2Payload {
        point: overrides.point.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct NegG2SuccessReturnOverrides<G> {
    pub negation: Option<G>,
}

impl<G> Default for NegG2SuccessReturnOverrides<G> {
    fn default() -> Self {
        Self { negation: None }
    }
}

pub fn build_neg_g2_success_return<P: IPairingArithmetic>(
    pairing: &P,
    overrides: NegG2SuccessReturnOverrides<P::G2>,
) -> NegG2SuccessReturn<P::G2> {
    NegG2SuccessReturn {
        negation: overrides.negation.unwrap_or_else(|| default_g2(pairing)),
    }
}

pub struct IsIdentityG1PayloadOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for IsIdentityG1PayloadOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_is_identity_g1_payload<P: IPairingArithmetic>(
    pairing: &P,
    overrides: IsIdentityG1PayloadOverrides<P::G1>,
) -> IsIdentityG1Payload<P::G1> {
    IsIdentityG1Payload {
        point: overrides.point.unwrap_or_else(|| default_g1(pairing)),
    }
}

#[derive(Default)]
pub struct IsIdentityG1SuccessReturnOverrides {
    pub is_identity: Option<bool>,
}

pub fn build_is_identity_g1_success_return(
    overrides: IsIdentityG1SuccessReturnOverrides,
) -> IsIdentityG1SuccessReturn {
    IsIdentityG1SuccessReturn {
        is_identity: overrides.is_identity.unwrap_or(false),
    }
}

pub struct IsIdentityG2PayloadOverrides<G> {
    pub point: Option<G>,
}

impl<G> Default for IsIdentityG2PayloadOverrides<G> {
    fn default() -> Self {
        Self { point: None }
    }
}

pub fn build_is_identity_g2_payload<P: IPairingArithmetic>(
    pairing: &P,
    overrides: IsIdentityG2PayloadOverrides<P::G2>,
) -> IsIdentityG2Payload<P::G2> {
    IsIdentityG2Payload {
        point: overrides.point.unwrap_or_else(|| default_g2(pairing)),
    }
}

#[derive(Default)]
pub struct IsIdentityG2SuccessReturnOverrides {
    pub is_identity: Option<bool>,
}

pub fn build_is_identity_g2_success_return(
    overrides: IsIdentityG2SuccessReturnOverrides,
) -> IsIdentityG2SuccessReturn {
    IsIdentityG2SuccessReturn {
        is_identity: overrides.is_identity.unwrap_or(false),
    }
}

pub struct PairingProductPayloadOverrides<G1, G2> {
    pub terms: Option<Vec<PairingProductTerm<G1, G2>>>,
}

impl<G1, G2> Default for PairingProductPayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { terms: None }
    }
}

pub fn build_pairing_product_payload<P: IPairingArithmetic>(
    _pairing: &P,
    overrides: PairingProductPayloadOverrides<P::G1, P::G2>,
) -> PairingProductPayload<P::G1, P::G2> {
    PairingProductPayload {
        terms: overrides.terms.unwrap_or_default(),
    }
}

pub struct PairingProductSuccessReturnOverrides<T> {
    pub product: Option<T>,
}

impl<T> Default for PairingProductSuccessReturnOverrides<T> {
    fn default() -> Self {
        Self { product: None }
    }
}

pub fn build_pairing_product_success_return<P: IPairingArithmetic>(
    pairing: &P,
    overrides: PairingProductSuccessReturnOverrides<P::Gt>,
) -> PairingProductSuccessReturn<P::Gt> {
    PairingProductSuccessReturn {
        product: overrides.product.unwrap_or_else(|| default_gt(pairing)),
    }
}

pub struct EncodeGtPayloadOverrides<T> {
    pub value: Option<T>,
}

impl<T> Default for EncodeGtPayloadOverrides<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

pub fn build_encode_gt_payload<P: IPairingArithmetic>(
    pairing: &P,
    overrides: EncodeGtPayloadOverrides<P::Gt>,
) -> EncodeGtPayload<P::Gt> {
    EncodeGtPayload {
        value: overrides.value.unwrap_or_else(|| default_gt(pairing)),
    }
}

pub struct EncodeGtSuccessReturnOverrides<E: Zeroize> {
    pub bytes: Option<Secret<E>>,
}

impl<E: Zeroize> Default for EncodeGtSuccessReturnOverrides<E> {
    fn default() -> Self {
        Self { bytes: None }
    }
}

pub fn build_encode_gt_success_return<P: IPairingArithmetic>(
    pairing: &P,
    overrides: EncodeGtSuccessReturnOverrides<P::EncodedGt>,
) -> EncodeGtSuccessReturn<P::EncodedGt> {
    EncodeGtSuccessReturn {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| default_encoded_gt(pairing)),
    }
}

#[derive(Default)]
pub struct SampleUniformScalarPayloadOverrides {
    pub uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_sample_uniform_scalar_payload(
    overrides: SampleUniformScalarPayloadOverrides,
) -> SampleUniformScalarPayload {
    SampleUniformScalarPayload {
        uniform: overrides
            .uniform
            .unwrap_or_else(|| build_secret(vec![0u8; 64], SecretConstructorParamsOverrides)),
    }
}

pub struct SampleUniformScalarSuccessReturnOverrides<S: Zeroize> {
    pub scalar: Option<Secret<S>>,
}

impl<S: Zeroize> Default for SampleUniformScalarSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { scalar: None }
    }
}

pub fn build_sample_uniform_scalar_success_return<P: IPairingAdapter>(
    _pairing: &P,
    overrides: SampleUniformScalarSuccessReturnOverrides<P::Scalar>,
) -> SampleUniformScalarSuccessReturn<P::Scalar> {
    SampleUniformScalarSuccessReturn {
        scalar: overrides.scalar.unwrap_or_else(default_scalar_secret::<P>),
    }
}

#[derive(Default)]
pub struct ScalarFieldOrderSuccessReturnOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_scalar_field_order_success_return(
    overrides: ScalarFieldOrderSuccessReturnOverrides,
) -> ScalarFieldOrderSuccessReturn {
    ScalarFieldOrderSuccessReturn {
        bytes: overrides.bytes.unwrap_or_default(),
    }
}

pub struct G1OutsideSubgroupEncodingSuccessReturnOverrides<E> {
    pub bytes: Option<Option<E>>,
}

impl<E> Default for G1OutsideSubgroupEncodingSuccessReturnOverrides<E> {
    fn default() -> Self {
        Self { bytes: None }
    }
}

pub fn build_g1_outside_subgroup_encoding_success_return<P: IPairingAdapter>(
    _pairing: &P,
    overrides: G1OutsideSubgroupEncodingSuccessReturnOverrides<P::EncodedG1>,
) -> G1OutsideSubgroupEncodingSuccessReturn<P::EncodedG1> {
    G1OutsideSubgroupEncodingSuccessReturn {
        bytes: overrides.bytes.unwrap_or_default(),
    }
}

pub struct G2OutsideSubgroupEncodingSuccessReturnOverrides<E> {
    pub bytes: Option<E>,
}

impl<E> Default for G2OutsideSubgroupEncodingSuccessReturnOverrides<E> {
    fn default() -> Self {
        Self { bytes: None }
    }
}

pub fn build_g2_outside_subgroup_encoding_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: G2OutsideSubgroupEncodingSuccessReturnOverrides<P::EncodedG2>,
) -> G2OutsideSubgroupEncodingSuccessReturn<P::EncodedG2> {
    G2OutsideSubgroupEncodingSuccessReturn {
        bytes: overrides
            .bytes
            .unwrap_or_else(|| default_encoded_g2(pairing)),
    }
}

pub(crate) const MOCK_GROUP_ORDER: u64 = 65521;

#[derive(Clone)]
pub(crate) struct MockScalar {
    residue: u64,
}

impl Zeroize for MockScalar {
    fn zeroize(&mut self) {
        self.residue.zeroize();
    }
}

impl Drop for MockScalar {
    fn drop(&mut self) {
        self.residue.zeroize();
    }
}

impl ZeroizeOnDrop for MockScalar {}

#[derive(Clone)]
pub(crate) struct MockG1 {
    residue: u64,
}

impl Zeroize for MockG1 {
    fn zeroize(&mut self) {
        self.residue.zeroize();
    }
}

impl Drop for MockG1 {
    fn drop(&mut self) {
        self.residue.zeroize();
    }
}

impl ZeroizeOnDrop for MockG1 {}

#[derive(Clone)]
pub(crate) struct MockG2 {
    residue: u64,
}

impl Zeroize for MockG2 {
    fn zeroize(&mut self) {
        self.residue.zeroize();
    }
}

impl Drop for MockG2 {
    fn drop(&mut self) {
        self.residue.zeroize();
    }
}

impl ZeroizeOnDrop for MockG2 {}

#[derive(Clone)]
pub(crate) struct MockGt {
    residue: u64,
}

impl Zeroize for MockGt {
    fn zeroize(&mut self) {
        self.residue.zeroize();
    }
}

impl Drop for MockGt {
    fn drop(&mut self) {
        self.residue.zeroize();
    }
}

impl ZeroizeOnDrop for MockGt {}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct MockEncodedG1 {
    bytes: [u8; 8],
}

impl AsRef<[u8]> for MockEncodedG1 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct MockEncodedG2 {
    bytes: [u8; 8],
}

impl AsRef<[u8]> for MockEncodedG2 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

pub(crate) struct MockEncodedScalar {
    bytes: [u8; 8],
}

impl AsRef<[u8]> for MockEncodedScalar {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Zeroize for MockEncodedScalar {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}

pub(crate) struct MockEncodedGt {
    bytes: [u8; 8],
}

impl AsRef<[u8]> for MockEncodedGt {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Zeroize for MockEncodedGt {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}

pub(crate) struct MockIPairingAdapterConstructorParams {
    pub failure_mode: MockIPairingAdapterFailureMode,
}

#[derive(Default)]
pub struct MockIPairingAdapterConstructorParamsOverrides {
    pub failure_mode: Option<MockIPairingAdapterFailureMode>,
}

pub(crate) fn build_mock_i_pairing_adapter_constructor_params(
    overrides: MockIPairingAdapterConstructorParamsOverrides,
) -> MockIPairingAdapterConstructorParams {
    MockIPairingAdapterConstructorParams {
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIPairingAdapterFailureMode::Succeeds),
    }
}

std::thread_local! {
    pub(crate) static MOCK_SAMPLING_MODE: Cell<MockIPairingAdapterFailureMode> =
        const { Cell::new(MockIPairingAdapterFailureMode::Succeeds) };
}

pub(crate) struct MockIPairingAdapter {
    pub(super) failure_mode: MockIPairingAdapterFailureMode,
}

pub(crate) type MockIPairingAdapterTryNewReturn = Result<MockIPairingAdapter, Infallible>;

impl MockIPairingAdapter {
    pub(crate) fn try_new(
        params: MockIPairingAdapterConstructorParams,
    ) -> MockIPairingAdapterTryNewReturn {
        MOCK_SAMPLING_MODE.with(|mode| mode.set(params.failure_mode));
        Ok(MockIPairingAdapter {
            failure_mode: params.failure_mode,
        })
    }
}

pub(crate) fn build_mock_i_pairing_adapter(
    overrides: MockIPairingAdapterConstructorParamsOverrides,
) -> MockIPairingAdapter {
    let Ok(adapter) =
        MockIPairingAdapter::try_new(build_mock_i_pairing_adapter_constructor_params(overrides));
    adapter
}

impl IPairingAdapter for MockIPairingAdapter {
    const DECLARATION: PairingDeclaration = PairingDeclaration {
        curve: PairingCurve::Mock,
        verifier_group_arithmetic: VerifierGroupArithmetic::FirstGroupOnly,
        precompile_encoding: PrecompileEncoding::Mock,
        target_group_encoding: TargetGroupEncodingIdentifier::Mock,
        adapter_version: 1,
        interface_version: PAIRING_INTERFACE_VERSION,
    };
    const CONCRETE: PairingConcrete =
        PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds);
    type Scalar = MockScalar;
    type G1 = MockG1;
    type G2 = MockG2;
    type EncodedG1 = MockEncodedG1;
    type EncodedG2 = MockEncodedG2;
    type EncodedScalar = MockEncodedScalar;

    fn g1_generator(
        &self,
        _params: G1GeneratorParams,
        _payload: G1GeneratorPayload,
    ) -> G1GeneratorReturn<Self::G1> {
        Ok(G1GeneratorSuccessReturn {
            point: MockG1 { residue: 1 },
        })
    }

    fn g2_generator(
        &self,
        _params: G2GeneratorParams,
        _payload: G2GeneratorPayload,
    ) -> G2GeneratorReturn<Self::G2> {
        Ok(G2GeneratorSuccessReturn {
            point: MockG2 { residue: 2 },
        })
    }

    fn add_g1(
        &self,
        _params: AddG1Params,
        payload: AddG1Payload<Self::G1>,
    ) -> AddG1Return<Self::G1> {
        Ok(AddG1SuccessReturn {
            sum: MockG1 {
                residue: (payload.left.residue + payload.right.residue) % MOCK_GROUP_ORDER,
            },
        })
    }

    fn add_g2(
        &self,
        _params: AddG2Params,
        payload: AddG2Payload<Self::G2>,
    ) -> AddG2Return<Self::G2> {
        Ok(AddG2SuccessReturn {
            sum: MockG2 {
                residue: (payload.left.residue + payload.right.residue) % (2 * MOCK_GROUP_ORDER),
            },
        })
    }

    fn mul_g1(
        &self,
        _params: MulG1Params,
        payload: MulG1Payload<Self::G1, Self::Scalar>,
    ) -> MulG1Return<Self::G1> {
        Ok(MulG1SuccessReturn {
            product: MockG1 {
                residue: (payload.point.residue * payload.scalar.residue) % MOCK_GROUP_ORDER,
            },
        })
    }

    fn mul_g2(
        &self,
        _params: MulG2Params,
        payload: MulG2Payload<Self::G2, Self::Scalar>,
    ) -> MulG2Return<Self::G2> {
        Ok(MulG2SuccessReturn {
            product: MockG2 {
                residue: (payload.point.residue * payload.scalar.residue) % (2 * MOCK_GROUP_ORDER),
            },
        })
    }

    fn msm_g1(
        &self,
        _params: MsmG1Params,
        payload: MsmG1Payload<Self::G1, Self::Scalar>,
    ) -> MsmG1Return<Self::G1> {
        let sum = payload.terms.iter().fold(0u64, |sum, term| {
            (sum + term.base.residue * term.scalar.residue) % MOCK_GROUP_ORDER
        });
        Ok(MsmG1SuccessReturn {
            sum: MockG1 { residue: sum },
        })
    }

    fn msm_g2(
        &self,
        _params: MsmG2Params,
        payload: MsmG2Payload<Self::G2, Self::Scalar>,
    ) -> MsmG2Return<Self::G2> {
        let sum = payload.terms.iter().fold(0u64, |sum, term| {
            (sum + term.base.residue * term.scalar.residue) % (2 * MOCK_GROUP_ORDER)
        });
        Ok(MsmG2SuccessReturn {
            sum: MockG2 { residue: sum },
        })
    }

    fn pairing_product_is_one(
        &self,
        _params: PairingProductIsOneParams,
        payload: PairingProductIsOnePayload<Self::G1, Self::G2>,
    ) -> PairingProductIsOneReturn {
        let product = payload.terms.iter().fold(0u64, |sum, term| {
            (sum + term.g1.residue * (term.g2.residue / 2)) % MOCK_GROUP_ORDER
        });
        Ok(PairingProductIsOneSuccessReturn {
            is_one: product == 0,
        })
    }

    fn decode_g1(&self, _params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1> {
        if payload.len() != 8 {
            return Err(DecodeG1ErrorReturn::WrongLength {
                expected: 8,
                actual: payload.len(),
            });
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(payload);
        let residue = u64::from_be_bytes(bytes);
        if residue >= MOCK_GROUP_ORDER {
            return Err(DecodeG1ErrorReturn::NonCanonicalCoordinate);
        }
        if residue == 0 {
            return Ok(DecodeG1SuccessReturn {
                point: MockG1 { residue: 0 },
            });
        }
        if self.failure_mode == MockIPairingAdapterFailureMode::DecodeG1NotOnCurve {
            return Err(DecodeG1ErrorReturn::NotOnCurve);
        }
        if self.failure_mode == MockIPairingAdapterFailureMode::DecodeG1NotInSubgroup {
            return Err(DecodeG1ErrorReturn::NotInSubgroup);
        }
        Ok(DecodeG1SuccessReturn {
            point: MockG1 { residue },
        })
    }

    fn decode_g2(&self, _params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Self::G2> {
        if payload.len() != 8 {
            return Err(DecodeG2ErrorReturn::WrongLength {
                expected: 8,
                actual: payload.len(),
            });
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(payload);
        let residue = u64::from_be_bytes(bytes);
        if residue >= 2 * MOCK_GROUP_ORDER {
            return Err(DecodeG2ErrorReturn::NonCanonicalCoordinate);
        }
        if residue == 0 {
            return Ok(DecodeG2SuccessReturn {
                point: MockG2 { residue: 0 },
            });
        }
        if self.failure_mode == MockIPairingAdapterFailureMode::DecodeG2NotOnCurve {
            return Err(DecodeG2ErrorReturn::NotOnCurve);
        }
        if residue % 2 != 0 {
            return Err(DecodeG2ErrorReturn::NotInSubgroup);
        }
        Ok(DecodeG2SuccessReturn {
            point: MockG2 { residue },
        })
    }

    fn decode_scalar(
        &self,
        _params: DecodeScalarParams,
        payload: &[u8],
    ) -> DecodeScalarReturn<Self::Scalar> {
        if payload.len() != 8 {
            return Err(DecodeScalarErrorReturn::WrongLength {
                expected: 8,
                actual: payload.len(),
            });
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(payload);
        let residue = u64::from_be_bytes(bytes);
        if residue >= MOCK_GROUP_ORDER {
            return Err(DecodeScalarErrorReturn::NonCanonical);
        }
        Ok(DecodeScalarSuccessReturn {
            scalar: MockScalar { residue },
        })
    }

    fn encode_g1(
        &self,
        _params: EncodeG1Params,
        payload: EncodeG1Payload<Self::G1>,
    ) -> EncodeG1Return<Self::EncodedG1> {
        Ok(EncodeG1SuccessReturn {
            bytes: MockEncodedG1 {
                bytes: payload.point.residue.to_be_bytes(),
            },
        })
    }

    fn encode_g2(
        &self,
        _params: EncodeG2Params,
        payload: EncodeG2Payload<Self::G2>,
    ) -> EncodeG2Return<Self::EncodedG2> {
        Ok(EncodeG2SuccessReturn {
            bytes: MockEncodedG2 {
                bytes: payload.point.residue.to_be_bytes(),
            },
        })
    }

    fn encode_scalar(
        &self,
        _params: EncodeScalarParams,
        payload: EncodeScalarPayload<Self::Scalar>,
    ) -> EncodeScalarReturn<Self::EncodedScalar> {
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: MockEncodedScalar {
                bytes: payload.scalar.residue.to_be_bytes(),
            },
        });
        Ok(EncodeScalarSuccessReturn { bytes })
    }
}

impl ISampleUniformScalar for MockScalar {
    const UNIFORM_BYTES_LENGTH: usize = 16;

    fn sample_from_uniform_bytes(
        _params: SampleUniformScalarParams,
        payload: SampleUniformScalarPayload,
    ) -> SampleUniformScalarReturn<Self> {
        let actual = payload.uniform.expose().len();
        if actual != Self::UNIFORM_BYTES_LENGTH
            || MOCK_SAMPLING_MODE.with(|mode| mode.get())
                == MockIPairingAdapterFailureMode::SampleScalarWrongLength
        {
            return Err(SampleUniformScalarErrorReturn::WrongLength {
                expected: Self::UNIFORM_BYTES_LENGTH,
                actual,
            });
        }
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(payload.uniform.expose());
        let residue = (u128::from_be_bytes(bytes) % u128::from(MOCK_GROUP_ORDER)) as u64;
        let Ok(scalar) = Secret::try_new(SecretConstructorParams {
            value: MockScalar { residue },
        });
        Ok(SampleUniformScalarSuccessReturn { scalar })
    }
}

impl IPairingArithmetic for MockIPairingAdapter {
    type Gt = MockGt;
    type EncodedGt = MockEncodedGt;

    fn add_scalar(
        &self,
        _params: AddScalarParams,
        payload: AddScalarPayload<Self::Scalar>,
    ) -> AddScalarReturn<Self::Scalar> {
        Ok(AddScalarSuccessReturn {
            sum: MockScalar {
                residue: (payload.left.residue + payload.right.residue) % MOCK_GROUP_ORDER,
            },
        })
    }

    fn mul_scalar(
        &self,
        _params: MulScalarParams,
        payload: MulScalarPayload<Self::Scalar>,
    ) -> MulScalarReturn<Self::Scalar> {
        Ok(MulScalarSuccessReturn {
            product: MockScalar {
                residue: (payload.left.residue * payload.right.residue) % MOCK_GROUP_ORDER,
            },
        })
    }

    fn neg_scalar(
        &self,
        _params: NegScalarParams,
        payload: NegScalarPayload<Self::Scalar>,
    ) -> NegScalarReturn<Self::Scalar> {
        let negation = if payload.scalar.residue == 0 {
            0
        } else {
            MOCK_GROUP_ORDER - payload.scalar.residue
        };
        Ok(NegScalarSuccessReturn {
            negation: MockScalar { residue: negation },
        })
    }

    fn neg_g1(
        &self,
        _params: NegG1Params,
        payload: NegG1Payload<Self::G1>,
    ) -> NegG1Return<Self::G1> {
        let negation = if payload.point.residue == 0 {
            0
        } else {
            MOCK_GROUP_ORDER - payload.point.residue
        };
        Ok(NegG1SuccessReturn {
            negation: MockG1 { residue: negation },
        })
    }

    fn neg_g2(
        &self,
        _params: NegG2Params,
        payload: NegG2Payload<Self::G2>,
    ) -> NegG2Return<Self::G2> {
        let negation = if payload.point.residue == 0 {
            0
        } else {
            2 * MOCK_GROUP_ORDER - payload.point.residue
        };
        Ok(NegG2SuccessReturn {
            negation: MockG2 { residue: negation },
        })
    }

    fn is_identity_g1(
        &self,
        _params: IsIdentityG1Params,
        payload: IsIdentityG1Payload<Self::G1>,
    ) -> IsIdentityG1Return {
        Ok(IsIdentityG1SuccessReturn {
            is_identity: payload.point.residue == 0,
        })
    }

    fn is_identity_g2(
        &self,
        _params: IsIdentityG2Params,
        payload: IsIdentityG2Payload<Self::G2>,
    ) -> IsIdentityG2Return {
        Ok(IsIdentityG2SuccessReturn {
            is_identity: payload.point.residue == 0,
        })
    }

    fn pairing_product(
        &self,
        _params: PairingProductParams,
        payload: PairingProductPayload<Self::G1, Self::G2>,
    ) -> PairingProductReturn<Self::Gt> {
        let product = payload.terms.iter().fold(0u64, |sum, term| {
            (sum + term.g1.residue * (term.g2.residue / 2)) % MOCK_GROUP_ORDER
        });
        Ok(PairingProductSuccessReturn {
            product: MockGt { residue: product },
        })
    }

    fn encode_gt(
        &self,
        _params: EncodeGtParams,
        payload: EncodeGtPayload<Self::Gt>,
    ) -> EncodeGtReturn<Self::EncodedGt> {
        let Ok(bytes) = Secret::try_new(SecretConstructorParams {
            value: MockEncodedGt {
                bytes: payload.value.residue.to_be_bytes(),
            },
        });
        Ok(EncodeGtSuccessReturn { bytes })
    }
}

impl IPairingReference for MockIPairingAdapter {
    fn scalar_field_order(
        &self,
        _params: ScalarFieldOrderParams,
        _payload: ScalarFieldOrderPayload,
    ) -> ScalarFieldOrderReturn {
        Ok(ScalarFieldOrderSuccessReturn {
            bytes: MOCK_GROUP_ORDER.to_be_bytes().to_vec(),
        })
    }

    fn g1_outside_subgroup_encoding(
        &self,
        _params: G1OutsideSubgroupEncodingParams,
        _payload: G1OutsideSubgroupEncodingPayload,
    ) -> G1OutsideSubgroupEncodingReturn<Self::EncodedG1> {
        if self.failure_mode == MockIPairingAdapterFailureMode::G1OutsideSubgroupSearchExhausted {
            return Err(G1OutsideSubgroupEncodingErrorReturn::SearchExhausted);
        }
        Ok(G1OutsideSubgroupEncodingSuccessReturn { bytes: None })
    }

    fn g2_outside_subgroup_encoding(
        &self,
        _params: G2OutsideSubgroupEncodingParams,
        _payload: G2OutsideSubgroupEncodingPayload,
    ) -> G2OutsideSubgroupEncodingReturn<Self::EncodedG2> {
        if self.failure_mode == MockIPairingAdapterFailureMode::G2OutsideSubgroupSearchExhausted {
            return Err(G2OutsideSubgroupEncodingErrorReturn::SearchExhausted);
        }
        Ok(G2OutsideSubgroupEncodingSuccessReturn {
            bytes: MockEncodedG2 {
                bytes: 1u64.to_be_bytes(),
            },
        })
    }
}

#[derive(Default)]
pub struct CreatePairingParamsOverrides {
    pub concrete: Option<PairingConcrete>,
    pub supported_encodings: Option<Vec<PrecompileEncoding>>,
    pub target_group_encoding: Option<TargetGroupEncodingIdentifier>,
}

pub fn build_create_pairing_params(overrides: CreatePairingParamsOverrides) -> CreatePairingParams {
    let concrete = overrides.concrete.unwrap_or(PairingConcrete::Mock(
        MockIPairingAdapterFailureMode::Succeeds,
    ));
    let target_group_encoding = overrides.target_group_encoding.unwrap_or(match concrete {
        PairingConcrete::Bn254Arkworks | PairingConcrete::Bn254Halo2curves => {
            TargetGroupEncodingIdentifier::Bn254V1
        }
        PairingConcrete::Bls12381Arkworks | PairingConcrete::Bls12381Halo2curves => {
            TargetGroupEncodingIdentifier::Bls12381V1
        }
        PairingConcrete::Mock(_) => TargetGroupEncodingIdentifier::Mock,
    });
    CreatePairingParams {
        concrete,
        supported_encodings: overrides.supported_encodings.unwrap_or_else(|| {
            vec![
                PrecompileEncoding::Eip196Eip197,
                PrecompileEncoding::Eip2537,
                PrecompileEncoding::Mock,
            ]
        }),
        target_group_encoding,
    }
}

#[derive(Default)]
pub struct CreatePairingSuccessReturnOverrides;

pub fn build_create_pairing_success_return<O>(
    output: O,
    _overrides: CreatePairingSuccessReturnOverrides,
) -> CreatePairingSuccessReturn<O> {
    CreatePairingSuccessReturn { output }
}

#[derive(Default)]
pub(crate) struct ConsumePairingPayloadOverrides;

pub(crate) fn build_consume_pairing_payload<P: IPairingAdapter>(
    adapter: P,
    _overrides: ConsumePairingPayloadOverrides,
) -> ConsumePairingPayload<P> {
    ConsumePairingPayload { adapter }
}

#[derive(Default)]
pub struct MockIPairingConsumer;

impl IPairingConsumer for MockIPairingConsumer {
    type Output = ();

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        _payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
    }
}

#[derive(Default)]
pub struct CreatePairingDepsOverrides;

pub fn build_create_pairing_deps<C: IPairingConsumer>(
    consumer: C,
    _overrides: CreatePairingDepsOverrides,
) -> CreatePairingDeps<C> {
    CreatePairingDeps { consumer }
}

pub fn mock_create_pairing<C: IPairingConsumer>(
    deps: &CreatePairingDeps<C>,
    _params: CreatePairingParams,
    _payload: CreatePairingPayload,
) -> CreatePairingReturn<C::Output> {
    let adapter = build_mock_i_pairing_adapter(Default::default());
    let output = deps.consumer.consume_pairing(
        ConsumePairingParams,
        build_consume_pairing_payload(adapter, Default::default()),
    );
    Ok(build_create_pairing_success_return(
        output,
        Default::default(),
    ))
}
