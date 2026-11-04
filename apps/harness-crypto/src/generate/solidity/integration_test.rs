#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CanonicalFieldKind, CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, DecodeParams,
    IDecoderAdapter, IEncoderAdapter, IEncodingContract, build_create_encoding_params,
};
use envelope::{DECLARED_KEY_AGREEMENT_SELECTIONS, IKeyAgreementAdapter};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarIdentifier, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{DECLARED_KEM_SELECTIONS, ICredentialKemAdapter};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    PAIRING_CONCRETES, PairingConcrete, PairingCurve, SampleUniformScalarErrorReturn,
    VerifierGroupArithmetic, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, IDeliveryProofAdapter, MintPurpose, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};
use std::cell::RefCell;

use super::super::factory::provides::{
    EmitErrorReturn, EmitPayloadOverrides, EmitReturn, IGenerateAdapter, VectorKind,
    VectorKindSettings, VectorKindSettingsOverrides, VectorKindSize, build_emit_params,
    build_emit_payload, build_vector_kind_settings,
};
use super::challenge_vectors::provides::build_challenge_vector_description;
use super::constants::provides::ConstantsErrorReturn;
use super::group_vectors::provides::{
    MsmTermCountConstructorParamsOverrides, VectorCountConstructorParamsOverrides,
    build_group_operation_vector_description, build_msm_term_count,
    build_pairing_check_vector_description, build_vector_count,
};
use super::interface::{
    ISolidityGenerateCaseConsumer, SOLIDITY_GENERATE_DECLARATION, SolidityGenerateErrorReturn,
    SolidityGenerateSettings,
};
use super::mapping_vectors::provides::{
    MappingVectorsErrorReturn, build_hash_to_scalar_vector_description,
    build_identity_mapping_vector_description,
};
use super::mock::*;
use super::possession_vectors::provides::{
    POSSESSION_VECTOR_MEMBER_NAMES, PossessionVectorDescription,
    build_possession_vector_description,
};
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
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: `emit` writes each emitted kind's file in declared order, then the
///   constants library's file, then the records library's, under the root, the
///   generated directory's segments, and the case's curve directory, each
///   existing.
/// Arrange: `generated_directory` of two distinct segments, `curve_directories`
///   of two distinct texts, file names distinct across the kinds and the
///   libraries; every case the private surface names: `PAIRING_CONCRETES` ×
///   the `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `Keccak256V1` ×
///   `DECLARED_ENCODING_SELECTIONS` × `CHAIN_FORMS_CONCRETES` ×
///   `DECLARED_KEM_SELECTIONS` × `DECLARED_KEY_AGREEMENT_SELECTIONS` ×
///   `DELIVERY_PROOF_CONCRETES` over `RandomSourceKind::OperatingSystem`.
/// Act:    `emit(build_emit_params(Default::default()), payload)` in the case
///   consumer's method, once per case through `run_solidity_generate_case`.
/// Assert: the runner's return `is_ok()`; `files` holds one entry per expected
///   path in order, each entry's `path` equal to it and existing, the entry's
///   `kind` `Some` of the kind whose file it is for a vector file and `None`
///   for a library.
/// Boundary: `emit` and the real chain `create_pairing` → `create_hash_to_scalar`
///   → `create_encoding` → `create_chain_forms` → `create_kem` →
///   `create_key_agreement` → `create_delivery_proof` → `create_random_source`.
/// Mocked:   the filesystem — a directory beneath the operating system's
///   temporary directory — and the operating system's generator, running real.
#[test]
fn emit_writes_each_emitted_kind_then_both_libraries_in_write_order_under_the_curve_directory() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        generated_directory: Some(build_generated_directory(
            GeneratedDirectoryConstructorParamsOverrides {
                segments: Some(vec![
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("alpha".to_string()),
                    }),
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("beta".to_string()),
                    }),
                ]),
            },
        )),
        curve_directories: Some(build_solidity_curve_directories(
            SolidityCurveDirectoriesOverrides {
                bn254: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("first-curve".to_string()),
                    },
                )),
                bls12_381: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("second-curve".to_string()),
                    },
                )),
            },
        )),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
        curve: RefCell<Option<PairingCurve>>,
        arithmetic: RefCell<Option<VerifierGroupArithmetic>>,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            *self.curve.borrow_mut() = Some(P::DECLARATION.curve);
            *self.arithmetic.borrow_mut() = Some(P::DECLARATION.verifier_group_arithmetic);
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let deps = build_solidity_generate_case_deps(
                                    Case {
                                        settings: settings.clone(),
                                        curve: RefCell::new(None),
                                        arithmetic: RefCell::new(None),
                                    },
                                    Default::default(),
                                );
                                let result = run_solidity_generate_case(
                                    &deps,
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let curve_ref = deps.consumer.curve.borrow();
                                let curve = curve_ref.as_ref().expect("the case ran");
                                let arithmetic =
                                    (*deps.consumer.arithmetic.borrow()).expect("the case ran");
                                let curve_directory = match curve {
                                    PairingCurve::Bn254 => {
                                        settings.curve_directories.bn254.as_str()
                                    }
                                    PairingCurve::Bls12381 => {
                                        settings.curve_directories.bls12_381.as_str()
                                    }
                                    PairingCurve::Mock => {
                                        panic!("PAIRING_CONCRETES names no Mock pairing")
                                    }
                                };
                                let emitted: Vec<VectorKind> = SOLIDITY_GENERATE_DECLARATION
                                    .kinds
                                    .iter()
                                    .filter(|declaration| match arithmetic {
                                        VerifierGroupArithmetic::FirstGroupOnly => {
                                            !declaration.requires_second_group_arithmetic
                                        }
                                        VerifierGroupArithmetic::BothGroups => true,
                                    })
                                    .map(|declaration| declaration.kind)
                                    .collect();
                                let result = result.expect("the case chain runs clean").output;
                                let Ok(success) = result else {
                                    panic!("emit admits the case's arrangement")
                                };
                                assert_eq!(success.files.len(), emitted.len() + 2);
                                for (index, kind) in emitted.iter().enumerate() {
                                    let names = settings
                                        .kinds
                                        .iter()
                                        .find(|entry| entry.kind == *kind)
                                        .map(|entry| &entry.names)
                                        .expect("the settings hold every emitted kind");
                                    let expected = root
                                        .join("alpha")
                                        .join("beta")
                                        .join(curve_directory)
                                        .join(names.file_name.as_str());
                                    assert_eq!(success.files[index].kind, Some(*kind));
                                    assert_eq!(success.files[index].path, expected);
                                    assert!(expected.exists());
                                }
                                let constants_path = root
                                    .join("alpha")
                                    .join("beta")
                                    .join(curve_directory)
                                    .join(settings.constants_library.file_name.as_str());
                                assert_eq!(success.files[emitted.len()].kind, None);
                                assert_eq!(success.files[emitted.len()].path, constants_path);
                                assert!(constants_path.exists());
                                let records_path = root
                                    .join("alpha")
                                    .join("beta")
                                    .join(curve_directory)
                                    .join(settings.records_library.file_name.as_str());
                                assert_eq!(success.files[emitted.len() + 1].kind, None);
                                assert_eq!(success.files[emitted.len() + 1].path, records_path);
                                assert!(records_path.exists());
                            }
                        }
                    }
                }
            }
        }
    }
}
// emit_writes_each_emitted_kind_then_both_libraries_in_write_order_under_the_curve_directory

