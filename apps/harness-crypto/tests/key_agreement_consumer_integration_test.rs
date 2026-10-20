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
    DECLARED_KEY_AGREEMENT_SELECTIONS, build_create_key_agreement_deps,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IKemConsumer, build_create_kem_deps, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    PAIRING_CONCRETES, build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use proof::{
    CreateDeliveryProofReturn, CreateDeliveryProofSuccessReturn, DELIVERY_PROOF_CONCRETES,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

use harness_crypto::{
    CreateGenerateReturn, CreateGenerateSuccessReturn, DeliveryProofSelectionOverrides, EmitReturn,
    EmitSuccessReturn, GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, HarnessStatements, HarnessStatementsOverrides,
    KeyAgreementConsumerConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    VectorKind, VectorKindConfiguration, VectorKindConfigurationOverrides,
    VectorKindQuantitiesOverrides, build_delivery_proof_selection, build_generate_concrete,
    build_generate_kind_configurations, build_harness_statements, build_key_agreement_consumer,
    build_vector_count, build_vector_kind_configuration, build_vector_kind_quantities,
};

/// Contract: entry `a_kinds_configured_count_reaches_the_concrete_as_the_
///   records_its_file_holds`, carried, in this node's terms: the kinds the
///   consumer is constructed over are the kinds the delivery-proof consumer
///   it constructs hands `create_generate`, so the possession kind's count in
///   them reaches the possession file as exactly that count of lines.
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
///   `DELIVERY_PROOF_CONCRETES` and `DECLARED_KEY_AGREEMENT_SELECTIONS` and
///   every case the private surface names for the other families.
/// Act:     `create_key_agreement` from `envelope` with the subject
///   `KeyAgreementConsumer`, built by `build_key_agreement_consumer` from
///   `harness_crypto` over each member of `DELIVERY_PROOF_CONCRETES`, as the
///   deps' consumer, in the innermost consumer.
/// Assert:  every output unpacks as `Ok(CreateDeliveryProofSuccessReturn {
///   output: Ok(CreateGenerateSuccessReturn { output: Ok(EmitSuccessReturn
///   { files }) }) })` by `let … else { panic!(…) };`, the entry whose
///   `kind` is `Some(VectorKind::Possession)` is found by `iter().find` and
///   unpacked the same way, its `path` read by `std::fs::read_to_string`,
///   and the text's `lines().count()` equals `7` by `assert_eq!`.
/// Boundary: the public `create_key_agreement` from `envelope`, the subject
///   built through `harness_crypto`'s public surface, and the chain of real
///   functions the public surface names — each family's `create_*` over its
///   declared set and each selected concrete's `try_new`, the subject's
///   `consume_key_agreement`, `DeliveryProofConsumer::try_new`,
///   `create_delivery_proof`, `DeliveryProofConsumer::consume_delivery_proof`,
///   `GenerateConsumer::try_new`, `create_generate`,
///   `GenerateConsumer::consume_generate`, `emit`, and the functions `emit`
///   runs — each real; the file read from the real filesystem.
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

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = Vec<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
            let scope = payload.scope();
            let mut outputs = Vec::new();
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let delivery_proof =
                        build_delivery_proof_selection(DeliveryProofSelectionOverrides {
                            concrete: Some(*concrete),
                            ..Default::default()
                        });
                    let created = create_key_agreement(
                        &build_create_key_agreement_deps(
                            self.pairing,
                            self.hash_to_scalar,
                            self.encoder,
                            self.random,
                            build_key_agreement_consumer::<P, E, K, F>(
                                self.pairing,
                                self.hash_to_scalar,
                                self.encoder,
                                self.random,
                                &payload.adapter,
                                self.generate,
                                self.kinds,
                                self.statements,
                                KeyAgreementConsumerConstructorParamsOverrides {
                                    scope: Some(scope),
                                    delivery_proof: Some(delivery_proof),
                                },
                            ),
                            Default::default(),
                        ),
                        *selection,
                        build_create_key_agreement_payload(Default::default()),
                    );
                    let Ok(success) = created else {
                        panic!("the declared key-agreement selection is created")
                    };
                    outputs.push(success.output);
                }
            }
            outputs
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        self.hash_to_scalar,
                        KemConsumer {
                            pairing: self.pairing,
                            hash_to_scalar: self.hash_to_scalar,
                            encoder: self.encoder,
                            random: self.random,
                            generate: self.generate,
                            kinds: self.kinds,
                            statements: self.statements,
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
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                            hash_to_scalar: self.hash_to_scalar,
                            encoder: &payload.adapter,
                            random: self.random,
                            generate: self.generate,
                            kinds: self.kinds,
                            statements: self.statements,
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
        type Output = Vec<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
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
                            hash_to_scalar: &*hashing.adapter,
                            random: self.random,
                            generate: self.generate,
                            kinds: self.kinds,
                            statements: self.statements,
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
        let Ok(CreateDeliveryProofSuccessReturn {
            output:
                Ok(CreateGenerateSuccessReturn {
                    output: Ok(EmitSuccessReturn { files }),
                }),
        }) = output
        else {
            panic!("the delivery-proof factory returns the files")
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
