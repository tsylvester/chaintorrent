#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{
    EncodingIdentifier, IDecoderAdapter, IEncoderAdapter, IEncodingContract, build_type_name_params,
};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_delivery_context,
};
use random::IRandomSourceAdapter;

use super::super::factory::provides::{
    EmitErrorReturn, EmitPayloadOverrides, EmitReturn, GENERATE_INTERFACE_VERSION,
    IGenerateAdapter, VectorKind, VectorKindDeclarationOverrides, VectorKindSettings,
    VectorKindSettingsOverrides, VectorKindSize, VectorKindSizing, build_emit_params,
    build_emit_payload, build_vector_kind_declaration, build_vector_kind_settings,
};
use super::challenge_vectors::provides::{
    CHALLENGE_VECTOR_MEMBER_NAMES, ChallengeVectorDescription, build_challenge_vector_description,
};
use super::group_vectors::provides::{build_msm_term_count, build_vector_count};
use super::interface::{
    GeneratedDirectory, GeneratedDirectoryTryNewErrorReturn, GeneratedFileName,
    GeneratedFileNameTryNewErrorReturn, ISolidityGenerateCaseConsumer,
    SOLIDITY_GENERATE_DECLARATION, SolidityGenerateErrorReturn, SolidityGenerateSettings,
};
use super::mock::*;
use super::possession_vectors::provides::{
    POSSESSION_VECTOR_MEMBER_NAMES, PossessionVectorDescription,
    build_possession_vector_description,
};
use super::proof_vectors::provides::{
    MINT_PROOF_VECTOR_MEMBER_NAMES, MintProofVectorDescription, build_mint_proof_vector_description,
};
use super::render::provides::{
    SolidityAbiTypeConstructorParamsOverrides, SolidityRecordMemberOverrides,
    build_solidity_abi_type, build_solidity_record_member,
};
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: a text whose every byte is an ASCII letter, digit, `.`, `_`, or
///   `-` is admitted.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"AZaz09._-"`, holding the first and last letter of each
///   case and the first and last digit.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: `result.map(|name| name.text)` equals `Ok("AZaz09._-")` by
///   `assert_eq!`.
#[test]
fn generated_file_name_admits_every_character_class_it_allows() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("AZaz09._-".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result.map(|name| name.text), Ok("AZaz09._-".to_string()));
}
// generated_file_name_admits_every_character_class_it_allows

/// Contract: no byte is refused `Empty`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `""`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(GeneratedFileNameTryNewErrorReturn::Empty)`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_empty_text() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some(String::new()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Empty));
}
// generated_file_name_refuses_empty_text

/// Contract: the text `.` is refused `Reserved`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"."`, a byte the character set admits, so only the
///   reserved decision refuses.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(GeneratedFileNameTryNewErrorReturn::Reserved)`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_single_dot_text() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some(".".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Reserved));
}
// generated_file_name_refuses_the_single_dot_text

/// Contract: the text `..` is refused `Reserved`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `".."`, bytes the character set admits, so only the
///   reserved decision refuses.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(GeneratedFileNameTryNewErrorReturn::Reserved)`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_double_dot_text() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("..".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Reserved));
}
// generated_file_name_refuses_the_double_dot_text

/// Contract: the lowest index whose byte is outside the admitted set is
///   refused `InvalidCharacter` at that index.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"ab/c?"`, holding two invalid bytes at different indexes.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals
///   `Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 2, byte: b'/' })`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_lowest_invalid_byte_index() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("ab/c?".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 2,
            byte: b'/'
        })
    );
}
// generated_file_name_refuses_the_lowest_invalid_byte_index

/// Contract: the byte below `b'A'` is refused `InvalidCharacter`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"@"`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(InvalidCharacter { index: 0, byte: b'@' })`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_before_the_uppercase_letters() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("@".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: b'@'
        })
    );
}
// generated_file_name_refuses_the_byte_before_the_uppercase_letters

/// Contract: the byte above `b'Z'` is refused `InvalidCharacter`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"["`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(InvalidCharacter { index: 0, byte: b'[' })`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_after_the_uppercase_letters() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("[".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: b'['
        })
    );
}
// generated_file_name_refuses_the_byte_after_the_uppercase_letters

/// Contract: the byte below `b'a'` is refused `InvalidCharacter`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"`"`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(InvalidCharacter { index: 0, byte: b'`' })`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_before_the_lowercase_letters() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("`".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: b'`'
        })
    );
}
// generated_file_name_refuses_the_byte_before_the_lowercase_letters

/// Contract: the byte above `b'z'` is refused `InvalidCharacter`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"{"`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(InvalidCharacter { index: 0, byte: b'{' })`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_after_the_lowercase_letters() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("{".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: b'{'
        })
    );
}
// generated_file_name_refuses_the_byte_after_the_lowercase_letters

/// Contract: the byte below `b'0'` is refused `InvalidCharacter`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"/"`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(InvalidCharacter { index: 0, byte: b'/' })`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_before_the_digits() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("/".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: b'/'
        })
    );
}
// generated_file_name_refuses_the_byte_before_the_digits

