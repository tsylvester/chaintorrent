#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::super::factory::provides::{
    EmitErrorReturn, EmittedFile, IGenerateAdapter, VectorKind, VectorKindSettings,
    VectorKindSettingsOverrides, VectorKindSize, build_emit_params, build_emit_payload,
    build_vector_kind_settings,
};
use super::challenge_vectors::provides::{
    build_canonical_delivery_context, build_challenge_vector_description,
};
use super::constants::provides::ConstantsErrorReturn;
use super::group_vectors::provides::{
    MsmTermCountConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    build_group_operation_vector_description, build_msm_term_count,
    build_pairing_check_vector_description, build_vector_count,
};
use super::interface::*;
use super::mapping_vectors::provides::{
    MappingVectorsErrorReturn, build_hash_to_scalar_vector_description,
    build_identity_mapping_vector_description,
};
use super::mock::*;
use super::possession_vectors::provides::build_possession_vector_description;
use super::proof_vectors::provides::{
    build_mint_proof_vector_description, build_transfer_proof_vector_description,
};
use super::rejection_vectors::provides::build_rejection_vector_description;
use super::render::provides::{
    SolidityCompilerVersionOverrides, SolidityConstantNameConstructorParamsOverrides,
    SolidityLibraryNameConstructorParamsOverrides, SolidityMemberNameConstructorParamsOverrides,
    SolidityRecordDeclarationsTryNewErrorReturn, SolidityTypeNameConstructorParamsOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, build_solidity_compiler_version,
    build_solidity_constant_name, build_solidity_library_name, build_solidity_member_name,
    build_solidity_type_name, build_spdx_license_identifier,
};
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
    build_decode_params, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, DECLARED_KEY_AGREEMENT_SELECTIONS,
    EnvelopeAlgebra, IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarIdentifier, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IKemConsumer, build_create_kem_deps, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, PairingCurve, SampleUniformScalarErrorReturn,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
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

/// Contract: entry
///   `emit_writes_each_emitted_kind_then_both_libraries_in_write_order_
///   under_the_curve_directory`; `emit` over a generated directory of two
///   distinct segments, a distinct directory text per curve, and file names
///   distinct across the kinds and the libraries → `Ok` whose `files` are,
///   in order, the root joined with the segments, the curve directory, and
///   the file name of each emitted kind in declared order, then the
///   constants library's file name, then the records library's, each
///   existing; the curve directory is the configured text of the curve the
///   case's pairing is on, and the kinds needing second-group arithmetic
///   are emitted only where the case's verifier has both groups.
/// Arrange: `generated_directory` of two distinct segments by
///   `build_generated_directory`, `curve_directories` of two distinct texts
///   by `build_evm_curve_directories`, and the default file names, already
///   distinct across the kinds and the libraries; the settings' root
///   removed before the act; `build_canonical_delivery_context` at its
///   default for the mint and the transfer, two distinct byte strings as
///   `identities`, `proof_identity` the first; every case the private
///   surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  the result is `Ok` whose `files` equals the expected
///   `EmittedFile`s in order, a vector file carrying `kind` `Some` of the
///   kind whose file it is and each library `None`, each entry's `path`
///   the path written and existing.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn emit_writes_each_emitted_kind_then_both_libraries_in_write_order_under_the_curve_directory() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        generated_directory: Some(build_generated_directory(
            GeneratedDirectoryConstructorParamsOverrides {
                segments: Some(vec![
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("generated".to_string()),
                    }),
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("vectors".to_string()),
                    }),
                ]),
            },
        )),
        curve_directories: Some(build_evm_curve_directories(EvmCurveDirectoriesOverrides {
            bn254: Some(build_generated_file_name(
                GeneratedFileNameConstructorParamsOverrides {
                    text: Some("bn254-curve".to_string()),
                },
            )),
            bls12_381: Some(build_generated_file_name(
                GeneratedFileNameConstructorParamsOverrides {
                    text: Some("bls12-381-curve".to_string()),
                },
            )),
        })),
        ..Default::default()
    });
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );
            let mut directory = self.settings.foundry_root.clone();
            for segment in &self.settings.generated_directory.segments {
                directory = directory.join(segment.as_str());
            }
            let curve_directory = match P::DECLARATION.curve {
                PairingCurve::Bn254 => self.settings.curve_directories.bn254.as_str(),
                PairingCurve::Bls12381 => self.settings.curve_directories.bls12_381.as_str(),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => self.settings.curve_directories.bn254.as_str(),
            };
            let directory = directory.join(curve_directory);
            let emitted: Vec<VectorKind> =
                <EvmGenerate<'_, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                    .kinds
                    .iter()
                    .filter(|declaration| {
                        !declaration.requires_second_group_arithmetic
                            || P::DECLARATION.verifier_group_arithmetic
                                == VerifierGroupArithmetic::BothGroups
                    })
                    .map(|declaration| declaration.kind)
                    .collect();
            let mut expected: Vec<EmittedFile> = Vec::with_capacity(emitted.len() + 2);
            for kind in &emitted {
                let names = &self
                    .settings
                    .kinds
                    .iter()
                    .find(|entry| entry.kind == *kind)
                    .expect("the settings name every emitted kind")
                    .names;
                expected.push(EmittedFile {
                    kind: Some(*kind),
                    path: directory.join(names.file_name.as_str()),
                });
            }
            expected.push(EmittedFile {
                kind: None,
                path: directory.join(self.settings.constants_library.file_name.as_str()),
            });
            expected.push(EmittedFile {
                kind: None,
                path: directory.join(self.settings.records_library.file_name.as_str()),
            });

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            let Ok(success) = result else {
                panic!("emit admits the case")
            };
            assert_eq!(success.files, expected);
            for file in &success.files {
                assert!(file.path.exists(), "the emitted file exists");
            }
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // emit_writes_each_emitted_kind_then_both_libraries_in_write_order_under_the_curve_directory
}

