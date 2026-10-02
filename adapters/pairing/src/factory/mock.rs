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
    DecodeG1Params, DecodeG1Return, DecodeG1SuccessReturn, DecodeG2Params, DecodeG2Return,
    DecodeG2SuccessReturn, DecodeScalarParams, DecodeScalarReturn, DecodeScalarSuccessReturn,
    EncodeG1Params, EncodeG1Payload, EncodeG1Return, EncodeG1SuccessReturn, EncodeG2Params,
    EncodeG2Payload, EncodeG2Return, EncodeG2SuccessReturn, EncodeGtParams, EncodeGtPayload,
    EncodeGtReturn, EncodeGtSuccessReturn, EncodeScalarParams, EncodeScalarPayload,
    EncodeScalarReturn, EncodeScalarSuccessReturn, G1GeneratorParams, G1GeneratorPayload,
    G1GeneratorReturn, G1GeneratorSuccessReturn, G2GeneratorParams, G2GeneratorPayload,
    G2GeneratorReturn, G2GeneratorSuccessReturn, IPairingAdapter, IPairingArithmetic,
    IPairingConsumer, ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1Payload,
    IsIdentityG1Return, IsIdentityG1SuccessReturn, IsIdentityG2Params, IsIdentityG2Payload,
    IsIdentityG2Return, IsIdentityG2SuccessReturn, MsmG1Params, MsmG1Payload, MsmG1Return,
    MsmG1SuccessReturn, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Return, MsmG2SuccessReturn,
    MsmG2Term, MulG1Params, MulG1Payload, MulG1Return, MulG1SuccessReturn, MulG2Params,
    MulG2Payload, MulG2Return, MulG2SuccessReturn, MulScalarParams, MulScalarPayload,
    MulScalarReturn, MulScalarSuccessReturn, NegG1Params, NegG1Payload, NegG1Return,
    NegG1SuccessReturn, NegG2Params, NegG2Payload, NegG2Return, NegG2SuccessReturn,
    NegScalarParams, NegScalarPayload, NegScalarReturn, NegScalarSuccessReturn,
    PAIRING_INTERFACE_VERSION, PairingConcrete, PairingCurve, PairingDeclaration,
    PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductIsOneReturn,
    PairingProductIsOneSuccessReturn, PairingProductParams, PairingProductPayload,
    PairingProductReturn, PairingProductSuccessReturn, PairingProductTerm, PrecompileEncoding,
    SampleUniformScalarPayload, SampleUniformScalarSuccessReturn, TargetGroupEncodingIdentifier,
    VerifierGroupArithmetic,
};
use core::marker::PhantomData;
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use zeroize::Zeroize;

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

#[derive(Default)]
pub struct G1GeneratorSuccessReturnOverrides<G> {
    pub point: Option<G>,
}

