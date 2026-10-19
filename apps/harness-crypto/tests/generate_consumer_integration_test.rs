#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;

use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, DECLARED_KEY_AGREEMENT_SELECTIONS,
    EnvelopeAlgebra, IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IKemConsumer, IdentityScope, build_create_kem_deps, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    PAIRING_CONCRETES, VerifierGroupArithmetic, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParamsOverrides,
    DELIVERY_PROOF_CONCRETES, DELIVERY_STATEMENT_VERSION_ONE, IDeliveryProofAdapter,
    IDeliveryProofConsumer, build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

use harness_crypto::{
    CreateGenerateParamsOverrides, EmitReturn, EmitSuccessReturn, GENERATE_CONCRETES,
    GenerateConcreteKind, GenerateConcreteOverrides, GenerateKindConfigurationsOverrides,
    HarnessStatements, HarnessStatementsOverrides, VectorKind, build_create_generate_deps,
    build_create_generate_params, build_create_generate_payload, build_generate_concrete,
    build_generate_consumer, build_generate_kind_configurations, build_harness_statements,
    create_generate,
};

/// Contract: entry `the_configured_identities_reach_the_identity_mapping_
///   file`; `create_generate`, over the selection `build_generate_concrete`
///   returns for a member of `GENERATE_CONCRETES` and the kinds
///   `build_generate_kind_configurations` returns for it, hands its adapter
///   to the consumer constructed over statements whose `identities` hold
///   three distinct identities → `Ok` whose one `EmittedFile` of `kind`
///   `Some(VectorKind::IdentityMapping)` names a file holding exactly three
///   lines.
/// Arrange: the statements `build_harness_statements` with `identities`
///   `vec![vec![0x0a; 32], vec![0x0b; 32], vec![0x0c; 32]]` and
///   `proof_identity` `vec![0x0a; 32]`, three distinct identities where
///   the builder's default and its proof identity hold one, so a
///   consumer that hands `emit` the builder's identities, the proof
///   identity in their place, or a shortened list fails; the random source
///   `RandomSourceKind::OperatingSystem`; every member of
///   `GENERATE_CONCRETES` crossed with every case the private surface
///   names for the other families.
/// Act:     `create_generate` from `harness_crypto` with the subject
///   `GenerateConsumer`, built by `build_generate_consumer`, as the deps'
///   consumer, in the innermost consumer.
/// Assert:  every output unpacks as `Ok(EmitSuccessReturn { files })`, the
///   entry whose `kind` is `Some(VectorKind::IdentityMapping)` found by
///   `iter().find`, its `path` read by `std::fs::read_to_string`, and the
///   text's `lines().count()` equals `3` by `assert_eq!`.
/// Boundary: `harness_crypto`'s `create_generate`, the selected concrete's
///   `try_new`, the subject's `consume_generate`, `emit`, and the
///   functions `emit` runs, each real, with the pairing, hash-to-scalar,
///   encoding, chain-forms, KEM, key-agreement, and delivery-proof
///   factories, each reached through its crate's public surface.
/// Mocked:  nothing; the filesystem is a directory beneath the operating
///   system's temporary directory and the operating system's generator
///   runs real.
#[test]
fn the_configured_identities_reach_the_identity_mapping_file() {
    // Arrange
    let statements = build_harness_statements(HarnessStatementsOverrides {
        identities: Some(vec![vec![0x0a; 32], vec![0x0b; 32], vec![0x0c; 32]]),
        proof_identity: Some(vec![0x0a; 32]),
        ..Default::default()
    });
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        statements: &'a HarnessStatements,
        scope: IdentityScope,
        kind: GenerateConcreteKind,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = Vec<EmitReturn>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let concrete = build_generate_concrete(self.kind, GenerateConcreteOverrides);

            // Act
            let created = create_generate(
                &build_create_generate_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    build_generate_consumer(self.statements, Default::default()),
                    Default::default(),
                ),
                build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(concrete),
                    kinds: Some(build_generate_kind_configurations(
                        self.kind,
                        GenerateKindConfigurationsOverrides,
                    )),
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the generate factory admits the selection")
            };
            vec![success.output]
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        statements: &'a HarnessStatements,
        scope: IdentityScope,
        verifier_form: VerifierGroupArithmetic,
        kind: GenerateConcreteKind,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = Vec<EmitReturn>;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            statements: self.statements,
                            scope: self.scope,
                            kind: self.kind,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared delivery-proof concrete is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        verifier_form: VerifierGroupArithmetic,
        kind: GenerateConcreteKind,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = Vec<EmitReturn>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
            let mut outputs = Vec::new();
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: &payload.adapter,
                            statements: self.statements,
                            scope,
                            verifier_form: self.verifier_form,
                            kind: self.kind,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared key-agreement selection is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        verifier_form: VerifierGroupArithmetic,
        kind: GenerateConcreteKind,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<EmitReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                let created = create_kem(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            statements: self.statements,
                            verifier_form: self.verifier_form,
                            kind: self.kind,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared kem selection is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        verifier_form: VerifierGroupArithmetic,
        kind: GenerateConcreteKind,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<EmitReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            statements: self.statements,
                            verifier_form: self.verifier_form,
                            kind: self.kind,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared chain-forms concrete is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        statements: &'a HarnessStatements,
        kind: GenerateConcreteKind,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = Vec<EmitReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            let mut outputs = Vec::new();
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &payload.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            statements: self.statements,
                            verifier_form,
                            kind: self.kind,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    let Ok(success) = created else {
                        panic!("the declared encoding selection is created")
                    };
                    outputs.extend(success.output);
                }
            }
            outputs
        }
    }

    let mut outputs = Vec::new();
    for member in GENERATE_CONCRETES {
        let consumer = PairingConsumer {
            random: &*source,
            statements: &statements,
            kind: *member,
        };
        for pairing_member in PAIRING_CONCRETES {
            let created = create_pairing(
                &build_create_pairing_deps(&consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_member),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );
            let Ok(success) = created else {
                panic!("the pairing factory ran the consumer")
            };
            outputs.extend(success.output);
        }
    }

    // Assert
    assert!(!outputs.is_empty(), "every named case ran");
    for output in outputs {
        let Ok(EmitSuccessReturn { files }) = output else {
            panic!("emit returns the files")
        };
        let Some(file) = files
            .iter()
            .find(|file| file.kind == Some(VectorKind::IdentityMapping))
        else {
            panic!("the emitted files hold the identity-mapping kind")
        };
        let text = std::fs::read_to_string(&file.path).expect("the identity-mapping file reads");
        assert_eq!(text.lines().count(), 3);
    }
    // the_configured_identities_reach_the_identity_mapping_file
}
