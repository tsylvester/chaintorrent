#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementDeps,
    CreateKeyAgreementParams, CreateKeyAgreementPayload, CreateKeyAgreementReturn,
    CreateKeyAgreementSuccessReturn, EnvelopeAlgebra, EnvelopeCoins, EnvelopeComponents,
    EnvelopeComponentsParams, EnvelopeComponentsPayload, EnvelopeComponentsSuccessReturn,
    EnvelopeFromComponentsParams, EnvelopeFromComponentsPayload,
    EnvelopeFromComponentsSuccessReturn, GenerateKeysParams, GenerateKeysPayload,
    GenerateKeysSuccessReturn, IKeyAgreementAdapter, IKeyAgreementConsumer,
    KEY_AGREEMENT_INTERFACE_VERSION, KeyAgreementConcrete, KeyAgreementDeclaration,
    KeyAgreementIdentifier, KeyPairComponents, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairComponentsSuccessReturn, KeyPairFromComponentsParams, KeyPairFromComponentsPayload,
    KeyPairFromComponentsSuccessReturn, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    KeyPairPublicKeysSuccessReturn, PossessionComponents, PossessionComponentsParams,
    PossessionComponentsPayload, PossessionComponentsSuccessReturn, PossessionFromComponentsParams,
    PossessionFromComponentsPayload, PossessionFromComponentsSuccessReturn, PublicKeysComponents,
    PublicKeysComponentsParams, PublicKeysComponentsPayload, PublicKeysComponentsSuccessReturn,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload,
    PublicKeysFromComponentsSuccessReturn, UnwrapParams, UnwrapPayload, UnwrapSuccessReturn,
    WrapToParams, WrapToPayload, WrapToSuccessReturn,
};
use core::marker::PhantomData;
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::IEncoderAdapter;
use kem::build_credential_components;
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingArithmetic,
    ISampleUniformScalar, SampleUniformScalarParams, SampleUniformScalarPayload,
};
use zeroize::Zeroize;

use crate::pairing_elgamal::provides::{
    PAIRING_ELGAMAL_POSSESSION_G1_TAG, PAIRING_ELGAMAL_POSSESSION_G2_TAG,
};

#[derive(Default)]
pub struct KeyAgreementDeclarationOverrides {
    pub identifier: Option<KeyAgreementIdentifier>,
    pub algebra: Option<EnvelopeAlgebra>,
    pub possession_g1_tag: Option<&'static [u8]>,
    pub possession_g2_tag: Option<&'static [u8]>,
    pub adapter_version: Option<u32>,
    pub interface_version: Option<u32>,
}

pub fn build_key_agreement_declaration(
    overrides: KeyAgreementDeclarationOverrides,
) -> KeyAgreementDeclaration {
    KeyAgreementDeclaration {
        identifier: overrides
            .identifier
            .unwrap_or(KeyAgreementIdentifier::PairingElGamalV1),
        algebra: overrides.algebra.unwrap_or(EnvelopeAlgebra::PairingElGamal),
        possession_g1_tag: overrides
            .possession_g1_tag
            .unwrap_or(PAIRING_ELGAMAL_POSSESSION_G1_TAG),
        possession_g2_tag: overrides
            .possession_g2_tag
            .unwrap_or(PAIRING_ELGAMAL_POSSESSION_G2_TAG),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(KEY_AGREEMENT_INTERFACE_VERSION),
    }
}

pub struct EnvelopeCoinsOverrides<S: Zeroize> {
    pub rho: Option<Secret<S>>,
    pub sigma: Option<Secret<S>>,
}

impl<S: Zeroize> Default for EnvelopeCoinsOverrides<S> {
    fn default() -> Self {
        Self {
            rho: None,
            sigma: None,
        }
    }
}

fn sample_scalar_secret<P: IPairingAdapter>(fill: u8) -> Secret<P::Scalar> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![fill; P::Scalar::UNIFORM_BYTES_LENGTH]),
            }),
        },
    )
    .unwrap()
    .scalar
}

