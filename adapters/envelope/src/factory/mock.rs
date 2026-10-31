#![allow(dead_code, clippy::expect_used, clippy::too_many_arguments)]

use super::interface::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementCaseDeps,
    CreateKeyAgreementCaseErrorReturn, CreateKeyAgreementCaseParams, CreateKeyAgreementCasePayload,
    CreateKeyAgreementCaseReturn, CreateKeyAgreementCaseSuccessReturn, CreateKeyAgreementDeps,
    CreateKeyAgreementFn, CreateKeyAgreementParams, CreateKeyAgreementPayload,
    CreateKeyAgreementReturn, CreateKeyAgreementSuccessReturn, EnvelopeAlgebra, EnvelopeCoins,
    EnvelopeComponents, EnvelopeComponentsParams, EnvelopeComponentsPayload,
    EnvelopeComponentsReturn, EnvelopeComponentsSuccessReturn, EnvelopeFromComponentsParams,
    EnvelopeFromComponentsPayload, EnvelopeFromComponentsReturn,
    EnvelopeFromComponentsSuccessReturn, GenerateKeysErrorReturn, GenerateKeysParams,
    GenerateKeysPayload, GenerateKeysReturn, GenerateKeysSuccessReturn,
    ICreateKeyAgreementCaseConsumer, IKeyAgreementAdapter, IKeyAgreementConsumer,
    KEY_AGREEMENT_INTERFACE_VERSION, KeyAgreementConcrete, KeyAgreementDeclaration,
    KeyAgreementIdentifier, KeyPairComponents, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairComponentsReturn, KeyPairComponentsSuccessReturn, KeyPairFromComponentsErrorReturn,
    KeyPairFromComponentsParams, KeyPairFromComponentsPayload, KeyPairFromComponentsReturn,
    KeyPairFromComponentsSuccessReturn, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    KeyPairPublicKeysReturn, KeyPairPublicKeysSuccessReturn, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    PossessionComponents, PossessionComponentsParams, PossessionComponentsPayload,
    PossessionComponentsReturn, PossessionComponentsSuccessReturn, PossessionFromComponentsParams,
    PossessionFromComponentsPayload, PossessionFromComponentsReturn,
    PossessionFromComponentsSuccessReturn, PublicKeysComponents, PublicKeysComponentsParams,
    PublicKeysComponentsPayload, PublicKeysComponentsReturn, PublicKeysComponentsSuccessReturn,
    PublicKeysFromComponentsErrorReturn, PublicKeysFromComponentsParams,
    PublicKeysFromComponentsPayload, PublicKeysFromComponentsReturn,
    PublicKeysFromComponentsSuccessReturn, RunCreateKeyAgreementCaseFn, UnwrapParams,
    UnwrapPayload, UnwrapReturn, UnwrapSuccessReturn, WrapToErrorReturn, WrapToParams,
    WrapToPayload, WrapToReturn, WrapToSuccessReturn,
};
use core::convert::Infallible;
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingParams,
    CreateEncodingParamsOverrides, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarParams, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemDeps, CreateKemParams, CredentialComponents,
    CredentialComponentsOverrides, ICredentialKemAdapter, IKemConsumer, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, build_credential_components, create_kem,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParams,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1Payload, IsIdentityG2Params,
    IsIdentityG2Payload, MsmG1Params, MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload,
    MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, MulScalarParams,
    MulScalarPayload, NegG1Params, NegG1Payload, NegG2Params, NegG2Payload, NegScalarParams,
    NegScalarPayload, PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductTerm,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use random::{
    CreateRandomSourceParams, CreateRandomSourceParamsOverrides, IRandomSourceAdapter,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};
use serde_json::{Map, Value};
use zeroize::Zeroize;

pub const MOCK_POSSESSION_G1_TAG: &[u8] = b"ChainTorrent-v1-envelope-possession-m1";
pub const MOCK_POSSESSION_G2_TAG: &[u8] = b"ChainTorrent-v1-envelope-possession-m2";

pub struct MockKeyPair<P: IPairingAdapter> {
    pub(crate) x: Secret<P::Scalar>,
    pub(crate) y: Secret<P::Scalar>,
    pub(crate) pk1: P::G1,
    pub(crate) pk2: P::G2,
}

pub struct MockPublicKeys<P: IPairingAdapter> {
    pub(crate) pk1: P::G1,
    pub(crate) pk2: P::G2,
}

pub struct MockPossession<P: IPairingAdapter> {
    pub(crate) r1: P::G1,
    pub(crate) z1: P::Scalar,
    pub(crate) r2: P::G2,
    pub(crate) z2: P::Scalar,
}

pub struct MockEnvelope<P: IPairingAdapter> {
    pub(crate) c1: P::G1,
    pub(crate) c2: P::G1,
    pub(crate) d1: P::G2,
    pub(crate) d2: P::G2,
}

pub struct MockIKeyAgreementAdapter<'a, P: IPairingArithmetic> {
    pub(crate) pairing: &'a P,
    pub(crate) failure_mode: MockIKeyAgreementAdapterFailureMode,
}

pub struct MockIKeyAgreementAdapterConstructorParams<'a, P: IPairingArithmetic> {
    pub pairing: &'a P,
    pub failure_mode: MockIKeyAgreementAdapterFailureMode,
}

impl<'a, P: IPairingArithmetic> MockIKeyAgreementAdapter<'a, P> {
    pub fn try_new(
        params: MockIKeyAgreementAdapterConstructorParams<'a, P>,
    ) -> Result<MockIKeyAgreementAdapter<'a, P>, Infallible> {
        Ok(MockIKeyAgreementAdapter {
            pairing: params.pairing,
            failure_mode: params.failure_mode,
        })
    }

    fn challenge_g1(&self, pk: &P::G1, commitment: &P::G1) -> P::Scalar {
        let Ok(encoded_pk) = self
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point: pk.clone() });
        let Ok(encoded_commitment) = self.pairing.encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: commitment.clone(),
            },
        );
        mock_challenge::<P>(
            MOCK_POSSESSION_G1_TAG,
            encoded_pk.bytes.as_ref(),
            encoded_commitment.bytes.as_ref(),
        )
    }

    fn challenge_g2(&self, pk: &P::G2, commitment: &P::G2) -> P::Scalar {
        let Ok(encoded_pk) = self
            .pairing
            .encode_g2(EncodeG2Params, EncodeG2Payload { point: pk.clone() });
        let Ok(encoded_commitment) = self.pairing.encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: commitment.clone(),
            },
        );
        mock_challenge::<P>(
            MOCK_POSSESSION_G2_TAG,
            encoded_pk.bytes.as_ref(),
            encoded_commitment.bytes.as_ref(),
        )
    }

    fn shared_secret_check(&self, pk1: &P::G1, pk2: &P::G2) -> bool {
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(neg_g1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: g1.point.clone(),
            },
        );
        let Ok(check) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: pk1.clone(),
                        g2: g2.point,
                    },
                    PairingProductTerm {
                        g1: neg_g1.negation,
                        g2: pk2.clone(),
                    },
                ],
            },
        );
        check.is_one
    }
}

impl<'a, P: IPairingArithmetic> IKeyAgreementAdapter for MockIKeyAgreementAdapter<'a, P> {
    const DECLARATION: KeyAgreementDeclaration = KeyAgreementDeclaration {
        identifier: KeyAgreementIdentifier::Mock,
        algebra: EnvelopeAlgebra::PairingElGamal,
        possession_g1_tag: MOCK_POSSESSION_G1_TAG,
        possession_g2_tag: MOCK_POSSESSION_G2_TAG,
        adapter_version: 1,
        interface_version: KEY_AGREEMENT_INTERFACE_VERSION,
    };

    type Pairing = P;
    type KeyPair = MockKeyPair<P>;
    type PublicKeys = MockPublicKeys<P>;
    type Possession = MockPossession<P>;
    type Envelope = MockEnvelope<P>;

