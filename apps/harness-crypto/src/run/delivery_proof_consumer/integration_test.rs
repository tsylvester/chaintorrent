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

use super::mock::{DeliveryProofConsumerConstructorParamsOverrides, build_delivery_proof_consumer};
use crate::config::provides::{
    HarnessStatements, HarnessStatementsOverrides, build_harness_statements,
};
use crate::generate::provides::{
    CreateGenerateReturn, CreateGenerateSuccessReturn, EmitErrorReturn, EmitReturn,
    EmitSuccessReturn, GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, VectorKind, VectorKindConfiguration,
    build_create_generate_params, build_generate_concrete, build_generate_kind_configurations,
};

/// Contract: entry `a_refused_write_returns_the_mock_concretes_own_error`,
///   absorbed: with the consumer's configured selection
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`,
///   the consumer's output is `Ok(CreateGenerateSuccessReturn { output:
///   Err(EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::
///   WriteRefused)) })`, the error whole.
/// Arrange: the generate concrete `GenerateConcrete::Mock(
///   MockIGenerateAdapterFailureMode::WriteRefused)` with the kinds
///   `build_create_generate_params(Default::default()).kinds`, and the
///   statements `build_harness_statements(Default::default())`, which the
///   mock concrete does not read; the random source
///   `RandomSourceKind::OperatingSystem`; every case of the other families
///   and no case of `GENERATE_CONCRETES`.
/// Act:     `create_delivery_proof` with the subject as the deps' consumer,
///   in the innermost consumer.
/// Assert:  every output unpacks as `Ok(CreateGenerateSuccessReturn {
///   output: Err(EmitErrorReturn::Mock(error)) })` by `let … else {
///   panic!(…) };` and `error` equals
///   `MockIGenerateAdapterEmitErrorReturn::WriteRefused` by `assert_eq!`.
/// Boundary: the chain of real functions the private surface names — each
///   family's `create_*` over its declared set, each selected concrete's
///   `try_new`, the subject's `consume_delivery_proof`,
///   `GenerateConsumer::try_new`, `create_generate`, the mock concrete's
///   `try_new`, `GenerateConsumer::consume_generate`, `emit`, and the
///   functions `emit` runs, each real.
/// Mocked:  the generate family's mock concrete under its failure mode,
///   which the declared set cannot reach; the filesystem beneath the
///   operating system's temporary directory and the operating system's
///   generator are the outer edges.
#[test]
fn a_refused_write_returns_the_mock_concretes_own_error() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused);
    let kinds = build_create_generate_params(Default::default()).kinds;
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the declared delivery-proof concrete is created: {error:?}")
                    }
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the declared key-agreement selection is created: {error:?}")
                    }
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => panic!("the declared kem selection is created: {error:?}"),
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the declared chain-forms concrete is created: {error:?}")
                    }
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
                let hashing = match create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) {
                    Ok(hashing) => hashing,
                    Err(error) => {
                        panic!("the declared hash-to-scalar selection is created: {error:?}")
                    }
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
                    let success = match created {
                        Ok(success) => success,
                        Err(error) => {
                            panic!("the declared encoding selection is created: {error:?}")
                        }
                    };
                    outputs.extend(success.output);
                }
            }
            outputs
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        generate: &generate,
        kinds: &kinds,
        statements: &statements,
    };

    let mut outputs = Vec::new();
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        let success = match created {
            Ok(success) => success,
            Err(error) => panic!("the pairing factory ran the consumer: {error:?}"),
        };
        outputs.extend(success.output);
    }

    // Assert
    assert!(!outputs.is_empty(), "every named case ran");
    for output in outputs {
        let Ok(CreateGenerateSuccessReturn {
            output: Err(EmitErrorReturn::Mock(error)),
        }) = output
        else {
            panic!("the generate concrete's refusal reaches the output")
        };
        assert_eq!(error, MockIGenerateAdapterEmitErrorReturn::WriteRefused);
    }
    // a_refused_write_returns_the_mock_concretes_own_error
}

