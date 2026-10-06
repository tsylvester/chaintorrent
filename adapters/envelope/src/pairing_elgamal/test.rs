#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    PAIRING_ELGAMAL_POSSESSION_G1_TAG, PAIRING_ELGAMAL_POSSESSION_G2_TAG,
    PairingElGamalGenerateKeysErrorReturn, PairingElGamalKeyAgreement,
    PairingElGamalKeyAgreementConstructorParams, PairingElGamalKeyPair,
    PairingElGamalKeyPairFromComponentsErrorReturn, PairingElGamalPossession,
    PairingElGamalPublicKeys, PairingElGamalPublicKeysFromComponentsErrorReturn,
};
use crate::factory::provides::{
    EnvelopeAlgebra, EnvelopeComponents, EnvelopeComponentsParams, EnvelopeComponentsPayload,
    EnvelopeFromComponentsParams, EnvelopeFromComponentsPayload, GenerateKeysErrorReturn,
    GenerateKeysParams, GenerateKeysPayload, GenerateKeysPayloadOverrides,
    GenerateKeysSuccessReturn, IKeyAgreementAdapter, KEY_AGREEMENT_INTERFACE_VERSION,
    KeyAgreementIdentifier, KeyPairComponents, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairFromComponentsErrorReturn, KeyPairFromComponentsParams, KeyPairFromComponentsPayload,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, PossessionComponents,
    PossessionComponentsParams, PossessionComponentsPayload, PossessionFromComponentsParams,
    PossessionFromComponentsPayload, PublicKeysComponents, PublicKeysFromComponentsErrorReturn,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, UnwrapParams, UnwrapPayload,
    WrapToParams, WrapToPayload, build_generate_keys_payload,
};
use crate::possession_statement::provides::{PossessionG1Statement, PossessionG2Statement};
use core::cell::Cell;
use domain::{Secret, SecretConstructorParamsOverrides, build_secret};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingDeps, CreateEncodingPayload,
    ENCODING_INTERFACE_VERSION, EncodeErrorReturn, EncodeParams, EncodeReturn, EncodingDeclaration,
    EncodingIdentifier, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_params, create_encoding,
};
use hash_to_scalar::{
    CreateHashToScalarDeps, CreateHashToScalarPayload, HashToScalarDeclaration, HashToScalarParams,
    HashToScalarPayload, HashToScalarReturn, IHashToScalarAdapter,
    build_create_hash_to_scalar_params, create_hash_to_scalar,
};
use kem::CredentialComponents;
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides, CreatePairingPayload,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingConsumer, ISampleUniformScalar, IsIdentityG1Params, IsIdentityG1Payload,
    IsIdentityG2Params, IsIdentityG2Payload, MsmG1Params, MsmG1Payload, MsmG1Term, MsmG2Params,
    MsmG2Payload, MsmG2Term, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, NegG1Params,
    NegG1Payload, NegG2Params, NegG2Payload, NegScalarParams, NegScalarPayload, PairingConcrete,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

fn uniform_draw<P: IPairingArithmetic>(byte: u8) -> Secret<Vec<u8>> {
    build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![byte; <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH]),
    })
}

fn draw_of(byte: u8, length: usize) -> Secret<Vec<u8>> {
    build_secret(SecretConstructorParamsOverrides {
        value: Some(vec![byte; length]),
    })
}

fn sample_scalar<P: IPairingArithmetic>(byte: u8) -> Secret<P::Scalar> {
    let Ok(sampled) = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: uniform_draw::<P>(byte),
        },
    ) else {
        panic!("a full-length draw samples");
    };
    sampled.scalar
}

fn g1<P: IPairingArithmetic>(pairing: &P) -> P::G1 {
    let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
    generator.point
}

fn g2<P: IPairingArithmetic>(pairing: &P) -> P::G2 {
    let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
    generator.point
}

fn identity_g1<P: IPairingArithmetic>(pairing: &P) -> P::G1 {
    let generator = g1(pairing);
    let Ok(negation) = pairing.neg_g1(
        NegG1Params,
        NegG1Payload {
            point: generator.clone(),
        },
    );
    let Ok(sum) = pairing.add_g1(
        AddG1Params,
        AddG1Payload {
            left: generator,
            right: negation.negation,
        },
    );
    sum.sum
}

fn identity_g2<P: IPairingArithmetic>(pairing: &P) -> P::G2 {
    let generator = g2(pairing);
    let Ok(negation) = pairing.neg_g2(
        NegG2Params,
        NegG2Payload {
            point: generator.clone(),
        },
    );
    let Ok(sum) = pairing.add_g2(
        AddG2Params,
        AddG2Payload {
            left: generator,
            right: negation.negation,
        },
    );
    sum.sum
}

fn encode_g1<P: IPairingArithmetic>(pairing: &P, point: &P::G1) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn encode_g2<P: IPairingArithmetic>(pairing: &P, point: &P::G2) -> Vec<u8> {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    encoded.bytes.as_ref().to_vec()
}

fn credential<P: IPairingArithmetic>(pairing: &P) -> CredentialComponents<P::G1, P::G2> {
    let Ok(a) = pairing.mul_g1(
        MulG1Params,
        MulG1Payload {
            point: g1(pairing),
            scalar: sample_scalar::<P>(0x55).expose().clone(),
        },
    );
    let Ok(b) = pairing.mul_g2(
        MulG2Params,
        MulG2Payload {
            point: g2(pairing),
            scalar: sample_scalar::<P>(0x66).expose().clone(),
        },
    );
    CredentialComponents {
        a: a.product,
        b: b.product,
    }
}

fn generate<P: IPairingArithmetic, E: IEncoderAdapter>(
    key_agreement: &PairingElGamalKeyAgreement<'_, P, E>,
    payload: GenerateKeysPayload,
) -> GenerateKeysSuccessReturn<PairingElGamalKeyPair<P>, PairingElGamalPossession<P>> {
    let Ok(generated) = key_agreement.generate_keys(GenerateKeysParams, payload) else {
        panic!("the key pair generates");
    };
    generated
}

fn admitted_public_keys<P: IPairingArithmetic, E: IEncoderAdapter>(
    key_agreement: &PairingElGamalKeyAgreement<'_, P, E>,
    key_pair: &PairingElGamalKeyPair<P>,
    possession: &PairingElGamalPossession<P>,
) -> PairingElGamalPublicKeys<P> {
    let Ok(read) = key_agreement.key_pair_public_keys(
        KeyPairPublicKeysParams,
        KeyPairPublicKeysPayload { key_pair },
    );
    let Ok(admitted) = key_agreement.public_keys_from_components(
        PublicKeysFromComponentsParams,
        PublicKeysFromComponentsPayload {
            components: read.components,
            possession,
        },
    ) else {
        panic!("the key pair's public keys are admitted with their proof of possession");
    };
    admitted.public_keys
}

fn run_probe<C: IPairingConsumer>(consumer: C, concrete: PairingConcrete) -> C::Output {
    let deps = CreatePairingDeps { consumer };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(concrete),
        ..Default::default()
    });
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("the named concrete is constructed and consumed");
    };
    success.output
}

