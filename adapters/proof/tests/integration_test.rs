#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use chain::MockIChainForms;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_params,
    create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementDeps,
    CreateKeyAgreementPayload, EnvelopeComponentsParams, EnvelopeComponentsPayload,
    GenerateKeysParams, GenerateKeysPayloadOverrides, IKeyAgreementAdapter, IKeyAgreementConsumer,
    KeyPairComponentsParams, KeyPairComponentsPayload, KeyPairPublicKeysParams,
    KeyPairPublicKeysPayload, PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload,
    UnwrapParams, UnwrapPayload, WrapToParams, WrapToPayload, build_create_key_agreement_params,
    build_generate_keys_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemPayload,
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, DeriveIdentityParams, DeriveIdentityPayload,
    ICredentialKemAdapter, IKemConsumer, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, IsValidParams, IsValidPayload, IssueParams, IssuePayload,
    KemIdentity, MasterScalarComponentsParams, MasterScalarComponentsPayload,
    ParameterSetComponentsParams, ParameterSetComponentsPayload, RerandomizeParams,
    RerandomizePayload, SetupPayloadOverrides, build_create_kem_params, build_setup_params,
    build_setup_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, PairingConcrete, VerifierGroupArithmetic, build_create_pairing_params,
    create_pairing,
};
use proof::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofDeps,
    CreateDeliveryProofParamsOverrides, CreateDeliveryProofPayload, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, IDeliveryProofAdapter,
    IDeliveryProofConsumer, MintPurpose, ProveMintParams, ProveMintPayload, ProveTransferParams,
    ProveTransferPayload, TransferPurpose, VerifyParams, VerifyPayload,
    build_create_delivery_proof_params, build_delivery_context, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, build_create_random_source_params, build_fill_bytes_payload,
    create_random_source,
};

struct ProofOutcome {
    mint_verifies: bool,
    transfer_verifies: bool,
    buyer_credential_is_valid: bool,
    replayed_transfer_rejected: bool,
}

struct DeliveryStep<'k, K, A> {
    kem: &'k K,
    key_agreement: &'k A,
    random: &'k dyn IRandomSourceAdapter,
}