pub fn build_g1_generator_success_return<G: Default>(
    overrides: G1GeneratorSuccessReturnOverrides<G>,
) -> G1GeneratorSuccessReturn<G> {
    G1GeneratorSuccessReturn {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct G2GeneratorSuccessReturnOverrides<G> {
    pub point: Option<G>,
}

pub fn build_g2_generator_success_return<G: Default>(
    overrides: G2GeneratorSuccessReturnOverrides<G>,
) -> G2GeneratorSuccessReturn<G> {
    G2GeneratorSuccessReturn {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct AddG1PayloadOverrides<G> {
    pub left: Option<G>,
    pub right: Option<G>,
}

pub fn build_add_g1_payload<G: Default>(overrides: AddG1PayloadOverrides<G>) -> AddG1Payload<G> {
    AddG1Payload {
        left: overrides.left.unwrap_or_default(),
        right: overrides.right.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct AddG1SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

pub fn build_add_g1_success_return<G: Default>(
    overrides: AddG1SuccessReturnOverrides<G>,
) -> AddG1SuccessReturn<G> {
    AddG1SuccessReturn {
        sum: overrides.sum.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct AddG2PayloadOverrides<G> {
    pub left: Option<G>,
    pub right: Option<G>,
}

pub fn build_add_g2_payload<G: Default>(overrides: AddG2PayloadOverrides<G>) -> AddG2Payload<G> {
    AddG2Payload {
        left: overrides.left.unwrap_or_default(),
        right: overrides.right.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct AddG2SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

pub fn build_add_g2_success_return<G: Default>(
    overrides: AddG2SuccessReturnOverrides<G>,
) -> AddG2SuccessReturn<G> {
    AddG2SuccessReturn {
        sum: overrides.sum.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MulG1PayloadOverrides<G, S> {
    pub point: Option<G>,
    pub scalar: Option<S>,
}

pub fn build_mul_g1_payload<G: Default, S: Default>(
    overrides: MulG1PayloadOverrides<G, S>,
) -> MulG1Payload<G, S> {
    MulG1Payload {
        point: overrides.point.unwrap_or_default(),
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MulG1SuccessReturnOverrides<G> {
    pub product: Option<G>,
}

pub fn build_mul_g1_success_return<G: Default>(
    overrides: MulG1SuccessReturnOverrides<G>,
) -> MulG1SuccessReturn<G> {
    MulG1SuccessReturn {
        product: overrides.product.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MulG2PayloadOverrides<G, S> {
    pub point: Option<G>,
    pub scalar: Option<S>,
}

pub fn build_mul_g2_payload<G: Default, S: Default>(
    overrides: MulG2PayloadOverrides<G, S>,
) -> MulG2Payload<G, S> {
    MulG2Payload {
        point: overrides.point.unwrap_or_default(),
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MulG2SuccessReturnOverrides<G> {
    pub product: Option<G>,
}

pub fn build_mul_g2_success_return<G: Default>(
    overrides: MulG2SuccessReturnOverrides<G>,
) -> MulG2SuccessReturn<G> {
    MulG2SuccessReturn {
        product: overrides.product.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MsmG1TermOverrides<G, S> {
    pub base: Option<G>,
    pub scalar: Option<S>,
}

pub fn build_msm_g1_term<G: Default, S: Default>(
    overrides: MsmG1TermOverrides<G, S>,
) -> MsmG1Term<G, S> {
    MsmG1Term {
        base: overrides.base.unwrap_or_default(),
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MsmG1PayloadOverrides<G, S> {
    pub terms: Option<Vec<MsmG1Term<G, S>>>,
}

pub fn build_msm_g1_payload<G: Default, S: Default>(
    overrides: MsmG1PayloadOverrides<G, S>,
) -> MsmG1Payload<G, S> {
    MsmG1Payload {
        terms: overrides.terms.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MsmG1SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

pub fn build_msm_g1_success_return<G: Default>(
    overrides: MsmG1SuccessReturnOverrides<G>,
) -> MsmG1SuccessReturn<G> {
    MsmG1SuccessReturn {
        sum: overrides.sum.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MsmG2TermOverrides<G, S> {
    pub base: Option<G>,
    pub scalar: Option<S>,
}

pub fn build_msm_g2_term<G: Default, S: Default>(
    overrides: MsmG2TermOverrides<G, S>,
) -> MsmG2Term<G, S> {
    MsmG2Term {
        base: overrides.base.unwrap_or_default(),
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MsmG2PayloadOverrides<G, S> {
    pub terms: Option<Vec<MsmG2Term<G, S>>>,
}

pub fn build_msm_g2_payload<G: Default, S: Default>(
    overrides: MsmG2PayloadOverrides<G, S>,
) -> MsmG2Payload<G, S> {
    MsmG2Payload {
        terms: overrides.terms.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MsmG2SuccessReturnOverrides<G> {
    pub sum: Option<G>,
}

pub fn build_msm_g2_success_return<G: Default>(
    overrides: MsmG2SuccessReturnOverrides<G>,
) -> MsmG2SuccessReturn<G> {
    MsmG2SuccessReturn {
        sum: overrides.sum.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct PairingProductTermOverrides<G1, G2> {
    pub g1: Option<G1>,
    pub g2: Option<G2>,
}

pub fn build_pairing_product_term<G1: Default, G2: Default>(
    overrides: PairingProductTermOverrides<G1, G2>,
) -> PairingProductTerm<G1, G2> {
    PairingProductTerm {
        g1: overrides.g1.unwrap_or_default(),
        g2: overrides.g2.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct PairingProductIsOnePayloadOverrides<G1, G2> {
    pub terms: Option<Vec<PairingProductTerm<G1, G2>>>,
}

pub fn build_pairing_product_is_one_payload<G1: Default, G2: Default>(
    overrides: PairingProductIsOnePayloadOverrides<G1, G2>,
) -> PairingProductIsOnePayload<G1, G2> {
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

#[derive(Default)]
pub struct DecodeG1SuccessReturnOverrides<G> {
    pub point: Option<G>,
}

pub fn build_decode_g1_success_return<G: Default>(
    overrides: DecodeG1SuccessReturnOverrides<G>,
) -> DecodeG1SuccessReturn<G> {
    DecodeG1SuccessReturn {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct DecodeG2SuccessReturnOverrides<G> {
    pub point: Option<G>,
}

pub fn build_decode_g2_success_return<G: Default>(
    overrides: DecodeG2SuccessReturnOverrides<G>,
) -> DecodeG2SuccessReturn<G> {
    DecodeG2SuccessReturn {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct DecodeScalarSuccessReturnOverrides<S> {
    pub scalar: Option<S>,
}

pub fn build_decode_scalar_success_return<S: Default>(
    overrides: DecodeScalarSuccessReturnOverrides<S>,
) -> DecodeScalarSuccessReturn<S> {
    DecodeScalarSuccessReturn {
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeG1PayloadOverrides<G> {
    pub point: Option<G>,
}

pub fn build_encode_g1_payload<G: Default>(
    overrides: EncodeG1PayloadOverrides<G>,
) -> EncodeG1Payload<G> {
    EncodeG1Payload {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeG1SuccessReturnOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_encode_g1_success_return(
    overrides: EncodeG1SuccessReturnOverrides,
) -> EncodeG1SuccessReturn {
    EncodeG1SuccessReturn {
        bytes: overrides.bytes.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeG2PayloadOverrides<G> {
    pub point: Option<G>,
}

pub fn build_encode_g2_payload<G: Default>(
    overrides: EncodeG2PayloadOverrides<G>,
) -> EncodeG2Payload<G> {
    EncodeG2Payload {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeG2SuccessReturnOverrides {
    pub bytes: Option<Vec<u8>>,
}

pub fn build_encode_g2_success_return(
    overrides: EncodeG2SuccessReturnOverrides,
) -> EncodeG2SuccessReturn {
    EncodeG2SuccessReturn {
        bytes: overrides.bytes.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeScalarPayloadOverrides<S> {
    pub scalar: Option<S>,
}

pub fn build_encode_scalar_payload<S: Default>(
    overrides: EncodeScalarPayloadOverrides<S>,
) -> EncodeScalarPayload<S> {
    EncodeScalarPayload {
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeScalarSuccessReturnOverrides {
    pub bytes: Option<Secret<Vec<u8>>>,
}

pub fn build_encode_scalar_success_return(
    overrides: EncodeScalarSuccessReturnOverrides,
) -> EncodeScalarSuccessReturn {
    EncodeScalarSuccessReturn {
        bytes: overrides.bytes.unwrap_or_else(|| {
            build_secret::<Vec<u8>>(SecretConstructorParamsOverrides::default())
        }),
    }
}

#[derive(Default)]
pub struct AddScalarPayloadOverrides<S> {
    pub left: Option<S>,
    pub right: Option<S>,
}

pub fn build_add_scalar_payload<S: Default>(
    overrides: AddScalarPayloadOverrides<S>,
) -> AddScalarPayload<S> {
    AddScalarPayload {
        left: overrides.left.unwrap_or_default(),
        right: overrides.right.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct AddScalarSuccessReturnOverrides<S> {
    pub sum: Option<S>,
}

pub fn build_add_scalar_success_return<S: Default>(
    overrides: AddScalarSuccessReturnOverrides<S>,
) -> AddScalarSuccessReturn<S> {
    AddScalarSuccessReturn {
        sum: overrides.sum.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MulScalarPayloadOverrides<S> {
    pub left: Option<S>,
    pub right: Option<S>,
}

pub fn build_mul_scalar_payload<S: Default>(
    overrides: MulScalarPayloadOverrides<S>,
) -> MulScalarPayload<S> {
    MulScalarPayload {
        left: overrides.left.unwrap_or_default(),
        right: overrides.right.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct MulScalarSuccessReturnOverrides<S> {
    pub product: Option<S>,
}

pub fn build_mul_scalar_success_return<S: Default>(
    overrides: MulScalarSuccessReturnOverrides<S>,
) -> MulScalarSuccessReturn<S> {
    MulScalarSuccessReturn {
        product: overrides.product.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct NegScalarPayloadOverrides<S> {
    pub scalar: Option<S>,
}

pub fn build_neg_scalar_payload<S: Default>(
    overrides: NegScalarPayloadOverrides<S>,
) -> NegScalarPayload<S> {
    NegScalarPayload {
        scalar: overrides.scalar.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct NegScalarSuccessReturnOverrides<S> {
    pub negation: Option<S>,
}

pub fn build_neg_scalar_success_return<S: Default>(
    overrides: NegScalarSuccessReturnOverrides<S>,
) -> NegScalarSuccessReturn<S> {
    NegScalarSuccessReturn {
        negation: overrides.negation.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct NegG1PayloadOverrides<G> {
    pub point: Option<G>,
}

pub fn build_neg_g1_payload<G: Default>(overrides: NegG1PayloadOverrides<G>) -> NegG1Payload<G> {
    NegG1Payload {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct NegG1SuccessReturnOverrides<G> {
    pub negation: Option<G>,
}

pub fn build_neg_g1_success_return<G: Default>(
    overrides: NegG1SuccessReturnOverrides<G>,
) -> NegG1SuccessReturn<G> {
    NegG1SuccessReturn {
        negation: overrides.negation.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct NegG2PayloadOverrides<G> {
    pub point: Option<G>,
}

pub fn build_neg_g2_payload<G: Default>(overrides: NegG2PayloadOverrides<G>) -> NegG2Payload<G> {
    NegG2Payload {
        point: overrides.point.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct NegG2SuccessReturnOverrides<G> {
    pub negation: Option<G>,
}

pub fn build_neg_g2_success_return<G: Default>(
    overrides: NegG2SuccessReturnOverrides<G>,
) -> NegG2SuccessReturn<G> {
    NegG2SuccessReturn {
        negation: overrides.negation.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct IsIdentityG1PayloadOverrides<G> {
    pub point: Option<G>,
}

pub fn build_is_identity_g1_payload<G: Default>(
    overrides: IsIdentityG1PayloadOverrides<G>,
) -> IsIdentityG1Payload<G> {
    IsIdentityG1Payload {
        point: overrides.point.unwrap_or_default(),
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

#[derive(Default)]
pub struct IsIdentityG2PayloadOverrides<G> {
    pub point: Option<G>,
}

pub fn build_is_identity_g2_payload<G: Default>(
    overrides: IsIdentityG2PayloadOverrides<G>,
) -> IsIdentityG2Payload<G> {
    IsIdentityG2Payload {
        point: overrides.point.unwrap_or_default(),
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

#[derive(Default)]
pub struct PairingProductPayloadOverrides<G1, G2> {
    pub terms: Option<Vec<PairingProductTerm<G1, G2>>>,
}

pub fn build_pairing_product_payload<G1: Default, G2: Default>(
    overrides: PairingProductPayloadOverrides<G1, G2>,
) -> PairingProductPayload<G1, G2> {
    PairingProductPayload {
        terms: overrides.terms.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct PairingProductSuccessReturnOverrides<T> {
    pub product: Option<T>,
}

pub fn build_pairing_product_success_return<T: Default>(
    overrides: PairingProductSuccessReturnOverrides<T>,
) -> PairingProductSuccessReturn<T> {
    PairingProductSuccessReturn {
        product: overrides.product.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeGtPayloadOverrides<T> {
    pub value: Option<T>,
}

pub fn build_encode_gt_payload<T: Default>(
    overrides: EncodeGtPayloadOverrides<T>,
) -> EncodeGtPayload<T> {
    EncodeGtPayload {
        value: overrides.value.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EncodeGtSuccessReturnOverrides {
    pub bytes: Option<Secret<Vec<u8>>>,
}

pub fn build_encode_gt_success_return(
    overrides: EncodeGtSuccessReturnOverrides,
) -> EncodeGtSuccessReturn {
    EncodeGtSuccessReturn {
        bytes: overrides.bytes.unwrap_or_else(|| {
            build_secret::<Vec<u8>>(SecretConstructorParamsOverrides::default())
        }),
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
        uniform: overrides.uniform.unwrap_or_else(|| {
            build_secret::<Vec<u8>>(SecretConstructorParamsOverrides {
                value: Some(vec![0u8; 64]),
            })
        }),
    }
}

#[derive(Default)]
pub struct SampleUniformScalarSuccessReturnOverrides<S: Zeroize> {
    pub scalar: Option<Secret<S>>,
}

pub fn build_sample_uniform_scalar_success_return<S: Zeroize + Default>(
    overrides: SampleUniformScalarSuccessReturnOverrides<S>,
) -> SampleUniformScalarSuccessReturn<S> {
    SampleUniformScalarSuccessReturn {
        scalar: overrides
            .scalar
            .unwrap_or_else(|| build_secret::<S>(SecretConstructorParamsOverrides::default())),
    }
}

pub struct MockIPairingAdapter<S, G1, G2, Gt> {
    pub scalar: PhantomData<S>,
    pub g1: PhantomData<G1>,
    pub g2: PhantomData<G2>,
    pub gt: PhantomData<Gt>,
}

impl<S, G1, G2, Gt> IPairingAdapter for MockIPairingAdapter<S, G1, G2, Gt>
where
    S: ISampleUniformScalar + Clone + Default,
    G1: Clone + Default,
    G2: Clone + Default,
{
    type Scalar = S;
    type G1 = G1;
    type G2 = G2;

    fn g1_generator(
        &self,
        _params: G1GeneratorParams,
        _payload: G1GeneratorPayload,
    ) -> G1GeneratorReturn<Self::G1> {
        Ok(build_g1_generator_success_return(Default::default()))
    }

    fn g2_generator(
        &self,
        _params: G2GeneratorParams,
        _payload: G2GeneratorPayload,
    ) -> G2GeneratorReturn<Self::G2> {
        Ok(build_g2_generator_success_return(Default::default()))
    }

    fn add_g1(
        &self,
        _params: AddG1Params,
        _payload: AddG1Payload<Self::G1>,
    ) -> AddG1Return<Self::G1> {
        Ok(build_add_g1_success_return(Default::default()))
    }

    fn add_g2(
        &self,
        _params: AddG2Params,
        _payload: AddG2Payload<Self::G2>,
    ) -> AddG2Return<Self::G2> {
        Ok(build_add_g2_success_return(Default::default()))
    }

    fn mul_g1(
        &self,
        _params: MulG1Params,
        _payload: MulG1Payload<Self::G1, Self::Scalar>,
    ) -> MulG1Return<Self::G1> {
        Ok(build_mul_g1_success_return(Default::default()))
    }

    fn mul_g2(
        &self,
        _params: MulG2Params,
        _payload: MulG2Payload<Self::G2, Self::Scalar>,
    ) -> MulG2Return<Self::G2> {
        Ok(build_mul_g2_success_return(Default::default()))
    }

    fn msm_g1(
        &self,
        _params: MsmG1Params,
        _payload: MsmG1Payload<Self::G1, Self::Scalar>,
    ) -> MsmG1Return<Self::G1> {
        Ok(build_msm_g1_success_return(Default::default()))
    }

    fn msm_g2(
        &self,
        _params: MsmG2Params,
        _payload: MsmG2Payload<Self::G2, Self::Scalar>,
    ) -> MsmG2Return<Self::G2> {
        Ok(build_msm_g2_success_return(Default::default()))
    }

    fn pairing_product_is_one(
        &self,
        _params: PairingProductIsOneParams,
        _payload: PairingProductIsOnePayload<Self::G1, Self::G2>,
    ) -> PairingProductIsOneReturn {
        Ok(build_pairing_product_is_one_success_return(
            Default::default(),
        ))
    }

    fn decode_g1(&self, _params: DecodeG1Params, _payload: &[u8]) -> DecodeG1Return<Self::G1> {
        Ok(build_decode_g1_success_return(Default::default()))
    }

    fn decode_g2(&self, _params: DecodeG2Params, _payload: &[u8]) -> DecodeG2Return<Self::G2> {
        Ok(build_decode_g2_success_return(Default::default()))
    }

    fn decode_scalar(
        &self,
        _params: DecodeScalarParams,
        _payload: &[u8],
    ) -> DecodeScalarReturn<Self::Scalar> {
        Ok(build_decode_scalar_success_return(Default::default()))
    }

    fn encode_g1(
        &self,
        _params: EncodeG1Params,
        _payload: EncodeG1Payload<Self::G1>,
    ) -> EncodeG1Return {
        Ok(build_encode_g1_success_return(Default::default()))
    }

    fn encode_g2(
        &self,
        _params: EncodeG2Params,
        _payload: EncodeG2Payload<Self::G2>,
    ) -> EncodeG2Return {
        Ok(build_encode_g2_success_return(Default::default()))
    }

    fn encode_scalar(
        &self,
        _params: EncodeScalarParams,
        _payload: EncodeScalarPayload<Self::Scalar>,
    ) -> EncodeScalarReturn {
        Ok(build_encode_scalar_success_return(Default::default()))
    }
}

impl<S, G1, G2, Gt> IPairingArithmetic for MockIPairingAdapter<S, G1, G2, Gt>
where
    S: ISampleUniformScalar + Clone + Default,
    G1: Zeroize + Clone + Default,
    G2: Zeroize + Clone + Default,
    Gt: Zeroize + Default,
{
    type Gt = Gt;

    fn add_scalar(
        &self,
        _params: AddScalarParams,
        _payload: AddScalarPayload<Self::Scalar>,
    ) -> AddScalarReturn<Self::Scalar> {
        Ok(build_add_scalar_success_return(Default::default()))
    }

    fn mul_scalar(
        &self,
        _params: MulScalarParams,
        _payload: MulScalarPayload<Self::Scalar>,
    ) -> MulScalarReturn<Self::Scalar> {
        Ok(build_mul_scalar_success_return(Default::default()))
    }

    fn neg_scalar(
        &self,
        _params: NegScalarParams,
        _payload: NegScalarPayload<Self::Scalar>,
    ) -> NegScalarReturn<Self::Scalar> {
        Ok(build_neg_scalar_success_return(Default::default()))
    }

    fn neg_g1(
        &self,
        _params: NegG1Params,
        _payload: NegG1Payload<Self::G1>,
    ) -> NegG1Return<Self::G1> {
        Ok(build_neg_g1_success_return(Default::default()))
    }

    fn neg_g2(
        &self,
        _params: NegG2Params,
        _payload: NegG2Payload<Self::G2>,
    ) -> NegG2Return<Self::G2> {
        Ok(build_neg_g2_success_return(Default::default()))
    }

    fn is_identity_g1(
        &self,
        _params: IsIdentityG1Params,
        _payload: IsIdentityG1Payload<Self::G1>,
    ) -> IsIdentityG1Return {
        Ok(build_is_identity_g1_success_return(Default::default()))
    }

    fn is_identity_g2(
        &self,
        _params: IsIdentityG2Params,
        _payload: IsIdentityG2Payload<Self::G2>,
    ) -> IsIdentityG2Return {
        Ok(build_is_identity_g2_success_return(Default::default()))
    }

    fn pairing_product(
        &self,
        _params: PairingProductParams,
        _payload: PairingProductPayload<Self::G1, Self::G2>,
    ) -> PairingProductReturn<Self::Gt> {
        Ok(build_pairing_product_success_return(Default::default()))
    }

    fn encode_gt(
        &self,
        _params: EncodeGtParams,
        _payload: EncodeGtPayload<Self::Gt>,
    ) -> EncodeGtReturn {
        Ok(build_encode_gt_success_return(Default::default()))
    }
}

#[derive(Default)]
pub struct CreatePairingParamsOverrides {
    pub concrete: Option<PairingConcrete>,
    pub supported_encodings: Option<Vec<PrecompileEncoding>>,
    pub target_group_encoding: Option<TargetGroupEncodingIdentifier>,
}

pub fn build_create_pairing_params(overrides: CreatePairingParamsOverrides) -> CreatePairingParams {
    let concrete = overrides.concrete.unwrap_or(PairingConcrete::Bn254Arkworks);
    let target_group_encoding = overrides.target_group_encoding.unwrap_or(match concrete {
        PairingConcrete::Bn254Arkworks | PairingConcrete::Bn254Halo2curves => {
            TargetGroupEncodingIdentifier::Bn254V1
        }
        PairingConcrete::Bls12381Arkworks | PairingConcrete::Bls12381Halo2curves => {
            TargetGroupEncodingIdentifier::Bls12381V1
        }
    });
    CreatePairingParams {
        concrete,
        supported_encodings: overrides.supported_encodings.unwrap_or_else(|| {
            vec![
                PrecompileEncoding::Eip196Eip197,
                PrecompileEncoding::Eip2537,
            ]
        }),
        target_group_encoding,
    }
}

#[derive(Default)]
pub struct CreatePairingSuccessReturnOverrides<O> {
    pub output: Option<O>,
}

pub fn build_create_pairing_success_return<O: Default>(
    overrides: CreatePairingSuccessReturnOverrides<O>,
) -> CreatePairingSuccessReturn<O> {
    CreatePairingSuccessReturn {
        output: overrides.output.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ConsumePairingPayloadOverrides<S, G1, G2, Gt> {
    pub adapter: Option<MockIPairingAdapter<S, G1, G2, Gt>>,
    pub declaration: Option<PairingDeclaration>,
}

pub fn build_consume_pairing_payload<S, G1, G2, Gt>(
    overrides: ConsumePairingPayloadOverrides<S, G1, G2, Gt>,
) -> ConsumePairingPayload<MockIPairingAdapter<S, G1, G2, Gt>> {
    ConsumePairingPayload {
        adapter: overrides.adapter.unwrap_or(MockIPairingAdapter {
            scalar: PhantomData,
            g1: PhantomData,
            g2: PhantomData,
            gt: PhantomData,
        }),
        declaration: overrides
            .declaration
            .unwrap_or_else(|| build_pairing_declaration(Default::default())),
    }
}

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

pub fn mock_create_pairing<C>(
    _deps: &CreatePairingDeps<C>,
    _params: CreatePairingParams,
    _payload: CreatePairingPayload,
) -> CreatePairingReturn<C::Output>
where
    C: IPairingConsumer,
    C::Output: Default,
{
    Ok(build_create_pairing_success_return(Default::default()))
}