struct RoundTripOutcome {
    first_round_trips: bool,
    second_round_trips: bool,
    rewrapped_round_trips: bool,
    rewrap_changes_every_element: bool,
}

struct RoundTripProbe;

struct RoundTripStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for RoundTripStep<'_, P> {
    type Output = RoundTripOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };

        let generated = generate(
            &key_agreement,
            build_generate_keys_payload(Default::default()),
        );
        let key_pair = generated.key_pair;
        let possession = generated.possession;
        let public_keys = admitted_public_keys(&key_agreement, &key_pair, &possession);

        let wrapped_components = credential(self.pairing);
        let wrapped_a = encode_g1(self.pairing, &wrapped_components.a);
        let wrapped_b = encode_g2(self.pairing, &wrapped_components.b);

        let Ok(wrapped) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &public_keys,
                credential: CredentialComponents {
                    a: wrapped_components.a.clone(),
                    b: wrapped_components.b.clone(),
                },
            },
        ) else {
            panic!("the credential wraps");
        };
        let Ok(unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &key_pair,
                envelope: &wrapped.envelope,
            },
        );
        let first_round_trips = encode_g1(self.pairing, &unwrapped.credential.a) == wrapped_a;
        let second_round_trips = encode_g2(self.pairing, &unwrapped.credential.b) == wrapped_b;

        let Ok(first_components) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &wrapped.envelope,
            },
        );

        let Ok(rewrapped) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &public_keys,
                credential: CredentialComponents {
                    a: wrapped_components.a.clone(),
                    b: wrapped_components.b.clone(),
                },
            },
        ) else {
            panic!("the credential rewraps");
        };
        let Ok(reunwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &key_pair,
                envelope: &rewrapped.envelope,
            },
        );
        let rewrapped_round_trips = encode_g1(self.pairing, &reunwrapped.credential.a) == wrapped_a
            && encode_g2(self.pairing, &reunwrapped.credential.b) == wrapped_b;

        let Ok(second_components) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &rewrapped.envelope,
            },
        );
        let first = first_components.components;
        let second = second_components.components;
        let rewrap_changes_every_element = encode_g1(self.pairing, &first.c1)
            != encode_g1(self.pairing, &second.c1)
            && encode_g1(self.pairing, &first.c2) != encode_g1(self.pairing, &second.c2)
            && encode_g2(self.pairing, &first.d1) != encode_g2(self.pairing, &second.d1)
            && encode_g2(self.pairing, &first.d2) != encode_g2(self.pairing, &second.d2);

        RoundTripOutcome {
            first_round_trips,
            second_round_trips,
            rewrapped_round_trips,
            rewrap_changes_every_element,
        }
    }
}

impl IPairingConsumer for RoundTripProbe {
    type Output = RoundTripOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = RoundTripStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

struct SwapOutcome {
    other_keys_change_the_first: bool,
    other_keys_change_the_second: bool,
    exchanged_secrets_change_the_first: bool,
    exchanged_secrets_change_the_second: bool,
    foreign_c1_changes_only_the_first: bool,
    foreign_d1_changes_only_the_second: bool,
}

struct SwapProbe;

struct SwapStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for SwapStep<'_, P> {
    type Output = SwapOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };

        let generated = generate(
            &key_agreement,
            build_generate_keys_payload(Default::default()),
        );
        let key_pair = generated.key_pair;
        let possession = generated.possession;
        let public_keys = admitted_public_keys(&key_agreement, &key_pair, &possession);

        let other_generated = generate(
            &key_agreement,
            build_generate_keys_payload(GenerateKeysPayloadOverrides {
                x_uniform: Some(uniform_draw::<P>(0xb1)),
                y_uniform: Some(uniform_draw::<P>(0xb2)),
            }),
        );
        let other_key_pair = other_generated.key_pair;

        let wrapped_components = credential(self.pairing);
        let wrapped_a = encode_g1(self.pairing, &wrapped_components.a);
        let wrapped_b = encode_g2(self.pairing, &wrapped_components.b);

        let Ok(wrapped) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &public_keys,
                credential: CredentialComponents {
                    a: wrapped_components.a.clone(),
                    b: wrapped_components.b.clone(),
                },
            },
        ) else {
            panic!("the credential wraps");
        };
        let Ok(second) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &public_keys,
                credential: CredentialComponents {
                    a: wrapped_components.a.clone(),
                    b: wrapped_components.b.clone(),
                },
            },
        ) else {
            panic!("the credential wraps a second time");
        };

        let Ok(other_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &other_key_pair,
                envelope: &wrapped.envelope,
            },
        );
        let other_keys_change_the_first =
            encode_g1(self.pairing, &other_unwrapped.credential.a) != wrapped_a;
        let other_keys_change_the_second =
            encode_g2(self.pairing, &other_unwrapped.credential.b) != wrapped_b;

        let Ok(secrets) = key_agreement.key_pair_components(
            KeyPairComponentsParams,
            KeyPairComponentsPayload {
                key_pair: &key_pair,
            },
        );
        let exchanged = secrets.components;
        let Ok(exchanged_key_pair) = key_agreement.key_pair_from_components(
            KeyPairFromComponentsParams,
            KeyPairFromComponentsPayload {
                components: KeyPairComponents {
                    x: exchanged.y,
                    y: exchanged.x,
                },
            },
        ) else {
            panic!("the exchanged secrets form a key pair");
        };
        let Ok(exchanged_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &exchanged_key_pair.key_pair,
                envelope: &wrapped.envelope,
            },
        );
        let exchanged_secrets_change_the_first =
            encode_g1(self.pairing, &exchanged_unwrapped.credential.a) != wrapped_a;
        let exchanged_secrets_change_the_second =
            encode_g2(self.pairing, &exchanged_unwrapped.credential.b) != wrapped_b;

        let Ok(first_components) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &wrapped.envelope,
            },
        );
        let Ok(second_components) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &second.envelope,
            },
        );
        let first = first_components.components;
        let second = second_components.components;

        let Ok(c1_swapped) = key_agreement.envelope_from_components(
            EnvelopeFromComponentsParams,
            EnvelopeFromComponentsPayload {
                components: EnvelopeComponents {
                    c1: second.c1.clone(),
                    c2: first.c2.clone(),
                    d1: first.d1.clone(),
                    d2: first.d2.clone(),
                },
            },
        );
        let Ok(c1_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &key_pair,
                envelope: &c1_swapped.envelope,
            },
        );
        let foreign_c1_changes_only_the_first = encode_g1(self.pairing, &c1_unwrapped.credential.a)
            != wrapped_a
            && encode_g2(self.pairing, &c1_unwrapped.credential.b) == wrapped_b;

        let Ok(d1_swapped) = key_agreement.envelope_from_components(
            EnvelopeFromComponentsParams,
            EnvelopeFromComponentsPayload {
                components: EnvelopeComponents {
                    c1: first.c1.clone(),
                    c2: first.c2.clone(),
                    d1: second.d1.clone(),
                    d2: first.d2.clone(),
                },
            },
        );
        let Ok(d1_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &key_pair,
                envelope: &d1_swapped.envelope,
            },
        );
        let foreign_d1_changes_only_the_second =
            encode_g1(self.pairing, &d1_unwrapped.credential.a) == wrapped_a
                && encode_g2(self.pairing, &d1_unwrapped.credential.b) != wrapped_b;

        SwapOutcome {
            other_keys_change_the_first,
            other_keys_change_the_second,
            exchanged_secrets_change_the_first,
            exchanged_secrets_change_the_second,
            foreign_c1_changes_only_the_first,
            foreign_d1_changes_only_the_second,
        }
    }
}

