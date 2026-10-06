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
use pairing::{IPairingAdapter, IPairingArithmetic};
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

#[derive(Default)]
pub struct EnvelopeCoinsOverrides<S: Zeroize> {
    pub rho: Option<Secret<S>>,
    pub sigma: Option<Secret<S>>,
}

pub fn build_envelope_coins<S: Zeroize + Default>(
    overrides: EnvelopeCoinsOverrides<S>,
) -> EnvelopeCoins<S> {
    EnvelopeCoins {
        rho: overrides
            .rho
            .unwrap_or_else(|| build_secret(SecretConstructorParamsOverrides::default())),
        sigma: overrides
            .sigma
            .unwrap_or_else(|| build_secret(SecretConstructorParamsOverrides::default())),
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

#[derive(Default)]
pub struct WrapToSuccessReturnOverrides<E, S: Zeroize> {
    pub envelope: Option<E>,
    pub coins: Option<EnvelopeCoins<S>>,
}

pub fn build_wrap_to_success_return<E: Default, S: Zeroize + Default>(
    overrides: WrapToSuccessReturnOverrides<E, S>,
) -> WrapToSuccessReturn<E, S> {
    WrapToSuccessReturn {
        envelope: overrides.envelope.unwrap_or_default(),
        coins: overrides
            .coins
            .unwrap_or_else(|| build_envelope_coins(Default::default())),
    }
}

#[derive(Default)]
pub struct UnwrapSuccessReturnOverrides<G1, G2> {
    pub credential: Option<kem::CredentialComponents<G1, G2>>,
}

pub fn build_unwrap_success_return<G1: Default, G2: Default>(
    overrides: UnwrapSuccessReturnOverrides<G1, G2>,
) -> UnwrapSuccessReturn<G1, G2> {
    UnwrapSuccessReturn {
        credential: overrides
            .credential
            .unwrap_or_else(|| build_credential_components(Default::default())),
    }
}

#[derive(Default)]
pub struct KeyPairComponentsOverrides<S: Zeroize> {
    pub x: Option<Secret<S>>,
    pub y: Option<Secret<S>>,
}

pub fn build_key_pair_components<S: Zeroize + Default>(
    overrides: KeyPairComponentsOverrides<S>,
) -> KeyPairComponents<S> {
    KeyPairComponents {
        x: overrides
            .x
            .unwrap_or_else(|| build_secret(SecretConstructorParamsOverrides::default())),
        y: overrides
            .y
            .unwrap_or_else(|| build_secret(SecretConstructorParamsOverrides::default())),
    }
}

#[derive(Default)]
pub struct PublicKeysComponentsOverrides<G1, G2> {
    pub pk1: Option<G1>,
    pub pk2: Option<G2>,
}

pub fn build_public_keys_components<G1: Default, G2: Default>(
    overrides: PublicKeysComponentsOverrides<G1, G2>,
) -> PublicKeysComponents<G1, G2> {
    PublicKeysComponents {
        pk1: overrides.pk1.unwrap_or_default(),
        pk2: overrides.pk2.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct PossessionComponentsOverrides<S, G1, G2> {
    pub r1: Option<G1>,
    pub z1: Option<S>,
    pub r2: Option<G2>,
    pub z2: Option<S>,
}

pub fn build_possession_components<S: Default, G1: Default, G2: Default>(
    overrides: PossessionComponentsOverrides<S, G1, G2>,
) -> PossessionComponents<S, G1, G2> {
    PossessionComponents {
        r1: overrides.r1.unwrap_or_default(),
        z1: overrides.z1.unwrap_or_default(),
        r2: overrides.r2.unwrap_or_default(),
        z2: overrides.z2.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct EnvelopeComponentsOverrides<G1, G2> {
    pub c1: Option<G1>,
    pub c2: Option<G1>,
    pub d1: Option<G2>,
    pub d2: Option<G2>,
}

pub fn build_envelope_components<G1: Default, G2: Default>(
    overrides: EnvelopeComponentsOverrides<G1, G2>,
) -> EnvelopeComponents<G1, G2> {
    EnvelopeComponents {
        c1: overrides.c1.unwrap_or_default(),
        c2: overrides.c2.unwrap_or_default(),
        d1: overrides.d1.unwrap_or_default(),
        d2: overrides.d2.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct KeyPairComponentsSuccessReturnOverrides<S: Zeroize> {
    pub components: Option<KeyPairComponents<S>>,
}

pub fn build_key_pair_components_success_return<S: Zeroize + Default>(
    overrides: KeyPairComponentsSuccessReturnOverrides<S>,
) -> KeyPairComponentsSuccessReturn<S> {
    KeyPairComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_key_pair_components(Default::default())),
    }
}

#[derive(Default)]
pub struct KeyPairPublicKeysSuccessReturnOverrides<G1, G2> {
    pub components: Option<PublicKeysComponents<G1, G2>>,
}

pub fn build_key_pair_public_keys_success_return<G1: Default, G2: Default>(
    overrides: KeyPairPublicKeysSuccessReturnOverrides<G1, G2>,
) -> KeyPairPublicKeysSuccessReturn<G1, G2> {
    KeyPairPublicKeysSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_public_keys_components(Default::default())),
    }
}

#[derive(Default)]
pub struct PublicKeysComponentsSuccessReturnOverrides<G1, G2> {
    pub components: Option<PublicKeysComponents<G1, G2>>,
}

pub fn build_public_keys_components_success_return<G1: Default, G2: Default>(
    overrides: PublicKeysComponentsSuccessReturnOverrides<G1, G2>,
) -> PublicKeysComponentsSuccessReturn<G1, G2> {
    PublicKeysComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_public_keys_components(Default::default())),
    }
}

#[derive(Default)]
pub struct PossessionComponentsSuccessReturnOverrides<S, G1, G2> {
    pub components: Option<PossessionComponents<S, G1, G2>>,
}

pub fn build_possession_components_success_return<S: Default, G1: Default, G2: Default>(
    overrides: PossessionComponentsSuccessReturnOverrides<S, G1, G2>,
) -> PossessionComponentsSuccessReturn<S, G1, G2> {
    PossessionComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_possession_components(Default::default())),
    }
}