/// Contract: entry
///   `each_kinds_file_holds_one_hex_line_per_record_at_its_admitted_size`;
///   `emit` with every kind's admitted count distinct from every other's,
///   the term count and the message length distinct from the counts, and
///   two distinct identities → each drawn kind's file holds its admitted
///   count of lines except the challenge kind's, which holds a mint line
///   and a transfer line per count, the hash-to-scalar file holds the
///   admitted count for each of the identity, possession first-group,
///   possession second-group, challenge, and weight domain tags, the
///   identity-mapping file holds one line per configured identity, and the
///   decode-rejection file holds one line for each candidate the case's
///   curve defines; every line is `0x` and an even run of lowercase hex
///   digits, each file ends in a line feed and holds no empty line.
/// Arrange: `sizes` whose drawn counts are `3` through `13` across the
///   kinds, the term count `21`, and the message length `48`, each count
///   differing from every other and from the quantities' defaults; two
///   identities; the settings' root removed before the act; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  each written kind's file holds the expected count of lines,
///   every line `0x` followed by an even run of lowercase hex digits, the
///   file ends in `\n`, and no line is empty.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn each_kinds_file_holds_one_hex_line_per_record_at_its_admitted_size() {
    fn drawn_count(sizes: &[VectorKindSettings], kind: VectorKind) -> usize {
        match sizes
            .iter()
            .find(|entry| entry.kind == kind)
            .map(|entry| entry.size)
        {
            Some(VectorKindSize::Drawn { count })
            | Some(VectorKindSize::DrawnTerms { count, .. })
            | Some(VectorKindSize::DrawnMessages { count, .. }) => count.get() as usize,
            _ => panic!("the kind carries a drawn count"),
        }
    }

    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(Default::default());
    let sizes = vec![
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::G1Addition),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(3) }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::G1Multiplication),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(4) }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::PairingCheck),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(5) }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::G2Addition),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(6) }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::G1MultiScalarMultiplication),
            size: Some(VectorKindSize::DrawnTerms {
                count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(7) }),
                terms: build_msm_term_count(MsmTermCountConstructorParamsOverrides {
                    count: Some(21),
                }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::G2MultiScalarMultiplication),
            size: Some(VectorKindSize::DrawnTerms {
                count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(8) }),
                terms: build_msm_term_count(MsmTermCountConstructorParamsOverrides {
                    count: Some(21),
                }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::DecodeRejection),
            size: Some(VectorKindSize::Enumerated),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::HashToScalar),
            size: Some(VectorKindSize::DrawnMessages {
                count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(9) }),
                message_length: 48,
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::IdentityMapping),
            size: Some(VectorKindSize::OverConfiguredInputs),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::Possession),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(10),
                }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::Challenge),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(11),
                }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::MintProof),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(12),
                }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::TransferProof),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(13),
                }),
            }),
        }),
    ];
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
        sizes: &'a [VectorKindSettings],
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    sizes: Some(self.sizes.to_vec()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );
            let mut directory = self.settings.foundry_root.clone();
            for segment in &self.settings.generated_directory.segments {
                directory = directory.join(segment.as_str());
            }
            let curve_directory = match P::DECLARATION.curve {
                PairingCurve::Bn254 => self.settings.curve_directories.bn254.as_str(),
                PairingCurve::Bls12381 => self.settings.curve_directories.bls12_381.as_str(),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => self.settings.curve_directories.bn254.as_str(),
            };
            let directory = directory.join(curve_directory);
            let emitted: Vec<VectorKind> =
                <EvmGenerate<'_, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                    .kinds
                    .iter()
                    .filter(|declaration| {
                        !declaration.requires_second_group_arithmetic
                            || P::DECLARATION.verifier_group_arithmetic
                                == VerifierGroupArithmetic::BothGroups
                    })
                    .map(|declaration| declaration.kind)
                    .collect();

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            let Ok(success) = result else {
                panic!("emit admits the case")
            };
            assert_eq!(success.files.len(), emitted.len() + 2);
            for kind in &emitted {
                let names = &self
                    .settings
                    .kinds
                    .iter()
                    .find(|entry| entry.kind == *kind)
                    .expect("the settings name every emitted kind")
                    .names;
                let path = directory.join(names.file_name.as_str());
                let contents =
                    std::fs::read_to_string(&path).expect("the written kind's file reads back");
                assert!(contents.ends_with('\n'), "the file ends in a line feed");
                let lines: Vec<&str> = contents.lines().collect();
                let expected = match kind {
                    VectorKind::Challenge => 2 * drawn_count(self.sizes, *kind),
                    VectorKind::HashToScalar => 5 * drawn_count(self.sizes, *kind),
                    VectorKind::IdentityMapping => identities.len(),
                    VectorKind::DecodeRejection => match P::DECLARATION.curve {
                        PairingCurve::Bn254 => 2,
                        PairingCurve::Bls12381 => 3,
                        #[cfg(any(test, feature = "mocks"))]
                        PairingCurve::Mock => 2,
                    },
                    _ => drawn_count(self.sizes, *kind),
                };
                assert_eq!(lines.len(), expected, "the kind's admitted size");
                for line in &lines {
                    assert!(!line.is_empty(), "the file holds no empty line");
                    assert!(line.starts_with("0x"), "the line opens with 0x");
                    let hex = &line[2..];
                    assert_eq!(hex.len() % 2, 0, "the hex run is even");
                    assert!(
                        hex.chars()
                            .all(|byte| matches!(byte, '0'..='9' | 'a'..='f')),
                        "the hex run is lowercase hex"
                    );
                }
            }
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        sizes: &'a [VectorKindSettings],
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
                            sizes: self.sizes,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        sizes: &'a [VectorKindSettings],
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            sizes: self.sizes,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        sizes: &'a [VectorKindSettings],
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            sizes: self.sizes,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        sizes: &'a [VectorKindSettings],
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            sizes: self.sizes,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
        sizes: &'a [VectorKindSettings],
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                            sizes: self.sizes,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
        sizes: &sizes,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // each_kinds_file_holds_one_hex_line_per_record_at_its_admitted_size
}