    fn generate_keys(
        &self,
        _params: GenerateKeysParams,
        payload: GenerateKeysPayload,
    ) -> GenerateKeysReturn<Self::KeyPair, Self::Possession> {
        let x =
            sample_secret::<P>(payload.x_uniform).map_err(GenerateKeysErrorReturn::XSampling)?;
        let y =
            sample_secret::<P>(payload.y_uniform).map_err(GenerateKeysErrorReturn::YSampling)?;
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(pk1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: x.expose().clone(),
            },
        );
        let Ok(pk2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: y.expose().clone(),
            },
        );
        let Ok(pk1_identity) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: pk1.product.clone(),
            },
        );
        if pk1_identity.is_identity {
            return Err(GenerateKeysErrorReturn::IdentityKeyG1);
        }
        let Ok(pk2_identity) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload {
                point: pk2.product.clone(),
            },
        );
        if pk2_identity.is_identity {
            return Err(GenerateKeysErrorReturn::IdentityKeyG2);
        }
        if self.shared_secret_check(&pk1.product, &pk2.product) {
            return Err(GenerateKeysErrorReturn::SharedSecret);
        }
        if self.failure_mode == MockIKeyAgreementAdapterFailureMode::NonceDrawRefused {
            return Err(GenerateKeysErrorReturn::Mock(
                MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
            ));
        }
        let k1 = sampled_scalar_secret::<P>(0x61);
        let k2 = sampled_scalar_secret::<P>(0x62);
        let Ok(r1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: k1.expose().clone(),
            },
        );
        let Ok(r2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: k2.expose().clone(),
            },
        );
        let c1 = self.challenge_g1(&pk1.product, &r1.product);
        let c2 = self.challenge_g2(&pk2.product, &r2.product);
        let Ok(cx) = self.pairing.mul_scalar(
            MulScalarParams,
            MulScalarPayload {
                left: c1,
                right: x.expose().clone(),
            },
        );
        let Ok(z1) = self.pairing.add_scalar(
            AddScalarParams,
            AddScalarPayload {
                left: k1.expose().clone(),
                right: cx.product,
            },
        );
        let Ok(cy) = self.pairing.mul_scalar(
            MulScalarParams,
            MulScalarPayload {
                left: c2,
                right: y.expose().clone(),
            },
        );
        let Ok(z2) = self.pairing.add_scalar(
            AddScalarParams,
            AddScalarPayload {
                left: k2.expose().clone(),
                right: cy.product,
            },
        );
        Ok(GenerateKeysSuccessReturn {
            key_pair: MockKeyPair {
                x,
                y,
                pk1: pk1.product,
                pk2: pk2.product,
            },
            possession: MockPossession {
                r1: r1.product,
                z1: z1.sum,
                r2: r2.product,
                z2: z2.sum,
            },
        })
    }

    fn wrap_to(
        &self,
        _params: WrapToParams,
        payload: WrapToPayload<'_, Self::PublicKeys, P::G1, P::G2>,
    ) -> WrapToReturn<Self::Envelope, P::Scalar> {
        if self.failure_mode == MockIKeyAgreementAdapterFailureMode::CoinDrawRefused {
            return Err(WrapToErrorReturn::Mock(
                MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused,
            ));
        }
        let rho = sampled_scalar_secret::<P>(0x71);
        let sigma = sampled_scalar_secret::<P>(0x72);
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(c1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point,
                scalar: rho.expose().clone(),
            },
        );
        let Ok(masked_a) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: payload.public_keys.pk1.clone(),
                scalar: rho.expose().clone(),
            },
        );
        let Ok(c2) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: payload.credential.a,
                right: masked_a.product,
            },
        );
        let Ok(d1) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point,
                scalar: sigma.expose().clone(),
            },
        );
        let Ok(masked_b) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: payload.public_keys.pk2.clone(),
                scalar: sigma.expose().clone(),
            },
        );
        let Ok(d2) = self.pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: payload.credential.b,
                right: masked_b.product,
            },
        );
        Ok(WrapToSuccessReturn {
            envelope: MockEnvelope {
                c1: c1.product,
                c2: c2.sum,
                d1: d1.product,
                d2: d2.sum,
            },
            coins: EnvelopeCoins { rho, sigma },
        })
    }

    fn unwrap(
        &self,
        _params: UnwrapParams,
        payload: UnwrapPayload<'_, Self::KeyPair, Self::Envelope>,
    ) -> UnwrapReturn<P::G1, P::G2> {
        let key_pair = payload.key_pair;
        let envelope = payload.envelope;
        let Ok(masked_c1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: envelope.c1.clone(),
                scalar: key_pair.x.expose().clone(),
            },
        );
        let Ok(neg_masked_c1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: masked_c1.product,
            },
        );
        let Ok(a) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: envelope.c2.clone(),
                right: neg_masked_c1.negation,
            },
        );
        let Ok(masked_d1) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: envelope.d1.clone(),
                scalar: key_pair.y.expose().clone(),
            },
        );
        let Ok(neg_masked_d1) = self.pairing.neg_g2(
            NegG2Params,
            NegG2Payload {
                point: masked_d1.product,
            },
        );
        let Ok(b) = self.pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: envelope.d2.clone(),
                right: neg_masked_d1.negation,
            },
        );
        Ok(UnwrapSuccessReturn {
            credential: CredentialComponents { a: a.sum, b: b.sum },
        })
    }

    fn key_pair_components(
        &self,
        _params: KeyPairComponentsParams,
        payload: KeyPairComponentsPayload<'_, Self::KeyPair>,
    ) -> KeyPairComponentsReturn<P::Scalar> {
        Ok(KeyPairComponentsSuccessReturn {
            components: KeyPairComponents {
                x: build_secret(
                    payload.key_pair.x.expose().clone(),
                    SecretConstructorParamsOverrides,
                ),
                y: build_secret(
                    payload.key_pair.y.expose().clone(),
                    SecretConstructorParamsOverrides,
                ),
            },
        })
    }

    fn key_pair_public_keys(
        &self,
        _params: KeyPairPublicKeysParams,
        payload: KeyPairPublicKeysPayload<'_, Self::KeyPair>,
    ) -> KeyPairPublicKeysReturn<P::G1, P::G2> {
        Ok(KeyPairPublicKeysSuccessReturn {
            components: PublicKeysComponents {
                pk1: payload.key_pair.pk1.clone(),
                pk2: payload.key_pair.pk2.clone(),
            },
        })
    }

    fn public_keys_components(
        &self,
        _params: PublicKeysComponentsParams,
        payload: PublicKeysComponentsPayload<'_, Self::PublicKeys>,
    ) -> PublicKeysComponentsReturn<P::G1, P::G2> {
        Ok(PublicKeysComponentsSuccessReturn {
            components: PublicKeysComponents {
                pk1: payload.public_keys.pk1.clone(),
                pk2: payload.public_keys.pk2.clone(),
            },
        })
    }

    fn possession_components(
        &self,
        _params: PossessionComponentsParams,
        payload: PossessionComponentsPayload<'_, Self::Possession>,
    ) -> PossessionComponentsReturn<P::Scalar, P::G1, P::G2> {
        Ok(PossessionComponentsSuccessReturn {
            components: PossessionComponents {
                r1: payload.possession.r1.clone(),
                z1: payload.possession.z1.clone(),
                r2: payload.possession.r2.clone(),
                z2: payload.possession.z2.clone(),
            },
        })
    }

    fn envelope_components(
        &self,
        _params: EnvelopeComponentsParams,
        payload: EnvelopeComponentsPayload<'_, Self::Envelope>,
    ) -> EnvelopeComponentsReturn<P::G1, P::G2> {
        Ok(EnvelopeComponentsSuccessReturn {
            components: EnvelopeComponents {
                c1: payload.envelope.c1.clone(),
                c2: payload.envelope.c2.clone(),
                d1: payload.envelope.d1.clone(),
                d2: payload.envelope.d2.clone(),
            },
        })
    }

    fn key_pair_from_components(
        &self,
        _params: KeyPairFromComponentsParams,
        payload: KeyPairFromComponentsPayload<P::Scalar>,
    ) -> KeyPairFromComponentsReturn<Self::KeyPair> {
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(pk1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point,
                scalar: payload.components.x.expose().clone(),
            },
        );
        let Ok(pk2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point,
                scalar: payload.components.y.expose().clone(),
            },
        );
        let Ok(pk1_identity) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: pk1.product.clone(),
            },
        );
        if pk1_identity.is_identity {
            return Err(KeyPairFromComponentsErrorReturn::IdentityKeyG1);
        }
        let Ok(pk2_identity) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload {
                point: pk2.product.clone(),
            },
        );
        if pk2_identity.is_identity {
            return Err(KeyPairFromComponentsErrorReturn::IdentityKeyG2);
        }
        if self.shared_secret_check(&pk1.product, &pk2.product) {
            return Err(KeyPairFromComponentsErrorReturn::SharedSecret);
        }
        Ok(KeyPairFromComponentsSuccessReturn {
            key_pair: MockKeyPair {
                x: payload.components.x,
                y: payload.components.y,
                pk1: pk1.product,
                pk2: pk2.product,
            },
        })
    }

    fn public_keys_from_components(
        &self,
        _params: PublicKeysFromComponentsParams,
        payload: PublicKeysFromComponentsPayload<'_, P::G1, P::G2, Self::Possession>,
    ) -> PublicKeysFromComponentsReturn<Self::PublicKeys> {
        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(pk1_identity) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: payload.components.pk1.clone(),
            },
        );
        if pk1_identity.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG1);
        }
        let Ok(pk2_identity) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload {
                point: payload.components.pk2.clone(),
            },
        );
        if pk2_identity.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::IdentityKeyG2);
        }
        if self.shared_secret_check(&payload.components.pk1, &payload.components.pk2) {
            return Err(PublicKeysFromComponentsErrorReturn::SharedSecret);
        }
        let c1 = self.challenge_g1(&payload.components.pk1, &payload.possession.r1);
        let Ok(neg_c1) = self
            .pairing
            .neg_scalar(NegScalarParams, NegScalarPayload { scalar: c1 });
        let Ok(g1_term) = self.pairing.msm_g1(
            MsmG1Params,
            MsmG1Payload {
                terms: vec![
                    MsmG1Term {
                        base: g1.point.clone(),
                        scalar: payload.possession.z1.clone(),
                    },
                    MsmG1Term {
                        base: payload.components.pk1.clone(),
                        scalar: neg_c1.negation,
                    },
                ],
            },
        );
        let Ok(neg_r1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: payload.possession.r1.clone(),
            },
        );
        let Ok(g1_check_point) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: g1_term.sum,
                right: neg_r1.negation,
            },
        );
        let Ok(g1_check) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: g1_check_point.sum,
            },
        );
        if !g1_check.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::PossessionG1);
        }
        let c2 = self.challenge_g2(&payload.components.pk2, &payload.possession.r2);
        let Ok(neg_c2) = self
            .pairing
            .neg_scalar(NegScalarParams, NegScalarPayload { scalar: c2 });
        let Ok(g2_term) = self.pairing.msm_g2(
            MsmG2Params,
            MsmG2Payload {
                terms: vec![
                    MsmG2Term {
                        base: g2.point.clone(),
                        scalar: payload.possession.z2.clone(),
                    },
                    MsmG2Term {
                        base: payload.components.pk2.clone(),
                        scalar: neg_c2.negation,
                    },
                ],
            },
        );
        let Ok(neg_r2) = self.pairing.neg_g2(
            NegG2Params,
            NegG2Payload {
                point: payload.possession.r2.clone(),
            },
        );
        let Ok(g2_check_point) = self.pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: g2_term.sum,
                right: neg_r2.negation,
            },
        );
        let Ok(g2_check) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload {
                point: g2_check_point.sum,
            },
        );
        if !g2_check.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::PossessionG2);
        }
        Ok(PublicKeysFromComponentsSuccessReturn {
            public_keys: MockPublicKeys {
                pk1: payload.components.pk1,
                pk2: payload.components.pk2,
            },
        })
    }

    fn possession_from_components(
        &self,
        _params: PossessionFromComponentsParams,
        payload: PossessionFromComponentsPayload<P::Scalar, P::G1, P::G2>,
    ) -> PossessionFromComponentsReturn<Self::Possession> {
        Ok(PossessionFromComponentsSuccessReturn {
            possession: MockPossession {
                r1: payload.components.r1,
                z1: payload.components.z1,
                r2: payload.components.r2,
                z2: payload.components.z2,
            },
        })
    }

    fn envelope_from_components(
        &self,
        _params: EnvelopeFromComponentsParams,
        payload: EnvelopeFromComponentsPayload<P::G1, P::G2>,
    ) -> EnvelopeFromComponentsReturn<Self::Envelope> {
        Ok(EnvelopeFromComponentsSuccessReturn {
            envelope: MockEnvelope {
                c1: payload.components.c1,
                c2: payload.components.c2,
                d1: payload.components.d1,
                d2: payload.components.d2,
            },
        })
    }
}