#[derive(Default)]
pub struct EnvelopeComponentsSuccessReturnOverrides<G1, G2> {
    pub components: Option<EnvelopeComponents<G1, G2>>,
}

pub fn build_envelope_components_success_return<G1: Default, G2: Default>(
    overrides: EnvelopeComponentsSuccessReturnOverrides<G1, G2>,
) -> EnvelopeComponentsSuccessReturn<G1, G2> {
    EnvelopeComponentsSuccessReturn {
        components: overrides
            .components
            .unwrap_or_else(|| build_envelope_components(Default::default())),
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

pub struct MockIKeyAgreementAdapter<P, KP, PK, PO, E> {
    pub pairing: PhantomData<P>,
    pub key_pair: PhantomData<KP>,
    pub public_keys: PhantomData<PK>,
    pub possession: PhantomData<PO>,
    pub envelope: PhantomData<E>,
}

impl<P, KP, PK, PO, E> IKeyAgreementAdapter for MockIKeyAgreementAdapter<P, KP, PK, PO, E>
where
    P: IPairingAdapter,
    P::Scalar: Default,
    P::G1: Default,
    P::G2: Default,
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
        Ok(build_wrap_to_success_return(Default::default()))
    }

    fn unwrap(
        &self,
        _params: UnwrapParams,
        _payload: UnwrapPayload<'_, Self::KeyPair, Self::Envelope>,
    ) -> super::interface::UnwrapReturn<P::G1, P::G2> {
        Ok(build_unwrap_success_return(Default::default()))
    }

    fn key_pair_components(
        &self,
        _params: KeyPairComponentsParams,
        _payload: KeyPairComponentsPayload<'_, Self::KeyPair>,
    ) -> super::interface::KeyPairComponentsReturn<P::Scalar> {
        Ok(build_key_pair_components_success_return(Default::default()))
    }

    fn key_pair_public_keys(
        &self,
        _params: KeyPairPublicKeysParams,
        _payload: KeyPairPublicKeysPayload<'_, Self::KeyPair>,
    ) -> super::interface::KeyPairPublicKeysReturn<P::G1, P::G2> {
        Ok(build_key_pair_public_keys_success_return(Default::default()))
    }

    fn public_keys_components(
        &self,
        _params: PublicKeysComponentsParams,
        _payload: PublicKeysComponentsPayload<'_, Self::PublicKeys>,
    ) -> super::interface::PublicKeysComponentsReturn<P::G1, P::G2> {
        Ok(build_public_keys_components_success_return(
            Default::default(),
        ))
    }

    fn possession_components(
        &self,
        _params: PossessionComponentsParams,
        _payload: PossessionComponentsPayload<'_, Self::Possession>,
    ) -> super::interface::PossessionComponentsReturn<P::Scalar, P::G1, P::G2> {
        Ok(build_possession_components_success_return(
            Default::default(),
        ))
    }

    fn envelope_components(
        &self,
        _params: EnvelopeComponentsParams,
        _payload: EnvelopeComponentsPayload<'_, Self::Envelope>,
    ) -> super::interface::EnvelopeComponentsReturn<P::G1, P::G2> {
        Ok(build_envelope_components_success_return(Default::default()))
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
