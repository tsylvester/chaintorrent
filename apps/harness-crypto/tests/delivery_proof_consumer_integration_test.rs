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
    IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
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
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

use harness_crypto::{
    CreateGenerateReturn, CreateGenerateSuccessReturn,
    DeliveryProofConsumerConstructorParamsOverrides, EmitReturn, EmitSuccessReturn,
    GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, HarnessStatements, HarnessStatementsOverrides,
    VectorCountConstructorParamsOverrides, VectorKind, VectorKindConfiguration,
    VectorKindConfigurationOverrides, VectorKindQuantitiesOverrides, build_delivery_proof_consumer,
    build_generate_concrete, build_generate_kind_configurations, build_harness_statements,
    build_vector_count, build_vector_kind_configuration, build_vector_kind_quantities,
};

/// Contract: entry `a_kinds_configured_count_reaches_the_concrete_as_the_
///   records_its_file_holds`, carried, in this node's terms: the kinds the
///   consumer is constructed over are the kinds `create_generate` admits
///   and sizes, so the possession kind's count in them reaches the
///   possession file as exactly that count of lines.
/// Arrange: for each member `kind` of `GENERATE_CONCRETES`, the generate
///   concrete `build_generate_concrete(kind, GenerateConcreteOverrides)` and
///   the kinds `build_generate_kind_configurations(kind,
///   GenerateKindConfigurationsOverrides)` returns, reversed, its
///   possession entry replaced by `build_vector_kind_configuration(
///   VectorKindConfigurationOverrides { kind: Some(VectorKind::Possession),
///   quantities: Some(build_vector_kind_quantities(
///   VectorKindQuantitiesOverrides { count: Some(Some(build_vector_count(
///   VectorCountConstructorParamsOverrides { count: Some(7) }))),
///   ..Default::default() })) })`, `7` other than the count builder's
///   default `2`, which every other drawn entry holds, so a size built in
///   entry order, from a default, or from kinds other than the consumer's
///   fails; the statements `build_harness_statements(
///   HarnessStatementsOverrides { identities: Some(vec![vec![0x0a; 32]]),
///   proof_identity: Some(vec![0x0a; 32]), ..Default::default() })`; the
///   random source `RandomSourceKind::OperatingSystem`; every member of
///   `GENERATE_CONCRETES` crossed with every member of
///   `DELIVERY_PROOF_CONCRETES` and every case the private surface names
///   for the other families.
/// Act:     `create_delivery_proof` from `proof` with the subject
///   `DeliveryProofConsumer`, built by `build_delivery_proof_consumer` from
///   `harness_crypto`, as the deps' consumer, in the innermost consumer.
/// Assert:  every output unpacks as `Ok(CreateGenerateSuccessReturn {
///   output: Ok(EmitSuccessReturn { files }) })` by `let … else {
///   panic!(…) };`, the entry whose `kind` is `Some(VectorKind::Possession)`
///   is found by `iter().find` and unpacked the same way, its `path` read
///   by `std::fs::read_to_string`, and the text's `lines().count()` equals
///   `7` by `assert_eq!`.
/// Boundary: the public `create_delivery_proof` from `proof`, the subject
///   built through `harness_crypto`'s public surface, and the chain of real
///   functions the public surface names — each family's `create_*` over its
///   declared set and each selected concrete's `try_new`, the subject's
///   `consume_delivery_proof`, `GenerateConsumer::try_new`,
///   `create_generate`, `GenerateConsumer::consume_generate`, `emit`, and
///   the functions `emit` runs — each real; the file read from the real
///   filesystem.
/// Mocked:  nothing; the filesystem is a directory beneath the operating
///   system's temporary directory and the operating system's generator runs
///   real.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    let statements = build_harness_statements(HarnessStatementsOverrides {
        identities: Some(vec![vec![0x0a; 32]]),
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

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        scope: IdentityScope,
        verifier_form: VerifierGroupArithmetic,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            // Act
            let mut outputs = Vec::new();
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, _>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        build_delivery_proof_consumer(
                            self.pairing,
                            self.hashing,
                            self.encoder,
                            self.random,
                            self.kem,
                            &payload.adapter,
                            self.generate,
                            self.kinds,
                            self.statements,
                            DeliveryProofConsumerConstructorParamsOverrides {
                                scope: Some(self.scope),
                                ..Default::default()
                            },
                        ),
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        verifier_form: Some(self.verifier_form),
                        ..Default::default()
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared delivery-proof concrete is created")
                };
                outputs.push(success.output);
            }
            outputs
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        verifier_form: VerifierGroupArithmetic,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

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
                            generate: self.generate,
                            kinds: self.kinds,
                            statements: self.statements,
                            scope,
                            verifier_form: self.verifier_form,
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
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        verifier_form: VerifierGroupArithmetic,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

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
                            generate: self.generate,
                            kinds: self.kinds,
                            statements: self.statements,
                            verifier_form: self.verifier_form,
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
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        verifier_form: VerifierGroupArithmetic,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

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
                            generate: self.generate,
                            kinds: self.kinds,
                            statements: self.statements,
                            verifier_form: self.verifier_form,
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
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

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
                            generate: self.generate,
                            kinds: self.kinds,
                            statements: self.statements,
                            verifier_form,
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
        let generate = build_generate_concrete(*member, GenerateConcreteOverrides);
        let mut kinds =
            build_generate_kind_configurations(*member, GenerateKindConfigurationsOverrides);
        kinds.reverse();
        for entry in kinds.iter_mut() {
            if entry.kind == VectorKind::Possession {
                *entry = build_vector_kind_configuration(VectorKindConfigurationOverrides {
                    kind: Some(VectorKind::Possession),
                    quantities: Some(build_vector_kind_quantities(
                        VectorKindQuantitiesOverrides {
                            count: Some(Some(build_vector_count(
                                VectorCountConstructorParamsOverrides { count: Some(7) },
                            ))),
                            ..Default::default()
                        },
                    )),
                });
            }
        }
        let consumer = PairingConsumer {
            random: &*source,
            generate: &generate,
            kinds: &kinds,
            statements: &statements,
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
        let Ok(CreateGenerateSuccessReturn {
            output: Ok(EmitSuccessReturn { files }),
        }) = output
        else {
            panic!("emit returns the files")
        };
        let Some(file) = files
            .iter()
            .find(|file| file.kind == Some(VectorKind::Possession))
        else {
            panic!("the emitted files hold the possession kind")
        };
        let text = std::fs::read_to_string(&file.path).expect("the possession file reads");
        assert_eq!(text.lines().count(), 7);
    }
    // a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
}