/// Contract: entry `every_written_line_decodes_under_its_kinds_description`;
///   every line of every written kind's file, its hex after `0x` read back
///   into bytes, decoded through the case's encoder under the description
///   of the kind that wrote it → every line decodes `Ok` and the decoded
///   records are as many as the file's lines. The absorbed
///   `encoding/factory` entry `every_concrete_decodes_what_it_encodes` is
///   proven here: the bytes `encode` returns for a record under its kind's
///   description, written as its line, decode through `decode` under the
///   same description.
/// Arrange: the default `sizes` and each kind's description by its builder;
///   the settings' root removed before the act; every case the private
///   surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  each written file's lines, read back from their hex into bytes,
///   each return `Ok` from the encoder's `decode` under the kind's
///   description, and the decoded records are as many as the file's lines.
/// Boundary: `emit`, the encoder's `encode` and `decode`, and the rest of
///   the real chain the private surface names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn every_written_line_decodes_under_its_kinds_description() {
    fn decode_lines<E: IEncoderAdapter + IDecoderAdapter, C: encoding::IEncodingContract>(
        encoder: &E,
        path: &Path,
        description: &C,
    ) {
        let contents = std::fs::read_to_string(path).expect("the written kind's file reads back");
        let lines: Vec<&str> = contents.lines().collect();
        let mut decoded = 0usize;
        for line in &lines {
            assert!(line.starts_with("0x"), "the written line opens with 0x");
            let hex = &line[2..];
            assert_eq!(hex.len() % 2, 0, "the hex run is even");
            let mut bytes = Vec::with_capacity(hex.len() / 2);
            let mut index = 0;
            while index < hex.len() {
                bytes.push(
                    u8::from_str_radix(&hex[index..index + 2], 16)
                        .expect("the written line is hex"),
                );
                index += 2;
            }
            encoder
                .decode(build_decode_params(description), &bytes)
                .expect("the written line decodes under the kind's description");
            decoded += 1;
        }
        assert_eq!(decoded, lines.len(), "as many records as lines");
    }

    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(Default::default());
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );
            let mut directory = self.settings.foundry_root.clone();
            for segment in &self.settings.generated_directory.segments {
                directory = directory.join(segment.as_str());
            }
            let curve_directory = match P::DECLARATION.curve {
                PairingCurve::Bn254 => self.settings.curve_directories.bn254.as_str(),
                PairingCurve::Bls12381 => self.settings.curve_directories.bls12_381.as_str(),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => self.settings.curve_directories.bn254.as_str(),
            };
            let directory = directory.join(curve_directory);
            let emitted: Vec<VectorKind> =
                <EvmGenerate<'_, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                    .kinds
                    .iter()
                    .filter(|declaration| {
                        !declaration.requires_second_group_arithmetic
                            || P::DECLARATION.verifier_group_arithmetic
                                == VerifierGroupArithmetic::BothGroups
                    })
                    .map(|declaration| declaration.kind)
                    .collect();

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            let Ok(_) = result else {
                panic!("emit admits the case")
            };
            for kind in &emitted {
                let names = &self
                    .settings
                    .kinds
                    .iter()
                    .find(|entry| entry.kind == *kind)
                    .expect("the settings name every emitted kind")
                    .names;
                let path = directory.join(names.file_name.as_str());
                match kind {
                    VectorKind::G1Addition
                    | VectorKind::G1Multiplication
                    | VectorKind::G2Addition
                    | VectorKind::G1MultiScalarMultiplication
                    | VectorKind::G2MultiScalarMultiplication => decode_lines(
                        self.encoder,
                        &path,
                        &build_group_operation_vector_description(Default::default()),
                    ),
                    VectorKind::PairingCheck => decode_lines(
                        self.encoder,
                        &path,
                        &build_pairing_check_vector_description(Default::default()),
                    ),
                    VectorKind::DecodeRejection => decode_lines(
                        self.encoder,
                        &path,
                        &build_rejection_vector_description(Default::default()),
                    ),
                    VectorKind::HashToScalar => decode_lines(
                        self.encoder,
                        &path,
                        &build_hash_to_scalar_vector_description(Default::default()),
                    ),
                    VectorKind::IdentityMapping => decode_lines(
                        self.encoder,
                        &path,
                        &build_identity_mapping_vector_description(Default::default()),
                    ),
                    VectorKind::Possession => decode_lines(
                        self.encoder,
                        &path,
                        &build_possession_vector_description(Default::default()),
                    ),
                    VectorKind::Challenge => decode_lines(
                        self.encoder,
                        &path,
                        &build_challenge_vector_description(Default::default()),
                    ),
                    VectorKind::MintProof => decode_lines(
                        self.encoder,
                        &path,
                        &build_mint_proof_vector_description::<F>(Default::default()),
                    ),
                    VectorKind::TransferProof => decode_lines(
                        self.encoder,
                        &path,
                        &build_transfer_proof_vector_description::<F>(Default::default()),
                    ),
                }
            }
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // every_written_line_decodes_under_its_kinds_description
}