/// Contract: each kind's file holds one hex line per record at the kind's
///   admitted size, each line `0x` and lowercase hex, each file ending in a
///   line feed.
/// Arrange: `sizes` with every kind's count distinct from every other's, the
///   term count and the message length distinct from the counts, and two
///   identities; every case the private surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: each drawn kind's file holds its count of lines except the
///   challenge kind's, which holds twice its count, the hash-to-scalar file
///   holds its count for each of the identity, possession first-group,
///   possession second-group, challenge, and weight domain tags, the
///   identity-mapping file holds one line per configured identity, and the
///   decode-rejection file holds lines, every line `0x` followed by an even
///   run of lowercase hex digits, each file ending in `\n` and holding no
///   empty line.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn each_kinds_file_holds_one_hex_line_per_record_at_its_admitted_size() {
    // Arrange
    let count = |value: u32| {
        build_vector_count(VectorCountConstructorParamsOverrides { count: Some(value) })
    };
    let drawn = |kind: VectorKind, value: u32| {
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(kind),
            size: Some(VectorKindSize::Drawn {
                count: count(value),
            }),
        })
    };
    let sizes: Vec<VectorKindSettings> = vec![
        drawn(VectorKind::G1Addition, 3),
        drawn(VectorKind::G1Multiplication, 5),
        drawn(VectorKind::PairingCheck, 7),
        drawn(VectorKind::G2Addition, 11),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::G1MultiScalarMultiplication),
            size: Some(VectorKindSize::DrawnTerms {
                count: count(13),
                terms: build_msm_term_count(MsmTermCountConstructorParamsOverrides {
                    count: Some(19),
                }),
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::G2MultiScalarMultiplication),
            size: Some(VectorKindSize::DrawnTerms {
                count: count(17),
                terms: build_msm_term_count(MsmTermCountConstructorParamsOverrides {
                    count: Some(23),
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
                count: count(29),
                message_length: 47,
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::IdentityMapping),
            size: Some(VectorKindSize::OverConfiguredInputs),
        }),
        drawn(VectorKind::Possession, 31),
        drawn(VectorKind::Challenge, 37),
        drawn(VectorKind::MintProof, 41),
        drawn(VectorKind::TransferProof, 43),
    ];
    let expected_lines = |kind: VectorKind| -> usize {
        match kind {
            VectorKind::G1Addition => 3,
            VectorKind::G1Multiplication => 5,
            VectorKind::PairingCheck => 7,
            VectorKind::G2Addition => 11,
            VectorKind::G1MultiScalarMultiplication => 13,
            VectorKind::G2MultiScalarMultiplication => 17,
            VectorKind::DecodeRejection => usize::MAX,
            VectorKind::HashToScalar => 5 * 29,
            VectorKind::IdentityMapping => 2,
            VectorKind::Possession => 31,
            VectorKind::Challenge => 2 * 37,
            VectorKind::MintProof => 41,
            VectorKind::TransferProof => 43,
        }
    };
    let settings = build_solidity_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        sizes: Vec<VectorKindSettings>,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    sizes: Some(self.sizes.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(
                                        Case {
                                            sizes: sizes.clone(),
                                        },
                                        Default::default(),
                                    ),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                let Ok(success) = result else {
                                    panic!("emit admits the case's arrangement")
                                };
                                for file in &success.files {
                                    let Some(kind) = file.kind else {
                                        continue;
                                    };
                                    let text = std::fs::read_to_string(&file.path)
                                        .expect("the kind's file was written");
                                    let lines: Vec<&str> = text.lines().collect();
                                    if kind == VectorKind::DecodeRejection {
                                        assert!(!lines.is_empty());
                                    } else {
                                        assert_eq!(lines.len(), expected_lines(kind));
                                    }
                                    for line in &lines {
                                        assert!(line.starts_with("0x"));
                                        let hex = &line[2..];
                                        assert!(hex.len() % 2 == 0);
                                        assert!(hex.chars().all(|character| {
                                            character.is_ascii_hexdigit()
                                                && !character.is_ascii_uppercase()
                                        }));
                                    }
                                    assert!(text.ends_with('\n'));
                                    assert!(lines.iter().all(|line| !line.is_empty()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
// each_kinds_file_holds_one_hex_line_per_record_at_its_admitted_size

/// Contract: each written line, its hex after `0x` read back into bytes,
///   decodes through the case's encoder under the description of the kind
///   that wrote it.
/// Arrange: the default `sizes`; every case the private surface names, so
///   every member of `DECLARED_ENCODING_SELECTIONS` runs.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: every written file's lines, read back from their hex into bytes,
///   each return `Ok` from `decode` under the kind's description, and the
///   decoded records are as many as the file's lines, for every kind written.
/// Boundary: `emit`, `decode`, and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn every_written_line_decodes_under_its_kinds_description() {
    // Arrange
    let settings = build_solidity_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case;

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                Default::default(),
            );
            let result = generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            );
            let group_operation = build_group_operation_vector_description(Default::default());
            let pairing_check_description =
                build_pairing_check_vector_description(Default::default());
            let rejection = build_rejection_vector_description(Default::default());
            let hash_to_scalar_description =
                build_hash_to_scalar_vector_description(Default::default());
            let identity_mapping = build_identity_mapping_vector_description(Default::default());
            let possession = build_possession_vector_description(Default::default());
            let challenge = build_challenge_vector_description(Default::default());
            let mint_proof = build_mint_proof_vector_description::<F>(Default::default());
            let transfer_proof = build_transfer_proof_vector_description::<F>(Default::default());
            let hex_bytes = |hex: &str| -> Vec<u8> {
                (0..hex.len())
                    .step_by(2)
                    .map(|index| {
                        u8::from_str_radix(&hex[index..index + 2], 16)
                            .expect("every written line is hex")
                    })
                    .collect()
            };
            if let Ok(success) = &result {
                for file in &success.files {
                    let Some(kind) = file.kind else {
                        continue;
                    };
                    let text =
                        std::fs::read_to_string(&file.path).expect("the kind's file was written");
                    let decoded = match kind {
                        VectorKind::G1Addition
                        | VectorKind::G1Multiplication
                        | VectorKind::G2Addition
                        | VectorKind::G1MultiScalarMultiplication
                        | VectorKind::G2MultiScalarMultiplication => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &group_operation,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::PairingCheck => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &pairing_check_description,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::DecodeRejection => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &rejection,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::HashToScalar => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &hash_to_scalar_description,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::IdentityMapping => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &identity_mapping,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::Possession => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &possession,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::Challenge => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &challenge,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::MintProof => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &mint_proof,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                        VectorKind::TransferProof => {
                            let mut decoded = 0usize;
                            for line in text.lines() {
                                assert!(
                                    decoder
                                        .decode(
                                            DecodeParams {
                                                description: &transfer_proof,
                                            },
                                            hex_bytes(&line[2..]).as_slice(),
                                        )
                                        .is_ok()
                                );
                                decoded += 1;
                            }
                            decoded
                        }
                    };
                    assert_eq!(decoded, text.lines().count());
                }
            }
            result
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(Case, Default::default()),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                assert!(result.is_ok());
                            }
                        }
                    }
                }
            }
        }
    }
}
// every_written_line_decodes_under_its_kinds_description

