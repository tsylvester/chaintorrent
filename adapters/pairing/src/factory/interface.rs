use core::convert::Infallible;
use domain::Secret;
use zeroize::Zeroize;

pub const PAIRING_INTERFACE_VERSION: u32 = 1;

pub enum PairingCurve {
    Bn254,
    Bls12381,
}

pub enum VerifierGroupArithmetic {
    FirstGroupOnly,
    BothGroups,
}

#[derive(PartialEq, Eq)]
pub enum PrecompileEncoding {
    Eip196Eip197,
    Eip2537,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetGroupEncodingIdentifier {
    /// The optimal ate pairing as EIP-197 fixes it, over the EIP-197
    /// generators, on the tower `Fp2 = Fp[u] / (u^2 + 1)`,
    /// `Fp6 = Fp2[v] / (v^3 - (u + 9))`, `Fp12 = Fp6[w] / (w^2 - v)`; the
    /// pairing value is the Miller loop's value raised to the exact exponent
    /// `(p^12 - 1) / r`, not a fixed multiple of it; a target-group element is
    /// serialized as its twelve base-field coefficients in tower order, each
    /// the coefficient's 32-byte big-endian canonical integer.
    Bn254V1,
    /// The optimal ate pairing as the CFRG pairing-friendly-curves draft fixes
    /// it, over the EIP-2537 generators, on the tower `Fp2 = Fp[u] / (u^2 + 1)`,
    /// `Fp6 = Fp2[v] / (v^3 - (u + 1))`, `Fp12 = Fp6[w] / (w^2 - v)`; the
    /// pairing value is the Miller loop's value raised to the exact exponent
    /// `(p^12 - 1) / r`, not a fixed multiple of it; a target-group element is
    /// serialized as its twelve base-field coefficients in tower order, each
    /// the coefficient's 48-byte big-endian canonical integer.
    Bls12381V1,
}

pub struct PairingDeclaration {
    pub curve: PairingCurve,
    pub verifier_group_arithmetic: VerifierGroupArithmetic,
    pub precompile_encoding: PrecompileEncoding,
    pub target_group_encoding: TargetGroupEncodingIdentifier,
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct SampleUniformScalarParams;

pub struct SampleUniformScalarPayload {
    pub uniform: Secret<Vec<u8>>,
}

pub struct SampleUniformScalarSuccessReturn<S: Zeroize> {
    pub scalar: Secret<S>,
}

pub enum SampleUniformScalarErrorReturn {
    WrongLength { expected: usize, actual: usize },
}

pub type SampleUniformScalarReturn<S> =
    Result<SampleUniformScalarSuccessReturn<S>, SampleUniformScalarErrorReturn>;

pub trait ISampleUniformScalar: Zeroize + Sized {
    const UNIFORM_BYTES_LENGTH: usize;