fn sample_secret<P: IPairingArithmetic>(
    uniform: Secret<Vec<u8>>,
) -> Result<Secret<P::Scalar>, SampleUniformScalarErrorReturn> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload { uniform },
    )
    .map(|sampled| sampled.scalar)
}

fn sampled_scalar_secret<P: IPairingAdapter>(fill: u8) -> Secret<P::Scalar> {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(
                vec![fill; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            ),
        },
    )
    .expect("sampling a default scalar always succeeds")
    .scalar
}

fn sampled_scalar<P: IPairingAdapter>(fill: u8) -> P::Scalar {
    sampled_scalar_secret::<P>(fill).expose().clone()
}

fn mock_challenge<P: IPairingArithmetic>(
    tag: &[u8],
    encoded_key: &[u8],
    encoded_commitment: &[u8],
) -> P::Scalar {
    let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
    let mut uniform = vec![0u8; length];
    for (index, byte) in tag
        .iter()
        .chain(encoded_key.iter())
        .chain(encoded_commitment.iter())
        .enumerate()
    {
        uniform[index % length] ^= *byte;
    }
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: build_secret(uniform, SecretConstructorParamsOverrides),
        },
    )
    .expect("sampling a default scalar always succeeds")
    .scalar
    .expose()
    .clone()
}

fn default_g1<P: IPairingAdapter>(pairing: &P) -> P::G1 {
    let Ok(success) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    success.point
}

fn default_g2<P: IPairingAdapter>(pairing: &P) -> P::G2 {
    let Ok(success) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    success.point
}

fn g1_mul<P: IPairingAdapter>(pairing: &P, fill: u8) -> P::G1 {
    let Ok(success) = pairing.mul_g1(
        MulG1Params,
        MulG1Payload {
            point: default_g1(pairing),
            scalar: sampled_scalar::<P>(fill),
        },
    );
    success.product
}

fn g2_mul<P: IPairingAdapter>(pairing: &P, fill: u8) -> P::G2 {
    let Ok(success) = pairing.mul_g2(
        MulG2Params,
        MulG2Payload {
            point: default_g2(pairing),
            scalar: sampled_scalar::<P>(fill),
        },
    );
    success.product
}

fn byte_array(bytes: &[u8]) -> Value {
    Value::Array(bytes.iter().map(|byte| Value::from(*byte)).collect())
}

fn encoded_g1_value<P: IPairingAdapter>(pairing: &P, point: &P::G1) -> Value {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    byte_array(encoded.bytes.as_ref())
}

fn encoded_g2_value<P: IPairingAdapter>(pairing: &P, point: &P::G2) -> Value {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    byte_array(encoded.bytes.as_ref())
}

fn encoded_scalar_value<P: IPairingAdapter>(pairing: &P, scalar: &P::Scalar) -> Value {
    let Ok(encoded) = pairing.encode_scalar(
        EncodeScalarParams,
        EncodeScalarPayload {
            scalar: scalar.clone(),
        },
    );
    byte_array(encoded.bytes.expose().as_ref())
}

fn key_pair_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &KeyPairComponents<P::Scalar>,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "x".to_string(),
        encoded_scalar_value(pairing, components.x.expose()),
    );
    fields.insert(
        "y".to_string(),
        encoded_scalar_value(pairing, components.y.expose()),
    );
    Value::Object(fields)
}

fn public_keys_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &PublicKeysComponents<P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert(
        "pk1".to_string(),
        encoded_g1_value(pairing, &components.pk1),
    );
    fields.insert(
        "pk2".to_string(),
        encoded_g2_value(pairing, &components.pk2),
    );
    Value::Object(fields)
}

fn possession_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &PossessionComponents<P::Scalar, P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert("r1".to_string(), encoded_g1_value(pairing, &components.r1));
    fields.insert(
        "z1".to_string(),
        encoded_scalar_value(pairing, &components.z1),
    );
    fields.insert("r2".to_string(), encoded_g2_value(pairing, &components.r2));
    fields.insert(
        "z2".to_string(),
        encoded_scalar_value(pairing, &components.z2),
    );
    Value::Object(fields)
}

fn envelope_components_value<P: IPairingAdapter>(
    pairing: &P,
    components: &EnvelopeComponents<P::G1, P::G2>,
) -> Value {
    let mut fields = Map::new();
    fields.insert("c1".to_string(), encoded_g1_value(pairing, &components.c1));
    fields.insert("c2".to_string(), encoded_g1_value(pairing, &components.c2));
    fields.insert("d1".to_string(), encoded_g2_value(pairing, &components.d1));
    fields.insert("d2".to_string(), encoded_g2_value(pairing, &components.d2));
    Value::Object(fields)
}

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
        identifier: overrides.identifier.unwrap_or(KeyAgreementIdentifier::Mock),
        algebra: overrides.algebra.unwrap_or(EnvelopeAlgebra::PairingElGamal),
        possession_g1_tag: overrides
            .possession_g1_tag
            .unwrap_or(MOCK_POSSESSION_G1_TAG),
        possession_g2_tag: overrides
            .possession_g2_tag
            .unwrap_or(MOCK_POSSESSION_G2_TAG),
        adapter_version: overrides.adapter_version.unwrap_or(1),
        interface_version: overrides
            .interface_version
            .unwrap_or(KEY_AGREEMENT_INTERFACE_VERSION),
    }
}

