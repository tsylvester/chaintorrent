#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{
    CanonicalFieldKind, IDecoderAdapter, IEncoderAdapter, IEncodingContract, build_decode_params,
};
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::{
    G1OutsideSubgroupEncodingErrorReturn, IPairingAdapter, IPairingArithmetic, IPairingReference,
};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_delivery_context,
};

use super::super::factory::provides::{
    EmitErrorReturn, EmitPayloadOverrides, EmitReturn, EmittedFile, IGenerateAdapter,
    VectorCountConstructorParamsOverrides, VectorKind, VectorKindSettings,
    VectorKindSettingsOverrides, VectorKindSize, VectorKindSizing, build_emit_params,
    build_emit_payload, build_msm_term_count, build_vector_count, build_vector_kind_settings,
};
use super::challenge_vectors::provides::{
    ChallengeVectorDescription, ChallengeVectorsDeps, ChallengeVectorsErrorReturn,
    ChallengeVectorsParams, ChallengeVectorsPayload, ChallengeVectorsReturn,
    ChallengeVectorsSuccessReturnOverrides, build_challenge_vector,
    build_challenge_vector_description, build_challenge_vectors_success_return,
};
use super::constants::provides::{
    CellValuesDescription, CellValuesDescriptionConstructorParamsOverrides, ConstantsDeps,
    ConstantsErrorReturn, ConstantsParams, ConstantsPayload, ConstantsReturn,
    build_cell_values_description,
};
use super::group_vectors::provides::{
    FirstGroupVectorsOverrides, GroupOperationVectorDescription, GroupVectorsDeps,
    GroupVectorsErrorReturn, GroupVectorsParams, GroupVectorsPayload, GroupVectorsReturn,
    GroupVectorsSuccessReturnOverrides, PairingCheckVectorDescription, build_first_group_vectors,
    build_group_operation_vector, build_group_operation_vector_description,
    build_group_vectors_success_return, build_pairing_check_vector,
    build_pairing_check_vector_description,
};
use super::interface::{
    ConsumeSolidityGenerateCaseParams, ConsumeSolidityGenerateCasePayload,
    ISolidityGenerateCaseConsumer, SolidityGenerateErrorReturn,
};
use super::mapping_vectors::provides::{
    HashToScalarVectorDescription, HashToScalarVectorOverrides, IdentityMappingVectorDescription,
    MappingVectorsDeps, MappingVectorsErrorReturn, MappingVectorsParams, MappingVectorsPayload,
    MappingVectorsReturn, MappingVectorsSuccessReturnOverrides, build_hash_to_scalar_vector,
    build_hash_to_scalar_vector_description, build_identity_mapping_vector,
    build_identity_mapping_vector_description, build_mapping_vectors_success_return,
};
use super::mock::*;
use super::possession_vectors::provides::{
    PossessionVectorDescription, PossessionVectorsDeps, PossessionVectorsErrorReturn,
    PossessionVectorsParams, PossessionVectorsPayload, PossessionVectorsReturn,
    PossessionVectorsSuccessReturnOverrides, build_possession_vector,
    build_possession_vector_description, build_possession_vectors_success_return,
};
use super::proof_vectors::provides::{
    MintProofVectorDescription, ProofVectorsDeps, ProofVectorsErrorReturn, ProofVectorsParams,
    ProofVectorsPayload, ProofVectorsReturn, ProofVectorsSuccessReturnOverrides,
    TransferProofVectorDescription, build_mint_proof_vector, build_mint_proof_vector_description,
    build_proof_vectors_success_return, build_transfer_proof_vector,
    build_transfer_proof_vector_description,
};
use super::rejection_vectors::provides::{
    RejectionVectorDescription, RejectionVectorsDeps, RejectionVectorsErrorReturn,
    RejectionVectorsParams, RejectionVectorsPayload, RejectionVectorsReturn,
    build_rejection_vector_description,
};
use super::render::provides::{
    RenderDeps, RenderParams, RenderPayload, RenderReturn, RenderSuccessReturnOverrides,
    SolidityAbiTypeConstructorParamsOverrides, SolidityLibraryBody, SolidityMemberName,
    SolidityRecordDeclaration, SolidityRecordDeclarationOverrides, SolidityRecordDeclarations,
    SolidityRecordDeclarationsConstructorParams, SolidityRecordDeclarationsTryNewErrorReturn,
    SolidityRecordMemberOverrides, SolidityRecordMembers,
    SolidityRecordMembersConstructorParamsOverrides, SoliditySourceTextOverrides,
    SolidityStringLiteralConstructorParamsOverrides, build_render_success_return,
    build_solidity_abi_type, build_solidity_record_declaration, build_solidity_record_member,
    build_solidity_record_members, build_solidity_source_text, build_solidity_string_literal,
};
use super::settings::provides::{
    GeneratedDirectoryConstructorParamsOverrides, GeneratedFileNameConstructorParamsOverrides,
    SOLIDITY_GENERATE_DECLARATION, SolidityGenerateSettings, SolidityGenerateSettingsOverrides,
    SolidityVectorKindNames, SolidityVectorKindSettingsOverrides, build_generated_directory,
    build_generated_file_name, build_solidity_generate_settings,
    build_solidity_vector_kind_settings,
};

