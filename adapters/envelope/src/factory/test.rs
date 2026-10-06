#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_key_agreement;
use super::interface::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementDeps,
    CreateKeyAgreementPayload, EnvelopeAlgebra, IKeyAgreementAdapter, IKeyAgreementConsumer,
    KEY_AGREEMENT_INTERFACE_VERSION, KeyAgreementDeclaration, KeyAgreementIdentifier,
};
use super::mock::build_create_key_agreement_params;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_params,
    create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    PairingConcrete, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

struct DeclarationProbe;

impl<P: IPairingAdapter> IKeyAgreementConsumer<P> for DeclarationProbe {
    type Output = KeyAgreementDeclaration;

    fn consume_key_agreement<K: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        _payload: ConsumeKeyAgreementPayload<K>,
    ) -> Self::Output {
        K::DECLARATION
    }
}

struct FactoryStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for FactoryStep<'_, P> {
    type Output = KeyAgreementDeclaration;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(success) = create_key_agreement(
            &CreateKeyAgreementDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
                consumer: DeclarationProbe,
            },
            build_create_key_agreement_params(Default::default()),
            CreateKeyAgreementPayload,
        ) else {
            panic!("the named concrete is admitted, constructed, and consumed");
        };

        success.output
    }
}

struct FactoryProbe;

impl IPairingConsumer for FactoryProbe {
    type Output = KeyAgreementDeclaration;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        // Arrange
        let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the default hash-to-scalar concrete is constructed");
        };
        let Ok(source) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );

        // Act
        let Ok(success) = create_encoding(
            &CreateEncodingDeps {
                consumer: FactoryStep {
                    pairing: &payload.adapter,
                    hash_to_scalar: hashing.adapter.as_ref(),
                    random: source.adapter.as_ref(),
                },
            },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the default encoding concrete is constructed and consumed");
        };

        success.output
    }
}

/// Contract: the pairing ElGamal concrete, admitted for its identifier and its
///   algebra, reaches the consumer with its declaration.
/// Arrange: `FactoryProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `identifier` is `KeyAgreementIdentifier::PairingElGamalV1`,
///   `algebra` is `EnvelopeAlgebra::PairingElGamal`, `possession_g1_tag` is
///   `b"ChainTorrent-v1-envelope-possession-g1"`, `possession_g2_tag` is
///   `b"ChainTorrent-v1-envelope-possession-g2"`, `adapter_version` is `1`, and
///   `interface_version` is `KEY_AGREEMENT_INTERFACE_VERSION`.
#[test]
fn create_key_agreement_hands_the_consumer_the_pairing_elgamal_concrete_and_its_declaration() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: FactoryProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named pairing concrete is constructed and consumed");
    };

    // Assert
    assert!(matches!(
        success.output.identifier,
        KeyAgreementIdentifier::PairingElGamalV1
    ));
    assert_eq!(success.output.algebra, EnvelopeAlgebra::PairingElGamal);
    assert_eq!(
        success.output.possession_g1_tag,
        b"ChainTorrent-v1-envelope-possession-g1"
    );
    assert_eq!(
        success.output.possession_g2_tag,
        b"ChainTorrent-v1-envelope-possession-g2"
    );
    assert_eq!(success.output.adapter_version, 1);
    assert_eq!(
        success.output.interface_version,
        KEY_AGREEMENT_INTERFACE_VERSION
    );
}