fn sample_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    sample_scalar_secret::<P>(fill).expose().clone()
}

fn g1_element<P: IPairingAdapter>(pairing: &P, additions: usize) -> P::G1 {
    let generator = pairing
        .g1_generator(G1GeneratorParams, G1GeneratorPayload)
        .unwrap()
        .point;
    let mut element = generator.clone();
    for _ in 0..additions {
        element = pairing
            .add_g1(
                AddG1Params,
                AddG1Payload {
                    left: element,
                    right: generator.clone(),
                },
            )
            .unwrap()
            .sum;
    }
    element
}

fn g2_element<P: IPairingAdapter>(pairing: &P, additions: usize) -> P::G2 {
    let generator = pairing
        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
        .unwrap()
        .point;
    let mut element = generator.clone();
    for _ in 0..additions {
        element = pairing
            .add_g2(
                AddG2Params,
                AddG2Payload {
                    left: element,
                    right: generator.clone(),
                },
            )
            .unwrap()
            .sum;
    }
    element
}

pub fn build_envelope_coins<P: IPairingAdapter>(
    _pairing: &P,
    overrides: EnvelopeCoinsOverrides<P::Scalar>,
) -> EnvelopeCoins<P::Scalar> {
    EnvelopeCoins {
        rho: overrides
            .rho
            .unwrap_or_else(|| sample_scalar_secret::<P>(0x51)),
        sigma: overrides
            .sigma
            .unwrap_or_else(|| sample_scalar_secret::<P>(0x52)),
    }
}

#[derive(Default)]
pub struct GenerateKeysPayloadOverrides {
    pub x_uniform: Option<Secret<Vec<u8>>>,
    pub y_uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_generate_keys_payload(overrides: GenerateKeysPayloadOverrides) -> GenerateKeysPayload {
    GenerateKeysPayload {
        x_uniform: overrides.x_uniform.unwrap_or_else(|| {
            build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x11; 64]),
            })
        }),
        y_uniform: overrides.y_uniform.unwrap_or_else(|| {
            build_secret(SecretConstructorParamsOverrides {
                value: Some(vec![0x22; 64]),
            })
        }),
    }
}

#[derive(Default)]
pub struct GenerateKeysSuccessReturnOverrides<KP, PO> {
    pub key_pair: Option<KP>,
    pub possession: Option<PO>,
}

pub fn build_generate_keys_success_return<KP: Default, PO: Default>(
    overrides: GenerateKeysSuccessReturnOverrides<KP, PO>,
) -> GenerateKeysSuccessReturn<KP, PO> {
    GenerateKeysSuccessReturn {
        key_pair: overrides.key_pair.unwrap_or_default(),
        possession: overrides.possession.unwrap_or_default(),
    }
}

pub struct WrapToSuccessReturnOverrides<E, S: Zeroize> {
    pub envelope: Option<E>,
    pub coins: Option<EnvelopeCoins<S>>,
}

impl<E, S: Zeroize> Default for WrapToSuccessReturnOverrides<E, S> {
    fn default() -> Self {
        Self {
            envelope: None,
            coins: None,
        }
    }
}

pub fn build_wrap_to_success_return<P: IPairingAdapter, E: Default>(
    pairing: &P,
    overrides: WrapToSuccessReturnOverrides<E, P::Scalar>,
) -> WrapToSuccessReturn<E, P::Scalar> {
    WrapToSuccessReturn {
        envelope: overrides.envelope.unwrap_or_default(),
        coins: overrides
            .coins
            .unwrap_or_else(|| build_envelope_coins(pairing, Default::default())),
    }
}

pub struct UnwrapSuccessReturnOverrides<G1, G2> {
    pub credential: Option<kem::CredentialComponents<G1, G2>>,
}

impl<G1, G2> Default for UnwrapSuccessReturnOverrides<G1, G2> {
    fn default() -> Self {
        Self { credential: None }
    }
}