#[derive(Default)]
pub struct KeyAgreementDeclarationCorruptions {
    pub identifier: Option<Value>,
    pub algebra: Option<Value>,
    pub possession_g1_tag: Option<Value>,
    pub possession_g2_tag: Option<Value>,
    pub adapter_version: Option<Value>,
    pub interface_version: Option<Value>,
}

pub fn invalidate_key_agreement_declaration(
    corruptions: KeyAgreementDeclarationCorruptions,
) -> Value {
    let declaration = build_key_agreement_declaration(Default::default());
    let mut fields = Map::new();
    fields.insert(
        "identifier".to_string(),
        Value::from(match declaration.identifier {
            KeyAgreementIdentifier::PairingElGamalV1 => "PairingElGamalV1",
            KeyAgreementIdentifier::Mock => "Mock",
        }),
    );
    fields.insert(
        "algebra".to_string(),
        Value::from(match declaration.algebra {
            EnvelopeAlgebra::PairingElGamal => "PairingElGamal",
        }),
    );
    fields.insert(
        "possession_g1_tag".to_string(),
        byte_array(declaration.possession_g1_tag),
    );
    fields.insert(
        "possession_g2_tag".to_string(),
        byte_array(declaration.possession_g2_tag),
    );
    fields.insert(
        "adapter_version".to_string(),
        Value::from(declaration.adapter_version),
    );
    fields.insert(
        "interface_version".to_string(),
        Value::from(declaration.interface_version),
    );
    if let Some(identifier) = corruptions.identifier {
        fields.insert("identifier".to_string(), identifier);
    }
    if let Some(algebra) = corruptions.algebra {
        fields.insert("algebra".to_string(), algebra);
    }
    if let Some(possession_g1_tag) = corruptions.possession_g1_tag {
        fields.insert("possession_g1_tag".to_string(), possession_g1_tag);
    }
    if let Some(possession_g2_tag) = corruptions.possession_g2_tag {
        fields.insert("possession_g2_tag".to_string(), possession_g2_tag);
    }
    if let Some(adapter_version) = corruptions.adapter_version {
        fields.insert("adapter_version".to_string(), adapter_version);
    }
    if let Some(interface_version) = corruptions.interface_version {
        fields.insert("interface_version".to_string(), interface_version);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct GenerateKeysParamsOverrides;

pub fn build_generate_keys_params(_overrides: GenerateKeysParamsOverrides) -> GenerateKeysParams {
    GenerateKeysParams
}

#[derive(Default)]
pub struct GenerateKeysPayloadOverrides {
    pub x_uniform: Option<Secret<Vec<u8>>>,
    pub y_uniform: Option<Secret<Vec<u8>>>,
}

pub fn build_generate_keys_payload<P: IPairingAdapter>(
    _pairing: &P,
    overrides: GenerateKeysPayloadOverrides,
) -> GenerateKeysPayload {
    GenerateKeysPayload {
        x_uniform: overrides.x_uniform.unwrap_or_else(|| {
            build_secret(
                vec![0x0Du8; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            )
        }),
        y_uniform: overrides.y_uniform.unwrap_or_else(|| {
            build_secret(
                vec![0x0Eu8; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH],
                SecretConstructorParamsOverrides,
            )
        }),
    }
}

#[derive(Default)]
pub struct GenerateKeysPayloadCorruptions {
    pub x_uniform: Option<Value>,
    pub y_uniform: Option<Value>,
}

pub fn invalidate_generate_keys_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: GenerateKeysPayloadCorruptions,
) -> Value {
    let payload = build_generate_keys_payload::<P>(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "x_uniform".to_string(),
        byte_array(payload.x_uniform.expose()),
    );
    fields.insert(
        "y_uniform".to_string(),
        byte_array(payload.y_uniform.expose()),
    );
    if let Some(x_uniform) = corruptions.x_uniform {
        fields.insert("x_uniform".to_string(), x_uniform);
    }
    if let Some(y_uniform) = corruptions.y_uniform {
        fields.insert("y_uniform".to_string(), y_uniform);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct GenerateKeysSuccessReturnOverrides;

pub fn build_generate_keys_success_return<KP, PO>(
    key_pair: KP,
    possession: PO,
    _overrides: GenerateKeysSuccessReturnOverrides,
) -> GenerateKeysSuccessReturn<KP, PO> {
    GenerateKeysSuccessReturn {
        key_pair,
        possession,
    }
}

#[derive(Default)]
pub struct WrapToParamsOverrides;

pub fn build_wrap_to_params(_overrides: WrapToParamsOverrides) -> WrapToParams {
    WrapToParams
}

pub struct WrapToPayloadOverrides<G1, G2> {
    pub credential: Option<CredentialComponents<G1, G2>>,
}

impl<G1, G2> Default for WrapToPayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { credential: None }
    }
}

pub fn build_wrap_to_payload<'a, P: IPairingAdapter, PK>(
    pairing: &P,
    public_keys: &'a PK,
    overrides: WrapToPayloadOverrides<P::G1, P::G2>,
) -> WrapToPayload<'a, PK, P::G1, P::G2> {
    WrapToPayload {
        public_keys,
        credential: overrides.credential.unwrap_or_else(|| {
            build_credential_components(pairing, CredentialComponentsOverrides::default())
        }),
    }
}

pub struct WrapToSuccessReturnOverrides<S: Zeroize> {
    pub coins: Option<EnvelopeCoins<S>>,
}

impl<S: Zeroize> Default for WrapToSuccessReturnOverrides<S> {
    fn default() -> Self {
        Self { coins: None }
    }
}

pub fn build_wrap_to_success_return<P: IPairingAdapter, E>(
    pairing: &P,
    envelope: E,
    overrides: WrapToSuccessReturnOverrides<P::Scalar>,
) -> WrapToSuccessReturn<E, P::Scalar> {
    WrapToSuccessReturn {
        envelope,
        coins: overrides
            .coins
            .unwrap_or_else(|| build_envelope_coins(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct UnwrapParamsOverrides;

pub fn build_unwrap_params(_overrides: UnwrapParamsOverrides) -> UnwrapParams {
    UnwrapParams
}

#[derive(Default)]
pub struct UnwrapPayloadOverrides;

pub fn build_unwrap_payload<'a, KP, E>(
    key_pair: &'a KP,
    envelope: &'a E,
    _overrides: UnwrapPayloadOverrides,
) -> UnwrapPayload<'a, KP, E> {
    UnwrapPayload { key_pair, envelope }
}

pub struct UnwrapSuccessReturnOverrides<G1, G2> {
    pub credential: Option<CredentialComponents<G1, G2>>,
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
        credential: overrides.credential.unwrap_or_else(|| {
            build_credential_components(pairing, CredentialComponentsOverrides::default())
        }),
    }
}

#[derive(Default)]
pub struct UnwrapSuccessReturnCorruptions {
    pub credential: Option<Value>,
}

pub fn invalidate_unwrap_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: UnwrapSuccessReturnCorruptions,
) -> Value {
    let success = build_unwrap_success_return(pairing, Default::default());
    let mut fields = Map::new();
    let Ok(encoded_a) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: success.credential.a.clone(),
        },
    );
    let Ok(encoded_b) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: success.credential.b.clone(),
        },
    );
    let mut credential = Map::new();
    credential.insert("a".to_string(), byte_array(encoded_a.bytes.as_ref()));
    credential.insert("b".to_string(), byte_array(encoded_b.bytes.as_ref()));
    fields.insert("credential".to_string(), Value::Object(credential));
    if let Some(value) = corruptions.credential {
        fields.insert("credential".to_string(), value);
    }
    Value::Object(fields)
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
            .unwrap_or_else(|| sampled_scalar_secret::<P>(0x01)),
        y: overrides
            .y
            .unwrap_or_else(|| sampled_scalar_secret::<P>(0x02)),
    }
}

#[derive(Default)]
pub struct KeyPairComponentsCorruptions {
    pub x: Option<Value>,
    pub y: Option<Value>,
}