impl IPairingConsumer for SwapProbe {
    type Output = SwapOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = SwapStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

struct AdmissionOutcome {
    own_keys_admitted: bool,
    other_possession_refusal: Option<PublicKeysFromComponentsErrorReturn>,
    foreign_g2_possession_refusal: Option<PublicKeysFromComponentsErrorReturn>,
    identity_first_key_refusal: Option<PublicKeysFromComponentsErrorReturn>,
    identity_second_key_refusal: Option<PublicKeysFromComponentsErrorReturn>,
    shared_secret_refusal: Option<PublicKeysFromComponentsErrorReturn>,
    rebuilt_possession_admitted: bool,
}

struct AdmissionProbe;

struct AdmissionStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for AdmissionStep<'_, P> {
    type Output = AdmissionOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };

        let generated = generate(
            &key_agreement,
            build_generate_keys_payload(Default::default()),
        );
        let possession = generated.possession;
        let other_generated = generate(
            &key_agreement,
            build_generate_keys_payload(GenerateKeysPayloadOverrides {
                x_uniform: Some(uniform_draw::<P>(0xb1)),
                y_uniform: Some(uniform_draw::<P>(0xb2)),
            }),
        );
        let other_possession = other_generated.possession;

        let Ok(read) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &generated.key_pair,
            },
        );
        let own_components = read.components;

        let own_keys_admitted = key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: own_components.pk1.clone(),
                        pk2: own_components.pk2.clone(),
                    },
                    possession: &possession,
                },
            )
            .is_ok();

        let other_possession_refusal = key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: own_components.pk1.clone(),
                        pk2: own_components.pk2.clone(),
                    },
                    possession: &other_possession,
                },
            )
            .err();

        let Ok(own_possession_components) = key_agreement.possession_components(
            PossessionComponentsParams,
            PossessionComponentsPayload {
                possession: &possession,
            },
        );
        let Ok(other_possession_components) = key_agreement.possession_components(
            PossessionComponentsParams,
            PossessionComponentsPayload {
                possession: &other_possession,
            },
        );
        let own = own_possession_components.components;
        let other = other_possession_components.components;
        let Ok(foreign_g2_possession) = key_agreement.possession_from_components(
            PossessionFromComponentsParams,
            PossessionFromComponentsPayload {
                components: PossessionComponents {
                    r1: own.r1,
                    z1: own.z1,
                    r2: other.r2,
                    z2: other.z2,
                },
            },
        );
        let foreign_g2_possession_refusal = key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: own_components.pk1.clone(),
                        pk2: own_components.pk2.clone(),
                    },
                    possession: &foreign_g2_possession.possession,
                },
            )
            .err();

        let identity_first_key_refusal = key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: identity_g1(self.pairing),
                        pk2: own_components.pk2.clone(),
                    },
                    possession: &possession,
                },
            )
            .err();
        let identity_second_key_refusal = key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: own_components.pk1.clone(),
                        pk2: identity_g2(self.pairing),
                    },
                    possession: &possession,
                },
            )
            .err();

        let shared = sample_scalar::<P>(0xc1);
        let Ok(shared_pk1) = self.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1(self.pairing),
                scalar: shared.expose().clone(),
            },
        );
        let Ok(shared_pk2) = self.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2(self.pairing),
                scalar: shared.expose().clone(),
            },
        );
        let shared_secret_refusal = key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: shared_pk1.product,
                        pk2: shared_pk2.product,
                    },
                    possession: &possession,
                },
            )
            .err();

        let Ok(own_possession_components) = key_agreement.possession_components(
            PossessionComponentsParams,
            PossessionComponentsPayload {
                possession: &possession,
            },
        );
        let Ok(rebuilt_possession) = key_agreement.possession_from_components(
            PossessionFromComponentsParams,
            PossessionFromComponentsPayload {
                components: own_possession_components.components,
            },
        );
        let rebuilt_possession_admitted = key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: PublicKeysComponents {
                        pk1: own_components.pk1.clone(),
                        pk2: own_components.pk2.clone(),
                    },
                    possession: &rebuilt_possession.possession,
                },
            )
            .is_ok();

        AdmissionOutcome {
            own_keys_admitted,
            other_possession_refusal,
            foreign_g2_possession_refusal,
            identity_first_key_refusal,
            identity_second_key_refusal,
            shared_secret_refusal,
            rebuilt_possession_admitted,
        }
    }
}

impl IPairingConsumer for AdmissionProbe {
    type Output = AdmissionOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = AdmissionStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

struct KeyPairRefusalOutcome {
    shared_secret_refusal: Option<KeyPairFromComponentsErrorReturn>,
    zero_first_secret_refusal: Option<KeyPairFromComponentsErrorReturn>,
    zero_second_secret_refusal: Option<KeyPairFromComponentsErrorReturn>,
}

struct KeyPairRefusalProbe;

struct KeyPairRefusalStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for KeyPairRefusalStep<'_, P> {
    type Output = KeyPairRefusalOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };

        let shared_secret_refusal = key_agreement
            .key_pair_from_components(
                KeyPairFromComponentsParams,
                KeyPairFromComponentsPayload {
                    components: KeyPairComponents {
                        x: sample_scalar::<P>(0x11),
                        y: sample_scalar::<P>(0x11),
                    },
                },
            )
            .err();
        let zero_first_secret_refusal = key_agreement
            .key_pair_from_components(
                KeyPairFromComponentsParams,
                KeyPairFromComponentsPayload {
                    components: KeyPairComponents {
                        x: sample_scalar::<P>(0x00),
                        y: sample_scalar::<P>(0x22),
                    },
                },
            )
            .err();
        let zero_second_secret_refusal = key_agreement
            .key_pair_from_components(
                KeyPairFromComponentsParams,
                KeyPairFromComponentsPayload {
                    components: KeyPairComponents {
                        x: sample_scalar::<P>(0x11),
                        y: sample_scalar::<P>(0x00),
                    },
                },
            )
            .err();

        KeyPairRefusalOutcome {
            shared_secret_refusal,
            zero_first_secret_refusal,
            zero_second_secret_refusal,
        }
    }
}

