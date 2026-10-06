mod interface;
pub(crate) mod provides;
#[cfg(test)]
mod test;

use crate::factory::provides::{
    EnvelopeAlgebra, EnvelopeCoins, EnvelopeComponents, EnvelopeComponentsParams,
    EnvelopeComponentsPayload, EnvelopeComponentsReturn, EnvelopeComponentsSuccessReturn,
    EnvelopeFromComponentsParams, EnvelopeFromComponentsPayload, EnvelopeFromComponentsReturn,
    EnvelopeFromComponentsSuccessReturn, GenerateKeysErrorReturn, GenerateKeysParams,
    GenerateKeysPayload, GenerateKeysReturn, GenerateKeysSuccessReturn, IKeyAgreementAdapter,
    KEY_AGREEMENT_INTERFACE_VERSION, KeyAgreementDeclaration, KeyAgreementIdentifier,
    KeyPairComponents, KeyPairComponentsParams, KeyPairComponentsPayload, KeyPairComponentsReturn,
    KeyPairComponentsSuccessReturn, KeyPairFromComponentsErrorReturn, KeyPairFromComponentsParams,
    KeyPairFromComponentsPayload, KeyPairFromComponentsReturn, KeyPairFromComponentsSuccessReturn,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, KeyPairPublicKeysReturn,
    KeyPairPublicKeysSuccessReturn, PossessionComponents, PossessionComponentsParams,
    PossessionComponentsPayload, PossessionComponentsReturn, PossessionComponentsSuccessReturn,
    PossessionFromComponentsParams, PossessionFromComponentsPayload,
    PossessionFromComponentsReturn, PossessionFromComponentsSuccessReturn, PublicKeysComponents,
    PublicKeysComponentsParams, PublicKeysComponentsPayload, PublicKeysComponentsReturn,
    PublicKeysComponentsSuccessReturn, PublicKeysFromComponentsErrorReturn,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload,
    PublicKeysFromComponentsReturn, PublicKeysFromComponentsSuccessReturn, UnwrapParams,
    UnwrapPayload, UnwrapReturn, UnwrapSuccessReturn, WrapToErrorReturn, WrapToParams,
    WrapToPayload, WrapToReturn, WrapToSuccessReturn,
};
use crate::possession_statement::provides::{
    PossessionG1Statement, PossessionG1StatementDescription,
    PossessionG1StatementDescriptionConstructorParams, PossessionG2Statement,
    PossessionG2StatementDescription, PossessionG2StatementDescriptionConstructorParams,
};
use domain::{Secret, SecretConstructorParams};
use encoding::{EncodeErrorReturn, EncodeParams, IEncoderAdapter};
use hash_to_scalar::{
    DomainTag, DomainTagConstructorParams, HashToScalarErrorReturn, HashToScalarParams,
    HashToScalarPayload,
};
use interface::{
    PAIRING_ELGAMAL_POSSESSION_G1_TAG, PAIRING_ELGAMAL_POSSESSION_G2_TAG, PairingElGamalEnvelope,
    PairingElGamalGenerateKeysErrorReturn, PairingElGamalKeyAgreement,
    PairingElGamalKeyAgreementConstructorParams, PairingElGamalKeyAgreementTryNewErrorReturn,
    PairingElGamalKeyAgreementTryNewReturn, PairingElGamalKeyPair,
    PairingElGamalKeyPairFromComponentsErrorReturn, PairingElGamalPossession,
    PairingElGamalPublicKeys, PairingElGamalPublicKeysFromComponentsErrorReturn,
    PairingElGamalWrapToErrorReturn,
};
use kem::CredentialComponents;
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1Payload,
    IsIdentityG2Params, IsIdentityG2Payload, MsmG1Params, MsmG1Payload, MsmG1Term, MsmG2Params,
    MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, MulScalarParams,
    MulScalarPayload, NegG1Params, NegG1Payload, NegG2Params, NegG2Payload, NegScalarParams,
    NegScalarPayload, PairingProductIsOneParams, PairingProductIsOnePayload, PairingProductTerm,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
};
use random::{FillBytesErrorReturn, FillBytesParams, FillBytesPayload};