/// Contract: `Constants.sol` opens with the configured license, compiler
///   version, and library name and holds the delivery proof declaration's
///   challenge tag.
/// Arrange: `license` and `compiler_version` each built with a value other
///   than its builder default and `constants_library`'s `library_name` other
///   than `Constants`; every case the private surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`; the file under the configured file
///   name begins with `// SPDX-License-Identifier: ` and the configured
///   license, `\npragma solidity ` and the configured version, `;\n\nlibrary `
///   and the configured name, and ` {\n`, and holds the line
///   `    bytes internal constant CHALLENGE_TAG = hex"` followed by the
///   lowercase hex of the declaration's `challenge_tag` and `";`.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn the_constants_library_opens_with_the_configured_license_version_and_name_and_holds_the_tag_entries()
 {
    // Arrange
    let license = build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
        text: Some("Apache-2.0".to_string()),
    });
    let compiler_version = build_solidity_compiler_version(SolidityCompilerVersionOverrides {
        major: Some(0),
        minor: Some(9),
        patch: Some(1),
    });
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        license: Some(license.clone()),
        compiler_version: Some(compiler_version),
        constants_library: Some(build_solidity_library_names(
            SolidityLibraryNamesOverrides {
                file_name: Some(build_generated_file_name(
                    GeneratedFileNameConstructorParamsOverrides {
                        text: Some("CustomConstants.sol".to_string()),
                    },
                )),
                library_name: Some(build_solidity_library_name(
                    SolidityLibraryNameConstructorParamsOverrides {
                        text: Some("ConfiguredConstants".to_string()),
                    },
                )),
            },
        )),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
        challenge_tag: RefCell<Option<&'static [u8]>>,
        curve: RefCell<Option<PairingCurve>>,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            *self.challenge_tag.borrow_mut() = Some(D::DECLARATION.challenge_tag);
            *self.curve.borrow_mut() = Some(P::DECLARATION.curve);
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let deps = build_solidity_generate_case_deps(
                                    Case {
                                        settings: settings.clone(),
                                        challenge_tag: RefCell::new(None),
                                        curve: RefCell::new(None),
                                    },
                                    Default::default(),
                                );
                                let result = run_solidity_generate_case(
                                    &deps,
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let challenge_tag =
                                    (*deps.consumer.challenge_tag.borrow()).expect("the case ran");
                                let curve_ref = deps.consumer.curve.borrow();
                                let curve = curve_ref.as_ref().expect("the case ran");
                                let curve_directory = match curve {
                                    PairingCurve::Bn254 => {
                                        settings.curve_directories.bn254.as_str()
                                    }
                                    PairingCurve::Bls12381 => {
                                        settings.curve_directories.bls12_381.as_str()
                                    }
                                    PairingCurve::Mock => {
                                        panic!("PAIRING_CONCRETES names no Mock pairing")
                                    }
                                };
                                let result = result.expect("the case chain runs clean").output;
                                let Ok(_success) = result else {
                                    panic!("emit admits the case's arrangement")
                                };
                                let library_path = root
                                    .join("generated")
                                    .join(curve_directory)
                                    .join(settings.constants_library.file_name.as_str());
                                let text = std::fs::read_to_string(&library_path)
                                    .expect("the constants library was written");
                                assert!(text.starts_with(&format!(
                                    "// SPDX-License-Identifier: {}\npragma solidity {}.{}.{};\n\nlibrary {} {{\n",
                                    license.as_str(),
                                    compiler_version.major,
                                    compiler_version.minor,
                                    compiler_version.patch,
                                    settings.constants_library.library_name.as_str()
                                )));
                                let tag_hex = challenge_tag
                                    .iter()
                                    .map(|byte| format!("{byte:02x}"))
                                    .collect::<String>();
                                assert!(text.contains(&format!(
                                    "    bytes internal constant CHALLENGE_TAG = hex\"{}\";",
                                    tag_hex
                                )));
                            }
                        }
                    }
                }
            }
        }
    }
}
// the_constants_library_opens_with_the_configured_license_version_and_name_and_holds_the_tag_entries