impl IPairingConsumer for KeyPairRefusalProbe {
    type Output = KeyPairRefusalOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = KeyPairRefusalStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

struct GenerationRefusalOutcome {
    equal_draws_refusal: Option<GenerateKeysErrorReturn>,
    zero_x_refusal: Option<GenerateKeysErrorReturn>,
    zero_y_refusal: Option<GenerateKeysErrorReturn>,
    short_x_refusal: Option<GenerateKeysErrorReturn>,
    short_y_refusal: Option<GenerateKeysErrorReturn>,
}

struct GenerationRefusalProbe;

struct GenerationRefusalStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for GenerationRefusalStep<'_, P> {
    type Output = GenerationRefusalOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };

        let equal_draws_refusal = key_agreement
            .generate_keys(
                GenerateKeysParams,
                build_generate_keys_payload(GenerateKeysPayloadOverrides {
                    x_uniform: Some(uniform_draw::<P>(0x11)),
                    y_uniform: Some(uniform_draw::<P>(0x11)),
                }),
            )
            .err();
        let zero_x_refusal = key_agreement
            .generate_keys(
                GenerateKeysParams,
                build_generate_keys_payload(GenerateKeysPayloadOverrides {
                    x_uniform: Some(draw_of(
                        0x00,
                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    )),
                    ..Default::default()
                }),
            )
            .err();
        let zero_y_refusal = key_agreement
            .generate_keys(
                GenerateKeysParams,
                build_generate_keys_payload(GenerateKeysPayloadOverrides {
                    y_uniform: Some(draw_of(
                        0x00,
                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    )),
                    ..Default::default()
                }),
            )
            .err();
        let short_x_refusal = key_agreement
            .generate_keys(
                GenerateKeysParams,
                build_generate_keys_payload(GenerateKeysPayloadOverrides {
                    x_uniform: Some(draw_of(0xee, 32)),
                    ..Default::default()
                }),
            )
            .err();
        let short_y_refusal = key_agreement
            .generate_keys(
                GenerateKeysParams,
                build_generate_keys_payload(GenerateKeysPayloadOverrides {
                    y_uniform: Some(draw_of(0xdd, 32)),
                    ..Default::default()
                }),
            )
            .err();

        GenerationRefusalOutcome {
            equal_draws_refusal,
            zero_x_refusal,
            zero_y_refusal,
            short_x_refusal,
            short_y_refusal,
        }
    }
}

impl IPairingConsumer for GenerationRefusalProbe {
    type Output = GenerationRefusalOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = GenerationRefusalStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

struct ComponentsOutcome {
    public_keys_are_the_secrets_times_the_generators: bool,
    first_coin_elements_are_the_coins_times_the_generators: bool,
    second_coin_elements_mask_the_credential: bool,
    rebuilt_envelope_opens: bool,
    restored_key_pair_opens: bool,
    possession_challenges_hash_under_the_declared_tags: bool,
}

struct ComponentsProbe;

struct ComponentsStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for ComponentsStep<'_, P> {
    type Output = ComponentsOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let pairing = self.pairing;
        let Ok(key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };

        let generated = generate(
            &key_agreement,
            build_generate_keys_payload(Default::default()),
        );
        let key_pair = generated.key_pair;
        let possession = generated.possession;
        let public_keys = admitted_public_keys(&key_agreement, &key_pair, &possession);

