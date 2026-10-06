#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_params,
    create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementDeps,
    CreateKeyAgreementPayload, GenerateKeysParams, GenerateKeysPayloadOverrides,
    IKeyAgreementAdapter, IKeyAgreementConsumer, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, UnwrapParams, UnwrapPayload,
    WrapToParams, WrapToPayload, build_create_key_agreement_params, build_generate_keys_payload,
    create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemPayload,
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, DeriveIdentityParams, DeriveIdentityPayload,
    ICredentialKemAdapter, IKemConsumer, IsValidParams, IsValidPayload, IssueParams, IssuePayload,
    KemIdentity, SetupPayloadOverrides, build_create_kem_params, build_setup_params,
    build_setup_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, PairingConcrete, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, build_create_random_source_params, build_fill_bytes_payload,
    create_random_source,
};

struct DeliveryOutcome {
    recipient_credential_is_valid: bool,
    other_credential_is_valid: bool,
}

struct DeliveryStep<'k, K> {
    kem: &'k K,
    random: &'k dyn IRandomSourceAdapter,
    identity: Vec<u8>,
}

impl<P, K> IKeyAgreementConsumer<P> for DeliveryStep<'_, K>
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
{
    type Output = DeliveryOutcome;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        // Arrange
        let kem = self.kem;
        let key_agreement = payload.adapter;
        let draw = || {
            let Ok(success) = self.random.fill_bytes(
                FillBytesParams,
                build_fill_bytes_payload(FillBytesPayloadOverrides {
                    length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                }),
            ) else {
                panic!("the operating system's generator fills the draw");
            };
            success.bytes
        };

        // Act
        let Ok(setup) = kem.setup(
            build_setup_params(Default::default()),
            build_setup_payload(SetupPayloadOverrides {
                master_uniform: Some(draw()),
                u0_uniform: Some(draw()),
                u1_uniform: Some(draw()),
            }),
        ) else {
            panic!("the parameter set is set up from operating-system draws");
        };
        let Ok(identity_element) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &setup.parameter_set,
                identity: KemIdentity::Entitlement {
                    canonical: self.identity.as_slice(),
                },
            },
        ) else {
            panic!("the identity element is derived");
        };
        let Ok(issued) = kem.issue(
            IssueParams,
            IssuePayload {
                parameter_set: &setup.parameter_set,
                master_scalar: &setup.master_scalar,
                identity_element: &identity_element.identity_element,
                uniform: draw(),
            },
        ) else {
            panic!("a credential is issued");
        };
        let Ok(credential_components) = kem.credential_components(
            CredentialComponentsParams,
            CredentialComponentsPayload {
                credential: &issued.credential,
            },
        );
        let Ok(recipient) = key_agreement.generate_keys(
            GenerateKeysParams,
            build_generate_keys_payload(GenerateKeysPayloadOverrides {
                x_uniform: Some(draw()),
                y_uniform: Some(draw()),
            }),
        ) else {
            panic!("the recipient's key pair is generated");
        };
        let Ok(other) = key_agreement.generate_keys(
            GenerateKeysParams,
            build_generate_keys_payload(GenerateKeysPayloadOverrides {
                x_uniform: Some(draw()),
                y_uniform: Some(draw()),
            }),
        ) else {
            panic!("the other key pair is generated");
        };
        let Ok(recipient_components) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &recipient.key_pair,
            },
        );
        let Ok(public_keys) = key_agreement.public_keys_from_components(
            PublicKeysFromComponentsParams,
            PublicKeysFromComponentsPayload {
                components: recipient_components.components,
                possession: &recipient.possession,
            },
        ) else {
            panic!("the recipient's public keys are admitted with their proof of possession");
        };
        let Ok(wrapped) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &public_keys.public_keys,
                credential: credential_components.components,
            },
        ) else {
            panic!("the credential's components are wrapped to the recipient's public keys");
        };
        let Ok(recipient_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &recipient.key_pair,
                envelope: &wrapped.envelope,
            },
        );
        let Ok(other_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &other.key_pair,
                envelope: &wrapped.envelope,
            },
        );
        let Ok(recipient_credential) = kem.credential_from_components(
            CredentialFromComponentsParams,
            CredentialFromComponentsPayload {
                components: recipient_unwrapped.credential,
            },
        );
        let Ok(other_credential) = kem.credential_from_components(
            CredentialFromComponentsParams,
            CredentialFromComponentsPayload {
                components: other_unwrapped.credential,
            },
        );
        let Ok(recipient_validity) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &setup.parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &recipient_credential.credential,
            },
        );
        let Ok(other_validity) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &setup.parameter_set,
                identity_element: &identity_element.identity_element,
                credential: &other_credential.credential,
            },
        );

        DeliveryOutcome {
            recipient_credential_is_valid: recipient_validity.is_valid,
            other_credential_is_valid: other_validity.is_valid,
        }
    }
}