impl<'a, P: IPairingArithmetic, E: IEncoderAdapter> PairingElGamalKeyAgreement<'a, P, E> {
    pub const DECLARATION: KeyAgreementDeclaration = KeyAgreementDeclaration {
        identifier: KeyAgreementIdentifier::PairingElGamalV1,
        algebra: EnvelopeAlgebra::PairingElGamal,
        possession_g1_tag: PAIRING_ELGAMAL_POSSESSION_G1_TAG,
        possession_g2_tag: PAIRING_ELGAMAL_POSSESSION_G2_TAG,
        adapter_version: 1,
        interface_version: KEY_AGREEMENT_INTERFACE_VERSION,
    };

    pub fn try_new(
        params: PairingElGamalKeyAgreementConstructorParams<'a, P, E>,
    ) -> PairingElGamalKeyAgreementTryNewReturn<'a, P, E> {
        let g1_tag = match DomainTag::try_new(DomainTagConstructorParams {
            bytes: PAIRING_ELGAMAL_POSSESSION_G1_TAG.to_vec(),
        }) {
            Ok(tag) => tag,
            Err(error) => {
                return Err(PairingElGamalKeyAgreementTryNewErrorReturn::PossessionG1Tag(error));
            }
        };
        let g2_tag = match DomainTag::try_new(DomainTagConstructorParams {
            bytes: PAIRING_ELGAMAL_POSSESSION_G2_TAG.to_vec(),
        }) {
            Ok(tag) => tag,
            Err(error) => {
                return Err(PairingElGamalKeyAgreementTryNewErrorReturn::PossessionG2Tag(error));
            }
        };
        let Ok(g1_statement_description) = PossessionG1StatementDescription::try_new(
            PossessionG1StatementDescriptionConstructorParams {
                pairing: params.pairing,
            },
        );
        let Ok(g2_statement_description) = PossessionG2StatementDescription::try_new(
            PossessionG2StatementDescriptionConstructorParams {
                pairing: params.pairing,
            },
        );
        Ok(PairingElGamalKeyAgreement {
            pairing: params.pairing,
            hash_to_scalar: params.hash_to_scalar,
            encoder: params.encoder,
            random: params.random,
            g1_statement_description,
            g2_statement_description,
            g1_tag,
            g2_tag,
        })
    }

    fn draw_uniform_secret(
        &self,
    ) -> Result<Secret<P::Scalar>, Result<FillBytesErrorReturn, SampleUniformScalarErrorReturn>>
    {
        let drawn = match self.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(drawn) => drawn.bytes,
            Err(error) => return Err(Ok(error)),
        };
        match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload { uniform: drawn },
        ) {
            Ok(sampled) => Ok(sampled.scalar),
            Err(error) => Err(Err(error)),
        }
    }

    fn challenge_g1(
        &self,
        key: &P::G1,
        commitment: &P::G1,
    ) -> Result<P::Scalar, Result<EncodeErrorReturn, HashToScalarErrorReturn>> {
        let encoded = match self.encoder.encode(
            EncodeParams {
                description: &self.g1_statement_description,
            },
            &PossessionG1Statement::<P> {
                key: key.clone(),
                commitment: commitment.clone(),
            },
        ) {
            Ok(encoded) => encoded.bytes,
            Err(error) => return Err(Ok(error)),
        };
        match self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams { tag: &self.g1_tag },
            HashToScalarPayload { message: &encoded },
        ) {
            Ok(hashed) => Ok(hashed.scalar),
            Err(error) => Err(Err(error)),
        }
    }

    fn challenge_g2(
        &self,
        key: &P::G2,
        commitment: &P::G2,
    ) -> Result<P::Scalar, Result<EncodeErrorReturn, HashToScalarErrorReturn>> {
        let encoded = match self.encoder.encode(
            EncodeParams {
                description: &self.g2_statement_description,
            },
            &PossessionG2Statement::<P> {
                key: key.clone(),
                commitment: commitment.clone(),
            },
        ) {
            Ok(encoded) => encoded.bytes,
            Err(error) => return Err(Ok(error)),
        };
        match self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams { tag: &self.g2_tag },
            HashToScalarPayload { message: &encoded },
        ) {
            Ok(hashed) => Ok(hashed.scalar),
            Err(error) => Err(Err(error)),
        }
    }
}