/// Contract: entry
///   `the_constants_library_opens_with_the_configured_license_version_and_
///   name_and_holds_the_tag_entries`; `emit` with a license, a compiler
///   version, and a constants library name each other than its builder
///   default → `Constants.sol`, under the configured file name, begins
///   with the configured license line, the pragma of the configured
///   compiler version, a blank line, and `library` followed by the
///   configured name and ` {`, and holds the challenge-tag line carrying
///   the delivery proof declaration's `challenge_tag` in lowercase hex.
/// Arrange: `license` `Apache-2.0`, `compiler_version` `9.27.111`, and
///   `constants_library.library_name` `TagEntries`; the settings' root
///   removed before the act; two distinct identities; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  the constants library file, read from the root, the generated
///   directory, the curve directory, and its configured file name, starts
///   with the configured license line, pragma, blank line, and `library`
///   opening, and holds the line `    bytes internal constant
///   CHALLENGE_TAG = hex"` followed by the lowercase hex of the delivery
///   proof declaration's `challenge_tag` and `";`.
/// Boundary: `emit`, `constants`, `render`, and the rest of the real chain
///   the private surface names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn the_constants_library_opens_with_the_configured_license_version_and_name_and_holds_the_tag_entries()
 {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("Apache-2.0".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(9),
                minor: Some(27),
                patch: Some(111),
            },
        )),
        constants_library: Some(build_evm_library_names(EvmLibraryNamesOverrides {
            library_name: Some(build_solidity_library_name(
                SolidityLibraryNameConstructorParamsOverrides {
                    text: Some("TagEntries".to_string()),
                },
            )),
            ..Default::default()
        })),
        ..Default::default()
    });
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );
            let mut directory = self.settings.foundry_root.clone();
            for segment in &self.settings.generated_directory.segments {
                directory = directory.join(segment.as_str());
            }
            let curve_directory = match P::DECLARATION.curve {
                PairingCurve::Bn254 => self.settings.curve_directories.bn254.as_str(),
                PairingCurve::Bls12381 => self.settings.curve_directories.bls12_381.as_str(),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => self.settings.curve_directories.bn254.as_str(),
            };
            let directory = directory.join(curve_directory);

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            let Ok(_) = result else {
                panic!("emit admits the case")
            };
            let path = directory.join(self.settings.constants_library.file_name.as_str());
            let contents =
                std::fs::read_to_string(&path).expect("the constants library reads back");
            assert!(
                contents.starts_with(
                    "// SPDX-License-Identifier: Apache-2.0\npragma solidity 9.27.111;\n\nlibrary TagEntries {\n"
                ),
                "the library opens with the configured license, version, and name"
            );
            let challenge_hex: String = D::DECLARATION
                .challenge_tag
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            assert!(
                contents.contains(&format!(
                    "    bytes internal constant CHALLENGE_TAG = hex\"{challenge_hex}\";"
                )),
                "the library holds the delivery proof's challenge tag"
            );
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // the_constants_library_opens_with_the_configured_license_version_and_name_and_holds_the_tag_entries
}