impl<P, K, A> IDeliveryProofConsumer<P, MockIChainForms> for DeliveryStep<'_, K, A>
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
{
    type Output = ProofOutcome;

    fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = MockIChainForms>>(
        &self,
        _params: ConsumeDeliveryProofParams,
        payload: ConsumeDeliveryProofPayload<D>,
    ) -> Self::Output {
        // Arrange
        let kem = self.kem;
        let key_agreement = self.key_agreement;
        let adapter = payload.adapter;
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
        let Ok(identity) = kem.derive_identity(
            DeriveIdentityParams,
            DeriveIdentityPayload {
                parameter_set: &setup.parameter_set,
                identity: KemIdentity::Entitlement {
                    canonical: b"entitlement-one",
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
                identity_element: &identity.identity_element,
                uniform: draw(),
            },
        ) else {
            panic!("a credential is issued");
        };
        let Ok(seller) = key_agreement.generate_keys(
            GenerateKeysParams,
            build_generate_keys_payload(GenerateKeysPayloadOverrides {
                x_uniform: Some(draw()),
                y_uniform: Some(draw()),
            }),
        ) else {
            panic!("the seller's key pair is generated");
        };
        let Ok(buyer) = key_agreement.generate_keys(
            GenerateKeysParams,
            build_generate_keys_payload(GenerateKeysPayloadOverrides {
                x_uniform: Some(draw()),
                y_uniform: Some(draw()),
            }),
        ) else {
            panic!("the buyer's key pair is generated");
        };
        let Ok(seller_components) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &seller.key_pair,
            },
        );
        let Ok(seller_public) = key_agreement.public_keys_from_components(
            PublicKeysFromComponentsParams,
            PublicKeysFromComponentsPayload {
                components: seller_components.components,
                possession: &seller.possession,
            },
        ) else {
            panic!("the seller's public keys are admitted with their proof of possession");
        };
        let Ok(buyer_components) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &buyer.key_pair,
            },
        );
        let Ok(buyer_public) = key_agreement.public_keys_from_components(
            PublicKeysFromComponentsParams,
            PublicKeysFromComponentsPayload {
                components: buyer_components.components,
                possession: &buyer.possession,
            },
        ) else {
            panic!("the buyer's public keys are admitted with their proof of possession");
        };
        let Ok(issued_components) = kem.credential_components(
            CredentialComponentsParams,
            CredentialComponentsPayload {
                credential: &issued.credential,
            },
        );
        let Ok(old_wrap) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &seller_public.public_keys,
                credential: issued_components.components,
            },
        ) else {
            panic!("the credential's components are wrapped to the seller's public keys");
        };
        let Ok(parameter_set) = kem.parameter_set_components(
            ParameterSetComponentsParams,
            ParameterSetComponentsPayload {
                parameter_set: &setup.parameter_set,
            },
        );
        let Ok(identity_components) = kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity.identity_element,
            },
        );
        let Ok(seller_keys) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &seller.key_pair,
            },
        );
        let Ok(old_envelope) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &old_wrap.envelope,
            },
        );
        let mint_statement = AlgebraicMintStatement {
            context: build_delivery_context::<MockIChainForms, MintPurpose>(
                DeliveryContextOverrides {
                    purpose: Some(DELIVERY_PURPOSE_MINT),
                    ..Default::default()
                },
            ),
            hpub: parameter_set.components.hpub,
            identity_element: identity_components.components.element,
            buyer_keys: seller_keys.components,
            envelope: old_envelope.components,
        };
        let Ok(master_scalar) = kem.master_scalar_components(
            MasterScalarComponentsParams,
            MasterScalarComponentsPayload {
                master_scalar: &setup.master_scalar,
            },
        );
        let Ok(minted) = adapter.prove_mint(
            ProveMintParams,
            ProveMintPayload {
                master_scalar: master_scalar.components,
                credential_randomness: issued.randomness,
                coins: old_wrap.coins,
                statement: &mint_statement,
            },
        ) else {
            panic!("the mint proof is produced");
        };
        let Ok(mint_verify) = adapter.verify(
            VerifyParams,
            VerifyPayload {
                statement: AlgebraicStatement::Mint(&mint_statement),
                proof: &minted.proof,
            },
        ) else {
            panic!("the mint proof is checked");
        };
        let Ok(seller_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &seller.key_pair,
                envelope: &old_wrap.envelope,
            },
        );
        let Ok(seller_credential) = kem.credential_from_components(
            CredentialFromComponentsParams,
            CredentialFromComponentsPayload {
                components: seller_unwrapped.credential,
            },
        );
        let Ok(rerandomized) = kem.rerandomize(
            RerandomizeParams,
            RerandomizePayload {
                parameter_set: &setup.parameter_set,
                identity_element: &identity.identity_element,
                credential: &seller_credential.credential,
                uniform: draw(),
            },
        ) else {
            panic!("the seller's credential is rerandomized");
        };
        let Ok(rerandomized_components) = kem.credential_components(
            CredentialComponentsParams,
            CredentialComponentsPayload {
                credential: &rerandomized.credential,
            },
        );
        let Ok(new_wrap) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &buyer_public.public_keys,
                credential: rerandomized_components.components,
            },
        ) else {
            panic!(
                "the rerandomized credential's components are wrapped to the buyer's public keys"
            );
        };
        let Ok(identity_components) = kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity.identity_element,
            },
        );
        let Ok(seller_keys) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &seller.key_pair,
            },
        );
        let Ok(buyer_keys) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &buyer.key_pair,
            },
        );
        let Ok(old_envelope) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &old_wrap.envelope,
            },
        );
        let Ok(new_envelope) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &new_wrap.envelope,
            },
        );
        let transfer_statement = AlgebraicTransferStatement {
            context: build_delivery_context::<MockIChainForms, TransferPurpose>(
                DeliveryContextOverrides {
                    purpose: Some(DELIVERY_PURPOSE_TRANSFER),
                    ..Default::default()
                },
            ),
            identity_element: identity_components.components.element,
            seller_keys: seller_keys.components,
            buyer_keys: buyer_keys.components,
            old_envelope: old_envelope.components,
            new_envelope: new_envelope.components,
        };
        let Ok(seller_secrets) = key_agreement.key_pair_components(
            KeyPairComponentsParams,
            KeyPairComponentsPayload {
                key_pair: &seller.key_pair,
            },
        );
        let Ok(transferred) = adapter.prove_transfer(
            ProveTransferParams,
            ProveTransferPayload {
                seller_secrets: seller_secrets.components,
                offset: rerandomized.offset,
                coins: new_wrap.coins,
                statement: &transfer_statement,
            },
        ) else {
            panic!("the transfer proof is produced");
        };
        let Ok(transfer_verify) = adapter.verify(
            VerifyParams,
            VerifyPayload {
                statement: AlgebraicStatement::Transfer(&transfer_statement),
                proof: &transferred.proof,
            },
        ) else {
            panic!("the transfer proof is checked");
        };
        let Ok(buyer_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &buyer.key_pair,
                envelope: &new_wrap.envelope,
            },
        );
        let Ok(buyer_credential) = kem.credential_from_components(
            CredentialFromComponentsParams,
            CredentialFromComponentsPayload {
                components: buyer_unwrapped.credential,
            },
        );
        let Ok(buyer_validity) = kem.is_valid(
            IsValidParams,
            IsValidPayload {
                parameter_set: &setup.parameter_set,
                identity_element: &identity.identity_element,
                credential: &buyer_credential.credential,
            },
        );
        let Ok(identity_components) = kem.identity_element_components(
            IdentityElementComponentsParams,
            IdentityElementComponentsPayload {
                identity_element: &identity.identity_element,
            },
        );
        let Ok(seller_keys) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &seller.key_pair,
            },
        );
        let Ok(buyer_keys) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &buyer.key_pair,
            },
        );
        let Ok(old_envelope) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &old_wrap.envelope,
            },
        );
        let Ok(new_envelope) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &new_wrap.envelope,
            },
        );
        let replayed_statement = AlgebraicTransferStatement {
            context: build_delivery_context::<MockIChainForms, TransferPurpose>(
                DeliveryContextOverrides {
                    purpose: Some(DELIVERY_PURPOSE_TRANSFER),
                    expiry: Some(1_700_000_001),
                    ..Default::default()
                },
            ),
            identity_element: identity_components.components.element,
            seller_keys: seller_keys.components,
            buyer_keys: buyer_keys.components,
            old_envelope: old_envelope.components,
            new_envelope: new_envelope.components,
        };
        let Ok(replay_verify) = adapter.verify(
            VerifyParams,
            VerifyPayload {
                statement: AlgebraicStatement::Transfer(&replayed_statement),
                proof: &transferred.proof,
            },
        ) else {
            panic!("the replayed transfer proof is checked");
        };

        ProofOutcome {
            mint_verifies: mint_verify.is_valid,
            transfer_verifies: transfer_verify.is_valid,
            buyer_credential_is_valid: buyer_validity.is_valid,
            replayed_transfer_rejected: !replay_verify.is_valid,
        }
    }
}