pub fn build_unwrap_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: UnwrapSuccessReturnOverrides<P::G1, P::G2>,
) -> UnwrapSuccessReturn<P::G1, P::G2> {
    UnwrapSuccessReturn {
        credential: overrides
            .credential
            .unwrap_or_else(|| build_credential_components(pairing, Default::default())),
    }
}

pub struct KeyPairComponentsOverrides<S: Zeroize> {
    pub x: Option<Secret<S>>,
    pub y: Option<Secret<S>>,
}

impl<S: Zeroize> Default for KeyPairComponentsOverrides<S> {
    fn default() -> Self {
        Self { x: None, y: None }
    }
}

pub fn build_key_pair_components<P: IPairingAdapter>(
    _pairing: &P,
    overrides: KeyPairComponentsOverrides<P::Scalar>,
) -> KeyPairComponents<P::Scalar> {
    KeyPairComponents {
        x: overrides
            .x
            .unwrap_or_else(|| sample_scalar_secret::<P>(0x53)),
        y: overrides
            .y
            .unwrap_or_else(|| sample_scalar_secret::<P>(0x54)),
    }
}

pub struct PublicKeysComponentsOverrides<G1, G2> {
    pub pk1: Option<G1>,
    pub pk2: Option<G2>,
}

impl<G1, G2> Default for PublicKeysComponentsOverrides<G1, G2> {
    fn default() -> Self {
        Self {
            pk1: None,
            pk2: None,
        }
    }
}

pub fn build_public_keys_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: PublicKeysComponentsOverrides<P::G1, P::G2>,
) -> PublicKeysComponents<P::G1, P::G2> {
    PublicKeysComponents {
        pk1: overrides.pk1.unwrap_or_else(|| g1_element(pairing, 0)),
        pk2: overrides.pk2.unwrap_or_else(|| g2_element(pairing, 0)),
    }
}

pub struct PossessionComponentsOverrides<S, G1, G2> {
    pub r1: Option<G1>,
    pub z1: Option<S>,
    pub r2: Option<G2>,
    pub z2: Option<S>,
}

impl<S, G1, G2> Default for PossessionComponentsOverrides<S, G1, G2> {
    fn default() -> Self {
        Self {
            r1: None,
            z1: None,
            r2: None,
            z2: None,
        }
    }
}

pub fn build_possession_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: PossessionComponentsOverrides<P::Scalar, P::G1, P::G2>,
) -> PossessionComponents<P::Scalar, P::G1, P::G2> {
    PossessionComponents {
        r1: overrides.r1.unwrap_or_else(|| g1_element(pairing, 0)),
        z1: overrides.z1.unwrap_or_else(|| sample_scalar::<P>(0x55)),
        r2: overrides.r2.unwrap_or_else(|| g2_element(pairing, 0)),
        z2: overrides.z2.unwrap_or_else(|| sample_scalar::<P>(0x56)),
    }
}

pub struct EnvelopeComponentsOverrides<G1, G2> {
    pub c1: Option<G1>,
    pub c2: Option<G1>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

impl<G1, G2> Default for EnvelopeComponentsOverrides<G1, G2> {
    fn default() -> Self {
        Self {
            c1: None,
            c2: None,
            d1: None,
            d2: None,
        }
    }
}

pub fn build_envelope_components<P: IPairingAdapter>(
    pairing: &P,
    overrides: EnvelopeComponentsOverrides<P::G1, P::G2>,
) -> EnvelopeComponents<P::G1, P::G2> {
    EnvelopeComponents {
        c1: overrides.c1.unwrap_or_else(|| g1_element(pairing, 0)),
        c2: overrides.c2.unwrap_or_else(|| g1_element(pairing, 1)),
        d1: overrides.d1.unwrap_or_else(|| g2_element(pairing, 0)),
        d2: overrides.d2.unwrap_or_else(|| g2_element(pairing, 1)),
    }
}

pub struct KeyPairComponentsSuccessReturnOverrides<S: Zeroize> {
    pub components: Option<KeyPairComponents<S>>,
}

impl<S: Zeroize> Default for KeyPairComponentsSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_key_pair_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: KeyPairComponentsSuccessReturnOverrides<P::Scalar>,
) -> KeyPairComponentsSuccessReturn<P::Scalar> {
    KeyPairComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_key_pair_components(pairing, Default::default())),
    }
}