/// Contract: entry
///   `the_records_library_declares_each_written_kinds_struct_decode_
///   function_and_path`; `emit` with a records library name other than its
///   builder default, each kind's names distinct, and a generated
///   directory of two distinct segments → `VectorRecords.sol` begins with
///   the configured header and library name and, for each written kind,
///   holds its struct, its decode function with the kind's struct and
///   name, and its path constant holding the segments, the curve
///   directory, and the kind's file name joined by `/`, and holds no
///   struct, decode function, or path constant of a kind not written.
/// Arrange: `records_library.library_name` `DeliveryVectorRecords`, the
///   `license` `Apache-2.0`, the `compiler_version` `9.27.111`, a
///   `generated_directory` of the two segments `generated` and `vectors`,
///   and the default kind names, already distinct; the settings' root
///   removed before the act; two distinct identities; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  the records library, read from its configured file name,
///   starts with the configured header and library name, and for each
///   written kind holds `    struct {name} {`, `    string internal
///   constant {NAME}_PATH = "{segments}/{curve}/{file}";`, and
///   `    function {decode}(bytes memory line) internal pure returns
///   ({name} memory) {`, and holds `G2AddVector`, `G1MsmVector`, and
///   `G2MsmVector` only on the curve whose verifier has both groups.
/// Boundary: `emit`, `render`, and the rest of the real chain the private
///   surface names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn the_records_library_declares_each_written_kinds_struct_decode_function_and_path() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("Apache-2.0".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(9),
                minor: Some(27),
                patch: Some(111),
            },
        )),
        records_library: Some(build_evm_library_names(EvmLibraryNamesOverrides {
            library_name: Some(build_solidity_library_name(
                SolidityLibraryNameConstructorParamsOverrides {
                    text: Some("DeliveryVectorRecords".to_string()),
                },
            )),
            ..Default::default()
        })),
        generated_directory: Some(build_generated_directory(
            GeneratedDirectoryConstructorParamsOverrides {
                segments: Some(vec![
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("generated".to_string()),
                    }),
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("vectors".to_string()),
                    }),
                ]),
            },
        )),
        ..Default::default()
    });
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );
            let mut directory = self.settings.foundry_root.clone();
            for segment in &self.settings.generated_directory.segments {
                directory = directory.join(segment.as_str());
            }
            let curve_directory = match P::DECLARATION.curve {
                PairingCurve::Bn254 => self.settings.curve_directories.bn254.as_str(),
                PairingCurve::Bls12381 => self.settings.curve_directories.bls12_381.as_str(),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => self.settings.curve_directories.bn254.as_str(),
            };
            let directory = directory.join(curve_directory);
            let emitted: Vec<VectorKind> =
                <EvmGenerate<'_, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                    .kinds
                    .iter()
                    .filter(|declaration| {
                        !declaration.requires_second_group_arithmetic
                            || P::DECLARATION.verifier_group_arithmetic
                                == VerifierGroupArithmetic::BothGroups
                    })
                    .map(|declaration| declaration.kind)
                    .collect();

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            let Ok(_) = result else {
                panic!("emit admits the case")
            };
            let path = directory.join(self.settings.records_library.file_name.as_str());
            let contents = std::fs::read_to_string(&path).expect("the records library reads back");
            assert!(
                contents.starts_with(
                    "// SPDX-License-Identifier: Apache-2.0\npragma solidity 9.27.111;\n\nlibrary DeliveryVectorRecords {\n"
                ),
                "the library opens with the configured header and name"
            );
            let segments = self
                .settings
                .generated_directory
                .segments
                .iter()
                .map(|segment| segment.as_str())
                .collect::<Vec<_>>()
                .join("/");
            for kind in &emitted {
                let (file_name, struct_name, decode_function, path_constant) = match kind {
                    VectorKind::G1Addition => (
                        "g1_add.vectors",
                        "G1AddVector",
                        "decodeG1Add",
                        "G1_ADD_PATH",
                    ),
                    VectorKind::G1Multiplication => (
                        "g1_mul.vectors",
                        "G1MulVector",
                        "decodeG1Mul",
                        "G1_MUL_PATH",
                    ),
                    VectorKind::PairingCheck => (
                        "pairing_check.vectors",
                        "PairingCheckVector",
                        "decodePairingCheck",
                        "PAIRING_CHECK_PATH",
                    ),
                    VectorKind::G2Addition => (
                        "g2_add.vectors",
                        "G2AddVector",
                        "decodeG2Add",
                        "G2_ADD_PATH",
                    ),
                    VectorKind::G1MultiScalarMultiplication => (
                        "g1_msm.vectors",
                        "G1MsmVector",
                        "decodeG1Msm",
                        "G1_MSM_PATH",
                    ),
                    VectorKind::G2MultiScalarMultiplication => (
                        "g2_msm.vectors",
                        "G2MsmVector",
                        "decodeG2Msm",
                        "G2_MSM_PATH",
                    ),
                    VectorKind::DecodeRejection => (
                        "decode_rejection.vectors",
                        "DecodeRejectionVector",
                        "decodeDecodeRejection",
                        "DECODE_REJECTION_PATH",
                    ),
                    VectorKind::HashToScalar => (
                        "hash_to_scalar.vectors",
                        "HashToScalarVector",
                        "decodeHashToScalar",
                        "HASH_TO_SCALAR_PATH",
                    ),
                    VectorKind::IdentityMapping => (
                        "identity_mapping.vectors",
                        "IdentityMappingVector",
                        "decodeIdentityMapping",
                        "IDENTITY_MAPPING_PATH",
                    ),
                    VectorKind::Possession => (
                        "possession.vectors",
                        "PossessionVector",
                        "decodePossession",
                        "POSSESSION_PATH",
                    ),
                    VectorKind::Challenge => (
                        "challenge.vectors",
                        "ChallengeVector",
                        "decodeChallenge",
                        "CHALLENGE_PATH",
                    ),
                    VectorKind::MintProof => (
                        "mint_proof.vectors",
                        "MintProofVector",
                        "decodeMintProof",
                        "MINT_PROOF_PATH",
                    ),
                    VectorKind::TransferProof => (
                        "transfer_proof.vectors",
                        "TransferProofVector",
                        "decodeTransferProof",
                        "TRANSFER_PROOF_PATH",
                    ),
                };
                let relative = format!("{segments}/{curve_directory}/{file_name}");
                assert!(
                    contents.contains(&format!("    struct {struct_name} {{")),
                    "the kind's struct is declared"
                );
                assert!(
                    contents.contains(&format!(
                        "    string internal constant {path_constant} = \"{relative}\";"
                    )),
                    "the kind's path constant holds the relative path"
                );
                assert!(
                    contents.contains(&format!(
                        "    function {decode_function}(bytes memory line) internal pure returns ({struct_name} memory) {{"
                    )),
                    "the kind's decode function is declared"
                );
            }
            for unwritten in ["G2AddVector", "G1MsmVector", "G2MsmVector"] {
                assert_eq!(
                    contents.contains(unwritten),
                    P::DECLARATION.verifier_group_arithmetic == VerifierGroupArithmetic::BothGroups,
                    "the second-group kind's struct is declared only where written"
                );
            }
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // the_records_library_declares_each_written_kinds_struct_decode_function_and_path
}