struct KeyAgreementStep<'a, P: IPairingArithmetic, E: IEncoderAdapter, K> {
    kem: &'a K,
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    verifier_form: VerifierGroupArithmetic,
}

impl<P, E, K> IKeyAgreementConsumer<P> for KeyAgreementStep<'_, P, E, K>
where
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
{
    type Output = ProofOutcome;

    fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKeyAgreementParams,
        payload: ConsumeKeyAgreementPayload<A>,
    ) -> Self::Output {
        let Ok(success) = create_delivery_proof(
            &CreateDeliveryProofDeps {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: self.encoder,
                random: self.random,
                consumer: DeliveryStep {
                    kem: self.kem,
                    key_agreement: &payload.adapter,
                    random: self.random,
                },
            },
            build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                verifier_form: Some(self.verifier_form),
                ..Default::default()
            }),
            CreateDeliveryProofPayload,
        ) else {
            panic!("the Schnorr concrete is admitted, constructed, and consumed");
        };

        success.output
    }
}

struct KemStep<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    verifier_form: VerifierGroupArithmetic,
}

impl<P: IPairingArithmetic, E: IEncoderAdapter> IKemConsumer<P> for KemStep<'_, P, E> {
    type Output = ProofOutcome;

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
                consumer: KeyAgreementStep {
                    kem: &payload.adapter,
                    pairing: self.pairing,
                    hash_to_scalar: self.hash_to_scalar,
                    encoder: self.encoder,
                    random: self.random,
                    verifier_form: self.verifier_form,
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
    verifier_form: VerifierGroupArithmetic,
}