struct KemStep<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    identity: Vec<u8>,
}

impl<P: IPairingArithmetic, E: IEncoderAdapter> IKemConsumer<P> for KemStep<'_, P, E> {
    type Output = DeliveryOutcome;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let Ok(success) = create_key_agreement(
            &CreateKeyAgreementDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                consumer: DeliveryStep {
                    kem: &payload.adapter,
                    random: self.random,
                    identity: self.identity.clone(),
                },
            },
            build_create_key_agreement_params(Default::default()),
            CreateKeyAgreementPayload,
        ) else {
            panic!("the named concrete is admitted, constructed, and consumed");
        };

        success.output
    }
}

struct EncodingStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
    identity: Vec<u8>,
}

impl<P: IPairingArithmetic> IEncodingConsumer for EncodingStep<'_, P> {
    type Output = DeliveryOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(success) = create_kem(
            &CreateKemDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                consumer: KemStep {
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: &payload.adapter,
                    random: self.random,
                    identity: self.identity.clone(),
                },
            },
            build_create_kem_params(Default::default()),
            CreateKemPayload,
        ) else {
            panic!("the named concrete is admitted, constructed, and consumed");
        };

        success.output
    }
}

struct DeliveryProbe {
    identity: Vec<u8>,
}

impl IPairingConsumer for DeliveryProbe {
    type Output = DeliveryOutcome;

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
                consumer: EncodingStep {
                    pairing: &payload.adapter,
                    hash_to_scalar: hashing.adapter.as_ref(),
                    random: source.adapter.as_ref(),
                    identity: self.identity.clone(),
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

/// Contract: a credential the KEM factory's concrete issued, wrapped through
///   the key-agreement factory's concrete to a recipient's public keys
///   admitted with their proof of possession, opens under the recipient's key
///   pair to components the KEM rebuilds into a valid credential, and under
///   another key pair to components it rebuilds into an invalid one (CR-04
///   exact round trip and swapped keys; LC-08 admission at the algebra level).
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_random_source`,
///   `create_encoding` with the ABI concrete, `create_kem` with the Boneh–Boyen
///   concrete, `create_key_agreement`, and the pairing ElGamal concrete, each
///   real.
/// Mocked:  nothing; the curve libraries, `sha3`, `alloy`, and the operating
///   system's generator are the outer edges.
/// Arrange: `DeliveryProbe` over `b"entitlement-one"`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `recipient_credential_is_valid` is `true` and
///   `other_credential_is_valid` is `false`.
#[test]
fn a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair_on_bls12_381_arkworks()
 {
    // Arrange
    let consumer = DeliveryProbe {
        identity: b"entitlement-one".to_vec(),
    };
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.recipient_credential_is_valid);
    assert!(!success.output.other_credential_is_valid);
}

/// Contract: a credential the KEM factory's concrete issued, wrapped through
///   the key-agreement factory's concrete to a recipient's public keys
///   admitted with their proof of possession, opens under the recipient's key
///   pair to components the KEM rebuilds into a valid credential, and under
///   another key pair to components it rebuilds into an invalid one (CR-04
///   exact round trip and swapped keys; LC-08 admission at the algebra level).
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_random_source`,
///   `create_encoding` with the ABI concrete, `create_kem` with the Boneh–Boyen
///   concrete, `create_key_agreement`, and the pairing ElGamal concrete, each
///   real.
/// Mocked:  nothing; the curve libraries, `sha3`, `alloy`, and the operating
///   system's generator are the outer edges.
/// Arrange: `DeliveryProbe` over `b"entitlement-one"`.
/// Act:     `create_pairing` on `PairingConcrete::Bn254Halo2curves`.
/// Assert:  `recipient_credential_is_valid` is `true` and
///   `other_credential_is_valid` is `false`.
#[test]
fn a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair_on_bn254_halo2curves()
 {
    // Arrange
    let consumer = DeliveryProbe {
        identity: b"entitlement-one".to_vec(),
    };
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.recipient_credential_is_valid);
    assert!(!success.output.other_credential_is_valid);
}