    fn sample_from_uniform_bytes(
        params: SampleUniformScalarParams,
        payload: SampleUniformScalarPayload,
    ) -> SampleUniformScalarReturn<Self>;
}

pub struct G1GeneratorParams;

pub struct G1GeneratorPayload;

pub struct G1GeneratorSuccessReturn<G> {
    pub point: G,
}

pub type G1GeneratorReturn<G> = Result<G1GeneratorSuccessReturn<G>, Infallible>;

pub struct G2GeneratorParams;

pub struct G2GeneratorPayload;

pub struct G2GeneratorSuccessReturn<G> {
    pub point: G,
}

pub type G2GeneratorReturn<G> = Result<G2GeneratorSuccessReturn<G>, Infallible>;

pub struct AddG1Params;

pub struct AddG1Payload<G> {
    pub left: G,
    pub right: G,
}

pub struct AddG1SuccessReturn<G> {
    pub sum: G,
}

pub type AddG1Return<G> = Result<AddG1SuccessReturn<G>, Infallible>;

pub struct AddG2Params;

pub struct AddG2Payload<G> {
    pub left: G,
    pub right: G,
}

pub struct AddG2SuccessReturn<G> {
    pub sum: G,
}

pub type AddG2Return<G> = Result<AddG2SuccessReturn<G>, Infallible>;

pub struct MulG1Params;

pub struct MulG1Payload<G, S> {
    pub point: G,
    pub scalar: S,
}

pub struct MulG1SuccessReturn<G> {
    pub product: G,
}

pub type MulG1Return<G> = Result<MulG1SuccessReturn<G>, Infallible>;

pub struct MulG2Params;

pub struct MulG2Payload<G, S> {
    pub point: G,
    pub scalar: S,
}

pub struct MulG2SuccessReturn<G> {
    pub product: G,
}

pub type MulG2Return<G> = Result<MulG2SuccessReturn<G>, Infallible>;

pub struct MsmG1Params;

pub struct MsmG1Term<G, S> {
    pub base: G,
    pub scalar: S,
}

pub struct MsmG1Payload<G, S> {
    pub terms: Vec<MsmG1Term<G, S>>,
}

pub struct MsmG1SuccessReturn<G> {
    pub sum: G,
}

pub type MsmG1Return<G> = Result<MsmG1SuccessReturn<G>, Infallible>;

pub struct MsmG2Params;

pub struct MsmG2Term<G, S> {
    pub base: G,
    pub scalar: S,
}

pub struct MsmG2Payload<G, S> {
    pub terms: Vec<MsmG2Term<G, S>>,
}

pub struct MsmG2SuccessReturn<G> {
    pub sum: G,
}

pub type MsmG2Return<G> = Result<MsmG2SuccessReturn<G>, Infallible>;

pub struct PairingProductIsOneParams;

pub struct PairingProductTerm<G1, G2> {
    pub g1: G1,
    pub g2: G2,
}

pub struct PairingProductIsOnePayload<G1, G2> {
    pub terms: Vec<PairingProductTerm<G1, G2>>,
}

pub struct PairingProductIsOneSuccessReturn {
    pub is_one: bool,
}

pub type PairingProductIsOneReturn = Result<PairingProductIsOneSuccessReturn, Infallible>;

pub struct DecodeG1Params;

pub struct DecodeG1SuccessReturn<G> {
    pub point: G,
}

pub enum DecodeG1ErrorReturn {
    WrongLength { expected: usize, actual: usize },
    NonCanonicalCoordinate,
    NotOnCurve,
    NotInSubgroup,
}

pub type DecodeG1Return<G> = Result<DecodeG1SuccessReturn<G>, DecodeG1ErrorReturn>;

pub struct DecodeG2Params;

pub struct DecodeG2SuccessReturn<G> {
    pub point: G,
}

pub enum DecodeG2ErrorReturn {
    WrongLength { expected: usize, actual: usize },
    NonCanonicalCoordinate,
    NotOnCurve,
    NotInSubgroup,
}

pub type DecodeG2Return<G> = Result<DecodeG2SuccessReturn<G>, DecodeG2ErrorReturn>;

pub struct DecodeScalarParams;

pub struct DecodeScalarSuccessReturn<S> {
    pub scalar: S,
}

pub enum DecodeScalarErrorReturn {
    WrongLength { expected: usize, actual: usize },
    NonCanonical,
}

pub type DecodeScalarReturn<S> = Result<DecodeScalarSuccessReturn<S>, DecodeScalarErrorReturn>;

pub struct EncodeG1Params;

pub struct EncodeG1Payload<G> {
    pub point: G,
}

pub struct EncodeG1SuccessReturn {
    pub bytes: Vec<u8>,
}

pub type EncodeG1Return = Result<EncodeG1SuccessReturn, Infallible>;

pub struct EncodeG2Params;

pub struct EncodeG2Payload<G> {
    pub point: G,
}

pub struct EncodeG2SuccessReturn {
    pub bytes: Vec<u8>,
}

pub type EncodeG2Return = Result<EncodeG2SuccessReturn, Infallible>;

pub struct EncodeScalarParams;

pub struct EncodeScalarPayload<S> {
    pub scalar: S,
}

pub struct EncodeScalarSuccessReturn {
    pub bytes: Secret<Vec<u8>>,
}

pub type EncodeScalarReturn = Result<EncodeScalarSuccessReturn, Infallible>;

pub struct AddScalarParams;

pub struct AddScalarPayload<S> {
    pub left: S,
    pub right: S,
}

pub struct AddScalarSuccessReturn<S> {
    pub sum: S,
}

pub type AddScalarReturn<S> = Result<AddScalarSuccessReturn<S>, Infallible>;

pub struct MulScalarParams;

pub struct MulScalarPayload<S> {
    pub left: S,
    pub right: S,
}

pub struct MulScalarSuccessReturn<S> {
    pub product: S,
}

pub type MulScalarReturn<S> = Result<MulScalarSuccessReturn<S>, Infallible>;

pub struct NegScalarParams;

pub struct NegScalarPayload<S> {
    pub scalar: S,
}

pub struct NegScalarSuccessReturn<S> {
    pub negation: S,
}

pub type NegScalarReturn<S> = Result<NegScalarSuccessReturn<S>, Infallible>;

pub struct NegG1Params;

pub struct NegG1Payload<G> {
    pub point: G,
}

pub struct NegG1SuccessReturn<G> {
    pub negation: G,
}

pub type NegG1Return<G> = Result<NegG1SuccessReturn<G>, Infallible>;

pub struct NegG2Params;

pub struct NegG2Payload<G> {
    pub point: G,
}

pub struct NegG2SuccessReturn<G> {
    pub negation: G,
}

pub type NegG2Return<G> = Result<NegG2SuccessReturn<G>, Infallible>;

pub struct IsIdentityG1Params;

pub struct IsIdentityG1Payload<G> {
    pub point: G,
}

pub struct IsIdentityG1SuccessReturn {
    pub is_identity: bool,
}

pub type IsIdentityG1Return = Result<IsIdentityG1SuccessReturn, Infallible>;

pub struct IsIdentityG2Params;

pub struct IsIdentityG2Payload<G> {
    pub point: G,
}

pub struct IsIdentityG2SuccessReturn {
    pub is_identity: bool,
}

pub type IsIdentityG2Return = Result<IsIdentityG2SuccessReturn, Infallible>;

pub struct PairingProductParams;

pub struct PairingProductPayload<G1, G2> {
    pub terms: Vec<PairingProductTerm<G1, G2>>,
}

pub struct PairingProductSuccessReturn<T> {
    pub product: T,
}

pub type PairingProductReturn<T> = Result<PairingProductSuccessReturn<T>, Infallible>;

pub struct EncodeGtParams;

pub struct EncodeGtPayload<T> {
    pub value: T,
}

pub struct EncodeGtSuccessReturn {
    pub bytes: Secret<Vec<u8>>,
}

pub type EncodeGtReturn = Result<EncodeGtSuccessReturn, Infallible>;

pub trait IPairingAdapter {
    type Scalar: ISampleUniformScalar + Clone;
    type G1: Clone;
    type G2: Clone;

