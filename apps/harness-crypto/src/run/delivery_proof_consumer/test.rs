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
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, IKeyAgreementAdapter,
    IKeyAgreementConsumer, build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    build_create_kem_deps, build_create_kem_params, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, IPairingArithmetic, IPairingConsumer,
    IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    build_create_pairing_payload, create_pairing,
};
use proof::{
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

use super::mock::{DeliveryProofConsumerConstructorParamsOverrides, build_delivery_proof_consumer};
use crate::config::provides::{HarnessStatements, build_harness_statements};
use crate::generate::provides::{
    CreateGenerateErrorReturn, CreateGenerateReturn, CreateGenerateSuccessReturn, EmitErrorReturn,
    EmitReturn, EmitSuccessReturn, EmittedFileOverrides, GenerateConcrete,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode, VectorKind,
    VectorKindConfiguration, VectorKindConfigurationOverrides, build_create_generate_params,
    build_emitted_file, build_vector_kind_configuration,
};

/// Contract: generated: `create_generate` returns `Ok(success)` whose
///   `output` is `Ok`, the files `emit` wrote → the output is `Ok(success)`,
///   unchanged.
/// Arrange: the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`,
///   the kinds `build_create_generate_params(Default::default()).kinds`, the
///   mock declaration's kinds in declared order, and the statements
///   `build_harness_statements(Default::default())`.
/// Act:     `create_delivery_proof` with the subject as the deps' consumer,
///   called in the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateGenerateSuccessReturn { output: Ok(EmitSuccessReturn { files }) })`
///   by `let … else { panic!(…) };` and `files` equals, in order, an
///   `EmittedFile` per declared kind, each `path` its kind's name, by
///   `assert_eq!`, so an output emptied, reordered, or not taken from
///   `create_generate` fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories; the generate
///   family's mock concrete under `NoFailure`; the subject
///   `DeliveryProofConsumer` in the deps.
#[test]
fn consume_delivery_proof_returns_the_files_the_generate_factory_returns() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;

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
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            // Act
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
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory ran the subject")
            };
            success.output
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
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
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
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
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
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
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
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateGenerateReturn<EmitReturn>;

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
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    generate: self.generate,
                    kinds: self.kinds,
                    statements: self.statements,
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
    let Ok(CreateGenerateSuccessReturn {
        output: Ok(EmitSuccessReturn { files }),
    }) = output
    else {
        panic!("the generate factory returns the files")
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
    // consume_delivery_proof_returns_the_files_the_generate_factory_returns
}

/// Contract: emission refused: `create_generate` returns `Ok(success)` whose
///   `output` is `Err`, the generate concrete's refusal → the output is
///   `Ok(success)`, unchanged, the refusal whole inside it.
/// Arrange: the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`,
///   a mode other than `NoFailure`, so a consumer that hands `create_generate`
///   a selection other than its own fails, and the kinds and statements of
///   the block above.
/// Act:     `create_delivery_proof` with the subject as the deps' consumer,
///   called in the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateGenerateSuccessReturn { output: Err(EmitErrorReturn::Mock(error)) })`
///   by `let … else { panic!(…) };` and `error` equals
///   `MockIGenerateAdapterEmitErrorReturn::WriteRefused` by `assert_eq!`.
/// Collaborators: as
///   `consume_delivery_proof_returns_the_files_the_generate_factory_returns`;
///   the generate family's mock concrete under `WriteRefused`.
#[test]
fn consume_delivery_proof_returns_the_emit_refusal_unchanged() {
    // Arrange
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused);
    let kinds = build_create_generate_params(Default::default()).kinds;

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
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            // Act
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
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory ran the subject")
            };
            success.output
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
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
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
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
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
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
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
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateGenerateReturn<EmitReturn>;

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
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    generate: self.generate,
                    kinds: self.kinds,
                    statements: self.statements,
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
    let Ok(CreateGenerateSuccessReturn {
        output: Err(EmitErrorReturn::Mock(error)),
    }) = output
    else {
        panic!("the generate concrete's refusal reaches the output")
    };
    assert_eq!(error, MockIGenerateAdapterEmitErrorReturn::WriteRefused);
    // consume_delivery_proof_returns_the_emit_refusal_unchanged
}

/// Contract: generation refused: `create_generate` returns `Err(error)`, the
///   generate factory's refusal of the configured kinds → the output is
///   `Err(error)`, unchanged.
/// Arrange: the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`, the
///   kinds `build_create_generate_params(Default::default()).kinds` followed
///   by `build_vector_kind_configuration(VectorKindConfigurationOverrides
///   { kind: Some(VectorKind::Possession), ..Default::default() })`, a kind
///   the mock declaration does not name, so a consumer that hands
///   `create_generate` any kinds but its own is admitted and fails, and the
///   statements of the first block.
/// Act:     `create_delivery_proof` with the subject as the deps' consumer,
///   called in the innermost consumer's method.
/// Assert:  the output unpacks as `Err(error)` by
///   `let … else { panic!(…) };` and `error` equals
///   `CreateGenerateErrorReturn::UndeclaredKind { kind: VectorKind::Possession }`
///   by `assert_eq!`.
/// Collaborators: as
///   `consume_delivery_proof_returns_the_files_the_generate_factory_returns`.
#[test]
fn consume_delivery_proof_returns_the_generate_factorys_refusal_unchanged() {
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
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            // Act
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
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory ran the subject")
            };
            success.output
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
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let scope = payload.scope();
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
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_key_agreement_params(Default::default()),
                build_create_key_agreement_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the key-agreement factory admits the selection")
            };
            success.output
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
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
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
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateGenerateReturn<EmitReturn>;

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
                    hashing: &*hashing.adapter,
                    random: &*source.adapter,
                    generate: self.generate,
                    kinds: self.kinds,
                    statements: self.statements,
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
        panic!("the generate factory refuses the kinds")
    };
    assert_eq!(
        error,
        CreateGenerateErrorReturn::UndeclaredKind {
            kind: VectorKind::Possession,
        }
    );
    // consume_delivery_proof_returns_the_generate_factorys_refusal_unchanged
}