impl<P: IPairingArithmetic> IEncodingConsumer for EncodingStep<'_, P> {
    type Output = ProofOutcome;

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
                    verifier_form: self.verifier_form,
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

struct ProofProbe;

impl IPairingConsumer for ProofProbe {
    type Output = ProofOutcome;

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
                    verifier_form: P::DECLARATION.verifier_group_arithmetic,
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

/// Contract: a mint proof over a credential the KEM issued and the key
///   agreement wrapped verifies, a transfer proof from the seller's fresh
///   decryption and the KEM's rerandomization offset verifies, the buyer's
///   credential opens valid, and the transfer proof fails against another
///   settlement (CR-09; CD-01; CD-02).
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_random_source`,
///   `create_encoding` with the ABI concrete, `create_kem` with the Boneh–Boyen
///   concrete, `create_key_agreement` with the pairing ElGamal concrete, and
///   `create_delivery_proof` with the Schnorr concrete in the both-groups form
///   the pairing declares, each real.
/// Mocked:  nothing but the chain's forms, `MockIChainForms`; the curve
///   libraries, `sha3`, `alloy`, and the operating system's generator are the
///   outer edges.
/// Arrange: `ProofProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bls12381Arkworks`.
/// Assert:  `mint_verifies`, `transfer_verifies`, `buyer_credential_is_valid`,
///   and `replayed_transfer_rejected` are each `true`.
#[test]
fn a_minted_and_transferred_credential_is_proved_and_verified_through_the_families_on_bls12_381_arkworks()
 {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: ProofProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.mint_verifies);
    assert!(success.output.transfer_verifies);
    assert!(success.output.buyer_credential_is_valid);
    assert!(success.output.replayed_transfer_rejected);
}

/// Contract: a mint proof over a credential the KEM issued and the key
///   agreement wrapped verifies, a transfer proof from the seller's fresh
///   decryption and the KEM's rerandomization offset verifies, the buyer's
///   credential opens valid, and the transfer proof fails against another
///   settlement (CR-09; CD-01; CD-02).
/// Boundary: `create_pairing`, `create_hash_to_scalar`, `create_random_source`,
///   `create_encoding` with the ABI concrete, `create_kem` with the Boneh–Boyen
///   concrete, `create_key_agreement` with the pairing ElGamal concrete, and
///   `create_delivery_proof` with the Schnorr concrete in the first-group-only
///   form the pairing declares, each real.
/// Mocked:  nothing but the chain's forms, `MockIChainForms`; the curve
///   libraries, `sha3`, `alloy`, and the operating system's generator are the
///   outer edges.
/// Arrange: `ProofProbe`.
/// Act:     `create_pairing` on `PairingConcrete::Bn254Halo2curves`.
/// Assert:  `mint_verifies`, `transfer_verifies`, `buyer_credential_is_valid`,
///   and `replayed_transfer_rejected` are each `true`.
#[test]
fn a_minted_and_transferred_credential_is_proved_and_verified_through_the_families_on_bn254_halo2curves()
 {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: ProofProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };

    // Assert
    assert!(success.output.mint_verifies);
    assert!(success.output.transfer_verifies);
    assert!(success.output.buyer_credential_is_valid);
    assert!(success.output.replayed_transfer_rejected);
}
