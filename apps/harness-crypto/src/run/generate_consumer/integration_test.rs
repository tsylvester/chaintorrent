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

use super::mock::build_generate_consumer;
use crate::config::provides::{HarnessStatements, build_harness_statements};
use crate::generate::provides::{
    CreateGenerateParamsOverrides, EmitErrorReturn, EmitReturn, EmitSuccessReturn,
    GENERATE_CONCRETES, GenerateConcrete, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, VectorCountConstructorParamsOverrides, VectorKind,
    VectorKindConfigurationOverrides, VectorKindQuantitiesOverrides, build_create_generate_deps,
    build_create_generate_params, build_create_generate_payload, build_generate_concrete,
    build_generate_kind_configurations, build_vector_count, build_vector_kind_configuration,
    build_vector_kind_quantities, create_generate,
};

/// Contract: entry `a_refused_write_returns_the_mock_concretes_own_error`,
///   absorbed: under `MockIGenerateAdapterFailureMode::WriteRefused` the
///   consumer's output is `Err(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused))`, the error whole.
/// Arrange: the statements `build_harness_statements(Default::default())`;
///   the selection `GenerateConcrete::Mock(
///   MockIGenerateAdapterFailureMode::WriteRefused)` with the kinds
///   `build_create_generate_params` defaults to; the random source
///   `RandomSourceKind::OperatingSystem`; every case of the other families
///   and no case of `GENERATE_CONCRETES`.
/// Act:     `create_generate` with the subject as the deps' consumer, in
///   the innermost consumer.
/// Assert:  every output unpacks as `Err(EmitErrorReturn::Mock(error))`
///   and `error` equals `MockIGenerateAdapterEmitErrorReturn::WriteRefused`
///   by `assert_eq!`.
/// Boundary: `create_generate`, the mock concrete's `try_new`, the
///   subject's `consume_generate`, and `emit`, each real.
/// Mocked:  the generate family's mock concrete under its failure mode,
///   which the declared set cannot reach.
#[test]
fn a_refused_write_returns_the_mock_concretes_own_error() {
    // Arrange
    let statements = build_harness_statements(Default::default());
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
                    concrete: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::WriteRefused,
                    )),
                    scope: Some(self.scope),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            );
            let success = match created {
                Ok(success) => success,
                Err(error) => panic!("the generate factory admits the selection: {error:?}"),
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the declared delivery-proof concrete is created: {error:?}")
                    }
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
        let error = match output {
            Err(EmitErrorReturn::Mock(error)) => error,
            other => panic!("emit refuses the write: {other:?}"),
        };
        assert_eq!(error, MockIGenerateAdapterEmitErrorReturn::WriteRefused);
    }
    // a_refused_write_returns_the_mock_concretes_own_error
}

/// Contract: entry
///   `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_
///   holds`, absorbed: the possession kind's configured count reaches,
///   through the consumer's `emit`, the possession file as exactly that
///   count of lines.
/// Arrange: the statements `build_harness_statements(Default::default())`;
///   for each member `kind` of `GENERATE_CONCRETES`, the selection
///   `build_generate_concrete(kind, GenerateConcreteOverrides)`, and the
///   kinds
///   `build_generate_kind_configurations` returns, reversed, the possession
///   entry replaced by a configuration of `VectorKind::Possession` whose
///   `count` is `build_vector_count` of `7`, other than the count builder's
///   default `2` which every other drawn entry holds, so a size built in
///   entry order or from a default fails; the random source
///   `RandomSourceKind::OperatingSystem`; every case the private surface
///   names.
/// Act:     `create_generate` with the subject as the deps' consumer, in
///   the innermost consumer.
/// Assert:  every output unpacks as `Ok(EmitSuccessReturn { files })`, the
///   entry whose `kind` is `Some(VectorKind::Possession)` is found by
///   `iter().find`, its file read by `std::fs::read_to_string`, and the
///   text's `lines().count()` equals `7` by `assert_eq!`.
/// Boundary: `build_generate_concrete`, `create_generate`, the selected
///   concrete's `try_new`, the subject's `consume_generate`, `emit`, and
///   the functions `emit` runs, each real; the file read from the real
///   filesystem.
/// Mocked:  nothing on the chain; the filesystem is a directory beneath
///   the operating system's temporary directory and the operating system's
///   generator runs real.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    let statements = build_harness_statements(Default::default());
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
            // Act
            let mut outputs = Vec::new();
            for member in GENERATE_CONCRETES {
                let concrete = build_generate_concrete(*member, GenerateConcreteOverrides);
                let mut kinds = build_generate_kind_configurations(
                    *member,
                    GenerateKindConfigurationsOverrides,
                );
                kinds.reverse();
                for entry in kinds.iter_mut() {
                    if entry.kind == VectorKind::Possession {
                        *entry =
                            build_vector_kind_configuration(VectorKindConfigurationOverrides {
                                kind: Some(VectorKind::Possession),
                                quantities: Some(build_vector_kind_quantities(
                                    VectorKindQuantitiesOverrides {
                                        count: Some(Some(build_vector_count(
                                            VectorCountConstructorParamsOverrides {
                                                count: Some(7),
                                            },
                                        ))),
                                        ..Default::default()
                                    },
                                )),
                            });
                    }
                }
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
                        kinds: Some(kinds),
                        scope: Some(self.scope),
                        ..Default::default()
                    }),
                    build_create_generate_payload(Default::default()),
                );
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the generate factory admits the selection: {error:?}")
                    }
                };
                outputs.push(success.output);
            }
            outputs
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
                let success = match created {
                    Ok(success) => success,
                    Err(error) => {
                        panic!("the declared delivery-proof concrete is created: {error:?}")
                    }
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
        let EmitSuccessReturn { files } = match output {
            Ok(success) => success,
            Err(error) => panic!("emit returns the files: {error:?}"),
        };
        let Some(file) = files
            .iter()
            .find(|file| file.kind == Some(VectorKind::Possession))
        else {
            panic!("the emitted files hold the possession kind: {files:?}")
        };
        let text = std::fs::read_to_string(&file.path).expect("the possession file reads");
        assert_eq!(text.lines().count(), 7);
    }
    // a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
}
