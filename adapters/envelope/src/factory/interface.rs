use core::convert::Infallible;
use domain::Secret;
use encoding::IEncoderAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::CredentialComponents;
use pairing::{IPairingAdapter, IPairingArithmetic};
use random::IRandomSourceAdapter;
use zeroize::Zeroize;

use crate::pairing_elgamal::provides::{
    PairingElGamalGenerateKeysErrorReturn, PairingElGamalKeyAgreementTryNewErrorReturn,
    PairingElGamalKeyPairFromComponentsErrorReturn,
    PairingElGamalPublicKeysFromComponentsErrorReturn, PairingElGamalWrapToErrorReturn,
};

pub const KEY_AGREEMENT_INTERFACE_VERSION: u32 = 1;

#[derive(PartialEq, Eq)]
pub enum KeyAgreementIdentifier {
    PairingElGamalV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvelopeAlgebra {
    PairingElGamal,
}

pub struct KeyAgreementDeclaration {
    pub identifier: KeyAgreementIdentifier,
    pub algebra: EnvelopeAlgebra,
    pub possession_g1_tag: &'static [u8],
    pub possession_g2_tag: &'static [u8],
    pub adapter_version: u32,
    pub interface_version: u32,
}

pub struct EnvelopeCoins<S: Zeroize> {
    pub rho: Secret<S>,
    pub sigma: Secret<S>,
}

pub struct GenerateKeysParams;

pub struct GenerateKeysPayload {
    pub x_uniform: Secret<Vec<u8>>,
    pub y_uniform: Secret<Vec<u8>>,
}

pub struct GenerateKeysSuccessReturn<KP, PO> {
    pub key_pair: KP,
    pub possession: PO,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GenerateKeysErrorReturn {
    PairingElGamal(PairingElGamalGenerateKeysErrorReturn),
}

pub type GenerateKeysReturn<KP, PO> =
    Result<GenerateKeysSuccessReturn<KP, PO>, GenerateKeysErrorReturn>;

pub struct WrapToParams;

pub struct WrapToPayload<'a, PK, G1, G2> {
    pub public_keys: &'a PK,
    pub credential: CredentialComponents<G1, G2>,
}

pub struct WrapToSuccessReturn<E, S: Zeroize> {
    pub envelope: E,
    pub coins: EnvelopeCoins<S>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WrapToErrorReturn {
    PairingElGamal(PairingElGamalWrapToErrorReturn),
}

pub type WrapToReturn<E, S> = Result<WrapToSuccessReturn<E, S>, WrapToErrorReturn>;

pub struct UnwrapParams;

pub struct UnwrapPayload<'a, KP, E> {
    pub key_pair: &'a KP,
    pub envelope: &'a E,
}

pub struct UnwrapSuccessReturn<G1, G2> {
    pub credential: CredentialComponents<G1, G2>,
}

pub type UnwrapReturn<G1, G2> = Result<UnwrapSuccessReturn<G1, G2>, Infallible>;

pub struct KeyPairComponents<S: Zeroize> {
    pub x: Secret<S>,
    pub y: Secret<S>,
}

pub struct PublicKeysComponents<G1, G2> {
    pub pk1: G1,
    pub pk2: G2,
}

pub struct PossessionComponents<S, G1, G2> {
    pub r1: G1,
    pub z1: S,
    pub r2: G2,
    pub z2: S,
}

pub struct EnvelopeComponents<G1, G2> {
    pub c1: G1,
    pub c2: G1,
    pub d1: G2,
    pub d2: G2,
}

pub struct KeyPairComponentsParams;

pub struct KeyPairComponentsPayload<'a, KP> {
    pub key_pair: &'a KP,
}

pub struct KeyPairComponentsSuccessReturn<S: Zeroize> {
    pub components: KeyPairComponents<S>,
}

pub type KeyPairComponentsReturn<S> = Result<KeyPairComponentsSuccessReturn<S>, Infallible>;

pub struct KeyPairPublicKeysParams;

pub struct KeyPairPublicKeysPayload<'a, KP> {
    pub key_pair: &'a KP,
}

pub struct KeyPairPublicKeysSuccessReturn<G1, G2> {
    pub components: PublicKeysComponents<G1, G2>,
}

pub type KeyPairPublicKeysReturn<G1, G2> =
    Result<KeyPairPublicKeysSuccessReturn<G1, G2>, Infallible>;

