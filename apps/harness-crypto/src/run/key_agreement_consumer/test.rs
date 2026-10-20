#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;
use std::path::PathBuf;

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::{
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, ICredentialKemAdapter, IKemConsumer,
    build_create_kem_deps, build_create_kem_params, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use proof::{
    CreateDeliveryProofErrorReturn, CreateDeliveryProofReturn, CreateDeliveryProofSuccessReturn,
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryProofConcrete, MockIDeliveryProofAdapterFailureMode,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

use super::mock::{KeyAgreementConsumerConstructorParamsOverrides, build_key_agreement_consumer};
use crate::config::provides::{
    DeliveryProofSelection, DeliveryProofSelectionOverrides, HarnessStatements,
    build_delivery_proof_selection, build_harness_statements,
};
use crate::generate::provides::{
    CreateGenerateErrorReturn, CreateGenerateReturn, CreateGenerateSuccessReturn, EmitErrorReturn,
    EmitReturn, EmitSuccessReturn, EmittedFileOverrides, GenerateConcrete,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode, VectorKind,
    VectorKindConfiguration, VectorKindConfigurationOverrides, build_create_generate_params,
    build_emitted_file, build_vector_kind_configuration,
};

/// Contract: admitted: `create_delivery_proof` returns `Ok(success)` whose
///   `output` is `create_generate`'s success holding the files `emit` wrote →
///   the output is `Ok(success)`, unchanged.
/// Arrange: the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`,
///   the kinds `build_create_generate_params(Default::default()).kinds`, the
///   mock declaration's kinds in declared order, the statements
///   `build_harness_statements(Default::default())`, and the selection
///   `build_delivery_proof_selection` over the delivery proof family's mock
///   concrete under `NoFailure`, its version the builder's default
///   `DELIVERY_STATEMENT_VERSION_ONE`, which the family's mock concrete
///   declares.
/// Act:     `create_key_agreement` with the subject as the deps' consumer,
///   called in the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateDeliveryProofSuccessReturn { output:
///   Ok(CreateGenerateSuccessReturn { output: Ok(EmitSuccessReturn { files })
///   }) })` by `let … else { panic!(…) };` and `files` equals, in order, an
///   `EmittedFile` per declared kind, each `path` its kind's name, by
///   `assert_eq!`, so an output emptied, reordered, or not taken from
///   `create_delivery_proof` fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories; the generate
///   family's mock concrete under `NoFailure`; the subject
///   `KeyAgreementConsumer` in the deps.
#[test]
fn consume_key_agreement_returns_the_files_the_delivery_proof_factory_returns() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let selection = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
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
                            scope: Some(payload.scope()),
                            delivery_proof: Some(*self.selection),
                        },
                    ),
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory ran the subject")
            };
            success.output
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
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                        selection: self.selection,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the kem factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
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
                        selection: self.selection,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer<'a> {
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*source.adapter,
                    generate: self.generate,
                    kinds: self.kinds,
                    statements: self.statements,
                    selection: self.selection,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the encoding factory admits the selection")
            };
            success.output
        }
    }

    let consumer = PairingConsumer {
        generate: &generate,
        kinds: &kinds,
        statements: &statements,
        selection: &selection,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let output = success.output;

    // Assert
    let Ok(CreateDeliveryProofSuccessReturn {
        output:
            Ok(CreateGenerateSuccessReturn {
                output: Ok(EmitSuccessReturn { files }),
            }),
    }) = output
    else {
        panic!("the delivery-proof factory returns the files")
    };
    assert_eq!(
        files,
        vec![
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::G1Addition)),
                path: Some(PathBuf::from("G1Addition")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::G1MultiScalarMultiplication)),
                path: Some(PathBuf::from("G1MultiScalarMultiplication")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::HashToScalar)),
                path: Some(PathBuf::from("HashToScalar")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::DecodeRejection)),
                path: Some(PathBuf::from("DecodeRejection")),
            }),
            build_emitted_file(EmittedFileOverrides {
                kind: Some(Some(VectorKind::IdentityMapping)),
                path: Some(PathBuf::from("IdentityMapping")),
            }),
        ]
    );
    // consume_key_agreement_returns_the_files_the_delivery_proof_factory_returns
}

/// Contract: admitted: `create_delivery_proof` returns `Ok(success)` whose
///   `output` is `create_generate`'s success holding the generate concrete's
///   refusal → the output is `Ok(success)`, unchanged.
/// Arrange: the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`, a
///   mode other than `NoFailure`, so a consumer that constructs its
///   delivery-proof consumer over a selection other than its own fails, and
///   the kinds, statements, and selection of the block above.
/// Act:     `create_key_agreement` with the subject as the deps' consumer,
///   called in the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateDeliveryProofSuccessReturn { output:
///   Ok(CreateGenerateSuccessReturn { output: Err(EmitErrorReturn::Mock(error))
///   }) })` by `let … else { panic!(…) };` and `error` equals
///   `MockIGenerateAdapterEmitErrorReturn::WriteRefused` by `assert_eq!`.
/// Collaborators: as
///   `consume_key_agreement_returns_the_files_the_delivery_proof_factory_returns`;
///   the generate family's mock concrete under `WriteRefused`.
#[test]
fn consume_key_agreement_returns_the_emit_refusal_unchanged() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let selection = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
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
                            scope: Some(payload.scope()),
                            delivery_proof: Some(*self.selection),
                        },
                    ),
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory ran the subject")
            };
            success.output
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
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                        selection: self.selection,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the kem factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
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
                        selection: self.selection,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer<'a> {
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*source.adapter,
                    generate: self.generate,
                    kinds: self.kinds,
                    statements: self.statements,
                    selection: self.selection,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the encoding factory admits the selection")
            };
            success.output
        }
    }

    let consumer = PairingConsumer {
        generate: &generate,
        kinds: &kinds,
        statements: &statements,
        selection: &selection,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let output = success.output;

    // Assert
    let Ok(CreateDeliveryProofSuccessReturn {
        output:
            Ok(CreateGenerateSuccessReturn {
                output: Err(EmitErrorReturn::Mock(error)),
            }),
    }) = output
    else {
        panic!("the delivery-proof factory returns the emit refusal")
    };
    assert_eq!(error, MockIGenerateAdapterEmitErrorReturn::WriteRefused);
    // consume_key_agreement_returns_the_emit_refusal_unchanged
}