/// Contract: entry
///   `a_constants_refusal_reaches_the_caller_unchanged_and_writes_nothing`;
///   `emit` with a statement version other than
///   `DELIVERY_STATEMENT_VERSION_ONE` →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Constants(
///   ConstantsErrorReturn::UnsupportedStatementVersion { version })))`,
///   `version` the configured value, and the root does not exist.
/// Arrange: `statement_version` overridden to `2`; the settings' root
///   removed before the act; two distinct identities; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn
///   ::Constants(ConstantsErrorReturn::UnsupportedStatementVersion
///   { version: 2 }))))` and the root does not exist.
/// Boundary: `emit`, `constants`, and the rest of the real chain the
///   private surface names; the refusal is `constants`'s own, carried
///   whole.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn a_constants_refusal_reaches_the_caller_unchanged_and_writes_nothing() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(Default::default());
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    statement_version: Some(2),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Constants(
                        ConstantsErrorReturn::UnsupportedStatementVersion { version: 2 }
                    )))
                ),
                "the constants refusal reaches the caller unchanged"
            );
            assert!(!self.settings.foundry_root.exists(), "nothing is written");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_constants_refusal_reaches_the_caller_unchanged_and_writes_nothing
}

/// Contract: entry
///   `a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_
///   nothing`; `emit` over the hash-to-scalar adapter `create_hash_to_scalar`
///   returns for `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailure
///   Mode::SamplingRefused)` →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MappingVectors(
///   MappingVectorsErrorReturn::HashToScalar(HashToScalarErrorReturn::
///   Sampling(SampleUniformScalarErrorReturn::WrongLength { expected,
///   actual })))))`, `expected` `<P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH` and `actual` `0`, the length the family declares
///   for its `SamplingRefused` mode, and the root does not exist. The
///   absorbed `hash-to-scalar/factory` entry `every_concrete_returns_the_
///   sampling_bound_refusal_whole` is proven here: the refusal reaches the
///   caller of `emit` whole as
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MappingVectors(
///   MappingVectorsErrorReturn::HashToScalar(error))))`.
/// Arrange: the hash-to-scalar adapter from `create_hash_to_scalar` with
///   its selection's `concrete` overridden to the family's mock concrete
///   under `SamplingRefused`, which differs from the builder's default of
///   the declared selection, every other selection real; the settings'
///   root removed before the act; two distinct identities; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!` on the whole nested error with `expected` the
///   scalar's `UNIFORM_BYTES_LENGTH` and `actual` `0`, and the root does
///   not exist.
/// Boundary: `emit`, `mapping_vectors`, the mock concrete's
///   `hash_to_scalar`, and the rest of the real chain the private surface
///   names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the hash-to-scalar family's mock concrete under
///   `SamplingRefused`; the operating-system generator runs real.
#[test]
fn a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(Default::default());
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MappingVectors(
                        MappingVectorsErrorReturn::HashToScalar(
                            HashToScalarErrorReturn::Sampling(
                                SampleUniformScalarErrorReturn::WrongLength { expected, actual }
                            )
                        )
                    ))) if expected
                        == <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                        && actual == 0
                ),
                "the mapping-vectors refusal reaches the caller unchanged"
            );
            assert!(!self.settings.foundry_root.exists(), "nothing is written");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            ) else {
                panic!("the mock selection admits under the identifier it declares")
            };
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &consumed.adapter,
                        hashing: &*hashing.adapter,
                        random: self.random,
                        verifier_form,
                        settings: self.settings,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(encoding_selection.concrete),
                        identifier: Some(encoding_selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing
}

/// Contract: entry
///   `a_repeated_struct_name_is_refused_with_its_indexes_and_writes_
///   nothing`; `emit` over settings whose first two emitted kinds' entries
///   carry the same struct name →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
///   SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName
///   { first_index: 0, duplicate_index: 1 })))`, the error whole, and the
///   root does not exist.
/// Arrange: `settings.kinds` one `build_evm_vector_kind_settings` per kind
///   the default list names, the first two emitted kinds' entries equal in
///   the struct name `SharedVector` and distinct in every other name, the
///   remaining entries at their defaults; the settings' root removed
///   before the act; two distinct identities; every case the private
///   surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!` on the whole nested error and the root does not
///   exist.
/// Boundary: `emit`, the declarations' `try_new` through `render`, and the
///   rest of the real chain the private surface names; the refusal occurs
///   before any directory is created.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn a_repeated_struct_name_is_refused_with_its_indexes_and_writes_nothing() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let default = build_evm_generate_settings(Default::default());
    let shared = build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
        text: Some("SharedVector".to_string()),
    });
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(if index < 2 {
                    build_evm_vector_kind_names(EvmVectorKindNamesOverrides {
                        file_name: Some(entry.names.file_name),
                        struct_name: Some(shared.clone()),
                        decode_function_name: Some(entry.names.decode_function_name),
                        path_constant_name: Some(entry.names.path_constant_name),
                    })
                } else {
                    entry.names
                }),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
                        SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                            first_index: 0,
                            duplicate_index: 1
                        }
                    )))
                ),
                "the declarations refusal reaches the caller unchanged"
            );
            assert!(!self.settings.foundry_root.exists(), "nothing is written");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_repeated_struct_name_is_refused_with_its_indexes_and_writes_nothing
}