        let Ok(secrets) = key_agreement.key_pair_components(
            KeyPairComponentsParams,
            KeyPairComponentsPayload {
                key_pair: &key_pair,
            },
        );
        let Ok(keys) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &key_pair,
            },
        );
        let x = secrets.components.x.expose().clone();
        let y = secrets.components.y.expose().clone();
        let pk1 = keys.components.pk1.clone();
        let pk2 = keys.components.pk2.clone();

        let Ok(expected_pk1) = pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1(pairing),
                scalar: x.clone(),
            },
        );
        let Ok(expected_pk2) = pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2(pairing),
                scalar: y.clone(),
            },
        );
        let public_keys_are_the_secrets_times_the_generators =
            encode_g1(pairing, &expected_pk1.product) == encode_g1(pairing, &pk1)
                && encode_g2(pairing, &expected_pk2.product) == encode_g2(pairing, &pk2);

        let wrapped_components = credential(pairing);
        let wrapped_a = encode_g1(pairing, &wrapped_components.a);
        let wrapped_b = encode_g2(pairing, &wrapped_components.b);
        let Ok(wrapped) = key_agreement.wrap_to(
            WrapToParams,
            WrapToPayload {
                public_keys: &public_keys,
                credential: CredentialComponents {
                    a: wrapped_components.a.clone(),
                    b: wrapped_components.b.clone(),
                },
            },
        ) else {
            panic!("the credential wraps");
        };
        let rho = wrapped.coins.rho.expose().clone();
        let sigma = wrapped.coins.sigma.expose().clone();

        let Ok(elements) = key_agreement.envelope_components(
            EnvelopeComponentsParams,
            EnvelopeComponentsPayload {
                envelope: &wrapped.envelope,
            },
        );
        let elements = elements.components;

        let Ok(expected_c1) = pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1(pairing),
                scalar: rho.clone(),
            },
        );
        let Ok(expected_d1) = pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2(pairing),
                scalar: sigma.clone(),
            },
        );
        let first_coin_elements_are_the_coins_times_the_generators =
            encode_g1(pairing, &elements.c1) == encode_g1(pairing, &expected_c1.product)
                && encode_g2(pairing, &elements.d1) == encode_g2(pairing, &expected_d1.product);

        let Ok(rho_pk1) = pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: pk1.clone(),
                scalar: rho.clone(),
            },
        );
        let Ok(expected_c2) = pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: wrapped_components.a.clone(),
                right: rho_pk1.product,
            },
        );
        let Ok(sigma_pk2) = pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: pk2.clone(),
                scalar: sigma.clone(),
            },
        );
        let Ok(expected_d2) = pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: wrapped_components.b.clone(),
                right: sigma_pk2.product,
            },
        );
        let second_coin_elements_mask_the_credential = encode_g1(pairing, &elements.c2)
            == encode_g1(pairing, &expected_c2.sum)
            && encode_g2(pairing, &elements.d2) == encode_g2(pairing, &expected_d2.sum);

        let Ok(rebuilt_envelope) = key_agreement.envelope_from_components(
            EnvelopeFromComponentsParams,
            EnvelopeFromComponentsPayload {
                components: EnvelopeComponents {
                    c1: elements.c1.clone(),
                    c2: elements.c2.clone(),
                    d1: elements.d1.clone(),
                    d2: elements.d2.clone(),
                },
            },
        );
        let Ok(rebuilt_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &key_pair,
                envelope: &rebuilt_envelope.envelope,
            },
        );
        let rebuilt_envelope_opens = encode_g1(pairing, &rebuilt_unwrapped.credential.a)
            == wrapped_a
            && encode_g2(pairing, &rebuilt_unwrapped.credential.b) == wrapped_b;

        let Ok(restored) = key_agreement.key_pair_from_components(
            KeyPairFromComponentsParams,
            KeyPairFromComponentsPayload {
                components: KeyPairComponents {
                    x: secrets.components.x,
                    y: secrets.components.y,
                },
            },
        ) else {
            panic!("the key pair restores from its components");
        };
        let Ok(restored_unwrapped) = key_agreement.unwrap(
            UnwrapParams,
            UnwrapPayload {
                key_pair: &restored.key_pair,
                envelope: &wrapped.envelope,
            },
        );
        let restored_key_pair_opens = encode_g1(pairing, &restored_unwrapped.credential.a)
            == wrapped_a
            && encode_g2(pairing, &restored_unwrapped.credential.b) == wrapped_b;

        let Ok(proof) = key_agreement.possession_components(
            PossessionComponentsParams,
            PossessionComponentsPayload {
                possession: &possession,
            },
        );
        let proof = proof.components;

        let Ok(encoded_g1) = payload.adapter.encode(
            EncodeParams {
                description: &key_agreement.g1_statement_description,
            },
            &PossessionG1Statement::<P> {
                key: pk1.clone(),
                commitment: proof.r1.clone(),
            },
        ) else {
            panic!("the first-group possession statement encodes");
        };
        let Ok(c1_challenge) = self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams {
                tag: &key_agreement.g1_tag,
            },
            HashToScalarPayload {
                message: &encoded_g1.bytes,
            },
        ) else {
            panic!("the first-group challenge hashes");
        };
        let Ok(encoded_g2) = payload.adapter.encode(
            EncodeParams {
                description: &key_agreement.g2_statement_description,
            },
            &PossessionG2Statement::<P> {
                key: pk2.clone(),
                commitment: proof.r2.clone(),
            },
        ) else {
            panic!("the second-group possession statement encodes");
        };
        let Ok(c2_challenge) = self.hash_to_scalar.hash_to_scalar(
            HashToScalarParams {
                tag: &key_agreement.g2_tag,
            },
            HashToScalarPayload {
                message: &encoded_g2.bytes,
            },
        ) else {
            panic!("the second-group challenge hashes");
        };

        let Ok(neg_c1) = pairing.neg_scalar(
            NegScalarParams,
            NegScalarPayload {
                scalar: c1_challenge.scalar,
            },
        );
        let Ok(lhs1) = pairing.msm_g1(
            MsmG1Params,
            MsmG1Payload {
                terms: vec![
                    MsmG1Term {
                        base: g1(pairing),
                        scalar: proof.z1.clone(),
                    },
                    MsmG1Term {
                        base: pk1.clone(),
                        scalar: neg_c1.negation,
                    },
                ],
            },
        );
        let Ok(neg_r1) = pairing.neg_g1(
            NegG1Params,
            NegG1Payload {
                point: proof.r1.clone(),
            },
        );
        let Ok(residual1) = pairing.add_g1(
            AddG1Params,
            AddG1Payload {
                left: lhs1.sum,
                right: neg_r1.negation,
            },
        );
        let Ok(identity1) = pairing.is_identity_g1(
            IsIdentityG1Params,
            IsIdentityG1Payload {
                point: residual1.sum,
            },
        );

        let Ok(neg_c2) = pairing.neg_scalar(
            NegScalarParams,
            NegScalarPayload {
                scalar: c2_challenge.scalar,
            },
        );
        let Ok(lhs2) = pairing.msm_g2(
            MsmG2Params,
            MsmG2Payload {
                terms: vec![
                    MsmG2Term {
                        base: g2(pairing),
                        scalar: proof.z2.clone(),
                    },
                    MsmG2Term {
                        base: pk2.clone(),
                        scalar: neg_c2.negation,
                    },
                ],
            },
        );
        let Ok(neg_r2) = pairing.neg_g2(
            NegG2Params,
            NegG2Payload {
                point: proof.r2.clone(),
            },
        );
        let Ok(residual2) = pairing.add_g2(
            AddG2Params,
            AddG2Payload {
                left: lhs2.sum,
                right: neg_r2.negation,
            },
        );
        let Ok(identity2) = pairing.is_identity_g2(
            IsIdentityG2Params,
            IsIdentityG2Payload {
                point: residual2.sum,
            },
        );

        let possession_challenges_hash_under_the_declared_tags =
            identity1.is_identity && identity2.is_identity;

        ComponentsOutcome {
            public_keys_are_the_secrets_times_the_generators,
            first_coin_elements_are_the_coins_times_the_generators,
            second_coin_elements_mask_the_credential,
            rebuilt_envelope_opens,
            restored_key_pair_opens,
            possession_challenges_hash_under_the_declared_tags,
        }
    }
}

impl IPairingConsumer for ComponentsProbe {
    type Output = ComponentsOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = ComponentsStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

struct DeclarationOutcome {
    identifier_matches: bool,
    algebra: EnvelopeAlgebra,
    possession_g1_tag: &'static [u8],
    possession_g2_tag: &'static [u8],
    adapter_version: u32,
    interface_version: u32,
}

struct DeclarationProbe;

struct DeclarationStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for DeclarationStep<'_, P> {
    type Output = DeclarationOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        _payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let _ = (
            self.pairing,
            self.hash_to_scalar,
            self.random,
            &_payload.adapter,
        );
        let declaration = PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION;
        DeclarationOutcome {
            identifier_matches: matches!(
                declaration.identifier,
                KeyAgreementIdentifier::PairingElGamalV1
            ),
            algebra: declaration.algebra,
            possession_g1_tag: declaration.possession_g1_tag,
            possession_g2_tag: declaration.possession_g2_tag,
            adapter_version: declaration.adapter_version,
            interface_version: declaration.interface_version,
        }
    }
}

impl IPairingConsumer for DeclarationProbe {
    type Output = DeclarationOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = DeclarationStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

struct RefusingEncoder;

impl IEncoderAdapter for RefusingEncoder {
    const DECLARATION: EncodingDeclaration = EncodingDeclaration {
        identifier: EncodingIdentifier::EthereumAbiV1,
        adapter_version: 1,
        interface_version: ENCODING_INTERFACE_VERSION,
    };

    fn encode<D: encoding::IEncodingContract>(
        &self,
        _params: EncodeParams<'_, D>,
        _payload: &D::Described,
    ) -> EncodeReturn {
        Err(EncodeErrorReturn::FieldCount {
            expected: 2,
            actual: 1,
        })
    }
}

struct SpyHash<'a, S> {
    inner: &'a dyn IHashToScalarAdapter<S>,
    calls: Cell<usize>,
}

impl<S: ISampleUniformScalar + Clone> IHashToScalarAdapter<S> for SpyHash<'_, S> {
    fn declaration(&self) -> HashToScalarDeclaration {
        self.inner.declaration()
    }

    fn hash_to_scalar(
        &self,
        params: HashToScalarParams<'_>,
        payload: HashToScalarPayload<'_>,
    ) -> HashToScalarReturn<S> {
        self.calls.set(self.calls.get() + 1);
        self.inner.hash_to_scalar(params, payload)
    }
}

struct EncodingRefusalOutcome {
    generation_refusal: Option<GenerateKeysErrorReturn>,
    admission_refusal: Option<PublicKeysFromComponentsErrorReturn>,
    hash_calls: usize,
}

struct EncodingRefusalProbe;