    fn g1_generator(
        &self,
        params: G1GeneratorParams,
        payload: G1GeneratorPayload,
    ) -> G1GeneratorReturn<Self::G1>;

    fn g2_generator(
        &self,
        params: G2GeneratorParams,
        payload: G2GeneratorPayload,
    ) -> G2GeneratorReturn<Self::G2>;

    fn add_g1(&self, params: AddG1Params, payload: AddG1Payload<Self::G1>)
    -> AddG1Return<Self::G1>;

    fn add_g2(&self, params: AddG2Params, payload: AddG2Payload<Self::G2>)
    -> AddG2Return<Self::G2>;

    fn mul_g1(
        &self,
        params: MulG1Params,
        payload: MulG1Payload<Self::G1, Self::Scalar>,
    ) -> MulG1Return<Self::G1>;

    fn mul_g2(
        &self,
        params: MulG2Params,
        payload: MulG2Payload<Self::G2, Self::Scalar>,
    ) -> MulG2Return<Self::G2>;

    fn msm_g1(
        &self,
        params: MsmG1Params,
        payload: MsmG1Payload<Self::G1, Self::Scalar>,
    ) -> MsmG1Return<Self::G1>;

    fn msm_g2(
        &self,
        params: MsmG2Params,
        payload: MsmG2Payload<Self::G2, Self::Scalar>,
    ) -> MsmG2Return<Self::G2>;

    fn pairing_product_is_one(
        &self,
        params: PairingProductIsOneParams,
        payload: PairingProductIsOnePayload<Self::G1, Self::G2>,
    ) -> PairingProductIsOneReturn;