/// Contract: entry
///   `a_repeated_decode_function_name_is_refused_with_its_indexes_and_
///   writes_nothing`; `emit` over settings whose first two emitted kinds'
///   entries carry the same decode function name →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
///   SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicateDecodeFunctionName { first_index: 0, duplicate_index: 1
///   })))`, the error whole, and the root does not exist.
/// Arrange: `settings.kinds` one `build_evm_vector_kind_settings` per kind
///   the default list names, the first two emitted kinds' entries equal in
///   the decode function name `sharedDecode` and distinct in every other
///   name, the remaining entries at their defaults; the settings' root
///   removed before the act; two distinct identities; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!` on the whole nested error and the root does not
///   exist.
/// Boundary: `emit`, the declarations' `try_new` through `render`, and the
///   rest of the real chain the private surface names; the refusal occurs
///   before any directory is created.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn a_repeated_decode_function_name_is_refused_with_its_indexes_and_writes_nothing() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let default = build_evm_generate_settings(Default::default());
    let shared = build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
        text: Some("sharedDecode".to_string()),
    });
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(if index < 2 {
                    build_evm_vector_kind_names(EvmVectorKindNamesOverrides {
                        file_name: Some(entry.names.file_name),
                        struct_name: Some(entry.names.struct_name),
                        decode_function_name: Some(shared.clone()),
                        path_constant_name: Some(entry.names.path_constant_name),
                    })
                } else {
                    entry.names
                }),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
                        SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                            first_index: 0,
                            duplicate_index: 1
                        }
                    )))
                ),
                "the declarations refusal reaches the caller unchanged"
            );
            assert!(!self.settings.foundry_root.exists(), "nothing is written");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_repeated_decode_function_name_is_refused_with_its_indexes_and_writes_nothing
}

/// Contract: entry
///   `a_repeated_path_constant_name_is_refused_with_its_indexes_and_
///   writes_nothing`; `emit` over settings whose first two emitted kinds'
///   entries carry the same path constant name →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
///   SolidityRecordDeclarationsTryNewErrorReturn::
///   DuplicatePathConstantName { first_index: 0, duplicate_index: 1
///   })))`, the error whole, and the root does not exist.
/// Arrange: `settings.kinds` one `build_evm_vector_kind_settings` per kind
///   the default list names, the first two emitted kinds' entries equal in
///   the path constant name `SHARED_PATH` and distinct in every other
///   name, the remaining entries at their defaults; the settings' root
///   removed before the act; two distinct identities; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!` on the whole nested error and the root does not
///   exist.
/// Boundary: `emit`, the declarations' `try_new` through `render`, and the
///   rest of the real chain the private surface names; the refusal occurs
///   before any directory is created.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn a_repeated_path_constant_name_is_refused_with_its_indexes_and_writes_nothing() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let default = build_evm_generate_settings(Default::default());
    let shared = build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
        text: Some("SHARED_PATH".to_string()),
    });
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(if index < 2 {
                    build_evm_vector_kind_names(EvmVectorKindNamesOverrides {
                        file_name: Some(entry.names.file_name),
                        struct_name: Some(entry.names.struct_name),
                        decode_function_name: Some(entry.names.decode_function_name),
                        path_constant_name: Some(shared.clone()),
                    })
                } else {
                    entry.names
                }),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
                        SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName {
                            first_index: 0,
                            duplicate_index: 1
                        }
                    )))
                ),
                "the declarations refusal reaches the caller unchanged"
            );
            assert!(!self.settings.foundry_root.exists(), "nothing is written");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_repeated_path_constant_name_is_refused_with_its_indexes_and_writes_nothing
}

/// Contract: entry
///   `a_repeated_struct_name_is_refused_before_a_repeated_decode_function_
///   name`; `emit` over settings whose first two emitted kinds' entries
///   carry the same struct name and the same decode function name →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
///   SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName
///   { first_index: 0, duplicate_index: 1 })))`, the error whole, and the
///   root does not exist.
/// Arrange: `settings.kinds` one `build_evm_vector_kind_settings` per kind
///   the default list names, the first two emitted kinds' entries equal in
///   the struct name `SharedVector` and in the decode function name
///   `sharedDecode` and distinct in every other name, the remaining
///   entries at their defaults; the settings' root removed before the
///   act; two distinct identities; every case the private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!` on the whole nested error, the struct name's
///   duplicate refused where the decode function name repeats as well,
///   and the root does not exist.
/// Boundary: `emit`, the declarations' `try_new` through `render`, and the
///   rest of the real chain the private surface names; the refusal occurs
///   before any directory is created.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn a_repeated_struct_name_is_refused_before_a_repeated_decode_function_name() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let default = build_evm_generate_settings(Default::default());
    let shared_struct = build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
        text: Some("SharedVector".to_string()),
    });
    let shared_decode = build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
        text: Some("sharedDecode".to_string()),
    });
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(if index < 2 {
                    build_evm_vector_kind_names(EvmVectorKindNamesOverrides {
                        file_name: Some(entry.names.file_name),
                        struct_name: Some(shared_struct.clone()),
                        decode_function_name: Some(shared_decode.clone()),
                        path_constant_name: Some(entry.names.path_constant_name),
                    })
                } else {
                    entry.names
                }),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(
                        SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                            first_index: 0,
                            duplicate_index: 1
                        }
                    )))
                ),
                "the struct name's duplicate is refused where the decode function name repeats as well"
            );
            assert!(!self.settings.foundry_root.exists(), "nothing is written");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_repeated_struct_name_is_refused_before_a_repeated_decode_function_name
}