pub fn invalidate_key_pair_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: KeyPairComponentsCorruptions,
) -> Value {
    let components = build_key_pair_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "x".to_string(),
        encoded_scalar_value(pairing, components.x.expose()),
    );
    fields.insert(
        "y".to_string(),
        encoded_scalar_value(pairing, components.y.expose()),
    );
    if let Some(x) = corruptions.x {
        fields.insert("x".to_string(), x);
    }
    if let Some(y) = corruptions.y {
        fields.insert("y".to_string(), y);
    }
    Value::Object(fields)
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
        pk1: overrides.pk1.unwrap_or_else(|| g1_mul(pairing, 0x01)),
        pk2: overrides.pk2.unwrap_or_else(|| g2_mul(pairing, 0x02)),
    }
}

#[derive(Default)]
pub struct PublicKeysComponentsCorruptions {
    pub pk1: Option<Value>,
    pub pk2: Option<Value>,
}

pub fn invalidate_public_keys_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: PublicKeysComponentsCorruptions,
) -> Value {
    let components = build_public_keys_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "pk1".to_string(),
        encoded_g1_value(pairing, &components.pk1),
    );
    fields.insert(
        "pk2".to_string(),
        encoded_g2_value(pairing, &components.pk2),
    );
    if let Some(pk1) = corruptions.pk1 {
        fields.insert("pk1".to_string(), pk1);
    }
    if let Some(pk2) = corruptions.pk2 {
        fields.insert("pk2".to_string(), pk2);
    }
    Value::Object(fields)
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
        r1: overrides.r1.unwrap_or_else(|| g1_mul(pairing, 0x03)),
        z1: overrides.z1.unwrap_or_else(|| sampled_scalar::<P>(0x04)),
        r2: overrides.r2.unwrap_or_else(|| g2_mul(pairing, 0x05)),
        z2: overrides.z2.unwrap_or_else(|| sampled_scalar::<P>(0x06)),
    }
}

#[derive(Default)]
pub struct PossessionComponentsCorruptions {
    pub r1: Option<Value>,
    pub z1: Option<Value>,
    pub r2: Option<Value>,
    pub z2: Option<Value>,
}

pub fn invalidate_possession_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: PossessionComponentsCorruptions,
) -> Value {
    let components = build_possession_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("r1".to_string(), encoded_g1_value(pairing, &components.r1));
    fields.insert(
        "z1".to_string(),
        encoded_scalar_value(pairing, &components.z1),
    );
    fields.insert("r2".to_string(), encoded_g2_value(pairing, &components.r2));
    fields.insert(
        "z2".to_string(),
        encoded_scalar_value(pairing, &components.z2),
    );
    if let Some(r1) = corruptions.r1 {
        fields.insert("r1".to_string(), r1);
    }
    if let Some(z1) = corruptions.z1 {
        fields.insert("z1".to_string(), z1);
    }
    if let Some(r2) = corruptions.r2 {
        fields.insert("r2".to_string(), r2);
    }
    if let Some(z2) = corruptions.z2 {
        fields.insert("z2".to_string(), z2);
    }
    Value::Object(fields)
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
        c1: overrides.c1.unwrap_or_else(|| g1_mul(pairing, 0x07)),
        c2: overrides.c2.unwrap_or_else(|| g1_mul(pairing, 0x08)),
        d1: overrides.d1.unwrap_or_else(|| g2_mul(pairing, 0x09)),
        d2: overrides.d2.unwrap_or_else(|| g2_mul(pairing, 0x0A)),
    }
}

#[derive(Default)]
pub struct EnvelopeComponentsCorruptions {
    pub c1: Option<Value>,
    pub c2: Option<Value>,
    pub d1: Option<Value>,
    pub d2: Option<Value>,
}

pub fn invalidate_envelope_components<P: IPairingAdapter>(
    pairing: &P,
    corruptions: EnvelopeComponentsCorruptions,
) -> Value {
    let components = build_envelope_components(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("c1".to_string(), encoded_g1_value(pairing, &components.c1));
    fields.insert("c2".to_string(), encoded_g1_value(pairing, &components.c2));
    fields.insert("d1".to_string(), encoded_g2_value(pairing, &components.d1));
    fields.insert("d2".to_string(), encoded_g2_value(pairing, &components.d2));
    if let Some(c1) = corruptions.c1 {
        fields.insert("c1".to_string(), c1);
    }
    if let Some(c2) = corruptions.c2 {
        fields.insert("c2".to_string(), c2);
    }
    if let Some(d1) = corruptions.d1 {
        fields.insert("d1".to_string(), d1);
    }
    if let Some(d2) = corruptions.d2 {
        fields.insert("d2".to_string(), d2);
    }
    Value::Object(fields)
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

pub fn build_envelope_coins<P: IPairingAdapter>(
    _pairing: &P,
    overrides: EnvelopeCoinsOverrides<P::Scalar>,
) -> EnvelopeCoins<P::Scalar> {
    EnvelopeCoins {
        rho: overrides
            .rho
            .unwrap_or_else(|| sampled_scalar_secret::<P>(0x0B)),
        sigma: overrides
            .sigma
            .unwrap_or_else(|| sampled_scalar_secret::<P>(0x0C)),
    }
}

#[derive(Default)]
pub struct EnvelopeCoinsCorruptions {
    pub rho: Option<Value>,
    pub sigma: Option<Value>,
}

pub fn invalidate_envelope_coins<P: IPairingAdapter>(
    pairing: &P,
    corruptions: EnvelopeCoinsCorruptions,
) -> Value {
    let coins = build_envelope_coins(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "rho".to_string(),
        encoded_scalar_value(pairing, coins.rho.expose()),
    );
    fields.insert(
        "sigma".to_string(),
        encoded_scalar_value(pairing, coins.sigma.expose()),
    );
    if let Some(rho) = corruptions.rho {
        fields.insert("rho".to_string(), rho);
    }
    if let Some(sigma) = corruptions.sigma {
        fields.insert("sigma".to_string(), sigma);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct KeyPairComponentsParamsOverrides;

pub fn build_key_pair_components_params(
    _overrides: KeyPairComponentsParamsOverrides,
) -> KeyPairComponentsParams {
    KeyPairComponentsParams
}

#[derive(Default)]
pub struct KeyPairComponentsPayloadOverrides;

pub fn build_key_pair_components_payload<'a, KP>(
    key_pair: &'a KP,
    _overrides: KeyPairComponentsPayloadOverrides,
) -> KeyPairComponentsPayload<'a, KP> {
    KeyPairComponentsPayload { key_pair }
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

#[derive(Default)]
pub struct KeyPairComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_key_pair_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: KeyPairComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_key_pair_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        key_pair_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct KeyPairPublicKeysParamsOverrides;

pub fn build_key_pair_public_keys_params(
    _overrides: KeyPairPublicKeysParamsOverrides,
) -> KeyPairPublicKeysParams {
    KeyPairPublicKeysParams
}

#[derive(Default)]
pub struct KeyPairPublicKeysPayloadOverrides;

pub fn build_key_pair_public_keys_payload<'a, KP>(
    key_pair: &'a KP,
    _overrides: KeyPairPublicKeysPayloadOverrides,
) -> KeyPairPublicKeysPayload<'a, KP> {
    KeyPairPublicKeysPayload { key_pair }
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

#[derive(Default)]
pub struct KeyPairPublicKeysSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_key_pair_public_keys_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: KeyPairPublicKeysSuccessReturnCorruptions,
) -> Value {
    let success = build_key_pair_public_keys_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        public_keys_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct PublicKeysComponentsParamsOverrides;

pub fn build_public_keys_components_params(
    _overrides: PublicKeysComponentsParamsOverrides,
) -> PublicKeysComponentsParams {
    PublicKeysComponentsParams
}

#[derive(Default)]
pub struct PublicKeysComponentsPayloadOverrides;

pub fn build_public_keys_components_payload<'a, PK>(
    public_keys: &'a PK,
    _overrides: PublicKeysComponentsPayloadOverrides,
) -> PublicKeysComponentsPayload<'a, PK> {
    PublicKeysComponentsPayload { public_keys }
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

#[derive(Default)]
pub struct PublicKeysComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_public_keys_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: PublicKeysComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_public_keys_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        public_keys_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct PossessionComponentsParamsOverrides;

pub fn build_possession_components_params(
    _overrides: PossessionComponentsParamsOverrides,
) -> PossessionComponentsParams {
    PossessionComponentsParams
}

#[derive(Default)]
pub struct PossessionComponentsPayloadOverrides;

pub fn build_possession_components_payload<'a, PO>(
    possession: &'a PO,
    _overrides: PossessionComponentsPayloadOverrides,
) -> PossessionComponentsPayload<'a, PO> {
    PossessionComponentsPayload { possession }
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