/// Contract: an emitted kind with no entry in `self.settings.kinds` is
///   refused `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MissingKindSettings { kind }))`,
///   `kind` the first such kind in declared order, before any function is
///   called.
/// Arrange: `settings.kinds` overridden to one
///   `build_solidity_vector_kind_settings` per kind the default list names
///   but the possession and challenge kinds, each with the kind and names
///   the default list holds, and `sizes` at its default, so two emitted
///   kinds lack settings; the root removed before the act.
/// Act:    `emit` once inside the case consumer's method.
/// Assert: `matches!(result, Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MissingKindSettings { kind: VectorKind::Possession })))`
///   and the root does not exist.
#[test]
fn emit_refuses_settings_missing_an_emitted_kind() {
    // Arrange
    let mut settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    settings.kinds = settings
        .kinds
        .iter()
        .filter(|kind_settings| {
            !matches!(
                kind_settings.kind,
                VectorKind::Possession | VectorKind::Challenge
            )
        })
        .map(|kind_settings| {
            build_solidity_vector_kind_settings(SolidityVectorKindSettingsOverrides {
                kind: Some(kind_settings.kind),
                names: Some(kind_settings.names.clone()),
            })
        })
        .collect();
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
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
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
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
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::MissingKindSettings {
                kind: VectorKind::Possession
            }
        ))
    ));
    assert!(!root.exists());
}
// emit_refuses_settings_missing_an_emitted_kind

/// Contract: an emitted kind with no entry in `self.sizes` is refused
///   `Err(EmitErrorReturn::MissingKindSize { kind })`, `kind` the first such
///   kind in declared order, before any function is called.
/// Arrange: `sizes` overridden to one `build_vector_kind_settings` per kind
///   the default list names but the challenge and mint-proof kinds, each
///   with the kind and size the default sizes hold, and `settings` at its
///   default, so two emitted kinds lack sizes; the root removed before the
///   act.
/// Act:    `emit` once inside the case consumer's method.
/// Assert: `matches!(result, Err(EmitErrorReturn::MissingKindSize { kind: VectorKind::Challenge }))`
///   and the root does not exist.
#[test]
fn emit_refuses_sizes_missing_an_emitted_kind() {
    // Arrange
    let sizes: Vec<VectorKindSettings> = SOLIDITY_GENERATE_DECLARATION
        .kinds
        .iter()
        .filter(|declaration| {
            !matches!(
                declaration.kind,
                VectorKind::Challenge | VectorKind::MintProof
            )
        })
        .map(|declaration| {
            build_vector_kind_settings(VectorKindSettingsOverrides {
                kind: Some(declaration.kind),
                size: Some(match declaration.sizing {
                    VectorKindSizing::Drawn => VectorKindSize::Drawn {
                        count: build_vector_count(Default::default()),
                    },
                    VectorKindSizing::DrawnTerms => VectorKindSize::DrawnTerms {
                        count: build_vector_count(Default::default()),
                        terms: build_msm_term_count(Default::default()),
                    },
                    VectorKindSizing::DrawnMessages => VectorKindSize::DrawnMessages {
                        count: build_vector_count(Default::default()),
                        message_length: 32,
                    },
                    VectorKindSizing::Enumerated => VectorKindSize::Enumerated,
                    VectorKindSizing::OverConfiguredInputs => VectorKindSize::OverConfiguredInputs,
                }),
            })
        })
        .collect();
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
        sizes: Vec<VectorKindSettings>,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
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
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    sizes: Some(self.sizes.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings, sizes }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::MissingKindSize {
            kind: VectorKind::Challenge
        })
    ));
    assert!(!root.exists());
}
// emit_refuses_sizes_missing_an_emitted_kind

/// Contract: under a pairing declaring `VerifierGroupArithmetic::FirstGroupOnly`,
///   `group_vectors` is called with `second_group` `None` and the kinds whose
///   declaration requires second-group arithmetic are not emitted → `Ok` whose
///   `files` hold the other kinds' files and both libraries.
/// Arrange: the default selections, whose pairing mock concrete declares
///   `FirstGroupOnly`; `group_vectors` overridden with a `GroupVectorsFn`
///   declared in the block that refuses a `Some` `second_group`, every other
///   handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: the result is `Ok` and the `kind` of each entry of `files` equals,
///   in order, `Some(VectorKind::G1Addition)`, `Some(VectorKind::G1Multiplication)`,
///   `Some(VectorKind::PairingCheck)`, `Some(VectorKind::DecodeRejection)`, `Some(VectorKind::HashToScalar)`,
///   `Some(VectorKind::IdentityMapping)`, `Some(VectorKind::Possession)`, `Some(VectorKind::Challenge)`,
///   `Some(VectorKind::MintProof)`, `Some(VectorKind::TransferProof)`, `Some(VectorKind::CellValues)`,
///   `None`, and `None`.
#[test]
fn emit_hands_second_group_settings_only_where_the_pairing_has_both_groups() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn second_group_refused<'a, P: IPairingArithmetic>(
                _deps: &GroupVectorsDeps<'a, P>,
                params: GroupVectorsParams,
                _payload: GroupVectorsPayload,
            ) -> GroupVectorsReturn {
                if params.second_group.is_some() {
                    Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly)
                } else {
                    Ok(build_group_vectors_success_return(Default::default()))
                }
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    group_vectors: Some(second_group_refused),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    let Ok(success) = result else {
        panic!("emit admits the case's arrangement")
    };
    assert_eq!(
        success
            .files
            .iter()
            .map(|file| file.kind)
            .collect::<Vec<_>>(),
        vec![
            Some(VectorKind::G1Addition),
            Some(VectorKind::G1Multiplication),
            Some(VectorKind::PairingCheck),
            Some(VectorKind::DecodeRejection),
            Some(VectorKind::HashToScalar),
            Some(VectorKind::IdentityMapping),
            Some(VectorKind::Possession),
            Some(VectorKind::Challenge),
            Some(VectorKind::MintProof),
            Some(VectorKind::TransferProof),
            Some(VectorKind::CellValues),
            None,
            None,
        ]
    );
}
// emit_hands_second_group_settings_only_where_the_pairing_has_both_groups