impl<'a, P: IPairingArithmetic, E: IEncoderAdapter> IKeyAgreementAdapter
    for PairingElGamalKeyAgreement<'a, P, E>
{
    const DECLARATION: KeyAgreementDeclaration =
        PairingElGamalKeyAgreement::<'a, P, E>::DECLARATION;

    type Pairing = P;
    type KeyPair = PairingElGamalKeyPair<P>;
    type PublicKeys = PairingElGamalPublicKeys<P>;
    type Possession = PairingElGamalPossession<P>;
    type Envelope = PairingElGamalEnvelope<P>;

    fn generate_keys(
        &self,
        _params: GenerateKeysParams,
        payload: GenerateKeysPayload,
    ) -> GenerateKeysReturn<Self::KeyPair, Self::Possession> {
        let x = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.x_uniform,
            },
        ) {
            Ok(sampled) => sampled.scalar,
            Err(error) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::XSampling(error),
                ));
            }
        };
        let y = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: payload.y_uniform,
            },
        ) {
            Ok(sampled) => sampled.scalar,
            Err(error) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::YSampling(error),
                ));
            }
        };

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
        let pk1 = pk1.product;
        let pk2 = pk2.product;

        let Ok(identity1) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload { point: pk1.clone() },
        );
        if identity1.is_identity {
            return Err(GenerateKeysErrorReturn::PairingElGamal(
                PairingElGamalGenerateKeysErrorReturn::IdentityKeyG1,
            ));
        }
        let Ok(identity2) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload { point: pk2.clone() },
        );
        if identity2.is_identity {
            return Err(GenerateKeysErrorReturn::PairingElGamal(
                PairingElGamalGenerateKeysErrorReturn::IdentityKeyG2,
            ));
        }

        let Ok(negated_g1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: g1.point.clone(),
            },
        );
        let Ok(shared) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: pk1.clone(),
                        g2: g2.point.clone(),
                    },
                    PairingProductTerm {
                        g1: negated_g1.negation,
                        g2: pk2.clone(),
                    },
                ],
            },
        );
        if shared.is_one {
            return Err(GenerateKeysErrorReturn::PairingElGamal(
                PairingElGamalGenerateKeysErrorReturn::SharedSecret,
            ));
        }

        let k1 = match self.draw_uniform_secret() {
            Ok(secret) => secret,
            Err(Ok(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::NonceDraw(error),
                ));
            }
            Err(Err(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::NonceSampling(error),
                ));
            }
        };
        let k2 = match self.draw_uniform_secret() {
            Ok(secret) => secret,
            Err(Ok(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::NonceDraw(error),
                ));
            }
            Err(Err(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::NonceSampling(error),
                ));
            }
        };

        let Ok(r1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.point.clone(),
                scalar: k1.expose().clone(),
            },
        );
        let r1 = r1.product;
        let Ok(r2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: k2.expose().clone(),
            },
        );
        let r2 = r2.product;

        let c1 = match self.challenge_g1(&pk1, &r1) {
            Ok(challenge) => challenge,
            Err(Ok(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::Encoding(error),
                ));
            }
            Err(Err(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::HashToScalar(error),
                ));
            }
        };
        let c2 = match self.challenge_g2(&pk2, &r2) {
            Ok(challenge) => challenge,
            Err(Ok(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::Encoding(error),
                ));
            }
            Err(Err(error)) => {
                return Err(GenerateKeysErrorReturn::PairingElGamal(
                    PairingElGamalGenerateKeysErrorReturn::HashToScalar(error),
                ));
            }
        };

        let Ok(c1_x) = self.pairing.mul_scalar(
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
                right: c1_x.product,
            },
        );
        let Ok(c2_y) = self.pairing.mul_scalar(
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
                right: c2_y.product,
            },
        );

        Ok(GenerateKeysSuccessReturn {
            key_pair: PairingElGamalKeyPair { x, y, pk1, pk2 },
            possession: PairingElGamalPossession {
                r1,
                z1: z1.sum,
                r2,
                z2: z2.sum,
            },
        })
    }

    fn wrap_to(
        &self,
        _params: WrapToParams,
        payload: WrapToPayload<'_, Self::PublicKeys, P::G1, P::G2>,
    ) -> WrapToReturn<Self::Envelope, P::Scalar> {
        let rho = match self.draw_uniform_secret() {
            Ok(secret) => secret,
            Err(Ok(error)) => {
                return Err(WrapToErrorReturn::PairingElGamal(
                    PairingElGamalWrapToErrorReturn::CoinDraw(error),
                ));
            }
            Err(Err(error)) => {
                return Err(WrapToErrorReturn::PairingElGamal(
                    PairingElGamalWrapToErrorReturn::CoinSampling(error),
                ));
            }
        };
        let sigma = match self.draw_uniform_secret() {
            Ok(secret) => secret,
            Err(Ok(error)) => {
                return Err(WrapToErrorReturn::PairingElGamal(
                    PairingElGamalWrapToErrorReturn::CoinDraw(error),
                ));
            }
            Err(Err(error)) => {
                return Err(WrapToErrorReturn::PairingElGamal(
                    PairingElGamalWrapToErrorReturn::CoinSampling(error),
                ));
            }
        };

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
        let Ok(rho_pk1) = self.pairing.mul_g1(
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
                right: rho_pk1.product,
            },
        );
        let Ok(d1) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point,
                scalar: sigma.expose().clone(),
            },
        );
        let Ok(sigma_pk2) = self.pairing.mul_g2(
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
                right: sigma_pk2.product,
            },
        );

        Ok(WrapToSuccessReturn {
            envelope: PairingElGamalEnvelope {
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
        let Ok(masked_g1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: payload.envelope.c1.clone(),
                scalar: payload.key_pair.x.expose().clone(),
            },
        );
        let Ok(negated_g1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: masked_g1.product,
            },
        );
        let Ok(a) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: payload.envelope.c2.clone(),
                right: negated_g1.negation,
            },
        );

        let Ok(masked_g2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: payload.envelope.d1.clone(),
                scalar: payload.key_pair.y.expose().clone(),
            },
        );
        let Ok(negated_g2) = self.pairing.neg_g2(
            NegG2Params,
            NegG2Payload {
                point: masked_g2.product,
            },
        );
        let Ok(b) = self.pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: payload.envelope.d2.clone(),
                right: negated_g2.negation,
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
        let Ok(x) = Secret::try_new(SecretConstructorParams {
            value: payload.key_pair.x.expose().clone(),
        });
        let Ok(y) = Secret::try_new(SecretConstructorParams {
            value: payload.key_pair.y.expose().clone(),
        });
        Ok(KeyPairComponentsSuccessReturn {
            components: KeyPairComponents { x, y },
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
        let components = payload.components;
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
                scalar: components.x.expose().clone(),
            },
        );
        let Ok(pk2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.point.clone(),
                scalar: components.y.expose().clone(),
            },
        );
        let pk1 = pk1.product;
        let pk2 = pk2.product;

        let Ok(identity1) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload { point: pk1.clone() },
        );
        if identity1.is_identity {
            return Err(KeyPairFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalKeyPairFromComponentsErrorReturn::IdentityKeyG1,
            ));
        }
        let Ok(identity2) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload { point: pk2.clone() },
        );
        if identity2.is_identity {
            return Err(KeyPairFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalKeyPairFromComponentsErrorReturn::IdentityKeyG2,
            ));
        }

        let Ok(negated_g1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: g1.point.clone(),
            },
        );
        let Ok(shared) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: pk1.clone(),
                        g2: g2.point.clone(),
                    },
                    PairingProductTerm {
                        g1: negated_g1.negation,
                        g2: pk2.clone(),
                    },
                ],
            },
        );
        if shared.is_one {
            return Err(KeyPairFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalKeyPairFromComponentsErrorReturn::SharedSecret,
            ));
        }

        Ok(KeyPairFromComponentsSuccessReturn {
            key_pair: PairingElGamalKeyPair {
                x: components.x,
                y: components.y,
                pk1,
                pk2,
            },
        })
    }

    fn public_keys_from_components(
        &self,
        _params: PublicKeysFromComponentsParams,
        payload: PublicKeysFromComponentsPayload<'_, P::G1, P::G2, Self::Possession>,
    ) -> PublicKeysFromComponentsReturn<Self::PublicKeys> {
        let components = payload.components;
        let possession = payload.possession;

        let Ok(identity1) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: components.pk1.clone(),
            },
        );
        if identity1.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalPublicKeysFromComponentsErrorReturn::IdentityKeyG1,
            ));
        }
        let Ok(identity2) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload {
                point: components.pk2.clone(),
            },
        );
        if identity2.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalPublicKeysFromComponentsErrorReturn::IdentityKeyG2,
            ));
        }

        let Ok(g1) = self
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2) = self
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(negated_g1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: g1.point.clone(),
            },
        );
        let Ok(shared) = self.pairing.pairing_product_is_one(
            PairingProductIsOneParams,
            PairingProductIsOnePayload {
                terms: vec![
                    PairingProductTerm {
                        g1: components.pk1.clone(),
                        g2: g2.point.clone(),
                    },
                    PairingProductTerm {
                        g1: negated_g1.negation,
                        g2: components.pk2.clone(),
                    },
                ],
            },
        );
        if shared.is_one {
            return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalPublicKeysFromComponentsErrorReturn::SharedSecret,
            ));
        }

        let c1 = match self.challenge_g1(&components.pk1, &possession.r1) {
            Ok(challenge) => challenge,
            Err(Ok(error)) => {
                return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                    PairingElGamalPublicKeysFromComponentsErrorReturn::Encoding(error),
                ));
            }
            Err(Err(error)) => {
                return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                    PairingElGamalPublicKeysFromComponentsErrorReturn::HashToScalar(error),
                ));
            }
        };
        let Ok(neg_c1) = self
            .pairing
            .neg_scalar(NegScalarParams, NegScalarPayload { scalar: c1 });
        let Ok(lhs1) = self.pairing.msm_g1(
            MsmG1Params,
            MsmG1Payload {
                terms: vec![
                    MsmG1Term {
                        base: g1.point.clone(),
                        scalar: possession.z1.clone(),
                    },
                    MsmG1Term {
                        base: components.pk1.clone(),
                        scalar: neg_c1.negation,
                    },
                ],
            },
        );
        let Ok(neg_r1) = self.pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: possession.r1.clone(),
            },
        );
        let Ok(residual1) = self.pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: lhs1.sum,
                right: neg_r1.negation,
            },
        );
        let Ok(identity1) = self.pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: residual1.sum,
            },
        );
        if !identity1.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalPublicKeysFromComponentsErrorReturn::PossessionG1,
            ));
        }

        let c2 = match self.challenge_g2(&components.pk2, &possession.r2) {
            Ok(challenge) => challenge,
            Err(Ok(error)) => {
                return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                    PairingElGamalPublicKeysFromComponentsErrorReturn::Encoding(error),
                ));
            }
            Err(Err(error)) => {
                return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                    PairingElGamalPublicKeysFromComponentsErrorReturn::HashToScalar(error),
                ));
            }
        };
        let Ok(neg_c2) = self
            .pairing
            .neg_scalar(NegScalarParams, NegScalarPayload { scalar: c2 });
        let Ok(lhs2) = self.pairing.msm_g2(
            MsmG2Params,
            MsmG2Payload {
                terms: vec![
                    MsmG2Term {
                        base: g2.point.clone(),
                        scalar: possession.z2.clone(),
                    },
                    MsmG2Term {
                        base: components.pk2.clone(),
                        scalar: neg_c2.negation,
                    },
                ],
            },
        );
        let Ok(neg_r2) = self.pairing.neg_g2(
            NegG2Params,
            NegG2Payload {
                point: possession.r2.clone(),
            },
        );
        let Ok(residual2) = self.pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: lhs2.sum,
                right: neg_r2.negation,
            },
        );
        let Ok(identity2) = self.pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload {
                point: residual2.sum,
            },
        );
        if !identity2.is_identity {
            return Err(PublicKeysFromComponentsErrorReturn::PairingElGamal(
                PairingElGamalPublicKeysFromComponentsErrorReturn::PossessionG2,
            ));
        }

        Ok(PublicKeysFromComponentsSuccessReturn {
            public_keys: PairingElGamalPublicKeys {
                pk1: components.pk1,
                pk2: components.pk2,
            },
        })
    }

    fn possession_from_components(
        &self,
        _params: PossessionFromComponentsParams,
        payload: PossessionFromComponentsPayload<P::Scalar, P::G1, P::G2>,
    ) -> PossessionFromComponentsReturn<Self::Possession> {
        let components = payload.components;
        Ok(PossessionFromComponentsSuccessReturn {
            possession: PairingElGamalPossession {
                r1: components.r1,
                z1: components.z1,
                r2: components.r2,
                z2: components.z2,
            },
        })
    }

    fn envelope_from_components(
        &self,
        _params: EnvelopeFromComponentsParams,
        payload: EnvelopeFromComponentsPayload<P::G1, P::G2>,
    ) -> EnvelopeFromComponentsReturn<Self::Envelope> {
        let components = payload.components;
        Ok(EnvelopeFromComponentsSuccessReturn {
            envelope: PairingElGamalEnvelope {
                c1: components.c1,
                c2: components.c2,
                d1: components.d1,
                d2: components.d2,
            },
        })
    }
}