/// Contract: the byte above `b'9'` is refused `InvalidCharacter`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `":"`.
/// Act:    `GeneratedFileName::try_new`.
/// Assert: the result equals `Err(InvalidCharacter { index: 0, byte: b':' })`
///   by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_after_the_digits() {
    // Arrange
    let params =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some(":".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(
        result,
        Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
            index: 0,
            byte: b':'
        })
    );
}
// generated_file_name_refuses_the_byte_after_the_digits

/// Contract: `as_str` returns the text the constructor held.
/// Arrange: `build_generated_file_name` with `text` overridden to
///   `"g1_add.vectors"`.
/// Act:    `as_str`.
/// Assert: the returned text equals `"g1_add.vectors"` by `assert_eq!`.
#[test]
fn generated_file_name_as_str_returns_its_text() {
    // Arrange
    let name = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("g1_add.vectors".to_string()),
    });

    // Act
    let text = name.as_str();

    // Assert
    assert_eq!(text, "g1_add.vectors");
}
// generated_file_name_as_str_returns_its_text

/// Contract: empty `segments` is refused `Empty`.
/// Arrange: `build_generated_directory_constructor_params` with `segments`
///   overridden to `Vec::new()`.
/// Act:    `GeneratedDirectory::try_new`.
/// Assert: the result equals `Err(GeneratedDirectoryTryNewErrorReturn::Empty)`
///   by `assert_eq!`.
#[test]
fn generated_directory_refuses_no_segments() {
    // Arrange
    let params = build_generated_directory_constructor_params(
        GeneratedDirectoryConstructorParamsOverrides {
            segments: Some(Vec::new()),
        },
    );

    // Act
    let result = GeneratedDirectory::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedDirectoryTryNewErrorReturn::Empty));
}
// generated_directory_refuses_no_segments

/// Contract: `params.segments` not empty is admitted, the segments in the
///   params' order.
/// Arrange: `build_generated_directory_constructor_params` with `segments`
///   overridden to two `build_generated_file_name` values over `"suite"` and
///   then `"generated"`, an order that is neither sorted nor the builder's.
/// Act:    `GeneratedDirectory::try_new`.
/// Assert: `result.map(|directory| directory.segments)` equals `Ok` of the
///   same two names in the same order by `assert_eq!`.
#[test]
fn generated_directory_admits_its_segments_in_order() {
    // Arrange
    let params = build_generated_directory_constructor_params(
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
    );

    // Act
    let result = GeneratedDirectory::try_new(params);

    // Assert
    assert_eq!(
        result.map(|directory| directory.segments),
        Ok(vec![
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("suite".to_string()),
            }),
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("generated".to_string()),
            }),
        ])
    );
}
// generated_directory_admits_its_segments_in_order

/// Contract: `SOLIDITY_GENERATE_DECLARATION` holds, in order, the kinds,
///   sizings, and second-group gates the interaction spec lists,
///   `adapter_version` `1`, `interface_version` `GENERATE_INTERFACE_VERSION`,
///   and `encoding_identifier` `EncodingIdentifier::EthereumAbiV1`.
/// Arrange: none, the declaration depending on no type parameter of
///   `SolidityGenerate`.
/// Act:    read `SOLIDITY_GENERATE_DECLARATION`.
/// Assert: its `kinds` equals, in the spec's order, one
///   `build_vector_kind_declaration` per kind with `kind`, `sizing`, and
///   `requires_second_group_arithmetic` overridden to the spec's values, and
///   its `adapter_version` equals `1`, its `interface_version` equals
///   `GENERATE_INTERFACE_VERSION`, and its `encoding_identifier` equals
///   `EncodingIdentifier::EthereumAbiV1`, by `assert_eq!`.
#[test]
fn solidity_generate_declares_its_kinds_with_their_sizing_and_second_group_gates() {
    // Arrange
    let expected = [
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::G1Addition),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::G1Multiplication),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::PairingCheck),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::G2Addition),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(true),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::G1MultiScalarMultiplication),
            sizing: Some(VectorKindSizing::DrawnTerms),
            requires_second_group_arithmetic: Some(true),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::G2MultiScalarMultiplication),
            sizing: Some(VectorKindSizing::DrawnTerms),
            requires_second_group_arithmetic: Some(true),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::DecodeRejection),
            sizing: Some(VectorKindSizing::Enumerated),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::HashToScalar),
            sizing: Some(VectorKindSizing::DrawnMessages),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::IdentityMapping),
            sizing: Some(VectorKindSizing::OverConfiguredInputs),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::Possession),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::Challenge),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::MintProof),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(false),
        }),
        build_vector_kind_declaration(VectorKindDeclarationOverrides {
            kind: Some(VectorKind::TransferProof),
            sizing: Some(VectorKindSizing::Drawn),
            requires_second_group_arithmetic: Some(false),
        }),
    ];

    // Act
    let declaration = SOLIDITY_GENERATE_DECLARATION;

    // Assert
    assert_eq!(declaration.kinds, expected.as_slice());
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, GENERATE_INTERFACE_VERSION);
    assert_eq!(
        declaration.encoding_identifier,
        EncodingIdentifier::EthereumAbiV1
    );
}
// solidity_generate_declares_its_kinds_with_their_sizing_and_second_group_gates

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
    let mut settings = build_solidity_generate_settings(Default::default());
    settings.kinds.retain(|kind_settings| {
        !matches!(
            kind_settings.kind,
            VectorKind::Possession | VectorKind::Challenge
        )
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
                kind: VectorKind::Possession,
            }
        ))
    ));
    assert!(!root.exists());
}
// emit_refuses_settings_missing_an_emitted_kind