/// Contract: each function receives the admitted size of its own kinds and each
///   kind's file holds as many lines as the records its function returned.
/// Arrange: `sizes` overridden to one `build_vector_kind_settings` per emitted
///   kind with a count distinct from every other kind's, the message length
///   distinct from the counts, two distinct identities, and `group_vectors`,
///   `mapping_vectors`, `possession_vectors`, `challenge_vectors`, and
///   `proof_vectors` each overridden with a function declared in the block that
///   returns its success-return builder with each record list overridden to as
///   many records as the count the function received for that list.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: the line count of each of those kinds' files equals the count the
///   block set for that kind, and the first line of the hash-to-scalar file,
///   decoded under `build_hash_to_scalar_vector_description`, holds a `message`
///   of the admitted message length.
#[test]
fn emit_routes_each_kinds_admitted_size_to_its_function() {
    // Arrange
    fn drawn(kind: VectorKind, count: u32) -> VectorKindSettings {
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(kind),
            size: Some(VectorKindSize::Drawn {
                count: build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(count),
                }),
            }),
        })
    }

    let sizes = vec![
        drawn(VectorKind::G1Addition, 3),
        drawn(VectorKind::G1Multiplication, 5),
        drawn(VectorKind::PairingCheck, 7),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::DecodeRejection),
            size: Some(VectorKindSize::Enumerated),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::HashToScalar),
            size: Some(VectorKindSize::DrawnMessages {
                count: build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(11),
                }),
                message_length: 37,
            }),
        }),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::IdentityMapping),
            size: Some(VectorKindSize::OverConfiguredInputs),
        }),
        drawn(VectorKind::Possession, 13),
        drawn(VectorKind::Challenge, 17),
        drawn(VectorKind::MintProof, 19),
        drawn(VectorKind::TransferProof, 23),
        build_vector_kind_settings(VectorKindSettingsOverrides {
            kind: Some(VectorKind::CellValues),
            size: Some(VectorKindSize::Enumerated),
        }),
    ];
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
        sizes: Vec<VectorKindSettings>,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = (EmitReturn, usize);

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn routed_group_vectors<'a, P: IPairingArithmetic>(
                _deps: &GroupVectorsDeps<'a, P>,
                params: GroupVectorsParams,
                _payload: GroupVectorsPayload,
            ) -> GroupVectorsReturn {
                if params.second_group.is_some() {
                    return Err(GroupVectorsErrorReturn::SecondGroupSettingsForFirstGroupOnly);
                }
                Ok(build_group_vectors_success_return(
                    GroupVectorsSuccessReturnOverrides {
                        first_group: Some(build_first_group_vectors(FirstGroupVectorsOverrides {
                            g1_add: Some(
                                (0..params.first_group.g1_add.get())
                                    .map(|_| build_group_operation_vector(Default::default()))
                                    .collect(),
                            ),
                            g1_mul: Some(
                                (0..params.first_group.g1_mul.get())
                                    .map(|_| build_group_operation_vector(Default::default()))
                                    .collect(),
                            ),
                            pairing_check: Some(
                                (0..params.first_group.pairing_check.get())
                                    .map(|_| build_pairing_check_vector(Default::default()))
                                    .collect(),
                            ),
                        })),
                        second_group: Some(None),
                    },
                ))
            }

            fn routed_mapping_vectors<
                'a,
                P: IPairingArithmetic,
                K: ICredentialKemAdapter<Pairing = P>,
            >(
                _deps: &MappingVectorsDeps<'a, P, K>,
                params: MappingVectorsParams,
                payload: MappingVectorsPayload<'_>,
            ) -> MappingVectorsReturn {
                Ok(build_mapping_vectors_success_return(
                    MappingVectorsSuccessReturnOverrides {
                        hash_to_scalar: Some(
                            (0..params.hash_to_scalar_count.get())
                                .map(|_| {
                                    build_hash_to_scalar_vector(HashToScalarVectorOverrides {
                                        message: Some(vec![0xAA; params.message_length]),
                                        ..Default::default()
                                    })
                                })
                                .collect(),
                        ),
                        identity_mapping: Some(
                            payload
                                .identities
                                .iter()
                                .map(|_| build_identity_mapping_vector(Default::default()))
                                .collect(),
                        ),
                    },
                ))
            }

            fn routed_possession_vectors<
                'a,
                P: IPairingArithmetic,
                K: IKeyAgreementAdapter<Pairing = P>,
            >(
                _deps: &PossessionVectorsDeps<'a, P, K>,
                params: PossessionVectorsParams,
                _payload: PossessionVectorsPayload,
            ) -> PossessionVectorsReturn {
                Ok(build_possession_vectors_success_return(
                    PossessionVectorsSuccessReturnOverrides {
                        records: Some(
                            (0..params.count.get())
                                .map(|_| build_possession_vector(Default::default()))
                                .collect(),
                        ),
                    },
                ))
            }

            fn routed_challenge_vectors<
                'a,
                P: IPairingAdapter,
                E: IEncoderAdapter,
                F: IChainForms,
            >(
                _deps: &ChallengeVectorsDeps<'a, P, E>,
                params: ChallengeVectorsParams,
                _payload: ChallengeVectorsPayload<'_, F>,
            ) -> ChallengeVectorsReturn {
                Ok(build_challenge_vectors_success_return(
                    ChallengeVectorsSuccessReturnOverrides {
                        records: Some(
                            (0..params.count.get())
                                .map(|_| build_challenge_vector(Default::default()))
                                .collect(),
                        ),
                    },
                ))
            }

            fn routed_proof_vectors<
                'a,
                P: IPairingAdapter,
                K: ICredentialKemAdapter<Pairing = P>,
                A: IKeyAgreementAdapter<Pairing = P>,
                D: IDeliveryProofAdapter<Pairing = P>,
            >(
                _deps: &ProofVectorsDeps<'a, P, K, A, D>,
                params: ProofVectorsParams,
                _payload: ProofVectorsPayload<'_, <D as IDeliveryProofAdapter>::Forms>,
            ) -> ProofVectorsReturn {
                Ok(build_proof_vectors_success_return(
                    ProofVectorsSuccessReturnOverrides {
                        mint: Some(
                            (0..params.mint_count.get())
                                .map(|_| build_mint_proof_vector(Default::default()))
                                .collect(),
                        ),
                        transfer: Some(
                            (0..params.transfer_count.get())
                                .map(|_| build_transfer_proof_vector(Default::default()))
                                .collect(),
                        ),
                    },
                ))
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    settings: Some(self.settings.clone()),
                    sizes: Some(self.sizes.clone()),
                    group_vectors: Some(routed_group_vectors),
                    mapping_vectors: Some(routed_mapping_vectors),
                    possession_vectors: Some(routed_possession_vectors),
                    challenge_vectors: Some(routed_challenge_vectors),
                    proof_vectors: Some(routed_proof_vectors),
                    ..Default::default()
                },
            );
            let result = generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            );
            let decoded_len = match &result {
                Ok(success) => {
                    let file = success
                        .files
                        .iter()
                        .find(|file| file.kind == Some(VectorKind::HashToScalar))
                        .expect("the hash-to-scalar file is written");
                    let contents =
                        std::fs::read_to_string(&file.path).expect("the file is readable");
                    let line = contents.lines().next().expect("the file holds a line");
                    let bytes: Vec<u8> = (2..line.len())
                        .step_by(2)
                        .map(|index| {
                            u8::from_str_radix(&line[index..index + 2], 16)
                                .expect("the line is lowercase hex")
                        })
                        .collect();
                    let decoded = payload
                        .decoder
                        .decode(
                            build_decode_params(&build_hash_to_scalar_vector_description(
                                Default::default(),
                            )),
                            &bytes,
                        )
                        .expect("the first line decodes under its description");
                    decoded.described.message.len()
                }
                Err(_) => usize::MAX,
            };
            (result, decoded_len)
        }
    }

    // Act
    let (result, decoded_len) = run_solidity_generate_case(
        &build_solidity_generate_case_deps(
            Case {
                settings: settings.clone(),
                sizes,
            },
            Default::default(),
        ),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    let Ok(success) = result else {
        panic!("emit admits the case's arrangement")
    };
    for (kind, expected) in [
        (VectorKind::G1Addition, 3),
        (VectorKind::G1Multiplication, 5),
        (VectorKind::PairingCheck, 7),
        (VectorKind::HashToScalar, 11),
        (VectorKind::IdentityMapping, 2),
        (VectorKind::Possession, 13),
        (VectorKind::Challenge, 17),
        (VectorKind::MintProof, 19),
        (VectorKind::TransferProof, 23),
    ] {
        let file = success
            .files
            .iter()
            .find(|file| file.kind == Some(kind))
            .expect("the kind's file is written");
        let lines = std::fs::read_to_string(&file.path)
            .expect("the kind's file is readable")
            .lines()
            .count();
        assert_eq!(lines, expected, "the {kind:?} file holds {expected} lines");
    }
    assert_eq!(decoded_len, 37);
}
// emit_routes_each_kinds_admitted_size_to_its_function

