#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::build_asset_coordinate;
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::{
    CreateKeyAgreementErrorReturn, CreateKeyAgreementReturn, CreateKeyAgreementSuccessReturn,
    KeyAgreementConcrete, KeyAgreementIdentifier, MockIKeyAgreementAdapterFailureMode,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    CreateKemParamsOverrides, IdentityScope, build_create_kem_deps, build_create_kem_params,
    build_create_kem_payload, create_kem,
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

use super::mock::{KemConsumerConstructorParamsOverrides, build_kem_consumer};
use crate::config::provides::{
    DeliveryProofSelection, DeliveryProofSelectionOverrides, HarnessStatements,
    HarnessStatementsOverrides, KeyAgreementSelection, KeyAgreementSelectionOverrides,
    build_delivery_proof_selection, build_harness_statements, build_key_agreement_selection,
};
use crate::generate::provides::{
    CreateGenerateErrorReturn, CreateGenerateReturn, CreateGenerateSuccessReturn, EmitErrorReturn,
    EmitReturn, EmitSuccessReturn, EmittedFileOverrides, GenerateConcrete,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode, VectorKind,
    VectorKindConfiguration, VectorKindConfigurationOverrides, build_create_generate_params,
    build_emitted_file, build_vector_kind_configuration,
};

/// Contract: admitted: `create_key_agreement` returns `Ok(success)` whose
///   `output` is `create_delivery_proof`'s success holding `create_generate`'s
///   success with the files written → the output is `Ok(success)`, unchanged.
/// Arrange: the scope `IdentityScope::Entitlement`, the default of
///   `build_create_kem_params`; the key-agreement selection
///   `build_key_agreement_selection` over the family's mock concrete under
///   `NoFailure` and the identifier `KeyAgreementIdentifier::Mock` the mock
///   concrete declares, where the builder's default names
///   `KeyAgreementIdentifier::PairingElGamalV1`, its algebra the builder's
///   default `EnvelopeAlgebra::PairingElGamal`, so a consumer that hands the
///   factory an identifier other than its selection's is refused; the
///   delivery-proof selection `build_delivery_proof_selection` over the
///   family's mock concrete under `NoFailure`, its version the builder's
///   default `DELIVERY_STATEMENT_VERSION_ONE`, which the family's mock
///   concrete declares; the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`;
///   the kinds `build_create_generate_params(Default::default()).kinds`, the
///   mock declaration's kinds in declared order; and the statements
///   `build_harness_statements(Default::default())`.
/// Act:     `create_kem` with the subject as the deps' consumer, called in
///   the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Ok(CreateDeliveryProofSuccessReturn { output:
///   Ok(CreateGenerateSuccessReturn { output: Ok(EmitSuccessReturn { files })
///   }) }) })` by `let … else { panic!(…) };` and `files` equals, in order,
///   an `EmittedFile` per declared kind, each `path` its kind's name, by
///   `assert_eq!`, so an output emptied, reordered, or not taken from
///   `create_key_agreement` fails.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, and random-source's mock concretes in their default modes through
///   their factories; the key-agreement family's mock concrete under
///   `NoFailure`; the delivery-proof family's mock concrete under `NoFailure`;
///   the generate family's mock concrete under `NoFailure`; the subject
///   `KemConsumer` in the deps.
#[test]
fn consume_kem_returns_the_files_the_key_agreement_factory_returns() {
    // Arrange
    let scope = IdentityScope::Entitlement;
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let key_agreement = build_key_agreement_selection(KeyAgreementSelectionOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        identifier: Some(KeyAgreementIdentifier::Mock),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    build_kem_consumer::<P, E, F>(
                        self.pairing,
                        self.hash_to_scalar,
                        self.encoder,
                        self.random,
                        self.generate,
                        self.kinds,
                        self.statements,
                        KemConsumerConstructorParamsOverrides {
                            key_agreement: Some(*self.key_agreement),
                            delivery_proof: Some(*self.delivery_proof),
                        },
                    ),
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        scope: self.scope,
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    scope: self.scope,
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
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        scope,
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
    let Ok(CreateKeyAgreementSuccessReturn {
        output:
            Ok(CreateDeliveryProofSuccessReturn {
                output:
                    Ok(CreateGenerateSuccessReturn {
                        output: Ok(EmitSuccessReturn { files }),
                    }),
            }),
    }) = output
    else {
        panic!("the key-agreement factory returns the files")
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
    // consume_kem_returns_the_files_the_key_agreement_factory_returns
}

/// Contract: admitted: `create_key_agreement` returns `Ok(success)` whose
///   `output` is `create_delivery_proof`'s success holding `create_generate`'s
///   success with the generate concrete's refusal → the output is
///   `Ok(success)`, unchanged.
/// Arrange: the base arrangement of
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns` with
///   the generate concrete
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`,
///   a mode other than `NoFailure`, so a consumer that constructs its
///   key-agreement consumer over a generate concrete other than its own fails.
/// Act:     `create_kem` with the subject as the deps' consumer, called in
///   the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Ok(CreateDeliveryProofSuccessReturn { output:
///   Ok(CreateGenerateSuccessReturn { output: Err(EmitErrorReturn::Mock(error))
///   }) }) })` by `let … else { panic!(…) };` and `error` equals
///   `MockIGenerateAdapterEmitErrorReturn::WriteRefused` by `assert_eq!`.
/// Collaborators: as
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`; the
///   generate family's mock concrete under `WriteRefused`.
#[test]
fn consume_kem_returns_the_emit_refusal_unchanged() {
    // Arrange
    let scope = IdentityScope::Entitlement;
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let key_agreement = build_key_agreement_selection(KeyAgreementSelectionOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        identifier: Some(KeyAgreementIdentifier::Mock),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    build_kem_consumer::<P, E, F>(
                        self.pairing,
                        self.hash_to_scalar,
                        self.encoder,
                        self.random,
                        self.generate,
                        self.kinds,
                        self.statements,
                        KemConsumerConstructorParamsOverrides {
                            key_agreement: Some(*self.key_agreement),
                            delivery_proof: Some(*self.delivery_proof),
                        },
                    ),
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        scope: self.scope,
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    scope: self.scope,
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
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        scope,
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
    let Ok(CreateKeyAgreementSuccessReturn {
        output:
            Ok(CreateDeliveryProofSuccessReturn {
                output:
                    Ok(CreateGenerateSuccessReturn {
                        output: Err(EmitErrorReturn::Mock(error)),
                    }),
            }),
    }) = output
    else {
        panic!("the key-agreement factory returns the emit refusal")
    };
    assert_eq!(error, MockIGenerateAdapterEmitErrorReturn::WriteRefused);
    // consume_kem_returns_the_emit_refusal_unchanged
}

/// Contract: admitted: `create_key_agreement` returns `Ok(success)` whose
///   `output` is `create_delivery_proof`'s success holding `create_generate`'s
///   refusal → the output is `Ok(success)`, unchanged.
/// Arrange: the base arrangement of
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns` with
///   the kinds `build_create_generate_params(Default::default()).kinds`
///   followed by `build_vector_kind_configuration` over
///   `VectorKind::Possession`, a kind the mock declaration does not name, so a
///   consumer that constructs its key-agreement consumer over any kinds but
///   its own is admitted and fails.
/// Act:     `create_kem` with the subject as the deps' consumer, called in
///   the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Ok(CreateDeliveryProofSuccessReturn { output: Err(error) }) })` by
///   `let … else { panic!(…) };` and `error` equals
///   `CreateGenerateErrorReturn::UndeclaredKind { kind:
///   VectorKind::Possession }` by `assert_eq!`.
/// Collaborators: as
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`.
#[test]
fn consume_kem_returns_the_generate_factorys_refusal_unchanged() {
    // Arrange
    let scope = IdentityScope::Entitlement;
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let mut kinds = build_create_generate_params(Default::default()).kinds;
    kinds.push(build_vector_kind_configuration(
        VectorKindConfigurationOverrides {
            kind: Some(VectorKind::Possession),
            ..Default::default()
        },
    ));
    let key_agreement = build_key_agreement_selection(KeyAgreementSelectionOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        identifier: Some(KeyAgreementIdentifier::Mock),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    build_kem_consumer::<P, E, F>(
                        self.pairing,
                        self.hash_to_scalar,
                        self.encoder,
                        self.random,
                        self.generate,
                        self.kinds,
                        self.statements,
                        KemConsumerConstructorParamsOverrides {
                            key_agreement: Some(*self.key_agreement),
                            delivery_proof: Some(*self.delivery_proof),
                        },
                    ),
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        scope: self.scope,
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    scope: self.scope,
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
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        scope,
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
    let Ok(CreateKeyAgreementSuccessReturn {
        output: Ok(CreateDeliveryProofSuccessReturn { output: Err(error) }),
    }) = output
    else {
        panic!("the key-agreement factory returns the generate refusal")
    };
    assert_eq!(
        error,
        CreateGenerateErrorReturn::UndeclaredKind {
            kind: VectorKind::Possession
        }
    );
    // consume_kem_returns_the_generate_factorys_refusal_unchanged
}

/// Contract: admitted: `create_key_agreement` returns `Ok(success)` whose
///   `output` is `create_delivery_proof`'s refusal → the output is
///   `Ok(success)`, unchanged.
/// Arrange: the base arrangement of
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns` with
///   the delivery-proof selection's version
///   `DELIVERY_STATEMENT_VERSION_ONE + 1`, a version the family's mock
///   concrete does not declare where the builder's default is one it
///   declares, so a consumer that constructs its key-agreement consumer over
///   any delivery-proof selection but its own is admitted and fails.
/// Act:     `create_kem` with the subject as the deps' consumer, called in
///   the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateKeyAgreementSuccessReturn { output: Err(error) })` by
///   `let … else { panic!(…) };` and `error` equals
///   `CreateDeliveryProofErrorReturn::UnsupportedStatementVersion` by
///   `assert_eq!`.
/// Collaborators: as
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`.
#[test]
fn consume_kem_returns_the_delivery_proof_factorys_refusal_unchanged() {
    // Arrange
    let scope = IdentityScope::Entitlement;
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let key_agreement = build_key_agreement_selection(KeyAgreementSelectionOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        identifier: Some(KeyAgreementIdentifier::Mock),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE + 1),
    });

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    build_kem_consumer::<P, E, F>(
                        self.pairing,
                        self.hash_to_scalar,
                        self.encoder,
                        self.random,
                        self.generate,
                        self.kinds,
                        self.statements,
                        KemConsumerConstructorParamsOverrides {
                            key_agreement: Some(*self.key_agreement),
                            delivery_proof: Some(*self.delivery_proof),
                        },
                    ),
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        scope: self.scope,
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    scope: self.scope,
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
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        scope,
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
    let Ok(CreateKeyAgreementSuccessReturn { output: Err(error) }) = output else {
        panic!("the key-agreement factory returns the delivery-proof refusal")
    };
    assert_eq!(
        error,
        CreateDeliveryProofErrorReturn::UnsupportedStatementVersion
    );
    // consume_kem_returns_the_delivery_proof_factorys_refusal_unchanged
}

/// Contract: refused: `create_key_agreement` returns `Err(error)` → the
///   output is `Err(error)`, unchanged.
/// Arrange: the base arrangement of
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns` with
///   the key-agreement selection's identifier the builder's default
///   `KeyAgreementIdentifier::PairingElGamalV1`, which the family's mock
///   concrete does not declare, so a consumer that hands the factory any
///   identifier but its selection's is admitted and fails.
/// Act:     `create_kem` with the subject as the deps' consumer, called in
///   the innermost consumer's method.
/// Assert:  the output unpacks as `Err(error)` by
///   `let … else { panic!(…) };` and `error` equals
///   `CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier` by
///   `assert_eq!`.
/// Collaborators: as
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`.
#[test]
fn consume_kem_returns_the_key_agreement_factorys_refusal_unchanged() {
    // Arrange
    let scope = IdentityScope::Entitlement;
    let statements = build_harness_statements(Default::default());
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let key_agreement = build_key_agreement_selection(KeyAgreementSelectionOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    build_kem_consumer::<P, E, F>(
                        self.pairing,
                        self.hash_to_scalar,
                        self.encoder,
                        self.random,
                        self.generate,
                        self.kinds,
                        self.statements,
                        KemConsumerConstructorParamsOverrides {
                            key_agreement: Some(*self.key_agreement),
                            delivery_proof: Some(*self.delivery_proof),
                        },
                    ),
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        scope: self.scope,
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    scope: self.scope,
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
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        scope,
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
        panic!("the key-agreement factory returns its refusal")
    };
    assert_eq!(
        error,
        CreateKeyAgreementErrorReturn::UnsupportedKeyAgreementIdentifier
    );
    // consume_kem_returns_the_key_agreement_factorys_refusal_unchanged
}

/// Contract: the scope `payload.scope()` returns is the scope the
///   key-agreement consumer carries to `create_generate`, so under the asset
///   scope `create_key_agreement` returns `Ok(success)` whose `output` is
///   `create_delivery_proof`'s success holding `create_generate`'s success
///   with the generate concrete's refusal of a canonical identity → the
///   output is `Ok(success)`, unchanged.
/// Arrange: the base arrangement of
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns` with
///   the scope `IdentityScope::Asset` and the statements' `identities` and
///   `proof_identity` the `as_ref()` bytes of `build_asset_coordinate`, a
///   canonical identity no asset identity hash admits, so a consumer that
///   hands its key-agreement consumer the entitlement scope in place of the
///   admitted one returns the files and fails.
/// Act:     `create_kem` with the subject as the deps' consumer, called in
///   the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Ok(CreateDeliveryProofSuccessReturn { output:
///   Ok(CreateGenerateSuccessReturn { output:
///   Err(EmitErrorReturn::AssetIdentityLength { index, actual }) }) }) })` by
///   `let … else { panic!(…) };` and `(index, actual)` equals
///   `(0, coordinate.len())` by `assert_eq!`.
/// Collaborators: as
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`; the
///   kem family's mock concrete admitting the asset scope.
#[test]
fn consume_kem_hands_the_admitted_asset_scope_to_the_generate_concrete() {
    // Arrange
    let scope = IdentityScope::Asset;
    let coordinate = build_asset_coordinate(Default::default()).as_ref().to_vec();
    let statements = build_harness_statements(HarnessStatementsOverrides {
        identities: Some(vec![coordinate.clone()]),
        proof_identity: Some(coordinate.clone()),
        ..Default::default()
    });
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let key_agreement = build_key_agreement_selection(KeyAgreementSelectionOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        identifier: Some(KeyAgreementIdentifier::Mock),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    build_kem_consumer::<P, E, F>(
                        self.pairing,
                        self.hash_to_scalar,
                        self.encoder,
                        self.random,
                        self.generate,
                        self.kinds,
                        self.statements,
                        KemConsumerConstructorParamsOverrides {
                            key_agreement: Some(*self.key_agreement),
                            delivery_proof: Some(*self.delivery_proof),
                        },
                    ),
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        scope: self.scope,
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    scope: self.scope,
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
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        scope,
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
    let Ok(CreateKeyAgreementSuccessReturn {
        output:
            Ok(CreateDeliveryProofSuccessReturn {
                output:
                    Ok(CreateGenerateSuccessReturn {
                        output: Err(EmitErrorReturn::AssetIdentityLength { index, actual }),
                    }),
            }),
    }) = output
    else {
        panic!("the key-agreement factory returns the asset-scope refusal")
    };
    assert_eq!((index, actual), (0, coordinate.len()));
    // consume_kem_hands_the_admitted_asset_scope_to_the_generate_concrete
}

/// Contract: the scope `payload.scope()` returns is the scope the
///   key-agreement consumer carries to `create_generate`, so under the
///   entitlement scope a canonical identity is admitted and
///   `create_key_agreement` returns `Ok(success)` holding the files written →
///   the output is `Ok(success)`, unchanged.
/// Arrange: the base arrangement of
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`, its
///   scope `IdentityScope::Entitlement`, with the statements of
///   `consume_kem_hands_the_admitted_asset_scope_to_the_generate_concrete`,
///   so a consumer that hands its key-agreement consumer the asset scope in
///   place of the admitted one is refused and fails.
/// Act:     `create_kem` with the subject as the deps' consumer, called in
///   the innermost consumer's method.
/// Assert:  the output unpacks as
///   `Ok(CreateKeyAgreementSuccessReturn { output:
///   Ok(CreateDeliveryProofSuccessReturn { output:
///   Ok(CreateGenerateSuccessReturn { output: Ok(EmitSuccessReturn { files })
///   }) }) })` by `let … else { panic!(…) };` and `files` equals the files
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`
///   asserts, by `assert_eq!`.
/// Collaborators: as
///   `consume_kem_returns_the_files_the_key_agreement_factory_returns`.
#[test]
fn consume_kem_hands_the_admitted_entitlement_scope_to_the_generate_concrete() {
    // Arrange
    let scope = IdentityScope::Entitlement;
    let coordinate = build_asset_coordinate(Default::default()).as_ref().to_vec();
    let statements = build_harness_statements(HarnessStatementsOverrides {
        identities: Some(vec![coordinate.clone()]),
        proof_identity: Some(coordinate.clone()),
        ..Default::default()
    });
    let generate = GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure);
    let kinds = build_create_generate_params(Default::default()).kinds;
    let key_agreement = build_key_agreement_selection(KeyAgreementSelectionOverrides {
        concrete: Some(KeyAgreementConcrete::Mock(
            MockIKeyAgreementAdapterFailureMode::NoFailure,
        )),
        identifier: Some(KeyAgreementIdentifier::Mock),
        ..Default::default()
    });
    let delivery_proof = build_delivery_proof_selection(DeliveryProofSelectionOverrides {
        concrete: Some(DeliveryProofConcrete::Mock(
            MockIDeliveryProofAdapterFailureMode::NoFailure,
        )),
        ..Default::default()
    });

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConcrete,
        kinds: &'a [VectorKindConfiguration],
        statements: &'a HarnessStatements,
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            // Act
            let created = create_kem(
                &build_create_kem_deps(
                    self.pairing,
                    self.hash_to_scalar,
                    build_kem_consumer::<P, E, F>(
                        self.pairing,
                        self.hash_to_scalar,
                        self.encoder,
                        self.random,
                        self.generate,
                        self.kinds,
                        self.statements,
                        KemConsumerConstructorParamsOverrides {
                            key_agreement: Some(*self.key_agreement),
                            delivery_proof: Some(*self.delivery_proof),
                        },
                    ),
                    Default::default(),
                ),
                build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(self.scope),
                    ..Default::default()
                }),
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                        key_agreement: self.key_agreement,
                        delivery_proof: self.delivery_proof,
                        scope: self.scope,
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
        key_agreement: &'a KeyAgreementSelection,
        delivery_proof: &'a DeliveryProofSelection,
        scope: IdentityScope,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output =
            CreateKeyAgreementReturn<CreateDeliveryProofReturn<CreateGenerateReturn<EmitReturn>>>;

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
                    key_agreement: self.key_agreement,
                    delivery_proof: self.delivery_proof,
                    scope: self.scope,
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
        key_agreement: &key_agreement,
        delivery_proof: &delivery_proof,
        scope,
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
    let Ok(CreateKeyAgreementSuccessReturn {
        output:
            Ok(CreateDeliveryProofSuccessReturn {
                output:
                    Ok(CreateGenerateSuccessReturn {
                        output: Ok(EmitSuccessReturn { files }),
                    }),
            }),
    }) = output
    else {
        panic!("the key-agreement factory returns the files")
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
    // consume_kem_hands_the_admitted_entitlement_scope_to_the_generate_concrete
}