struct EncodingRefusalStep<'a, P: IPairingArithmetic> {
    pairing: &'a P,
    hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
    random: &'a dyn IRandomSourceAdapter,
}

impl<P: IPairingArithmetic> IEncodingConsumer for EncodingRefusalStep<'_, P> {
    type Output = EncodingRefusalOutcome;

    fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
        &self,
        _params: ConsumeEncodingParams,
        payload: ConsumeEncodingPayload<E>,
    ) -> Self::Output {
        let Ok(key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: self.hash_to_scalar,
                encoder: &payload.adapter,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };
        let generated = generate(
            &key_agreement,
            build_generate_keys_payload(Default::default()),
        );
        let possession = generated.possession;
        let Ok(keys) = key_agreement.key_pair_public_keys(
            KeyPairPublicKeysParams,
            KeyPairPublicKeysPayload {
                key_pair: &generated.key_pair,
            },
        );

        let refusing = RefusingEncoder;
        let spy = SpyHash {
            inner: self.hash_to_scalar,
            calls: Cell::new(0),
        };
        let Ok(refusing_key_agreement) =
            PairingElGamalKeyAgreement::try_new(PairingElGamalKeyAgreementConstructorParams {
                pairing: self.pairing,
                hash_to_scalar: &spy,
                encoder: &refusing,
                random: self.random,
            })
        else {
            panic!("the declared tags are admitted");
        };

        let generation_refusal = refusing_key_agreement
            .generate_keys(
                GenerateKeysParams,
                build_generate_keys_payload(Default::default()),
            )
            .err();
        let admission_refusal = refusing_key_agreement
            .public_keys_from_components(
                PublicKeysFromComponentsParams,
                PublicKeysFromComponentsPayload {
                    components: keys.components,
                    possession: &possession,
                },
            )
            .err();
        let hash_calls = spy.calls.get();

        EncodingRefusalOutcome {
            generation_refusal,
            admission_refusal,
            hash_calls,
        }
    }
}

impl IPairingConsumer for EncodingRefusalProbe {
    type Output = EncodingRefusalOutcome;

    fn consume_pairing<P: IPairingArithmetic>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let pairing = &payload.adapter;
        let Ok(hash) = create_hash_to_scalar::<P::Scalar>(
            &CreateHashToScalarDeps,
            build_create_hash_to_scalar_params(Default::default()),
            CreateHashToScalarPayload,
        ) else {
            panic!("the hash-to-scalar concrete is constructed");
        };
        let Ok(random) = create_random_source(
            &CreateRandomSourceDeps,
            build_create_random_source_params(Default::default()),
            CreateRandomSourcePayload,
        );
        let step = EncodingRefusalStep {
            pairing,
            hash_to_scalar: hash.adapter.as_ref(),
            random: random.adapter.as_ref(),
        };
        let Ok(success) = create_encoding(
            &CreateEncodingDeps { consumer: step },
            build_create_encoding_params(Default::default()),
            CreateEncodingPayload,
        ) else {
            panic!("the encoding concrete is constructed and consumed");
        };
        success.output
    }
}

/// Contract: credential components wrapped to a key pair's public keys unwrap
///   under that key pair to themselves (CR-04 exact round trip).
/// Arrange: RoundTripProbe.
/// Act:     create_pairing.
/// Assert:  first_round_trips and second_round_trips.
#[test]
fn every_envelope_opens_under_its_key_pair_to_the_credential_it_wrapped() {
    // Arrange
    let probe = RoundTripProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.first_round_trips);
    assert!(outcome.second_round_trips);
}

/// Contract: the same credential components wrapped under other coins are a
///   wholly different envelope that opens to the same components (CR-04
///   independent coins).
/// Arrange: RoundTripProbe.
/// Act:     create_pairing.
/// Assert:  rewrap_changes_every_element and rewrapped_round_trips.
#[test]
fn independent_coins_change_every_envelope_element_and_still_open() {
    // Arrange
    let probe = RoundTripProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rewrap_changes_every_element);
    assert!(outcome.rewrapped_round_trips);
}

/// Contract: the construction holds on the bn254_arkworks pairing concrete.
/// Arrange: RoundTripProbe and the named concrete.
/// Act:     create_pairing.
/// Assert:  first_round_trips, second_round_trips, and rewrapped_round_trips.
#[test]
fn every_envelope_opens_under_its_key_pair_on_bn254_arkworks() {
    // Arrange
    let probe = RoundTripProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    assert!(outcome.first_round_trips);
    assert!(outcome.second_round_trips);
    assert!(outcome.rewrapped_round_trips);
}

/// Contract: the construction holds on the bn254_halo2curves pairing concrete.
/// Arrange: RoundTripProbe and the named concrete.
/// Act:     create_pairing.
/// Assert:  first_round_trips, second_round_trips, and rewrapped_round_trips.
#[test]
fn every_envelope_opens_under_its_key_pair_on_bn254_halo2curves() {
    // Arrange
    let probe = RoundTripProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bn254Halo2curves);

    // Assert
    assert!(outcome.first_round_trips);
    assert!(outcome.second_round_trips);
    assert!(outcome.rewrapped_round_trips);
}

/// Contract: the construction holds on the bls12_381_halo2curves pairing
///   concrete.
/// Arrange: RoundTripProbe and the named concrete.
/// Act:     create_pairing.
/// Assert:  first_round_trips, second_round_trips, and rewrapped_round_trips.
#[test]
fn every_envelope_opens_under_its_key_pair_on_bls12_381_halo2curves() {
    // Arrange
    let probe = RoundTripProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Halo2curves);

    // Assert
    assert!(outcome.first_round_trips);
    assert!(outcome.second_round_trips);
    assert!(outcome.rewrapped_round_trips);
}

/// Contract: an envelope opened under a key pair it was not wrapped to yields
///   neither credential component (CR-04 swapped keys).
/// Arrange: SwapProbe.
/// Act:     create_pairing.
/// Assert:  other_keys_change_the_first and other_keys_change_the_second.
#[test]
fn another_key_pair_opens_an_envelope_to_a_different_pair() {
    // Arrange
    let probe = SwapProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.other_keys_change_the_first);
    assert!(outcome.other_keys_change_the_second);
}

/// Contract: the first-group secret opens only the first-group half and the
///   second-group secret only the second.
/// Arrange: SwapProbe.
/// Act:     create_pairing.
/// Assert:  exchanged_secrets_change_the_first and
///   exchanged_secrets_change_the_second.
#[test]
fn exchanged_secrets_open_an_envelope_to_a_different_pair() {
    // Arrange
    let probe = SwapProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.exchanged_secrets_change_the_first);
    assert!(outcome.exchanged_secrets_change_the_second);
}

/// Contract: the two halves of an envelope are independent, so a substituted
///   first-group coin element corrupts only its own half (CR-04 swapped
///   elements).
/// Arrange: SwapProbe.
/// Act:     create_pairing.
/// Assert:  foreign_c1_changes_only_the_first.
#[test]
fn a_first_group_coin_element_from_another_envelope_changes_only_the_first() {
    // Arrange
    let probe = SwapProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.foreign_c1_changes_only_the_first);
}