/// Contract: `VectorRecords.sol` declares for each written kind its struct
///   with its members typed by the Ethereum ABI encoder's names for their
///   kinds, decode function, and path constant, the path joining the generated
///   directory's segments, the curve directory, and the kind's file name by
///   `/`, and declares no kind not written.
/// Arrange: `records_library`'s `library_name` other than `VectorRecords`,
///   each kind's names distinct, and `generated_directory` of two distinct
///   segments; every case the private surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`; the records library, read from its
///   configured file name, holds the configured header and library name, and
///   for the possession kind `    struct PossessionVector {` followed by one
///   member line per field of its description, each member's name the
///   description's `member_names()` entry and its type the name assembled in
///   the block from the ABI table for the kind the description's `FIELDS`
///   holds at its index, `    string internal constant POSSESSION_PATH = "`
///   followed by the segments, the curve directory text, and
///   `possession.vectors` joined by `/` and `";`, and
///   `    function decodePossession(bytes memory line) internal pure returns (PossessionVector memory) {`,
///   and holds `G2AddVector` only on the curve whose verifier writes the
///   second-group kinds.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn the_records_library_declares_each_written_kinds_struct_decode_function_and_path() {
    // Arrange
    let abi_name = |kind: &CanonicalFieldKind| -> &'static str {
        match kind {
            CanonicalFieldKind::FixedBytes32 => "bytes32",
            CanonicalFieldKind::Unsigned16 => "uint16",
            CanonicalFieldKind::Unsigned32 => "uint32",
            CanonicalFieldKind::Unsigned64 => "uint64",
            CanonicalFieldKind::Text => "string",
            CanonicalFieldKind::Bytes => "bytes",
            CanonicalFieldKind::FixedBytes20 => "bytes20",
            CanonicalFieldKind::Unsigned256 => "uint256",
        }
    };
    let member_names = POSSESSION_VECTOR_MEMBER_NAMES;
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        generated_directory: Some(build_generated_directory(
            GeneratedDirectoryConstructorParamsOverrides {
                segments: Some(vec![
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("suite".to_string()),
                    }),
                    build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                        text: Some("generated".to_string()),
                    }),
                ]),
            },
        )),
        records_library: Some(build_solidity_library_names(
            SolidityLibraryNamesOverrides {
                library_name: Some(build_solidity_library_name(
                    SolidityLibraryNameConstructorParamsOverrides {
                        text: Some("ConfiguredRecords".to_string()),
                    },
                )),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
        curve: RefCell<Option<PairingCurve>>,
        arithmetic: RefCell<Option<VerifierGroupArithmetic>>,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            *self.curve.borrow_mut() = Some(P::DECLARATION.curve);
            *self.arithmetic.borrow_mut() = Some(P::DECLARATION.verifier_group_arithmetic);
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let deps = build_solidity_generate_case_deps(
                                    Case {
                                        settings: settings.clone(),
                                        curve: RefCell::new(None),
                                        arithmetic: RefCell::new(None),
                                    },
                                    Default::default(),
                                );
                                let result = run_solidity_generate_case(
                                    &deps,
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let curve_ref = deps.consumer.curve.borrow();
                                let curve = curve_ref.as_ref().expect("the case ran");
                                let arithmetic =
                                    (*deps.consumer.arithmetic.borrow()).expect("the case ran");
                                let curve_directory = match curve {
                                    PairingCurve::Bn254 => {
                                        settings.curve_directories.bn254.as_str()
                                    }
                                    PairingCurve::Bls12381 => {
                                        settings.curve_directories.bls12_381.as_str()
                                    }
                                    PairingCurve::Mock => {
                                        panic!("PAIRING_CONCRETES names no Mock pairing")
                                    }
                                };
                                let result = result.expect("the case chain runs clean").output;
                                let Ok(_success) = result else {
                                    panic!("emit admits the case's arrangement")
                                };
                                let library_path = root
                                    .join("suite")
                                    .join("generated")
                                    .join(curve_directory)
                                    .join(settings.records_library.file_name.as_str());
                                let text = std::fs::read_to_string(&library_path)
                                    .expect("the records library was written");
                                assert!(text.contains(&format!(
                                    "library {} {{\n",
                                    settings.records_library.library_name.as_str()
                                )));
                                assert!(text.contains("    struct PossessionVector {"));
                                for (index, kind) in
                                    <PossessionVectorDescription as IEncodingContract>::FIELDS
                                        .iter()
                                        .enumerate()
                                {
                                    assert!(text.contains(&format!(
                                        "        {} {};",
                                        abi_name(kind),
                                        member_names[index]
                                    )));
                                }
                                assert!(text.contains(&format!(
                                    "    string internal constant POSSESSION_PATH = \"{}\";",
                                    format_args!(
                                        "suite/generated/{}/possession.vectors",
                                        curve_directory
                                    )
                                )));
                                assert!(text.contains(
                                    "    function decodePossession(bytes memory line) internal pure returns (PossessionVector memory) {"
                                ));
                                assert_eq!(
                                    text.contains("G2AddVector"),
                                    matches!(arithmetic, VerifierGroupArithmetic::BothGroups)
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
// the_records_library_declares_each_written_kinds_struct_decode_function_and_path

/// Contract: a statement version other than `DELIVERY_STATEMENT_VERSION_ONE`
///   → `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Constants(
///   ConstantsErrorReturn::UnsupportedStatementVersion { version })))`, the
///   error whole, before any filesystem touch.
/// Arrange: `statement_version` overridden to `2`; every case the private
///   surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`;
///   `matches!(result, Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Constants(ConstantsErrorReturn::UnsupportedStatementVersion { version: 2 }))))`
///   and the root does not exist.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn a_constants_refusal_reaches_the_caller_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case;

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    statement_version: Some(2),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(Case, Default::default()),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::Constants(
                                            ConstantsErrorReturn::UnsupportedStatementVersion {
                                                version: 2
                                            }
                                        )
                                    ))
                                ));
                                assert!(!root.exists());
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_constants_refusal_reaches_the_caller_unchanged_and_writes_nothing

/// Contract: the hash-to-scalar refusal inside `mapping_vectors` →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(
///   MappingVectorsErrorReturn::HashToScalar(error))))`, the error whole,
///   before any filesystem touch.
/// Arrange: the hash-to-scalar adapter's selection overridden to
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`,
///   which differs from the builder's default of the declared selection, every
///   other selection real; every case the private surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`; the return matches
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength { expected, actual })))))`
///   with `expected` the case scalar's `UNIFORM_BYTES_LENGTH` and `actual` the
///   length the hash-to-scalar family declares for `SamplingRefused`, and the
///   root does not exist.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem, the operating system's generator, and the
///   hash-to-scalar family's mock concrete under `SamplingRefused`.
#[test]
fn a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case;

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                Default::default(),
            );
            let result = generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            );
            if let Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(
                MappingVectorsErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                    SampleUniformScalarErrorReturn::WrongLength { expected, actual },
                )),
            ))) = &result
            {
                assert_eq!(
                    *expected,
                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                );
                assert_eq!(*actual, 0);
            }
            result
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(Case, Default::default()),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(HashToScalarConcrete::Mock(
                                                            MockIHashToScalarAdapterFailureMode::SamplingRefused,
                                                        )),
                                                        identifier: Some(HashToScalarIdentifier::Mock),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(
                                                build_create_chain_forms_params(
                                                    CreateChainFormsParamsOverrides {
                                                        concrete: Some(*forms_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(
                                                build_create_random_source_params(
                                                    CreateRandomSourceParamsOverrides {
                                                        kind: Some(RandomSourceKind::OperatingSystem),
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::MappingVectors(
                                            MappingVectorsErrorReturn::HashToScalar(
                                                HashToScalarErrorReturn::Sampling(
                                                    SampleUniformScalarErrorReturn::WrongLength { .. }
                                                )
                                            )
                                        )
                                    ))
                                ));
                                assert!(!root.exists());
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing

/// Contract: a struct name configured for two emitted kinds →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(
///   error)))`, the error whole, before any filesystem touch.
/// Arrange: `settings.kinds` with the first two emitted kinds' names equal in
///   the struct name and distinct in every other name, each entry through
///   `build_solidity_vector_kind_settings`; every case the private surface
///   names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`;
///   `matches!(result, Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName { first_index: 0, duplicate_index: 1 }))))`
///   and the root does not exist.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn a_repeated_struct_name_is_refused_with_its_indexes_and_writes_nothing() {
    // Arrange
    let mut settings = build_solidity_generate_settings(Default::default());
    let shared = build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
        text: Some("SharedVector".to_string()),
    });
    settings.kinds[0].names.struct_name = shared.clone();
    settings.kinds[1].names.struct_name = shared;
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(
                                        Case {
                                            settings: settings.clone(),
                                        },
                                        Default::default(),
                                    ),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::Declarations(
                                            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                                                first_index: 0,
                                                duplicate_index: 1
                                            }
                                        )
                                    ))
                                ));
                                assert!(!root.exists());
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_repeated_struct_name_is_refused_with_its_indexes_and_writes_nothing