#[derive(Default)]
pub struct PossessionComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_possession_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: PossessionComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_possession_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        possession_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EnvelopeComponentsParamsOverrides;

pub fn build_envelope_components_params(
    _overrides: EnvelopeComponentsParamsOverrides,
) -> EnvelopeComponentsParams {
    EnvelopeComponentsParams
}

#[derive(Default)]
pub struct EnvelopeComponentsPayloadOverrides;

pub fn build_envelope_components_payload<'a, E>(
    envelope: &'a E,
    _overrides: EnvelopeComponentsPayloadOverrides,
) -> EnvelopeComponentsPayload<'a, E> {
    EnvelopeComponentsPayload { envelope }
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
pub struct EnvelopeComponentsSuccessReturnCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_envelope_components_success_return<P: IPairingAdapter>(
    pairing: &P,
    corruptions: EnvelopeComponentsSuccessReturnCorruptions,
) -> Value {
    let success = build_envelope_components_success_return(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        envelope_components_value(pairing, &success.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct KeyPairFromComponentsParamsOverrides;

pub fn build_key_pair_from_components_params(
    _overrides: KeyPairFromComponentsParamsOverrides,
) -> KeyPairFromComponentsParams {
    KeyPairFromComponentsParams
}

pub struct KeyPairFromComponentsPayloadOverrides<S: Zeroize> {
    pub components: Option<KeyPairComponents<S>>,
}

impl<S: Zeroize> Default for KeyPairFromComponentsPayloadOverrides<S> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_key_pair_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: KeyPairFromComponentsPayloadOverrides<P::Scalar>,
) -> KeyPairFromComponentsPayload<P::Scalar> {
    KeyPairFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| build_key_pair_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct KeyPairFromComponentsPayloadCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_key_pair_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: KeyPairFromComponentsPayloadCorruptions,
) -> Value {
    let payload = build_key_pair_from_components_payload(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        key_pair_components_value(pairing, &payload.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct KeyPairFromComponentsSuccessReturnOverrides;

pub fn build_key_pair_from_components_success_return<KP>(
    key_pair: KP,
    _overrides: KeyPairFromComponentsSuccessReturnOverrides,
) -> KeyPairFromComponentsSuccessReturn<KP> {
    KeyPairFromComponentsSuccessReturn { key_pair }
}

#[derive(Default)]
pub struct PublicKeysFromComponentsParamsOverrides;

pub fn build_public_keys_from_components_params(
    _overrides: PublicKeysFromComponentsParamsOverrides,
) -> PublicKeysFromComponentsParams {
    PublicKeysFromComponentsParams
}

pub struct PublicKeysFromComponentsPayloadOverrides<G1, G2> {
    pub components: Option<PublicKeysComponents<G1, G2>>,
}

impl<G1, G2> Default for PublicKeysFromComponentsPayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_public_keys_from_components_payload<'a, P: IPairingAdapter, PO>(
    pairing: &P,
    possession: &'a PO,
    overrides: PublicKeysFromComponentsPayloadOverrides<P::G1, P::G2>,
) -> PublicKeysFromComponentsPayload<'a, P::G1, P::G2, PO> {
    PublicKeysFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| build_public_keys_components(pairing, Default::default())),
        possession,
    }
}

#[derive(Default)]
pub struct PublicKeysFromComponentsSuccessReturnOverrides;

pub fn build_public_keys_from_components_success_return<PK>(
    public_keys: PK,
    _overrides: PublicKeysFromComponentsSuccessReturnOverrides,
) -> PublicKeysFromComponentsSuccessReturn<PK> {
    PublicKeysFromComponentsSuccessReturn { public_keys }
}

#[derive(Default)]
pub struct PossessionFromComponentsParamsOverrides;

pub fn build_possession_from_components_params(
    _overrides: PossessionFromComponentsParamsOverrides,
) -> PossessionFromComponentsParams {
    PossessionFromComponentsParams
}

pub struct PossessionFromComponentsPayloadOverrides<S, G1, G2> {
    pub components: Option<PossessionComponents<S, G1, G2>>,
}

impl<S, G1, G2> Default for PossessionFromComponentsPayloadOverrides<S, G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_possession_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: PossessionFromComponentsPayloadOverrides<P::Scalar, P::G1, P::G2>,
) -> PossessionFromComponentsPayload<P::Scalar, P::G1, P::G2> {
    PossessionFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| build_possession_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct PossessionFromComponentsPayloadCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_possession_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: PossessionFromComponentsPayloadCorruptions,
) -> Value {
    let payload = build_possession_from_components_payload(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        possession_components_value(pairing, &payload.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct PossessionFromComponentsSuccessReturnOverrides;

pub fn build_possession_from_components_success_return<PO>(
    possession: PO,
    _overrides: PossessionFromComponentsSuccessReturnOverrides,
) -> PossessionFromComponentsSuccessReturn<PO> {
    PossessionFromComponentsSuccessReturn { possession }
}

#[derive(Default)]
pub struct EnvelopeFromComponentsParamsOverrides;

pub fn build_envelope_from_components_params(
    _overrides: EnvelopeFromComponentsParamsOverrides,
) -> EnvelopeFromComponentsParams {
    EnvelopeFromComponentsParams
}

pub struct EnvelopeFromComponentsPayloadOverrides<G1, G2> {
    pub components: Option<EnvelopeComponents<G1, G2>>,
}

impl<G1, G2> Default for EnvelopeFromComponentsPayloadOverrides<G1, G2> {
    fn default() -> Self {
        Self { components: None }
    }
}

pub fn build_envelope_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    overrides: EnvelopeFromComponentsPayloadOverrides<P::G1, P::G2>,
) -> EnvelopeFromComponentsPayload<P::G1, P::G2> {
    EnvelopeFromComponentsPayload {
        components: overrides
            .components
            .unwrap_or_else(|| build_envelope_components(pairing, Default::default())),
    }
}

#[derive(Default)]
pub struct EnvelopeFromComponentsPayloadCorruptions {
    pub components: Option<Value>,
}

pub fn invalidate_envelope_from_components_payload<P: IPairingAdapter>(
    pairing: &P,
    corruptions: EnvelopeFromComponentsPayloadCorruptions,
) -> Value {
    let payload = build_envelope_from_components_payload(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "components".to_string(),
        envelope_components_value(pairing, &payload.components),
    );
    if let Some(components) = corruptions.components {
        fields.insert("components".to_string(), components);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct EnvelopeFromComponentsSuccessReturnOverrides;

pub fn build_envelope_from_components_success_return<E>(
    envelope: E,
    _overrides: EnvelopeFromComponentsSuccessReturnOverrides,
) -> EnvelopeFromComponentsSuccessReturn<E> {
    EnvelopeFromComponentsSuccessReturn { envelope }
}

#[derive(Default)]
pub struct MockIKeyAgreementAdapterConstructorParamsOverrides {
    pub failure_mode: Option<MockIKeyAgreementAdapterFailureMode>,
}

pub fn build_mock_i_key_agreement_adapter_constructor_params<'a, P: IPairingArithmetic>(
    pairing: &'a P,
    overrides: MockIKeyAgreementAdapterConstructorParamsOverrides,
) -> MockIKeyAgreementAdapterConstructorParams<'a, P> {
    MockIKeyAgreementAdapterConstructorParams {
        pairing,
        failure_mode: overrides
            .failure_mode
            .unwrap_or(MockIKeyAgreementAdapterFailureMode::NoFailure),
    }
}

pub fn build_mock_i_key_agreement_adapter<'a, P: IPairingArithmetic>(
    pairing: &'a P,
    overrides: MockIKeyAgreementAdapterConstructorParamsOverrides,
) -> MockIKeyAgreementAdapter<'a, P> {
    let Ok(adapter) = MockIKeyAgreementAdapter::try_new(
        build_mock_i_key_agreement_adapter_constructor_params(pairing, overrides),
    );
    adapter
}

pub struct MockKeyPairOverrides<P: IPairingAdapter> {
    pub x: Option<Secret<P::Scalar>>,
    pub y: Option<Secret<P::Scalar>>,
    pub pk1: Option<P::G1>,
    pub pk2: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for MockKeyPairOverrides<P> {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            pk1: None,
            pk2: None,
        }
    }
}