pub struct KeyPairPublicKeysSuccessReturnOverrides<G1, G2> {
    pub components: Option<PublicKeysComponents<G1, G2>>,
}

impl<G1, G2> Default for KeyPairPublicKeysSuccessReturnOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_key_pair_public_keys_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: KeyPairPublicKeysSuccessReturnOverrides<P::G1, P::G2>,
) -> KeyPairPublicKeysSuccessReturn<P::G1, P::G2> {
    KeyPairPublicKeysSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
    }
}

pub struct PublicKeysComponentsSuccessReturnOverrides<G1, G2> {
    pub components: Option<PublicKeysComponents<G1, G2>>,
}

impl<G1, G2> Default for PublicKeysComponentsSuccessReturnOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_public_keys_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: PublicKeysComponentsSuccessReturnOverrides<P::G1, P::G2>,
) -> PublicKeysComponentsSuccessReturn<P::G1, P::G2> {
    PublicKeysComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
    }
}

pub struct PossessionComponentsSuccessReturnOverrides<S, G1, G2> {
    pub components: Option<PossessionComponents<S, G1, G2>>,
}

impl<S, G1, G2> Default for PossessionComponentsSuccessReturnOverrides<S, G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_possession_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: PossessionComponentsSuccessReturnOverrides<P::Scalar, P::G1, P::G2>,
) -> PossessionComponentsSuccessReturn<P::Scalar, P::G1, P::G2> {
    PossessionComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_possession_components(pairing, Default::default())),
    }
}

pub struct EnvelopeComponentsSuccessReturnOverrides<G1, G2> {
    pub components: Option<EnvelopeComponents<G1, G2>>,
}