/// Contract: `constants` returns an error →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Constants(error)))`,
///   the error whole.
/// Arrange: `constants` overridden with a `ConstantsFn` declared in the block
///   that returns `Err(UnsupportedStatementVersion { version: 7 })`, every
///   other handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on the refusal with `version` `7` and the root does not
///   exist.
#[test]
fn emit_returns_a_constants_refusal_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn refusing_constants<'a, P: IPairingReference, E: IEncoderAdapter>(
                _deps: &ConstantsDeps<'a, P, E>,
                _params: ConstantsParams,
                _payload: ConstantsPayload<'_>,
            ) -> ConstantsReturn {
                Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: 7 })
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    constants: Some(refusing_constants),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::Constants(
                ConstantsErrorReturn::UnsupportedStatementVersion { version: 7 }
            )
        ))
    ));
    assert!(!root.exists());
}
// emit_returns_a_constants_refusal_unchanged_and_writes_nothing

/// Contract: `group_vectors` returns an error →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::GroupVectors(error)))`,
///   the error whole.
/// Arrange: `group_vectors` overridden with a `GroupVectorsFn` declared in the
///   block that returns `Err(SecondGroupSettingsMissing)`, every other handle
///   at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on the refusal and the root does not exist.
#[test]
fn emit_returns_a_group_vectors_refusal_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn refusing_group_vectors<'a, P: IPairingArithmetic>(
                _deps: &GroupVectorsDeps<'a, P>,
                _params: GroupVectorsParams,
                _payload: GroupVectorsPayload,
            ) -> GroupVectorsReturn {
                Err(GroupVectorsErrorReturn::SecondGroupSettingsMissing)
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    group_vectors: Some(refusing_group_vectors),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::GroupVectors(
                GroupVectorsErrorReturn::SecondGroupSettingsMissing
            )
        ))
    ));
    assert!(!root.exists());
}
// emit_returns_a_group_vectors_refusal_unchanged_and_writes_nothing