    fn decode_g1(&self, params: DecodeG1Params, payload: &[u8]) -> DecodeG1Return<Self::G1>;

    fn decode_g2(&self, params: DecodeG2Params, payload: &[u8]) -> DecodeG2Return<Self::G2>;

    fn decode_scalar(
        &self,
        params: DecodeScalarParams,
        payload: &[u8],
    ) -> DecodeScalarReturn<Self::Scalar>;

    fn encode_g1(
        &self,
        params: EncodeG1Params,
        payload: EncodeG1Payload<Self::G1>,
    ) -> EncodeG1Return;

    fn encode_g2(
        &self,
        params: EncodeG2Params,
        payload: EncodeG2Payload<Self::G2>,
    ) -> EncodeG2Return;

    fn encode_scalar(
        &self,
        params: EncodeScalarParams,
        payload: EncodeScalarPayload<Self::Scalar>,
    ) -> EncodeScalarReturn;
}

pub trait IPairingArithmetic: IPairingAdapter<G1: Zeroize, G2: Zeroize> {
    type Gt: Zeroize;

    fn add_scalar(
        &self,
        params: AddScalarParams,
        payload: AddScalarPayload<Self::Scalar>,
    ) -> AddScalarReturn<Self::Scalar>;

    fn mul_scalar(
        &self,
        params: MulScalarParams,
        payload: MulScalarPayload<Self::Scalar>,
    ) -> MulScalarReturn<Self::Scalar>;

    fn neg_scalar(
        &self,
        params: NegScalarParams,
        payload: NegScalarPayload<Self::Scalar>,
    ) -> NegScalarReturn<Self::Scalar>;

    fn neg_g1(&self, params: NegG1Params, payload: NegG1Payload<Self::G1>)
    -> NegG1Return<Self::G1>;

    fn neg_g2(&self, params: NegG2Params, payload: NegG2Payload<Self::G2>)
    -> NegG2Return<Self::G2>;

    fn is_identity_g1(
        &self,
        params: IsIdentityG1Params,
        payload: IsIdentityG1Payload<Self::G1>,
    ) -> IsIdentityG1Return;

    fn is_identity_g2(
        &self,
        params: IsIdentityG2Params,
        payload: IsIdentityG2Payload<Self::G2>,
    ) -> IsIdentityG2Return;

    fn pairing_product(
        &self,
        params: PairingProductParams,
        payload: PairingProductPayload<Self::G1, Self::G2>,
    ) -> PairingProductReturn<Self::Gt>;

    fn encode_gt(
        &self,
        params: EncodeGtParams,
        payload: EncodeGtPayload<Self::Gt>,
    ) -> EncodeGtReturn;
}

pub enum PairingConcrete {
    Bn254Arkworks,
    Bn254Halo2curves,
    Bls12381Arkworks,
    Bls12381Halo2curves,
}

pub trait IPairingConsumer {
    type Output;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output;
}

pub struct ConsumePairingParams;

pub struct ConsumePairingPayload<P> {
    pub adapter: P,
    pub declaration: PairingDeclaration,
}

pub struct CreatePairingDeps<C> {
    pub consumer: C,
}

pub struct CreatePairingParams {
    pub concrete: PairingConcrete,
    pub supported_encodings: Vec<PrecompileEncoding>,
    pub target_group_encoding: TargetGroupEncodingIdentifier,
}

pub struct CreatePairingPayload;

pub struct CreatePairingSuccessReturn<O> {
    pub output: O,
}

pub enum CreatePairingErrorReturn {
    UnsupportedPrecompileEncoding,
    UnsupportedTargetGroupEncoding,
    Bn254Arkworks(Infallible),
    Bn254Halo2curves(Infallible),
    Bls12381Arkworks(Infallible),
    Bls12381Halo2curves(Infallible),
}

pub type CreatePairingReturn<O> = Result<CreatePairingSuccessReturn<O>, CreatePairingErrorReturn>;

pub type CreatePairingFn<C> = fn(
    &CreatePairingDeps<C>,
    CreatePairingParams,
    CreatePairingPayload,
) -> CreatePairingReturn<<C as IPairingConsumer>::Output>;