/// Contract: the two halves of an envelope are independent, so a substituted
///   second-group coin element corrupts only its own half (CR-04 swapped
///   elements).
/// Arrange: SwapProbe.
/// Act:     create_pairing.
/// Assert:  foreign_d1_changes_only_the_second.
#[test]
fn a_second_group_coin_element_from_another_envelope_changes_only_the_second() {
    // Arrange
    let probe = SwapProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.foreign_d1_changes_only_the_second);
}

/// Contract: a generated key pair's public keys with its proof of possession
///   pass the public keys' constructor (LC-08).
/// Arrange: AdmissionProbe.
/// Act:     create_pairing.
/// Assert:  own_keys_admitted.
#[test]
fn public_keys_are_admitted_with_their_own_proof_of_possession() {
    // Arrange
    let probe = AdmissionProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.own_keys_admitted);
}

/// Contract: a proof of possession for other keys does not prove possession of
///   these (LC-08 unproven keys).
/// Arrange: AdmissionProbe.
/// Act:     create_pairing.
/// Assert:  other_possession_refusal equals
///   Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
///   PairingElGamalPublicKeysFromComponentsErrorReturn::PossessionG1)).
#[test]
fn public_keys_are_refused_with_another_key_pairs_proof_of_possession() {
    // Arrange
    let probe = AdmissionProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.other_possession_refusal,
        Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalPublicKeysFromComponentsErrorReturn::PossessionG1
        ))
    );
}

/// Contract: a possession whose second-group proof belongs to another key pair
///   fails the second-group check.
/// Arrange: AdmissionProbe.
/// Act:     create_pairing.
/// Assert:  foreign_g2_possession_refusal equals
///   Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
///   PairingElGamalPublicKeysFromComponentsErrorReturn::PossessionG2)).
#[test]
fn public_keys_are_refused_with_a_second_group_proof_from_another_key_pair() {
    // Arrange
    let probe = AdmissionProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.foreign_g2_possession_refusal,
        Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalPublicKeysFromComponentsErrorReturn::PossessionG2
        ))
    );
}

/// Contract: an identity-element key is refused (CR-04; LC-08).
/// Arrange: AdmissionProbe.
/// Act:     create_pairing.
/// Assert:  identity_first_key_refusal equals
///   Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
///   PairingElGamalPublicKeysFromComponentsErrorReturn::IdentityKeyG1)) and
///   identity_second_key_refusal equals the same with IdentityKeyG2.
#[test]
fn public_keys_are_refused_with_an_identity_key_in_either_group() {
    // Arrange
    let probe = AdmissionProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.identity_first_key_refusal,
        Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalPublicKeysFromComponentsErrorReturn::IdentityKeyG1
        ))
    );
    assert_eq!(
        outcome.identity_second_key_refusal,
        Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalPublicKeysFromComponentsErrorReturn::IdentityKeyG2
        ))
    );
}

/// Contract: keys carrying one secret in both groups are refused from the
///   public keys alone (CR-04 shared secrets).
/// Arrange: AdmissionProbe.
/// Act:     create_pairing.
/// Assert:  shared_secret_refusal equals
///   Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
///   PairingElGamalPublicKeysFromComponentsErrorReturn::SharedSecret)).
#[test]
fn public_keys_are_refused_with_a_secret_shared_across_the_groups() {
    // Arrange
    let probe = AdmissionProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.shared_secret_refusal,
        Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalPublicKeysFromComponentsErrorReturn::SharedSecret
        ))
    );
}

/// Contract: the registry's rebuild of a proof of possession is the proof.
/// Arrange: AdmissionProbe.
/// Act:     create_pairing.
/// Assert:  rebuilt_possession_admitted.
#[test]
fn public_keys_are_admitted_with_a_proof_rebuilt_from_its_components() {
    // Arrange
    let probe = AdmissionProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rebuilt_possession_admitted);
}

/// Contract: a restored key pair with one secret in both groups is refused
///   (CR-04 shared secrets).
/// Arrange: KeyPairRefusalProbe.
/// Act:     create_pairing.
/// Assert:  shared_secret_refusal equals
///   Some(KeyPairFromComponentsErrorReturn::PairingElGamal(
///   PairingElGamalKeyPairFromComponentsErrorReturn::SharedSecret)).
#[test]
fn key_pair_from_components_refuses_a_secret_shared_across_the_groups() {
    // Arrange
    let probe = KeyPairRefusalProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.shared_secret_refusal,
        Some(KeyPairFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalKeyPairFromComponentsErrorReturn::SharedSecret
        ))
    );
}

/// Contract: a zero secret, whose key is the identity, is refused on restore
///   (CR-04 identity secrets).
/// Arrange: KeyPairRefusalProbe.
/// Act:     create_pairing.
/// Assert:  zero_first_secret_refusal equals
///   Some(KeyPairFromComponentsErrorReturn::PairingElGamal(
///   PairingElGamalKeyPairFromComponentsErrorReturn::IdentityKeyG1)) and
///   zero_second_secret_refusal equals the same with IdentityKeyG2.
#[test]
fn key_pair_from_components_refuses_a_zero_secret_in_either_group() {
    // Arrange
    let probe = KeyPairRefusalProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.zero_first_secret_refusal,
        Some(KeyPairFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalKeyPairFromComponentsErrorReturn::IdentityKeyG1
        ))
    );
    assert_eq!(
        outcome.zero_second_secret_refusal,
        Some(KeyPairFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalKeyPairFromComponentsErrorReturn::IdentityKeyG2
        ))
    );
}

/// Contract: equal secrets across the groups are refused at generation (CR-04
///   shared secrets).
/// Arrange: GenerationRefusalProbe.
/// Act:     create_pairing.
/// Assert:  equal_draws_refusal equals
///   Some(GenerateKeysErrorReturn::PairingElGamal(
///   PairingElGamalGenerateKeysErrorReturn::SharedSecret)).
#[test]
fn generate_keys_refuses_equal_secret_draws() {
    // Arrange
    let probe = GenerationRefusalProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.equal_draws_refusal,
        Some(GenerateKeysErrorReturn::PairingElGamal(
            PairingElGamalGenerateKeysErrorReturn::SharedSecret
        ))
    );
}

/// Contract: a zero secret is refused at generation (CR-04 identity secrets).
/// Arrange: GenerationRefusalProbe.
/// Act:     create_pairing.
/// Assert:  zero_x_refusal equals
///   Some(GenerateKeysErrorReturn::PairingElGamal(
///   PairingElGamalGenerateKeysErrorReturn::IdentityKeyG1)) and zero_y_refusal
///   equals the same with IdentityKeyG2.
#[test]
fn generate_keys_refuses_a_zero_secret_in_either_group() {
    // Arrange
    let probe = GenerationRefusalProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.zero_x_refusal,
        Some(GenerateKeysErrorReturn::PairingElGamal(
            PairingElGamalGenerateKeysErrorReturn::IdentityKeyG1
        ))
    );
    assert_eq!(
        outcome.zero_y_refusal,
        Some(GenerateKeysErrorReturn::PairingElGamal(
            PairingElGamalGenerateKeysErrorReturn::IdentityKeyG2
        ))
    );
}