/// Contract: `rejection_vectors` returns an error →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::RejectionVectors(error)))`,
///   the error whole.
/// Arrange: `rejection_vectors` overridden with a `RejectionVectorsFn` declared
///   in the block that returns `Err(G1OutsideSubgroupEncoding(SearchExhausted))`,
///   every other handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on the refusal and the root does not exist.
#[test]
fn emit_returns_a_rejection_vectors_refusal_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn refusing_rejection_vectors<'a, P: IPairingReference>(
                _deps: &RejectionVectorsDeps<'a, P>,
                _params: RejectionVectorsParams,
                _payload: RejectionVectorsPayload,
            ) -> RejectionVectorsReturn {
                Err(RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(
                    G1OutsideSubgroupEncodingErrorReturn::SearchExhausted,
                ))
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    rejection_vectors: Some(refusing_rejection_vectors),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::RejectionVectors(
                RejectionVectorsErrorReturn::G1OutsideSubgroupEncoding(
                    G1OutsideSubgroupEncodingErrorReturn::SearchExhausted
                )
            )
        ))
    ));
    assert!(!root.exists());
}
// emit_returns_a_rejection_vectors_refusal_unchanged_and_writes_nothing

/// Contract: `mapping_vectors` returns an error →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(error)))`,
///   the error whole.
/// Arrange: `mapping_vectors` overridden with a `MappingVectorsFn` declared in
///   the block that returns `Err(ScalarLength { actual: 3 })`, every other
///   handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on the refusal and the root does not exist.
#[test]
fn emit_returns_a_mapping_vectors_refusal_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn refusing_mapping_vectors<
                'a,
                P: IPairingArithmetic,
                K: ICredentialKemAdapter<Pairing = P>,
            >(
                _deps: &MappingVectorsDeps<'a, P, K>,
                _params: MappingVectorsParams,
                _payload: MappingVectorsPayload<'_>,
            ) -> MappingVectorsReturn {
                Err(MappingVectorsErrorReturn::ScalarLength { actual: 3 })
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    mapping_vectors: Some(refusing_mapping_vectors),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::ScalarLength {
                actual: 3
            })
        ))
    ));
    assert!(!root.exists());
}
// emit_returns_a_mapping_vectors_refusal_unchanged_and_writes_nothing

/// Contract: `possession_vectors` returns an error →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::PossessionVectors(error)))`,
///   the error whole.
/// Arrange: `possession_vectors` overridden with a `PossessionVectorsFn`
///   declared in the block that returns `Err(ScalarLength { actual: 5 })`,
///   every other handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on the refusal and the root does not exist.
#[test]
fn emit_returns_a_possession_vectors_refusal_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn refusing_possession_vectors<
                'a,
                P: IPairingArithmetic,
                K: IKeyAgreementAdapter<Pairing = P>,
            >(
                _deps: &PossessionVectorsDeps<'a, P, K>,
                _params: PossessionVectorsParams,
                _payload: PossessionVectorsPayload,
            ) -> PossessionVectorsReturn {
                Err(PossessionVectorsErrorReturn::ScalarLength { actual: 5 })
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    possession_vectors: Some(refusing_possession_vectors),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::PossessionVectors(
                PossessionVectorsErrorReturn::ScalarLength { actual: 5 }
            )
        ))
    ));
    assert!(!root.exists());
}
// emit_returns_a_possession_vectors_refusal_unchanged_and_writes_nothing

/// Contract: `challenge_vectors` returns an error →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::ChallengeVectors(error)))`,
///   the error whole.
/// Arrange: `challenge_vectors` overridden with a `ChallengeVectorsFn` declared
///   in the block that returns `Err(UnsupportedStatementVersion { version: 9 })`,
///   every other handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on the refusal and the root does not exist.
#[test]
fn emit_returns_a_challenge_vectors_refusal_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn refusing_challenge_vectors<
                'a,
                P: IPairingAdapter,
                E: IEncoderAdapter,
                F: IChainForms,
            >(
                _deps: &ChallengeVectorsDeps<'a, P, E>,
                _params: ChallengeVectorsParams,
                _payload: ChallengeVectorsPayload<'_, F>,
            ) -> ChallengeVectorsReturn {
                Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion { version: 9 })
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    challenge_vectors: Some(refusing_challenge_vectors),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::ChallengeVectors(
                ChallengeVectorsErrorReturn::UnsupportedStatementVersion { version: 9 }
            )
        ))
    ));
    assert!(!root.exists());
}
// emit_returns_a_challenge_vectors_refusal_unchanged_and_writes_nothing

/// Contract: `proof_vectors` returns an error →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::ProofVectors(error)))`,
///   the error whole.
/// Arrange: `proof_vectors` overridden with a `ProofVectorsFn` declared in the
///   block that returns `Err(ScalarLength { actual: 13 })`, every other handle
///   at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on the refusal and the root does not exist.
#[test]
fn emit_returns_a_proof_vectors_refusal_unchanged_and_writes_nothing() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn refusing_proof_vectors<
                'a,
                P: IPairingAdapter,
                K: ICredentialKemAdapter<Pairing = P>,
                A: IKeyAgreementAdapter<Pairing = P>,
                D: IDeliveryProofAdapter<Pairing = P>,
            >(
                _deps: &ProofVectorsDeps<'a, P, K, A, D>,
                _params: ProofVectorsParams,
                _payload: ProofVectorsPayload<'_, <D as IDeliveryProofAdapter>::Forms>,
            ) -> ProofVectorsReturn {
                Err(ProofVectorsErrorReturn::ScalarLength { actual: 13 })
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    proof_vectors: Some(refusing_proof_vectors),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::ProofVectors(ProofVectorsErrorReturn::ScalarLength {
                actual: 13
            })
        ))
    ));
    assert!(!root.exists());
}
// emit_returns_a_proof_vectors_refusal_unchanged_and_writes_nothing