/// Contract: a decode function name configured for two emitted kinds →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(
///   error)))`, the error whole, before any filesystem touch.
/// Arrange: the first two emitted kinds' names equal in the decode function
///   name and distinct in every other name; every case the private surface
///   names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`;
///   `matches!` against `DuplicateDecodeFunctionName { first_index: 0,
///   duplicate_index: 1 }` inside `Err(EmitErrorReturn::Solidity(
///   SolidityGenerateErrorReturn::Declarations(…)))` and the root does not
///   exist.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn a_repeated_decode_function_name_is_refused_with_its_indexes_and_writes_nothing() {
    // Arrange
    let mut settings = build_solidity_generate_settings(Default::default());
    let shared = build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
        text: Some("decodeShared".to_string()),
    });
    settings.kinds[0].names.decode_function_name = shared.clone();
    settings.kinds[1].names.decode_function_name = shared;
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(
                                        Case {
                                            settings: settings.clone(),
                                        },
                                        Default::default(),
                                    ),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::Declarations(
                                            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName {
                                                first_index: 0,
                                                duplicate_index: 1
                                            }
                                        )
                                    ))
                                ));
                                assert!(!root.exists());
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_repeated_decode_function_name_is_refused_with_its_indexes_and_writes_nothing

/// Contract: a path constant name configured for two emitted kinds →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(
///   error)))`, the error whole, before any filesystem touch.
/// Arrange: the first two emitted kinds' names equal in the path constant
///   name and distinct in every other name; every case the private surface
///   names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`;
///   `matches!` against `DuplicatePathConstantName { first_index: 0,
///   duplicate_index: 1 }` inside `Err(EmitErrorReturn::Solidity(
///   SolidityGenerateErrorReturn::Declarations(…)))` and the root does not
///   exist.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn a_repeated_path_constant_name_is_refused_with_its_indexes_and_writes_nothing() {
    // Arrange
    let mut settings = build_solidity_generate_settings(Default::default());
    let shared = build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
        text: Some("SHARED_PATH".to_string()),
    });
    settings.kinds[0].names.path_constant_name = shared.clone();
    settings.kinds[1].names.path_constant_name = shared;
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(
                                        Case {
                                            settings: settings.clone(),
                                        },
                                        Default::default(),
                                    ),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::Declarations(
                                            SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName {
                                                first_index: 0,
                                                duplicate_index: 1
                                            }
                                        )
                                    ))
                                ));
                                assert!(!root.exists());
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_repeated_path_constant_name_is_refused_with_its_indexes_and_writes_nothing

