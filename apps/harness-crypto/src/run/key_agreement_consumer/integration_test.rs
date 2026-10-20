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

use super::mock::{KeyAgreementConsumerConstructorParamsOverrides, build_key_agreement_consumer};
use crate::config::provides::{
    DeliveryProofSelectionOverrides, HarnessStatements, HarnessStatementsOverrides,
    build_delivery_proof_selection, build_harness_statements,
};
use crate::generate::provides::{
    CreateGenerateReturn, CreateGenerateSuccessReturn, EmitReturn, EmitSuccessReturn,
    GENERATE_CONCRETES, GenerateConcreteOverrides, GenerateKindConfigurationsOverrides, VectorKind,
    build_generate_concrete, build_generate_kind_configurations,
};

/// Contract: entry
///   `an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version`,
///   absorbed, in this node's terms: at the configured delivery-statement
///   version, which every member of `DELIVERY_PROOF_CONCRETES` declares, the
///   subject's `create_delivery_proof` admits each member on every pairing
///   concrete and runs the delivery-proof consumer it constructs over its
///   own statements, whose output reaches the subject's output unaltered.
/// Arrange: the statements `build_harness_statements(HarnessStatementsOverrides
///   { identities: Some(vec![vec![0x0a; 32], vec![0x0b; 32], vec![0x0c; 32]]),
///   proof_identity: Some(vec![0x0a; 32]), ..Default::default() })`, three
///   distinct identities where the builder's default holds one, every
///   identity 32 bytes long and nonzero so every case of
///   `DECLARED_KEM_SELECTIONS`, the asset scope included, admits them; the
///   selection's statement version the builder's default
///   `DELIVERY_STATEMENT_VERSION_ONE`, which every member of
///   `DELIVERY_PROOF_CONCRETES` declares, so a subject that hands the factory
///   any version but its own is refused, and one that returns anything other
///   than the factory's return or constructs its delivery-proof consumer
///   over other statements, fails; the random source
///   `RandomSourceKind::OperatingSystem`; every case the private surface
///   names.
/// Act:     `create_key_agreement` with the subject as the deps' consumer,
///   in the innermost consumer.
/// Assert:  every output unpacks as `Ok(CreateDeliveryProofSuccessReturn {
///   output: Ok(CreateGenerateSuccessReturn { output: Ok(EmitSuccessReturn {
///   files }) }) })`, the entry whose `kind` is
///   `Some(VectorKind::IdentityMapping)` is found by `iter().find` and
///   unpacked by `let … else { panic!(…) };`, its `path` is read by
///   `std::fs::read_to_string`, and the text's `lines().count()` equals `3`
///   by `assert_eq!`.
/// Boundary: the chain of real functions the private surface names — each
///   family's `create_*` over its declared set, each selected concrete's
///   `try_new`, the subject's `consume_key_agreement`,
///   `DeliveryProofConsumer::try_new`, `create_delivery_proof`, the selected
///   concrete's `try_new`, `DeliveryProofConsumer::consume_delivery_proof`,
///   `GenerateConsumer::try_new`, `create_generate`, the selected concrete's
///   `try_new`, `GenerateConsumer::consume_generate`, `emit`, and the
///   functions `emit` runs, each real; the file read from the real
///   filesystem.
/// Mocked:  nothing on the chain; the filesystem is a directory beneath the
///   operating system's temporary directory and the operating system's
///   generator runs real.
#[test]
fn every_declared_delivery_proof_concrete_admits_the_configured_version_and_the_files_reach_the_output()
 {
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

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
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
            for member in GENERATE_CONCRETES {
                let generate = build_generate_concrete(*member, GenerateConcreteOverrides);
                let kinds = build_generate_kind_configurations(
                    *member,
                    GenerateKindConfigurationsOverrides,
                );
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
                                    &generate,
                                    &kinds,
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
                        let success = match created {
                            Ok(success) => success,
                            Err(error) => {
                                panic!("the declared key-agreement selection is created: {error:?}")
                            }
                        };
                        outputs.push(success.output);
                    }
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
                            statements: self.statements,
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
        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
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
                            hash_to_scalar: &*hashing.adapter,
                            random: self.random,
                            statements: self.statements,
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
            .find(|file| file.kind == Some(VectorKind::IdentityMapping))
        else {
            panic!("the emitted files hold the identity-mapping kind")
        };
        let text = std::fs::read_to_string(&file.path).expect("the identity-mapping file reads");
        assert_eq!(text.lines().count(), 3);
    }
    // every_declared_delivery_proof_concrete_admits_the_configured_version_and_the_files_reach_the_output
}