pub fn build_mock_key_pair<P: IPairingAdapter>(
    pairing: &P,
    overrides: MockKeyPairOverrides<P>,
) -> MockKeyPair<P> {
    let components = build_key_pair_components(pairing, Default::default());
    let public_keys = build_public_keys_components(pairing, Default::default());
    MockKeyPair {
        x: overrides.x.unwrap_or(components.x),
        y: overrides.y.unwrap_or(components.y),
        pk1: overrides.pk1.unwrap_or(public_keys.pk1),
        pk2: overrides.pk2.unwrap_or(public_keys.pk2),
    }
}

#[derive(Default)]
pub struct MockKeyPairCorruptions {
    pub x: Option<Value>,
    pub y: Option<Value>,
    pub pk1: Option<Value>,
    pub pk2: Option<Value>,
}

pub fn invalidate_mock_key_pair<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MockKeyPairCorruptions,
) -> Value {
    let key_pair = build_mock_key_pair(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "x".to_string(),
        encoded_scalar_value(pairing, key_pair.x.expose()),
    );
    fields.insert(
        "y".to_string(),
        encoded_scalar_value(pairing, key_pair.y.expose()),
    );
    fields.insert("pk1".to_string(), encoded_g1_value(pairing, &key_pair.pk1));
    fields.insert("pk2".to_string(), encoded_g2_value(pairing, &key_pair.pk2));
    if let Some(x) = corruptions.x {
        fields.insert("x".to_string(), x);
    }
    if let Some(y) = corruptions.y {
        fields.insert("y".to_string(), y);
    }
    if let Some(pk1) = corruptions.pk1 {
        fields.insert("pk1".to_string(), pk1);
    }
    if let Some(pk2) = corruptions.pk2 {
        fields.insert("pk2".to_string(), pk2);
    }
    Value::Object(fields)
}

pub struct MockPublicKeysOverrides<P: IPairingAdapter> {
    pub pk1: Option<P::G1>,
    pub pk2: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for MockPublicKeysOverrides<P> {
    fn default() -> Self {
        Self {
            pk1: None,
            pk2: None,
        }
    }
}

pub fn build_mock_public_keys<P: IPairingAdapter>(
    pairing: &P,
    overrides: MockPublicKeysOverrides<P>,
) -> MockPublicKeys<P> {
    let components = build_public_keys_components(pairing, Default::default());
    MockPublicKeys {
        pk1: overrides.pk1.unwrap_or(components.pk1),
        pk2: overrides.pk2.unwrap_or(components.pk2),
    }
}

#[derive(Default)]
pub struct MockPublicKeysCorruptions {
    pub pk1: Option<Value>,
    pub pk2: Option<Value>,
}

pub fn invalidate_mock_public_keys<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MockPublicKeysCorruptions,
) -> Value {
    let public_keys = build_mock_public_keys(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert(
        "pk1".to_string(),
        encoded_g1_value(pairing, &public_keys.pk1),
    );
    fields.insert(
        "pk2".to_string(),
        encoded_g2_value(pairing, &public_keys.pk2),
    );
    if let Some(pk1) = corruptions.pk1 {
        fields.insert("pk1".to_string(), pk1);
    }
    if let Some(pk2) = corruptions.pk2 {
        fields.insert("pk2".to_string(), pk2);
    }
    Value::Object(fields)
}

pub struct MockPossessionOverrides<P: IPairingAdapter> {
    pub r1: Option<P::G1>,
    pub z1: Option<P::Scalar>,
    pub r2: Option<P::G2>,
    pub z2: Option<P::Scalar>,
}

impl<P: IPairingAdapter> Default for MockPossessionOverrides<P> {
    fn default() -> Self {
        Self {
            r1: None,
            z1: None,
            r2: None,
            z2: None,
        }
    }
}

pub fn build_mock_possession<P: IPairingAdapter>(
    pairing: &P,
    overrides: MockPossessionOverrides<P>,
) -> MockPossession<P> {
    let components = build_possession_components(pairing, Default::default());
    MockPossession {
        r1: overrides.r1.unwrap_or(components.r1),
        z1: overrides.z1.unwrap_or(components.z1),
        r2: overrides.r2.unwrap_or(components.r2),
        z2: overrides.z2.unwrap_or(components.z2),
    }
}

#[derive(Default)]
pub struct MockPossessionCorruptions {
    pub r1: Option<Value>,
    pub z1: Option<Value>,
    pub r2: Option<Value>,
    pub z2: Option<Value>,
}

pub fn invalidate_mock_possession<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MockPossessionCorruptions,
) -> Value {
    let possession = build_mock_possession(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("r1".to_string(), encoded_g1_value(pairing, &possession.r1));
    fields.insert(
        "z1".to_string(),
        encoded_scalar_value(pairing, &possession.z1),
    );
    fields.insert("r2".to_string(), encoded_g2_value(pairing, &possession.r2));
    fields.insert(
        "z2".to_string(),
        encoded_scalar_value(pairing, &possession.z2),
    );
    if let Some(r1) = corruptions.r1 {
        fields.insert("r1".to_string(), r1);
    }
    if let Some(z1) = corruptions.z1 {
        fields.insert("z1".to_string(), z1);
    }
    if let Some(r2) = corruptions.r2 {
        fields.insert("r2".to_string(), r2);
    }
    if let Some(z2) = corruptions.z2 {
        fields.insert("z2".to_string(), z2);
    }
    Value::Object(fields)
}

pub struct MockEnvelopeOverrides<P: IPairingAdapter> {
    pub c1: Option<P::G1>,
    pub c2: Option<P::G1>,
    pub d1: Option<P::G2>,
    pub d2: Option<P::G2>,
}

impl<P: IPairingAdapter> Default for MockEnvelopeOverrides<P> {
    fn default() -> Self {
        Self {
            c1: None,
            c2: None,
            d1: None,
            d2: None,
        }
    }
}

pub fn build_mock_envelope<P: IPairingAdapter>(
    pairing: &P,
    overrides: MockEnvelopeOverrides<P>,
) -> MockEnvelope<P> {
    let components = build_envelope_components(pairing, Default::default());
    MockEnvelope {
        c1: overrides.c1.unwrap_or(components.c1),
        c2: overrides.c2.unwrap_or(components.c2),
        d1: overrides.d1.unwrap_or(components.d1),
        d2: overrides.d2.unwrap_or(components.d2),
    }
}

#[derive(Default)]
pub struct MockEnvelopeCorruptions {
    pub c1: Option<Value>,
    pub c2: Option<Value>,
    pub d1: Option<Value>,
    pub d2: Option<Value>,
}

pub fn invalidate_mock_envelope<P: IPairingAdapter>(
    pairing: &P,
    corruptions: MockEnvelopeCorruptions,
) -> Value {
    let envelope = build_mock_envelope(pairing, Default::default());
    let mut fields = Map::new();
    fields.insert("c1".to_string(), encoded_g1_value(pairing, &envelope.c1));
    fields.insert("c2".to_string(), encoded_g1_value(pairing, &envelope.c2));
    fields.insert("d1".to_string(), encoded_g2_value(pairing, &envelope.d1));
    fields.insert("d2".to_string(), encoded_g2_value(pairing, &envelope.d2));
    if let Some(c1) = corruptions.c1 {
        fields.insert("c1".to_string(), c1);
    }
    if let Some(c2) = corruptions.c2 {
        fields.insert("c2".to_string(), c2);
    }
    if let Some(d1) = corruptions.d1 {
        fields.insert("d1".to_string(), d1);
    }
    if let Some(d2) = corruptions.d2 {
        fields.insert("d2".to_string(), d2);
    }
    Value::Object(fields)
}

#[derive(Default)]
pub struct ConsumeKeyAgreementParamsOverrides;

pub fn build_consume_key_agreement_params(
    _overrides: ConsumeKeyAgreementParamsOverrides,
) -> ConsumeKeyAgreementParams {
    ConsumeKeyAgreementParams
}

#[derive(Default)]
pub struct ConsumeKeyAgreementPayloadOverrides;

pub fn build_consume_key_agreement_payload<K: IKeyAgreementAdapter>(
    adapter: K,
    _overrides: ConsumeKeyAgreementPayloadOverrides,
) -> ConsumeKeyAgreementPayload<K> {
    ConsumeKeyAgreementPayload { adapter }
}

#[derive(Default)]
pub struct CreateKeyAgreementDepsOverrides;

pub fn build_create_key_agreement_deps<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    C: IKeyAgreementConsumer<P>,
>(
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    encoder: &'a E,
    random: &'a dyn IRandomSourceAdapter,
    consumer: C,
    _overrides: CreateKeyAgreementDepsOverrides,
) -> CreateKeyAgreementDeps<'a, P, E, C> {
    CreateKeyAgreementDeps {
        pairing,
        hash_to_scalar,
        encoder,
        random,
        consumer,
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
    let declaration = build_key_agreement_declaration(Default::default());
    CreateKeyAgreementParams {
        concrete: overrides.concrete.unwrap_or(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        identifier: overrides.identifier.unwrap_or(declaration.identifier),
        algebra: overrides.algebra.unwrap_or(declaration.algebra),
    }
}

#[derive(Default)]
pub struct CreateKeyAgreementPayloadOverrides;

pub fn build_create_key_agreement_payload(
    _overrides: CreateKeyAgreementPayloadOverrides,
) -> CreateKeyAgreementPayload {
    CreateKeyAgreementPayload
}

#[derive(Default)]
pub struct CreateKeyAgreementSuccessReturnOverrides;

pub fn build_create_key_agreement_success_return<O>(
    output: O,
    _overrides: CreateKeyAgreementSuccessReturnOverrides,
) -> CreateKeyAgreementSuccessReturn<O> {
    CreateKeyAgreementSuccessReturn { output }
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

pub fn mock_create_key_agreement<
    'a,
    P: IPairingArithmetic,
    E: IEncoderAdapter,
    C: IKeyAgreementConsumer<P>,
>(
    deps: &CreateKeyAgreementDeps<'a, P, E, C>,
    _params: CreateKeyAgreementParams,
    _payload: CreateKeyAgreementPayload,
) -> CreateKeyAgreementReturn<C::Output> {
    let _: CreateKeyAgreementFn<'a, P, E, C> = mock_create_key_agreement;
    let adapter = build_mock_i_key_agreement_adapter(deps.pairing, Default::default());
    let output = deps.consumer.consume_key_agreement(
        build_consume_key_agreement_params(Default::default()),
        build_consume_key_agreement_payload(adapter, Default::default()),
    );
    Ok(build_create_key_agreement_success_return(
        output,
        Default::default(),
    ))
}

#[derive(Default)]
pub struct CreateKeyAgreementCaseDepsOverrides;

pub fn build_create_key_agreement_case_deps<C: ICreateKeyAgreementCaseConsumer>(
    consumer: C,
    _overrides: CreateKeyAgreementCaseDepsOverrides,
) -> CreateKeyAgreementCaseDeps<C> {
    CreateKeyAgreementCaseDeps { consumer }
}

#[derive(Default)]
pub struct CreateKeyAgreementCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub encoding: Option<CreateEncodingParams>,
    pub hash_to_scalar: Option<CreateHashToScalarParams>,
    pub random_source: Option<CreateRandomSourceParams>,
    pub kem: Option<CreateKemParams>,
}

pub fn build_create_key_agreement_case_params(
    overrides: CreateKeyAgreementCaseParamsOverrides,
) -> CreateKeyAgreementCaseParams {
    CreateKeyAgreementCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        encoding: overrides
            .encoding
            .unwrap_or_else(|| build_create_encoding_params(Default::default())),
        hash_to_scalar: overrides
            .hash_to_scalar
            .unwrap_or_else(|| build_create_hash_to_scalar_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
        kem: overrides
            .kem
            .unwrap_or_else(|| build_create_kem_params(Default::default())),
    }
}

#[derive(Default)]
pub struct CreateKeyAgreementCasePayloadOverrides;

pub fn build_create_key_agreement_case_payload(
    _overrides: CreateKeyAgreementCasePayloadOverrides,
) -> CreateKeyAgreementCasePayload {
    CreateKeyAgreementCasePayload
}

#[derive(Default)]
pub struct CreateKeyAgreementCaseSuccessReturnOverrides;

pub fn build_create_key_agreement_case_success_return<O>(
    output: O,
    _overrides: CreateKeyAgreementCaseSuccessReturnOverrides,
) -> CreateKeyAgreementCaseSuccessReturn<O> {
    CreateKeyAgreementCaseSuccessReturn { output }
}

struct CaseAtPairing<'a, C: ICreateKeyAgreementCaseConsumer> {
    pub consumer: &'a C,
    pub encoding: CreateEncodingParams,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
    pub kem: CreateKemParams,
}

impl<C: ICreateKeyAgreementCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = CreateKeyAgreementCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let encoding_deps: CreateEncodingDeps<CaseAtEncoding<'_, '_, C, P>> =
            build_create_encoding_deps(CaseAtEncoding {
                consumer: self.consumer,
                pairing: &payload.adapter,
                hash_to_scalar: self.hash_to_scalar,
                random_source: build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(self.random_source.kind),
                    },
                ),
                kem: self.kem,
            });
        match create_encoding(
            &encoding_deps,
            build_create_encoding_params(CreateEncodingParamsOverrides {
                concrete: Some(self.encoding.concrete),
                identifier: Some(self.encoding.identifier),
            }),
            build_create_encoding_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateKeyAgreementCaseErrorReturn::CreateEncoding(error)),
        }
    }
}