/// Contract: an emitted kind with no entry in `self.sizes` is refused
///   `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MissingKindSettings { kind }))`,
///   `kind` the first such kind in declared order, before any function is
///   called.
/// Arrange: `sizes` overridden to one `build_vector_kind_settings` per kind
///   the default list names but the challenge and mint-proof kinds, each
///   with the kind and size the default sizes hold, and `settings` at its
///   default, so two emitted kinds lack sizes; the root removed before the
///   act.
/// Act:    `emit` once inside the case consumer's method.
/// Assert: `matches!(result, Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MissingKindSettings { kind: VectorKind::Challenge })))`
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
    let settings = build_solidity_generate_settings(Default::default());
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
        Err(EmitErrorReturn::Solidity(
            SolidityGenerateErrorReturn::MissingKindSettings {
                kind: VectorKind::Challenge,
            }
        ))
    ));
    assert!(!root.exists());
}
// emit_refuses_sizes_missing_an_emitted_kind

/// Contract: the concrete emits every declared kind over the encoding
///   family's mock encoder; each emitted kind's declaration holds its
///   members in order, each carrying the name its description's
///   `member_names()` declares at that index, the kind its description's
///   `FIELDS` holds at that index, and the `SolidityAbiType` whose text is
///   the mock's declared name for that kind.
/// Arrange: the default selections, which select the encoding family's mock
///   concrete; the expected members of the possession kind, the challenge
///   kind, and the mint-proof kind, whose descriptions hold different
///   kinds, each built by `build_solidity_record_member` from the expected
///   name, the kind in `FIELDS`, and `build_solidity_abi_type` over that
///   kind's mock name, so a member typed by another kind's name fails.
/// Act:    `emit`, once, in the case consumer's method.
/// Assert: the result is `Ok` and the records library, read from its
///   configured file name, holds for each of those kinds one member line
///   per field carrying the expected name and the expected mock type text.
#[test]
fn each_record_member_carries_the_encoders_type_name_for_its_kind() {
    // Arrange
    let settings = build_solidity_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let _ = std::fs::remove_dir_all(&root);
    let canonicals: &[&[u8]] = &[b"canonical-one".as_slice(), b"canonical-two".as_slice()];

    struct Case;

    impl ISolidityGenerateCaseConsumer for Case {
        type Output = (EmitReturn, Vec<String>);

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
            let kinds = [
                (
                    possession_names,
                    <PossessionVectorDescription as IEncodingContract>::FIELDS,
                    &POSSESSION_VECTOR_MEMBER_NAMES[..],
                ),
                (
                    challenge_names,
                    <ChallengeVectorDescription as IEncodingContract>::FIELDS,
                    &CHALLENGE_VECTOR_MEMBER_NAMES[..],
                ),
                (
                    mint_proof_names,
                    <MintProofVectorDescription<F> as IEncodingContract>::FIELDS,
                    &MINT_PROOF_VECTOR_MEMBER_NAMES[..],
                ),
            ];
            let mut expected_lines = Vec::new();
            for (names, fields, texts) in &kinds {
                for ((name, kind), text) in names.iter().zip(fields.iter()).zip(texts.iter()) {
                    let named = encoder
                        .type_name(build_type_name_params(Default::default()), *kind)
                        .expect("the selected encoder names every declared field kind");
                    let member = build_solidity_record_member(SolidityRecordMemberOverrides {
                        name: Some(name.clone()),
                        kind: Some(*kind),
                        abi_type: Some(build_solidity_abi_type(
                            SolidityAbiTypeConstructorParamsOverrides {
                                name: Some(named.name.as_str().to_owned()),
                            },
                        )),
                    });
                    expected_lines.push(format!("        {} {};", member.abi_type.as_str(), text));
                }
            }
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
            (
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
                ),
                expected_lines,
            )
        }
    }

    // Act
    let (result, expected_lines) = run_solidity_generate_case(
        &build_solidity_generate_case_deps(Case, Default::default()),
        build_solidity_generate_case_params(Default::default()),
        build_solidity_generate_case_payload(canonicals, &[], Default::default()),
    )
    .expect("the case chain runs clean")
    .output;

    // Assert
    let Ok(success) = result else {
        panic!("emit admits the case's arrangement")
    };
    let records = success
        .files
        .last()
        .expect("the last file written is the records library");
    let source = std::fs::read_to_string(&records.path).expect("the records library was written");
    for line in expected_lines {
        assert!(
            source.contains(&line),
            "the records library holds the member line {line:?}"
        );
    }
    assert!(root.exists());
}
// each_record_member_carries_the_encoders_type_name_for_its_kind