/// Contract: the struct name's duplicate is refused where the decode function
///   name repeats as well → `Err(EmitErrorReturn::Solidity(
///   SolidityGenerateErrorReturn::Declarations(error)))`, the error whole,
///   before any filesystem touch.
/// Arrange: the first two emitted kinds' names equal in both the struct name
///   and the decode function name and distinct in every other name; every
///   case the private surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`;
///   `matches!` against `DuplicateStructName { first_index: 0,
///   duplicate_index: 1 }` inside `Err(EmitErrorReturn::Solidity(
///   SolidityGenerateErrorReturn::Declarations(…)))` and the root does not
///   exist.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn a_repeated_struct_name_is_refused_before_a_repeated_decode_function_name() {
    // Arrange
    let mut settings = build_solidity_generate_settings(Default::default());
    let shared_struct = build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
        text: Some("SharedVector".to_string()),
    });
    let shared_decode = build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
        text: Some("decodeShared".to_string()),
    });
    settings.kinds[0].names.struct_name = shared_struct.clone();
    settings.kinds[1].names.struct_name = shared_struct;
    settings.kinds[0].names.decode_function_name = shared_decode.clone();
    settings.kinds[1].names.decode_function_name = shared_decode;
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(
                                        Case {
                                            settings: settings.clone(),
                                        },
                                        Default::default(),
                                    ),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::Declarations(
                                            SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName {
                                                first_index: 0,
                                                duplicate_index: 1
                                            }
                                        )
                                    ))
                                ));
                                assert!(!root.exists());
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_repeated_struct_name_is_refused_before_a_repeated_decode_function_name