/// Contract: entry
///   `a_root_that_is_a_file_refuses_directory_creation_with_its_path`;
///   `emit` over a regular file written at the root after the root's
///   removal →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::CreateDirectory
///   { path, error }))`, `path` the root joined with the generated
///   directory's segments and the case's curve directory, and `error` the
///   filesystem's refusal, equal in `kind()`, `raw_os_error()`, and
///   `to_string()` to the error `std::fs::create_dir_all` yields over that
///   path after `emit`.
/// Arrange: the settings' root removed, then a regular file written at the
///   root by `std::fs::write`; two distinct identities; every case the
///   private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!` on `CreateDirectory { path, error }` with `path`
///   the generated directory's path under the case's curve directory and
///   `error` equal in `kind()`, `raw_os_error()`, and `to_string()` to the
///   error the block obtains by calling `std::fs::create_dir_all` over
///   that path after `emit`.
/// Boundary: `emit`, its `std::fs::create_dir_all` over the generated
///   path, and the rest of the real chain the private surface names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn a_root_that_is_a_file_refuses_directory_creation_with_its_path() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(Default::default());
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            std::fs::write(&self.settings.foundry_root, b"not a directory")
                .expect("a regular file writes at the root");
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );
            let mut directory = self.settings.foundry_root.clone();
            for segment in &self.settings.generated_directory.segments {
                directory = directory.join(segment.as_str());
            }
            let curve_directory = match P::DECLARATION.curve {
                PairingCurve::Bn254 => self.settings.curve_directories.bn254.as_str(),
                PairingCurve::Bls12381 => self.settings.curve_directories.bls12_381.as_str(),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => self.settings.curve_directories.bn254.as_str(),
            };
            let directory = directory.join(curve_directory);

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            let reference = std::fs::create_dir_all(&directory)
                .expect_err("the file at the root refuses the directory");
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::CreateDirectory {
                        path,
                        error
                    })) if path == directory
                        && error.kind() == reference.kind()
                        && error.raw_os_error() == reference.raw_os_error()
                        && error.to_string() == reference.to_string()
                ),
                "the directory creation refusal carries its path and error"
            );
            let _ = std::fs::remove_file(&self.settings.foundry_root);
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_root_that_is_a_file_refuses_directory_creation_with_its_path
}

/// Contract: entry
///   `a_directory_at_a_kinds_file_path_refuses_the_write_with_its_path`;
///   `emit` over a directory created at the first emitted kind's file path
///   → `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::WriteFile
///   { path, error }))`, `path` that file path, `error` the filesystem's
///   refusal equal in `kind()`, `raw_os_error()`, and `to_string()` to the
///   error `std::fs::write` yields over it after `emit`, and the constants
///   library's file does not exist.
/// Arrange: the settings' root removed, then `std::fs::create_dir_all`
///   over the first emitted kind's file path, which creates the root, the
///   segments, the curve directory, and a directory at the kind's file
///   name; two distinct identities; every case the private surface names.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!` on `WriteFile { path, error }` with `path` the first
///   emitted kind's file path and `error` equal in `kind()`,
///   `raw_os_error()`, and `to_string()` to the error the block obtains by
///   calling `std::fs::write` over it after `emit`, and the constants
///   library's file does not exist.
/// Boundary: `emit`, its first `std::fs::write` over a kind's file path,
///   and the rest of the real chain the private surface names.
/// Mocked:  the filesystem, a directory beneath the operating system's
///   temporary directory; the operating-system generator runs real.
#[test]
fn a_directory_at_a_kinds_file_path_refuses_the_write_with_its_path() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let settings = build_evm_generate_settings(Default::default());
    let _ = std::fs::remove_dir_all(&settings.foundry_root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        settings: &'a EvmGenerateSettings,
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
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let _ = std::fs::remove_dir_all(&self.settings.foundry_root);
            let mut directory = self.settings.foundry_root.clone();
            for segment in &self.settings.generated_directory.segments {
                directory = directory.join(segment.as_str());
            }
            let curve_directory = match P::DECLARATION.curve {
                PairingCurve::Bn254 => self.settings.curve_directories.bn254.as_str(),
                PairingCurve::Bls12381 => self.settings.curve_directories.bls12_381.as_str(),
                #[cfg(any(test, feature = "mocks"))]
                PairingCurve::Mock => self.settings.curve_directories.bn254.as_str(),
            };
            let directory = directory.join(curve_directory);
            let first_emitted = <EvmGenerate<'_, P, E, K, A, D> as IGenerateAdapter>::DECLARATION
                .kinds
                .iter()
                .find(|declaration| {
                    !declaration.requires_second_group_arithmetic
                        || P::DECLARATION.verifier_group_arithmetic
                            == VerifierGroupArithmetic::BothGroups
                })
                .expect("the declaration holds an emitted kind")
                .kind;
            let kind_path = directory.join(
                self.settings
                    .kinds
                    .iter()
                    .find(|entry| entry.kind == first_emitted)
                    .expect("the settings name the first emitted kind")
                    .names
                    .file_name
                    .as_str(),
            );
            std::fs::create_dir_all(&kind_path)
                .expect("a directory is created at the first emitted kind's file path");
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &identities[0],
                Default::default(),
            );

            // Act
            let result = subject.emit(build_emit_params(Default::default()), emit_payload);

            // Assert
            let reference =
                std::fs::write(&kind_path, b"").expect_err("the directory refuses the write");
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::WriteFile {
                        path,
                        error
                    })) if path == kind_path
                        && error.kind() == reference.kind()
                        && error.raw_os_error() == reference.raw_os_error()
                        && error.to_string() == reference.to_string()
                ),
                "the write refusal carries its path and error"
            );
            assert!(
                !directory
                    .join(self.settings.constants_library.file_name.as_str())
                    .exists(),
                "the constants library is not written"
            );
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                            settings: self.settings,
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
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
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
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            settings: self.settings,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared encoding selection is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        settings: &'a EvmGenerateSettings,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
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
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            settings: self.settings,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        settings: &settings,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
    // a_directory_at_a_kinds_file_path_refuses_the_write_with_its_path
}