/// Contract: a sampling refusal returns in its draw's own variant unchanged.
/// Arrange: GenerationRefusalProbe.
/// Act:     create_pairing.
/// Assert:  short_x_refusal equals
///   Some(GenerateKeysErrorReturn::PairingElGamal(
///   PairingElGamalGenerateKeysErrorReturn::XSampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64,
///   actual: 32 }))) and short_y_refusal equals the same with YSampling.
#[test]
fn generate_keys_refuses_a_secret_draw_of_the_wrong_length() {
    // Arrange
    let probe = GenerationRefusalProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.short_x_refusal,
        Some(GenerateKeysErrorReturn::PairingElGamal(
            PairingElGamalGenerateKeysErrorReturn::XSampling(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 64,
                    actual: 32,
                }
            )
        ))
    );
    assert_eq!(
        outcome.short_y_refusal,
        Some(GenerateKeysErrorReturn::PairingElGamal(
            PairingElGamalGenerateKeysErrorReturn::YSampling(
                SampleUniformScalarErrorReturn::WrongLength {
                    expected: 64,
                    actual: 32,
                }
            )
        ))
    );
}

/// Contract: the components the delivery proof states hold `pk1 = x·g1` and
///   `pk2 = y·g2` for the `(x, y)` the transfer proof takes as its witnesses.
/// Arrange: ComponentsProbe.
/// Act:     create_pairing.
/// Assert:  public_keys_are_the_secrets_times_the_generators.
#[test]
fn the_public_keys_are_the_key_pairs_secrets_times_the_generators() {
    // Arrange
    let probe = ComponentsProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.public_keys_are_the_secrets_times_the_generators);
}

/// Contract: the returned `ρ` and `σ` are the delivery proof's witnesses for
///   the envelope's elements.
/// Arrange: ComponentsProbe.
/// Act:     create_pairing.
/// Assert:  first_coin_elements_are_the_coins_times_the_generators and
///   second_coin_elements_mask_the_credential.
#[test]
fn the_wrap_returns_the_coins_its_envelope_was_formed_with() {
    // Arrange
    let probe = ComponentsProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.first_coin_elements_are_the_coins_times_the_generators);
    assert!(outcome.second_coin_elements_mask_the_credential);
}

/// Contract: a holder's rebuild of its envelope from chain history is the
///   envelope.
/// Arrange: ComponentsProbe.
/// Act:     create_pairing.
/// Assert:  rebuilt_envelope_opens.
#[test]
fn an_envelope_rebuilt_from_its_components_opens() {
    // Arrange
    let probe = ComponentsProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.rebuilt_envelope_opens);
}

/// Contract: custody's restore of the envelope secrets is the key pair.
/// Arrange: ComponentsProbe.
/// Act:     create_pairing.
/// Assert:  restored_key_pair_opens.
#[test]
fn a_key_pair_restored_from_its_components_opens_the_envelope() {
    // Arrange
    let probe = ComponentsProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.restored_key_pair_opens);
}

/// Contract: each proof of possession satisfies `z·g = R + c·pk` for the
///   challenge that is the hash-to-scalar mapping, under the tag the
///   declaration carries for its group, of the encoded possession statement,
///   so a contract hashing under the mirrored tags verifies it (LC-08).
/// Arrange: ComponentsProbe.
/// Act:     create_pairing.
/// Assert:  possession_challenges_hash_under_the_declared_tags.
#[test]
fn each_proof_of_possession_hashes_its_challenge_under_the_declared_tag() {
    // Arrange
    let probe = ComponentsProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.possession_challenges_hash_under_the_declared_tags);
}

/// Contract: the concrete's declaration names the key-agreement identifier,
///   the envelope algebra, the proofs of possession's tags, its adapter
///   version, and the interface version it implements.
/// Arrange: DeclarationProbe.
/// Act:     read `PairingElGamalKeyAgreement::<'_, P, E>::DECLARATION` inside
///   the probe's encoding step.
/// Assert:  identifier matches `KeyAgreementIdentifier::PairingElGamalV1`,
///   algebra equals `EnvelopeAlgebra::PairingElGamal`, possession_g1_tag
///   equals `b"ChainTorrent-v1-envelope-possession-g1"`, possession_g2_tag
///   equals `b"ChainTorrent-v1-envelope-possession-g2"`, adapter_version
///   equals `1`, and interface_version equals `KEY_AGREEMENT_INTERFACE_VERSION`.
#[test]
fn pairing_elgamal_key_agreement_declares_its_identifier_algebra_and_versions() {
    // Arrange
    let probe = DeclarationProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert!(outcome.identifier_matches);
    assert_eq!(outcome.algebra, EnvelopeAlgebra::PairingElGamal);
    assert_eq!(outcome.possession_g1_tag, PAIRING_ELGAMAL_POSSESSION_G1_TAG);
    assert_eq!(outcome.possession_g2_tag, PAIRING_ELGAMAL_POSSESSION_G2_TAG);
    assert_eq!(outcome.adapter_version, 1);
    assert_eq!(outcome.interface_version, KEY_AGREEMENT_INTERFACE_VERSION);
}

/// Contract: an encoder refusal returns in the caller method's `Encoding`
///   variant unchanged, and the challenge's hash is never reached.
/// Arrange: EncodingRefusalProbe — a test-local encoder whose `encode` returns
///   `Err(EncodeErrorReturn::FieldCount { expected: 2, actual: 1 })` for a
///   valid typed possession statement, and a hash spy around the real adapter.
/// Act:     create_pairing.
/// Assert:  the key generation's refusal equals
///   `GenerateKeysErrorReturn::PairingElGamal(
///   PairingElGamalGenerateKeysErrorReturn::Encoding(
///   EncodeErrorReturn::FieldCount { expected: 2, actual: 1 }))`, the public
///   keys' refusal equals the same under `PublicKeysFromComponentsErrorReturn`,
///   and the hash spy records no call.
#[test]
fn encoding_refusal_propagates_from_key_generation_and_public_key_admission() {
    // Arrange
    let probe = EncodingRefusalProbe;

    // Act
    let outcome = run_probe(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    assert_eq!(
        outcome.generation_refusal,
        Some(GenerateKeysErrorReturn::PairingElGamal(
            PairingElGamalGenerateKeysErrorReturn::Encoding(EncodeErrorReturn::FieldCount {
                expected: 2,
                actual: 1,
            })
        ))
    );
    assert_eq!(
        outcome.admission_refusal,
        Some(PublicKeysFromComponentsErrorReturn::PairingElGamal(
            PairingElGamalPublicKeysFromComponentsErrorReturn::Encoding(
                EncodeErrorReturn::FieldCount {
                    expected: 2,
                    actual: 1,
                }
            )
        ))
    );
    assert_eq!(outcome.hash_calls, 0);
}