/// Contract: a failure of `create_dir_all` → `Err(EmitErrorReturn::Solidity(
///   SolidityGenerateErrorReturn::CreateDirectory { path, error }))`, carrying
///   the path and the filesystem's error.
/// Arrange: a regular file written at the root by `std::fs::write` after the
///   root's removal; every case the private surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`;
///   `matches!` with `path` equal to the root joined with the generated
///   directory's segments and the curve directory, and `error` equal in
///   `kind()`, `raw_os_error()`, and `to_string()` to the error the block
///   obtains by calling `std::fs::create_dir_all` over that path after `emit`.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn a_root_that_is_a_file_refuses_directory_creation_with_its_path() {
    // Arrange
    let settings = build_solidity_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    std::fs::write(&root, b"not a directory").expect("the root file is written");
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];

    struct Case {
        settings: SolidityGenerateSettings,
        curve: RefCell<Option<PairingCurve>>,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            *self.curve.borrow_mut() = Some(P::DECLARATION.curve);
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let deps = build_solidity_generate_case_deps(
                                    Case {
                                        settings: settings.clone(),
                                        curve: RefCell::new(None),
                                    },
                                    Default::default(),
                                );
                                let result = run_solidity_generate_case(
                                    &deps,
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let curve_ref = deps.consumer.curve.borrow();
                                let curve = curve_ref.as_ref().expect("the case ran");
                                let curve_directory = match curve {
                                    PairingCurve::Bn254 => {
                                        settings.curve_directories.bn254.as_str()
                                    }
                                    PairingCurve::Bls12381 => {
                                        settings.curve_directories.bls12_381.as_str()
                                    }
                                    PairingCurve::Mock => {
                                        panic!("PAIRING_CONCRETES names no Mock pairing")
                                    }
                                };
                                let expected = root.join("generated").join(curve_directory);
                                let probe = std::fs::create_dir_all(&expected).expect_err(
                                    "a regular file at the root refuses directory creation",
                                );
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::CreateDirectory {
                                            ref path,
                                            ref error
                                        }
                                    )) if *path == expected
                                        && error.kind() == probe.kind()
                                        && error.raw_os_error() == probe.raw_os_error()
                                        && error.to_string() == probe.to_string()
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_root_that_is_a_file_refuses_directory_creation_with_its_path