/// Contract: entry `the_configured_identities_reach_the_identity_mapping_file`,
///   absorbed, in this node's terms: the statements the consumer is
///   constructed over, holding three distinct identities, reach through the
///   generate consumer it constructs the identity-mapping file as exactly
///   three lines.
/// Arrange: for each member `kind` of `GENERATE_CONCRETES`, the selection
///   `build_generate_concrete(kind, GenerateConcreteOverrides)`, whose root
///   is a directory beneath the operating system's temporary directory, and
///   the kinds `build_generate_kind_configurations(kind,
///   GenerateKindConfigurationsOverrides)`; the statements
///   `build_harness_statements(HarnessStatementsOverrides { identities:
///   Some(vec![vec![0x0a; 32], vec![0x0b; 32], vec![0x0c; 32]]),
///   proof_identity: Some(vec![0x0a; 32]), ..Default::default() })`, three
///   distinct identities where the builder's default holds one, every
///   identity 32 bytes long and nonzero so every case of
///   `DECLARED_KEM_SELECTIONS`, the asset scope included, admits them; the
///   random source `RandomSourceKind::OperatingSystem`; every case the
///   private surface names.
/// Act:     `create_delivery_proof` with the subject as the deps' consumer,
///   in the innermost consumer.
/// Assert:  every output unpacks as `Ok(CreateGenerateSuccessReturn {
///   output: Ok(EmitSuccessReturn { files }) })`, the entry whose `kind` is
///   `Some(VectorKind::IdentityMapping)` is found by `iter().find` and
///   unpacked by `let … else { panic!(…) };`, its `path` is read by
///   `std::fs::read_to_string`, and the text's `lines().count()` equals `3`
///   by `assert_eq!`.
/// Boundary: the chain of real functions the private surface names — each
///   family's `create_*` over its declared set, each selected concrete's
///   `try_new`, the subject's `consume_delivery_proof`,
///   `GenerateConsumer::try_new`, `create_generate`, the selected concrete's
///   `try_new`, `GenerateConsumer::consume_generate`, `emit`, and the
///   functions `emit` runs, each real; the file read from the real
///   filesystem.
/// Mocked:  nothing on the chain; the filesystem is a directory beneath the
///   operating system's temporary directory and the operating system's
///   generator runs real.
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

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
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
            for member in GENERATE_CONCRETES {
                let generate = build_generate_concrete(*member, GenerateConcreteOverrides);
                let kinds = build_generate_kind_configurations(
                    *member,
                    GenerateKindConfigurationsOverrides,
                );
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
                                &generate,
                                &kinds,
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
                    let success = match created {
                        Ok(success) => success,
                        Err(error) => {
                            panic!("the declared delivery-proof concrete is created: {error:?}")
                        }
                    };
                    outputs.push(success.output);
                }
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the declared key-agreement selection is created: {error:?}")
                    }
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
                            statements: self.statements,
                            verifier_form: self.verifier_form,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                let success = match created {
                    Ok(success) => success,
                    Err(error) => panic!("the declared kem selection is created: {error:?}"),
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the declared chain-forms concrete is created: {error:?}")
                    }
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
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
                let hashing = match create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) {
                    Ok(hashing) => hashing,
                    Err(error) => {
                        panic!("the declared hash-to-scalar selection is created: {error:?}")
                    }
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &payload.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            statements: self.statements,
                            verifier_form,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    let success = match created {
                        Ok(success) => success,
                        Err(error) => {
                            panic!("the declared encoding selection is created: {error:?}")
                        }
                    };
                    outputs.extend(success.output);
                }
            }
            outputs
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        statements: &statements,
    };

    let mut outputs = Vec::new();
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        let success = match created {
            Ok(success) => success,
            Err(error) => panic!("the pairing factory ran the consumer: {error:?}"),
        };
        outputs.extend(success.output);
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
            .find(|file| file.kind == Some(VectorKind::IdentityMapping))
        else {
            panic!("the emitted files hold the identity-mapping kind")
        };
        let text = std::fs::read_to_string(&file.path).expect("the identity-mapping file reads");
        assert_eq!(text.lines().count(), 3);
    }
    // the_configured_identities_reach_the_identity_mapping_file
}