/// Contract: a struct name configured for two emitted kinds →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(error)))`,
///   the error whole.
/// Arrange: `settings.kinds` with the first two emitted kinds' names equal in
///   the struct name and distinct in every other name, each entry through
///   `build_solidity_vector_kind_settings`, every handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` on `DuplicateStructName { first_index: 0,
///   duplicate_index: 1 }` and the root does not exist.
#[test]
fn emit_refuses_a_repeated_struct_name_with_its_indexes_and_writes_nothing() {
    // Arrange
    let mut settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    settings.kinds[1].names.struct_name = settings.kinds[0].names.struct_name.clone();
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
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
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
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
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
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
// emit_refuses_a_repeated_struct_name_with_its_indexes_and_writes_nothing

/// Contract: the files are written in the emitted kinds' declared order, then
///   the constants library, then the records library, each vector file under
///   the root, the generated directory's segments, and the cell directory's
///   two segments and each library directly under the generated directory →
///   `Ok` whose `files` list them in that order, each existing.
/// Arrange: `generated_directory` of two distinct segments by
///   `build_generated_directory`, the default selections, every handle at its
///   function mock, and the default file names, which are distinct across the
///   kinds and the libraries.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `files` equals, in order, one entry per expected path, each `path`
///   the root joined with the two segments, the two declared names, and the
///   file name for a vector file and the root joined with the two segments and
///   the configured file name for a library, and each entry's `kind` `Some` of
///   the kind whose file it is for a vector file and `None` for the constants
///   library's file and the records library's, and each path exists.
#[test]
fn emit_writes_each_emitted_kind_then_both_libraries_in_write_order() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
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
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];
    let cell = |file: &str| {
        root.clone()
            .join("suite")
            .join("generated")
            .join("mock_concrete")
            .join("mock")
            .join(file)
    };
    let library = |file: &str| root.clone().join("suite").join("generated").join(file);
    let expected = vec![
        EmittedFile {
            kind: Some(VectorKind::G1Addition),
            path: cell("g1_addition.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::G1Multiplication),
            path: cell("g1_multiplication.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::PairingCheck),
            path: cell("pairing_check.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::DecodeRejection),
            path: cell("decode_rejection.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::HashToScalar),
            path: cell("hash_to_scalar.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::IdentityMapping),
            path: cell("identity_mapping.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::Possession),
            path: cell("possession.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::Challenge),
            path: cell("challenge.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::MintProof),
            path: cell("mint_proof.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::TransferProof),
            path: cell("transfer_proof.txt"),
        },
        EmittedFile {
            kind: Some(VectorKind::CellValues),
            path: cell("cell_values.txt"),
        },
        EmittedFile {
            kind: None,
            path: library("Constants.sol"),
        },
        EmittedFile {
            kind: None,
            path: library("VectorRecords.sol"),
        },
    ];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
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
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
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
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    let Ok(success) = result else {
        panic!("emit admits the case's arrangement")
    };
    assert_eq!(success.files, expected);
    assert!(expected.iter().all(|file| file.path.exists()));
}
// emit_writes_each_emitted_kind_then_both_libraries_in_write_order

/// Contract: the cell directory is the generated directory joined with the
///   name the received pairing's declaration supplies for its concrete and the
///   name the borrowed encoder's declaration supplies → every vector file is
///   written under it and the libraries outside it under their configured
///   names.
/// Arrange: the default selections, which select the pairing family's and the
///   encoding family's mock concretes, and file names distinct across the
///   kinds and the libraries, every handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: the result is `Ok`, every vector-file entry's `path` is the root
///   joined with the generated directory's segments, the concrete-name literal
///   `"mock_concrete"`, and the encoding-name literal `"mock"`, the two
///   library entries sit directly under the generated directory at their
///   configured file names, and no other directory exists under the generated
///   directory.
#[test]
fn emit_derives_the_cell_directory_from_the_received_declarations() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
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
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
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
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    let Ok(success) = result else {
        panic!("emit admits the case's arrangement")
    };
    let generated = root.join("generated");
    let cell = generated.join("mock_concrete").join("mock");
    for file in &success.files[..success.files.len() - 2] {
        assert_eq!(file.path.parent(), Some(cell.as_path()));
    }
    let libraries = &success.files[success.files.len() - 2..];
    assert_eq!(libraries[0].path, generated.join("Constants.sol"));
    assert_eq!(libraries[1].path, generated.join("VectorRecords.sol"));
    let directories: Vec<_> = std::fs::read_dir(&generated)
        .expect("the generated directory exists")
        .map(|entry| entry.expect("each entry reads").file_name())
        .filter(|name| generated.join(name).is_dir())
        .collect();
    assert_eq!(directories, vec![std::ffi::OsString::from("mock_concrete")]);
}
// emit_derives_the_cell_directory_from_the_received_declarations

/// Contract: a failure of `create_dir_all` →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::CreateDirectory { path, error }))`,
///   carrying the path and the filesystem's error.
/// Arrange: a regular file written at the root by `std::fs::write` after the
///   root's removal, every handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` with `path` equal to the root joined with the generated
///   directory's segments and the cell directory's two declared names, and
///   `error` equal in `kind()`, `raw_os_error()`, and `to_string()` to the
///   error the block obtains by calling `std::fs::create_dir_all` over that
///   path after `emit`.
#[test]
fn emit_returns_a_directory_creation_failure_with_its_path() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    std::fs::write(&root, b"not-a-directory").expect("the blocker file is written");
    let expected_path = root.join("generated").join("mock_concrete").join("mock");
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
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
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
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
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    let Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::CreateDirectory {
        path,
        error,
    })) = result
    else {
        panic!("the blocked root refuses directory creation")
    };
    assert_eq!(path, expected_path);
    let expected_error = std::fs::create_dir_all(&expected_path)
        .expect_err("the same path still refuses directory creation");
    assert_eq!(error.kind(), expected_error.kind());
    assert_eq!(error.raw_os_error(), expected_error.raw_os_error());
    assert_eq!(error.to_string(), expected_error.to_string());
}
// emit_returns_a_directory_creation_failure_with_its_path

/// Contract: a failure of `std::fs::write` →
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::WriteFile { path, error }))`,
///   carrying the path and the filesystem's error.
/// Arrange: a directory created by `std::fs::create_dir_all` at the first
///   emitted kind's file path, every handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: `matches!` with `path` equal to that file path and `error` equal in
///   `kind()`, `raw_os_error()`, and `to_string()` to the error the block
///   obtains by calling `std::fs::write` over it after `emit`, and the
///   constants library's file does not exist.
#[test]
fn emit_returns_a_file_write_failure_with_its_path() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let blocked = root
        .join("generated")
        .join("mock_concrete")
        .join("mock")
        .join("g1_addition.txt");
    std::fs::create_dir_all(&blocked).expect("the blocker directory is created");
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
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
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
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
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case { settings }, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    let Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::WriteFile { path, error })) =
        result
    else {
        panic!("the blocked file path refuses the write")
    };
    assert_eq!(path, blocked);
    let expected_error =
        std::fs::write(&blocked, b"still-blocked").expect_err("the same path still refuses writes");
    assert_eq!(error.kind(), expected_error.kind());
    assert_eq!(error.raw_os_error(), expected_error.raw_os_error());
    assert_eq!(error.to_string(), expected_error.to_string());
    assert!(!root.join("generated").join("Constants.sol").exists());
}
// emit_returns_a_file_write_failure_with_its_path

/// Contract: the concrete emits every declared kind over the encoding
///   family's mock encoder → each emitted kind's declaration, as `render`
///   receives it, holds its members in order, each carrying the name its
///   description's `member_names()` declares at that index, the kind its
///   description's `FIELDS` holds at that index, and the `SolidityAbiType`
///   whose text is the mock's declared name for that kind.
/// Arrange: the default selections, which select the encoding family's mock
///   concrete, and `render` overridden with a `RenderFn` declared in the block
///   that returns `Ok` of the source text `matches` when the received body is
///   `SolidityLibraryBody::Records` of the expected declarations the function
///   builds through `SolidityRecordDeclarations::try_new`, and `differs`
///   otherwise; every other handle at its function mock.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: the result is `Ok` and the records library, read from the generated
///   directory at its configured records file name, holds the text `matches`.
#[test]
fn each_record_member_carries_the_encoders_type_name_for_its_kind() {
    // Arrange
    let settings = build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
        foundry_root: Some(build_solidity_generate_root()),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case {
        settings: SolidityGenerateSettings,
    }

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = EmitReturn;

        fn consume_solidity_generate_case<P, E, F, K, A, D>(
            &self,
            _params: ConsumeSolidityGenerateCaseParams,
            payload: ConsumeSolidityGenerateCasePayload<'_, P, E, K, A, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            fn checking_render<F: IChainForms>(
                _deps: &RenderDeps,
                _params: RenderParams,
                payload: RenderPayload,
            ) -> RenderReturn {
                fn abi_text(kind: CanonicalFieldKind) -> &'static str {
                    match kind {
                        CanonicalFieldKind::FixedBytes32 => "mockFixedBytes32",
                        CanonicalFieldKind::Unsigned16 => "mockUnsigned16",
                        CanonicalFieldKind::Unsigned32 => "mockUnsigned32",
                        CanonicalFieldKind::Unsigned64 => "mockUnsigned64",
                        CanonicalFieldKind::Text => "mockText",
                        CanonicalFieldKind::Bytes => "mockBytes",
                        CanonicalFieldKind::FixedBytes20 => "mockFixedBytes20",
                        CanonicalFieldKind::Unsigned256 => "mockUnsigned256",
                    }
                }

                fn members(
                    names: Vec<SolidityMemberName>,
                    fields: &[CanonicalFieldKind],
                ) -> SolidityRecordMembers {
                    let members = names
                        .iter()
                        .zip(fields.iter())
                        .map(|(name, kind)| {
                            build_solidity_record_member(SolidityRecordMemberOverrides {
                                name: Some(name.clone()),
                                kind: Some(*kind),
                                abi_type: Some(build_solidity_abi_type(
                                    SolidityAbiTypeConstructorParamsOverrides {
                                        name: Some(abi_text(*kind).to_string()),
                                    },
                                )),
                            })
                        })
                        .collect();
                    build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
                        members: Some(members),
                    })
                }

                fn declaration(
                    names: &SolidityVectorKindNames,
                    members: SolidityRecordMembers,
                    path: &str,
                ) -> SolidityRecordDeclaration {
                    build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
                        struct_name: Some(names.struct_name.clone()),
                        members: Some(members),
                        decode_function_name: Some(names.decode_function_name.clone()),
                        path_constant_name: Some(names.path_constant_name.clone()),
                        path: Some(build_solidity_string_literal(
                            SolidityStringLiteralConstructorParamsOverrides {
                                text: Some(path.to_string()),
                            },
                        )),
                    })
                }

                let settings =
                    build_solidity_generate_settings(SolidityGenerateSettingsOverrides {
                        foundry_root: Some(build_solidity_generate_root()),
                        ..Default::default()
                    });
                let group = build_group_operation_vector_description(Default::default());
                let group_names = group
                    .member_names()
                    .expect("the group description declares its member names");
                let pairing_check = build_pairing_check_vector_description(Default::default());
                let pairing_check_names = pairing_check
                    .member_names()
                    .expect("the pairing-check description declares its member names");
                let rejection = build_rejection_vector_description(Default::default());
                let rejection_names = rejection
                    .member_names()
                    .expect("the rejection description declares its member names");
                let scalar = build_hash_to_scalar_vector_description(Default::default());
                let scalar_names = scalar
                    .member_names()
                    .expect("the hash-to-scalar description declares its member names");
                let mapping = build_identity_mapping_vector_description(Default::default());
                let mapping_names = mapping
                    .member_names()
                    .expect("the identity-mapping description declares its member names");
                let possession = build_possession_vector_description(Default::default());
                let possession_names = possession
                    .member_names()
                    .expect("the possession description declares its member names");
                let challenge = build_challenge_vector_description(Default::default());
                let challenge_names = challenge
                    .member_names()
                    .expect("the challenge description declares its member names");
                let mint_proof = build_mint_proof_vector_description::<F>(Default::default());
                let mint_proof_names = mint_proof
                    .member_names()
                    .expect("the mint-proof description declares its member names");
                let transfer_proof =
                    build_transfer_proof_vector_description::<F>(Default::default());
                let transfer_proof_names = transfer_proof
                    .member_names()
                    .expect("the transfer-proof description declares its member names");
                let cell_values =
                    build_cell_values_description(CellValuesDescriptionConstructorParamsOverrides);
                let cell_values_names = cell_values
                    .member_names()
                    .expect("the cell-values description declares its member names");
                let declarations = vec![
                    declaration(
                        &settings.kinds[0].names,
                        members(
                            group_names.clone(),
                            <GroupOperationVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "g1_addition.txt",
                    ),
                    declaration(
                        &settings.kinds[1].names,
                        members(
                            group_names,
                            <GroupOperationVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "g1_multiplication.txt",
                    ),
                    declaration(
                        &settings.kinds[2].names,
                        members(
                            pairing_check_names,
                            <PairingCheckVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "pairing_check.txt",
                    ),
                    declaration(
                        &settings.kinds[6].names,
                        members(
                            rejection_names,
                            <RejectionVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "decode_rejection.txt",
                    ),
                    declaration(
                        &settings.kinds[7].names,
                        members(
                            scalar_names,
                            <HashToScalarVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "hash_to_scalar.txt",
                    ),
                    declaration(
                        &settings.kinds[8].names,
                        members(
                            mapping_names,
                            <IdentityMappingVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "identity_mapping.txt",
                    ),
                    declaration(
                        &settings.kinds[9].names,
                        members(
                            possession_names,
                            <PossessionVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "possession.txt",
                    ),
                    declaration(
                        &settings.kinds[10].names,
                        members(
                            challenge_names,
                            <ChallengeVectorDescription as IEncodingContract>::FIELDS,
                        ),
                        "challenge.txt",
                    ),
                    declaration(
                        &settings.kinds[11].names,
                        members(
                            mint_proof_names,
                            <MintProofVectorDescription<F> as IEncodingContract>::FIELDS,
                        ),
                        "mint_proof.txt",
                    ),
                    declaration(
                        &settings.kinds[12].names,
                        members(
                            transfer_proof_names,
                            <TransferProofVectorDescription<F> as IEncodingContract>::FIELDS,
                        ),
                        "transfer_proof.txt",
                    ),
                    declaration(
                        &settings.kinds[13].names,
                        members(
                            cell_values_names,
                            <CellValuesDescription as IEncodingContract>::FIELDS,
                        ),
                        "cell_values.txt",
                    ),
                ];
                let expected = SolidityRecordDeclarations::try_new(
                    SolidityRecordDeclarationsConstructorParams { declarations },
                )
                .expect("the expected declarations admit");
                let text = match &payload.body {
                    SolidityLibraryBody::Records(declarations) if declarations == &expected => {
                        "matches"
                    }
                    _ => "differs",
                };
                Ok(build_render_success_return(RenderSuccessReturnOverrides {
                    source: Some(build_solidity_source_text(SoliditySourceTextOverrides {
                        text: Some(text.to_string()),
                    })),
                }))
            }

            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let generate = build_solidity_generate(
                payload.pairing,
                payload.hash_to_scalar,
                payload.encoder,
                payload.random,
                payload.kem,
                payload.key_agreement,
                payload.delivery_proof,
                SolidityGenerateConstructorParamsOverrides {
                    render: Some(checking_render::<F>),
                    settings: Some(self.settings.clone()),
                    ..Default::default()
                },
            );
            generate.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    &mint,
                    &transfer,
                    &payload.identities,
                    EmitPayloadOverrides {
                        proof_identity: Some(payload.identities[0]),
                    },
                ),
            )
        }
    }

    // Act
    let result = run_solidity_generate_case(
        &build_solidity_generate_case_deps(
            Case {
                settings: settings.clone(),
            },
            Default::default(),
        ),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    assert!(result.is_ok(), "emit admits the case's arrangement");
    let records = root
        .join("generated")
        .join(settings.records_library.file_name.as_str());
    let source = std::fs::read_to_string(&records).expect("the records library was written");
    assert!(
        source.contains("matches"),
        "the records library holds the records the render fn admits"
    );
}
// each_record_member_carries_the_encoders_type_name_for_its_kind