/// Contract: admitted: `create_delivery_proof` returns `Ok(success)` whose
///   `output` is `create_generate`'s refusal → the output is `Ok(success)`,
///   unchanged.
/// Arrange: the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`, the
///   kinds `build_create_generate_params(Default::default()).kinds` followed by
///   `build_vector_kind_configuration` over `VectorKind::Possession`, a kind
///   the mock declaration does not name, so a consumer that constructs its
///   delivery-proof consumer over any kinds but its own is admitted and
///   fails, and the statements and selection of the first block.
/// Act:     `create_key_agreement` with the subject as the deps' consumer,
///   called in the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateDeliveryProofSuccessReturn { output: Err(error) })` by
///   `let … else { panic!(…) };` and `error` equals
///   `CreateGenerateErrorReturn::UndeclaredKind { kind: VectorKind::Possession
///   }` by `assert_eq!`.
/// Collaborators: as
///   `consume_key_agreement_returns_the_files_the_delivery_proof_factory_returns`.
#[test]
fn consume_key_agreement_returns_the_generate_factorys_refusal_unchanged() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let mut kinds = build_create_generate_params(Default::default()).kinds;
    kinds.push(build_vector_kind_configuration(
        VectorKindConfigurationOverrides {
            kind: Some(VectorKind::Possession),
            ..Default::default()
        },
    ));
    let selection = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
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
                            scope: Some(payload.scope()),
                            delivery_proof: Some(*self.selection),
                        },
                    ),
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory ran the subject")
            };
            success.output
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
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                        selection: self.selection,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the kem factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
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
                        selection: self.selection,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer<'a> {
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*source.adapter,
                    generate: self.generate,
                    kinds: self.kinds,
                    statements: self.statements,
                    selection: self.selection,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the encoding factory admits the selection")
            };
            success.output
        }
    }

    let consumer = PairingConsumer {
        generate: &generate,
        kinds: &kinds,
        statements: &statements,
        selection: &selection,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let output = success.output;

    // Assert
    let Ok(CreateDeliveryProofSuccessReturn { output: Err(error) }) = output else {
        panic!("the delivery-proof factory returns the generate factory's refusal")
    };
    assert_eq!(
        error,
        CreateGenerateErrorReturn::UndeclaredKind {
            kind: VectorKind::Possession,
        }
    );
    // consume_key_agreement_returns_the_generate_factorys_refusal_unchanged
}

/// Contract: refused: `create_delivery_proof` returns `Err(error)` → the
///   output is `Err(error)`, unchanged.
/// Arrange: the generate concrete, kinds, and statements of the first block
///   and the selection `build_delivery_proof_selection` over the delivery
///   proof family's mock concrete under `NoFailure` with
///   `statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE + 1)`, a version
///   the family's mock concrete does not declare where the builder's default
///   is one it declares, so a consumer that hands the factory any version but
///   its own is admitted and fails.
/// Act:     `create_key_agreement` with the subject as the deps' consumer,
///   called in the innermost consumer's method.
/// Assert:  the output unpacks as `Err(error)` by
///   `let … else { panic!(…) };` and `error` equals
///   `CreateDeliveryProofErrorReturn::UnsupportedStatementVersion` by
///   `assert_eq!`.
/// Collaborators: as
///   `consume_key_agreement_returns_the_files_the_delivery_proof_factory_returns`.
#[test]
fn consume_key_agreement_returns_the_delivery_proof_factorys_refusal_unchanged() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let selection = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE + 1),
    });

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            // Act
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
                            scope: Some(payload.scope()),
                            delivery_proof: Some(*self.selection),
                        },
                    ),
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory ran the subject")
            };
            success.output
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
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                        selection: self.selection,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_kem_params(Default::default()),
                build_create_kem_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the kem factory admits the selection")
            };
            success.output
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
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
                        selection: self.selection,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the chain-forms factory admits the selection")
            };
            success.output
        }
    }

    struct PairingConsumer<'a> {
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        selection: &'a DeliveryProofSelection,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            );
            let Ok(hashing) = hashing else {
                panic!("the hash-to-scalar factory admits the selection")
            };
            let Ok(source) = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            );
            let created = create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*source.adapter,
                    generate: self.generate,
                    kinds: self.kinds,
                    statements: self.statements,
                    selection: self.selection,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the encoding factory admits the selection")
            };
            success.output
        }
    }

    let consumer = PairingConsumer {
        generate: &generate,
        kinds: &kinds,
        statements: &statements,
        selection: &selection,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let output = success.output;

    // Assert
    let Err(error) = output else {
        panic!("the delivery-proof factory refuses the version")
    };
    assert_eq!(
        error,
        CreateDeliveryProofErrorReturn::UnsupportedStatementVersion
    );
    // consume_key_agreement_returns_the_delivery_proof_factorys_refusal_unchanged
}