pub struct PublicKeysComponentsParams;

pub struct PublicKeysComponentsPayload<'a, PK> {
    pub public_keys: &'a PK,
}

pub struct PublicKeysComponentsSuccessReturn<G1, G2> {
    pub components: PublicKeysComponents<G1, G2>,
}

pub type PublicKeysComponentsReturn<G1, G2> =
    Result<PublicKeysComponentsSuccessReturn<G1, G2>, Infallible>;

pub struct PossessionComponentsParams;

pub struct PossessionComponentsPayload<'a, PO> {
    pub possession: &'a PO,
}

pub struct PossessionComponentsSuccessReturn<S, G1, G2> {
    pub components: PossessionComponents<S, G1, G2>,
}

pub type PossessionComponentsReturn<S, G1, G2> =
    Result<PossessionComponentsSuccessReturn<S, G1, G2>, Infallible>;

pub struct EnvelopeComponentsParams;

pub struct EnvelopeComponentsPayload<'a, E> {
    pub envelope: &'a E,
}

pub struct EnvelopeComponentsSuccessReturn<G1, G2> {
    pub components: EnvelopeComponents<G1, G2>,
}

pub type EnvelopeComponentsReturn<G1, G2> =
    Result<EnvelopeComponentsSuccessReturn<G1, G2>, Infallible>;

pub struct KeyPairFromComponentsParams;

pub struct KeyPairFromComponentsPayload<S: Zeroize> {
    pub components: KeyPairComponents<S>,
}

pub struct KeyPairFromComponentsSuccessReturn<KP> {
    pub key_pair: KP,
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyPairFromComponentsErrorReturn {
    PairingElGamal(PairingElGamalKeyPairFromComponentsErrorReturn),
}

pub type KeyPairFromComponentsReturn<KP> =
    Result<KeyPairFromComponentsSuccessReturn<KP>, KeyPairFromComponentsErrorReturn>;

pub struct PublicKeysFromComponentsParams;

pub struct PublicKeysFromComponentsPayload<'a, G1, G2, PO> {
    pub components: PublicKeysComponents<G1, G2>,
    pub possession: &'a PO,
}

pub struct PublicKeysFromComponentsSuccessReturn<PK> {
    pub public_keys: PK,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PublicKeysFromComponentsErrorReturn {
    PairingElGamal(PairingElGamalPublicKeysFromComponentsErrorReturn),
}

pub type PublicKeysFromComponentsReturn<PK> =
    Result<PublicKeysFromComponentsSuccessReturn<PK>, PublicKeysFromComponentsErrorReturn>;

pub struct PossessionFromComponentsParams;

pub struct PossessionFromComponentsPayload<S, G1, G2> {
    pub components: PossessionComponents<S, G1, G2>,
}

pub struct PossessionFromComponentsSuccessReturn<PO> {
    pub possession: PO,
}

pub type PossessionFromComponentsReturn<PO> =
    Result<PossessionFromComponentsSuccessReturn<PO>, Infallible>;

pub struct EnvelopeFromComponentsParams;

pub struct EnvelopeFromComponentsPayload<G1, G2> {
    pub components: EnvelopeComponents<G1, G2>,
}

pub struct EnvelopeFromComponentsSuccessReturn<E> {
    pub envelope: E,
}

pub type EnvelopeFromComponentsReturn<E> =
    Result<EnvelopeFromComponentsSuccessReturn<E>, Infallible>;

pub trait IKeyAgreementAdapter {
    const DECLARATION: KeyAgreementDeclaration;

    type Pairing: IPairingAdapter;
    type KeyPair;
    type PublicKeys;
    type Possession;
    type Envelope;

    fn generate_keys(
        &self,
        params: GenerateKeysParams,
        payload: GenerateKeysPayload,
    ) -> GenerateKeysReturn<Self::KeyPair, Self::Possession>;