/// Contract: a failure of `std::fs::write` → `Err(EmitErrorReturn::Solidity(
///   SolidityGenerateErrorReturn::WriteFile { path, error }))`, carrying the
///   path and the filesystem's error.
/// Arrange: a directory created by `std::fs::create_dir_all` at the first
///   emitted kind's file path; every case the private surface names.
/// Act:    `emit` in the case consumer's method, once per case.
/// Assert: the runner's return `is_ok()`;
///   `matches!` with `path` equal to that file path and `error` equal in
///   `kind()`, `raw_os_error()`, and `to_string()` to the error the block
///   obtains by calling `std::fs::write` over it after `emit`, and the
///   constants library's file does not exist.
/// Boundary: `emit` and the real chain the private surface names.
/// Mocked:   the filesystem and the operating system's generator, running real.
#[test]
fn a_directory_at_a_kinds_file_path_refuses_the_write_with_its_path() {
    // Arrange
    let settings = build_solidity_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"pkg@1.0.0#7".as_slice(), b"pkg@1.0.0#8".as_slice()];
    let root_alpha = build_root(Default::default());
    let root_beta = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: &[&Root] = &[&root_alpha, &root_beta];
    let first_file_name = settings.kinds[0].names.file_name.as_str().to_owned();

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                pairing,
                hash_to_scalar,
                encoder,
                random,
                kem,
                key_agreement,
                delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if hash_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let curve_directory = match pairing_concrete {
                                    PairingConcrete::Bn254Arkworks
                                    | PairingConcrete::Bn254Halo2curves
                                    | PairingConcrete::Mock(_) => {
                                        settings.curve_directories.bn254.as_str()
                                    }
                                    PairingConcrete::Bls12381Arkworks
                                    | PairingConcrete::Bls12381Halo2curves => {
                                        settings.curve_directories.bls12_381.as_str()
                                    }
                                };
                                let file_path = root
                                    .join("generated")
                                    .join(curve_directory)
                                    .join(&first_file_name);
                                std::fs::create_dir_all(&file_path)
                                    .expect("a directory occupies the first kind's file path");
                                let result = run_solidity_generate_case(
                                    &build_solidity_generate_case_deps(
                                        Case {
                                            settings: settings.clone(),
                                        },
                                        Default::default(),
                                    ),
                                    build_solidity_generate_case_params(
                                        SolidityGenerateCaseParamsOverrides {
                                            pairing: Some(build_create_pairing_params(
                                                CreatePairingParamsOverrides {
                                                    concrete: Some(*pairing_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(hash_selection.concrete),
                                                        identifier: Some(hash_selection.identifier),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_selection.concrete),
                                                    identifier: Some(encoding_selection.identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            kem: Some(*kem_selection),
                                            key_agreement: Some(*agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(RandomSourceKind::OperatingSystem),
                                                },
                                            )),
                                        },
                                    ),
                                    build_solidity_generate_case_payload(
                                        canonicals,
                                        roots,
                                        Default::default(),
                                    ),
                                );
                                assert!(result.is_ok());

                                // Assert
                                let probe = std::fs::write(&file_path, b"")
                                    .expect_err("a directory refuses the write");
                                let result = result.expect("the case chain runs clean").output;
                                assert!(matches!(
                                    result,
                                    Err(EmitErrorReturn::Solidity(
                                        SolidityGenerateErrorReturn::WriteFile {
                                            ref path,
                                            ref error
                                        }
                                    )) if *path == file_path
                                        && error.kind() == probe.kind()
                                        && error.raw_os_error() == probe.raw_os_error()
                                        && error.to_string() == probe.to_string()
                                ));
                                let constants_path = root
                                    .join("generated")
                                    .join(curve_directory)
                                    .join(settings.constants_library.file_name.as_str());
                                assert!(!constants_path.exists());
                                let _ = std::fs::remove_dir_all(&root);
                            }
                        }
                    }
                }
            }
        }
    }
}
// a_directory_at_a_kinds_file_path_refuses_the_write_with_its_path