struct CaseAtEncoding<
    'a,
    'p,
    C: ICreateKeyAgreementCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub hash_to_scalar: CreateHashToScalarParams,
    pub random_source: CreateRandomSourceParams,
    pub kem: CreateKemParams,
}

impl<C: ICreateKeyAgreementCaseConsumer, P: IPairingArithmetic + IPairingReference>
    IEncodingConsumer for CaseAtEncoding<'_, '_, C, P>
{
    type Output = CreateKeyAgreementCaseReturn<C::Output>;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let created_hash_to_scalar = match create_hash_to_scalar::<P::Scalar>(
            &build_create_hash_to_scalar_deps(Default::default()),
            self.hash_to_scalar,
            build_create_hash_to_scalar_payload(Default::default()),
        ) {
            Ok(created) => created,
            Err(error) => {
                return Err(CreateKeyAgreementCaseErrorReturn::CreateHashToScalar(error));
            }
        };
        let created_random = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(self.random_source.kind),
            }),
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(created) => created,
            Err(error) => {
                return Err(CreateKeyAgreementCaseErrorReturn::CreateRandomSource(error));
            }
        };
        let kem_deps: CreateKemDeps<'_, P, CaseAtKem<'_, '_, C, P, E>> = build_create_kem_deps(
            self.pairing,
            &*created_hash_to_scalar.adapter,
            CaseAtKem {
                consumer: self.consumer,
                pairing: self.pairing,
                hash_to_scalar: &*created_hash_to_scalar.adapter,
                encoder: &payload.adapter,
                random: &*created_random.adapter,
            },
            Default::default(),
        );
        match create_kem(
            &kem_deps,
            self.kem,
            build_create_kem_payload(Default::default()),
        ) {
            Ok(success) => success.output,
            Err(error) => Err(CreateKeyAgreementCaseErrorReturn::CreateKem(error)),
        }
    }
}

struct CaseAtKem<
    'a,
    'p,
    C: ICreateKeyAgreementCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> {
    pub consumer: &'a C,
    pub pairing: &'p P,
    pub hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    pub encoder: &'a E,
    pub random: &'a dyn IRandomSourceAdapter,
}

impl<
    C: ICreateKeyAgreementCaseConsumer,
    P: IPairingArithmetic + IPairingReference,
    E: IEncoderAdapter + IDecoderAdapter,
> IKemConsumer<P> for CaseAtKem<'_, '_, C, P, E>
{
    type Output = CreateKeyAgreementCaseReturn<C::Output>;

    fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
        &self,
        _params: ConsumeKemParams,
        payload: ConsumeKemPayload<K>,
    ) -> Self::Output {
        let output = self.consumer.consume_create_key_agreement_case(
            self.pairing,
            self.hash_to_scalar,
            self.encoder,
            self.random,
            &payload.adapter,
            &CreateKeyAgreementCasePayload,
        );
        Ok(CreateKeyAgreementCaseSuccessReturn { output })
    }
}

pub fn run_create_key_agreement_case<C: ICreateKeyAgreementCaseConsumer>(
    deps: &CreateKeyAgreementCaseDeps<C>,
    params: CreateKeyAgreementCaseParams,
    _payload: CreateKeyAgreementCasePayload,
) -> CreateKeyAgreementCaseReturn<C::Output> {
    let _: RunCreateKeyAgreementCaseFn<C> = run_create_key_agreement_case::<C>;
    let pairing_deps: CreatePairingDeps<CaseAtPairing<'_, C>> = build_create_pairing_deps(
        CaseAtPairing {
            consumer: &deps.consumer,
            encoding: params.encoding,
            hash_to_scalar: params.hash_to_scalar,
            random_source: params.random_source,
            kem: params.kem,
        },
        Default::default(),
    );
    match create_pairing(
        &pairing_deps,
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(CreateKeyAgreementCaseErrorReturn::CreatePairing(error)),
    }
}