impl<G1, G2> Default for EnvelopeComponentsSuccessReturnOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_envelope_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    overrides: EnvelopeComponentsSuccessReturnOverrides<P::G1, P::G2>,
) -> EnvelopeComponentsSuccessReturn<P::G1, P::G2> {
    EnvelopeComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct KeyPairFromComponentsSuccessReturnOverrides<KP> {
    pub key_pair: Option<KP>,
}

pub fn build_key_pair_from_components_success_return<KP: Default>(
    overrides: KeyPairFromComponentsSuccessReturnOverrides<KP>,
) -> KeyPairFromComponentsSuccessReturn<KP> {
    KeyPairFromComponentsSuccessReturn {
        key_pair: overrides.key_pair.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct PublicKeysFromComponentsSuccessReturnOverrides<PK> {
    pub public_keys: Option<PK>,
}

pub fn build_public_keys_from_components_success_return<PK: Default>(
    overrides: PublicKeysFromComponentsSuccessReturnOverrides<PK>,
) -> PublicKeysFromComponentsSuccessReturn<PK> {
    PublicKeysFromComponentsSuccessReturn {
        public_keys: overrides.public_keys.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct PossessionFromComponentsSuccessReturnOverrides<PO> {
    pub possession: Option<PO>,
}

pub fn build_possession_from_components_success_return<PO: Default>(
    overrides: PossessionFromComponentsSuccessReturnOverrides<PO>,
) -> PossessionFromComponentsSuccessReturn<PO> {
    PossessionFromComponentsSuccessReturn {
        possession: overrides.possession.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EnvelopeFromComponentsSuccessReturnOverrides<E> {
    pub envelope: Option<E>,
}

pub fn build_envelope_from_components_success_return<E: Default>(
    overrides: EnvelopeFromComponentsSuccessReturnOverrides<E>,
) -> EnvelopeFromComponentsSuccessReturn<E> {
    EnvelopeFromComponentsSuccessReturn {
        envelope: overrides.envelope.unwrap_or_default(),
    }
}

pub struct MockIKeyAgreementAdapter<'a, P, KP, PK, PO, E> {
    pub pairing: &'a P,
    pub key_pair: PhantomData<KP>,
    pub public_keys: PhantomData<PK>,
    pub possession: PhantomData<PO>,
    pub envelope: PhantomData<E>,
}

impl<'a, P, KP, PK, PO, E> IKeyAgreementAdapter for MockIKeyAgreementAdapter<'a, P, KP, PK, PO, E>
where
    P: IPairingAdapter,
    KP: Default,
    PK: Default,
    PO: Default,
    E: Default,
{
    const DECLARATION: KeyAgreementDeclaration = KeyAgreementDeclaration {
        identifier: KeyAgreementIdentifier::PairingElGamalV1,
        algebra: EnvelopeAlgebra::PairingElGamal,
        possession_g1_tag: PAIRING_ELGAMAL_POSSESSION_G1_TAG,
        possession_g2_tag: PAIRING_ELGAMAL_POSSESSION_G2_TAG,
        adapter_version: 1,
        interface_version: KEY_AGREEMENT_INTERFACE_VERSION,
    };

    type Pairing = P;
    type KeyPair = KP;
    type PublicKeys = PK;
    type Possession = PO;
    type Envelope = E;

    fn generate_keys(
        &self,
        _params: GenerateKeysParams,
        _payload: GenerateKeysPayload,
    ) -> super::interface::GenerateKeysReturn<Self::KeyPair, Self::Possession> {
        Ok(build_generate_keys_success_return(Default::default()))
    }

    fn wrap_to(
        &self,
        _params: WrapToParams,
        _payload: WrapToPayload<'_, Self::PublicKeys, P::G1, P::G2>,
    ) -> super::interface::WrapToReturn<Self::Envelope, P::Scalar> {
        Ok(build_wrap_to_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn unwrap(
        &self,
        _params: UnwrapParams,
        _payload: UnwrapPayload<'_, Self::KeyPair, Self::Envelope>,
    ) -> super::interface::UnwrapReturn<P::G1, P::G2> {
        Ok(build_unwrap_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn key_pair_components(
        &self,
        _params: KeyPairComponentsParams,
        _payload: KeyPairComponentsPayload<'_, Self::KeyPair>,
    ) -> super::interface::KeyPairComponentsReturn<P::Scalar> {
        Ok(build_key_pair_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn key_pair_public_keys(
        &self,
        _params: KeyPairPublicKeysParams,
        _payload: KeyPairPublicKeysPayload<'_, Self::KeyPair>,
    ) -> super::interface::KeyPairPublicKeysReturn<P::G1, P::G2> {
        Ok(build_key_pair_public_keys_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn public_keys_components(
        &self,
        _params: PublicKeysComponentsParams,
        _payload: PublicKeysComponentsPayload<'_, Self::PublicKeys>,
    ) -> super::interface::PublicKeysComponentsReturn<P::G1, P::G2> {
        Ok(build_public_keys_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn possession_components(
        &self,
        _params: PossessionComponentsParams,
        _payload: PossessionComponentsPayload<'_, Self::Possession>,
    ) -> super::interface::PossessionComponentsReturn<P::Scalar, P::G1, P::G2> {
        Ok(build_possession_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn envelope_components(
        &self,
        _params: EnvelopeComponentsParams,
        _payload: EnvelopeComponentsPayload<'_, Self::Envelope>,
    ) -> super::interface::EnvelopeComponentsReturn<P::G1, P::G2> {
        Ok(build_envelope_components_success_return(
            self.pairing,
            Default::default(),
        ))
    }

    fn key_pair_from_components(
        &self,
        _params: KeyPairFromComponentsParams,
        _payload: KeyPairFromComponentsPayload<P::Scalar>,
    ) -> super::interface::KeyPairFromComponentsReturn<Self::KeyPair> {
        Ok(build_key_pair_from_components_success_return(
            Default::default(),
        ))
    }

    fn public_keys_from_components(
        &self,
        _params: PublicKeysFromComponentsParams,
        _payload: PublicKeysFromComponentsPayload<'_, P::G1, P::G2, Self::Possession>,
    ) -> super::interface::PublicKeysFromComponentsReturn<Self::PublicKeys> {
        Ok(build_public_keys_from_components_success_return(
            Default::default(),
        ))
    }

    fn possession_from_components(
        &self,
        _params: PossessionFromComponentsParams,
        _payload: PossessionFromComponentsPayload<P::Scalar, P::G1, P::G2>,
    ) -> super::interface::PossessionFromComponentsReturn<Self::Possession> {
        Ok(build_possession_from_components_success_return(
            Default::default(),
        ))
    }

    fn envelope_from_components(
        &self,
        _params: EnvelopeFromComponentsParams,
        _payload: EnvelopeFromComponentsPayload<P::G1, P::G2>,
    ) -> super::interface::EnvelopeFromComponentsReturn<Self::Envelope> {
        Ok(build_envelope_from_components_success_return(
            Default::default(),
        ))
    }
}

#[derive(Default)]
pub struct CreateKeyAgreementParamsOverrides {
    pub concrete: Option<KeyAgreementConcrete>,
    pub identifier: Option<KeyAgreementIdentifier>,
    pub algebra: Option<EnvelopeAlgebra>,
}

pub fn build_create_key_agreement_params(
    overrides: CreateKeyAgreementParamsOverrides,
) -> CreateKeyAgreementParams {
    CreateKeyAgreementParams {
        concrete: overrides
            .concrete
            .unwrap_or(KeyAgreementConcrete::PairingElGamal),
        identifier: overrides
            .identifier
            .unwrap_or(KeyAgreementIdentifier::PairingElGamalV1),
        algebra: overrides.algebra.unwrap_or(EnvelopeAlgebra::PairingElGamal),
    }
}

#[derive(Default)]
pub struct CreateKeyAgreementSuccessReturnOverrides<O> {
    pub output: Option<O>,
}

pub fn build_create_key_agreement_success_return<O: Default>(
    overrides: CreateKeyAgreementSuccessReturnOverrides<O>,
) -> CreateKeyAgreementSuccessReturn<O> {
    CreateKeyAgreementSuccessReturn {
        output: overrides.output.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ConsumeKeyAgreementPayloadOverrides<K: IKeyAgreementAdapter> {
    pub adapter: Option<K>,
}

pub fn build_consume_key_agreement_payload<K: IKeyAgreementAdapter + Default>(
    overrides: ConsumeKeyAgreementPayloadOverrides<K>,
) -> ConsumeKeyAgreementPayload<K> {
    ConsumeKeyAgreementPayload {
        adapter: overrides.adapter.unwrap_or_default(),
    }
}

pub struct MockIKeyAgreementConsumer;

impl<P: IPairingAdapter> IKeyAgreementConsumer<P> for MockIKeyAgreementConsumer {
    type Output = ();

    fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        _payload: ConsumeKeyAgreementPayload<K>,
    ) -> Self::Output {
    }
}

pub fn mock_create_key_agreement<'a, P: IPairingArithmetic, E: IEncoderAdapter, C>(
    _deps: &CreateKeyAgreementDeps<'a, P, E, C>,
    _params: CreateKeyAgreementParams,
    _payload: CreateKeyAgreementPayload,
) -> CreateKeyAgreementReturn<C::Output>
where
    C: IKeyAgreementConsumer<P>,
    C::Output: Default,
{
    Ok(build_create_key_agreement_success_return(Default::default()))
}