    fn wrap_to(
        &self,
        params: WrapToParams,
        payload: WrapToPayload<
            '_,
            Self::PublicKeys,
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> WrapToReturn<Self::Envelope, <Self::Pairing as IPairingAdapter>::Scalar>;

    fn unwrap(
        &self,
        params: UnwrapParams,
        payload: UnwrapPayload<'_, Self::KeyPair, Self::Envelope>,
    ) -> UnwrapReturn<<Self::Pairing as IPairingAdapter>::G1, <Self::Pairing as IPairingAdapter>::G2>;

    fn key_pair_components(
        &self,
        params: KeyPairComponentsParams,
        payload: KeyPairComponentsPayload<'_, Self::KeyPair>,
    ) -> KeyPairComponentsReturn<<Self::Pairing as IPairingAdapter>::Scalar>;

    fn key_pair_public_keys(
        &self,
        params: KeyPairPublicKeysParams,
        payload: KeyPairPublicKeysPayload<'_, Self::KeyPair>,
    ) -> KeyPairPublicKeysReturn<
        <Self::Pairing as IPairingAdapter>::G1,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    fn public_keys_components(
        &self,
        params: PublicKeysComponentsParams,
        payload: PublicKeysComponentsPayload<'_, Self::PublicKeys>,
    ) -> PublicKeysComponentsReturn<
        <Self::Pairing as IPairingAdapter>::G1,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    #[allow(clippy::type_complexity)]
    fn possession_components(
        &self,
        params: PossessionComponentsParams,
        payload: PossessionComponentsPayload<'_, Self::Possession>,
    ) -> PossessionComponentsReturn<
        <Self::Pairing as IPairingAdapter>::Scalar,
        <Self::Pairing as IPairingAdapter>::G1,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    fn envelope_components(
        &self,
        params: EnvelopeComponentsParams,
        payload: EnvelopeComponentsPayload<'_, Self::Envelope>,
    ) -> EnvelopeComponentsReturn<
        <Self::Pairing as IPairingAdapter>::G1,
        <Self::Pairing as IPairingAdapter>::G2,
    >;

    fn key_pair_from_components(
        &self,
        params: KeyPairFromComponentsParams,
        payload: KeyPairFromComponentsPayload<<Self::Pairing as IPairingAdapter>::Scalar>,
    ) -> KeyPairFromComponentsReturn<Self::KeyPair>;

    fn public_keys_from_components(
        &self,
        params: PublicKeysFromComponentsParams,
        payload: PublicKeysFromComponentsPayload<
            '_,
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
            Self::Possession,
        >,
    ) -> PublicKeysFromComponentsReturn<Self::PublicKeys>;

    #[allow(clippy::type_complexity)]
    fn possession_from_components(
        &self,
        params: PossessionFromComponentsParams,
        payload: PossessionFromComponentsPayload<
            <Self::Pairing as IPairingAdapter>::Scalar,
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> PossessionFromComponentsReturn<Self::Possession>;

    fn envelope_from_components(
        &self,
        params: EnvelopeFromComponentsParams,
        payload: EnvelopeFromComponentsPayload<
            <Self::Pairing as IPairingAdapter>::G1,
            <Self::Pairing as IPairingAdapter>::G2,
        >,
    ) -> EnvelopeFromComponentsReturn<Self::Envelope>;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyAgreementConcrete {
    PairingElGamal,
}

pub trait IKeyAgreementConsumer<P: IPairingAdapter> {
    type Output;

    fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<K>,
    ) -> Self::Output;
}

pub struct ConsumeKeyAgreementParams;

pub struct ConsumeKeyAgreementPayload<K: IKeyAgreementAdapter> {
    pub adapter: K,
}

pub struct CreateKeyAgreementDeps<'a, P: IPairingArithmetic, E: IEncoderAdapter, C> {
    pub pairing: &'a P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
    pub consumer: C,
}

pub struct CreateKeyAgreementParams {
    pub concrete: KeyAgreementConcrete,
    pub identifier: KeyAgreementIdentifier,
    pub algebra: EnvelopeAlgebra,
}

pub struct CreateKeyAgreementPayload;

pub struct CreateKeyAgreementSuccessReturn<O> {
    pub output: O,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateKeyAgreementErrorReturn {
    UnsupportedKeyAgreementIdentifier,
    UnsupportedEnvelopeAlgebra,
    PairingElGamal(PairingElGamalKeyAgreementTryNewErrorReturn),
}

pub type CreateKeyAgreementReturn<O> =
    Result<CreateKeyAgreementSuccessReturn<O>, CreateKeyAgreementErrorReturn>;

pub type CreateKeyAgreementFn<'a, P, E, C> =
    fn(
        &CreateKeyAgreementDeps<'a, P, E, C>,
        CreateKeyAgreementParams,
        CreateKeyAgreementPayload,
    ) -> CreateKeyAgreementReturn<<C as IKeyAgreementConsumer<P>>::Output>;
