#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;
use std::path::Path;

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHashConstructorParamsOverrides,
    AssetIdentityHashTryNewErrorReturn, build_asset_identity_hash,
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
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, IDeliveryProofAdapter,
    IDeliveryProofConsumer, build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, IRandomSourceAdapter,
    build_create_random_source_params, create_random_source,
};

use super::super::factory::provides::{
    EmitErrorReturn, EmitReturn, GENERATE_INTERFACE_VERSION, IGenerateAdapter, VectorKind,
    VectorKindDeclarationOverrides, VectorKindSettings, VectorKindSettingsOverrides,
    VectorKindSizing, build_emit_params, build_emit_payload, build_vector_kind_declaration,
    build_vector_kind_settings,
};
use super::challenge_vectors::provides::build_canonical_delivery_context;
use super::interface::{
    EVM_GENERATE_DECLARATION, EvmGenerateErrorReturn, GeneratedDirectory,
    GeneratedDirectoryConstructorParams, GeneratedDirectoryTryNewErrorReturn, GeneratedFileName,
    GeneratedFileNameConstructorParams, GeneratedFileNameTryNewErrorReturn,
};
use super::mock::*;

/// Contract: a text whose every byte is an ASCII letter, digit, `.`, `_`,
///   or `-` → `Ok(GeneratedFileName { text })`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"AZaz09._-"`, holding the first and last letter of each
///   case and the first and last digit.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  `result.map(|name| name.text)` equals `Ok("AZaz09._-"
///   .to_string())` by `assert_eq!`.
#[test]
fn generated_file_name_admits_every_character_class_it_allows() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("AZaz09._-".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result.map(|name| name.text), Ok("AZaz09._-".to_string()));
    // generated_file_name_admits_every_character_class_it_allows
}

/// Contract: no byte → `Err(GeneratedFileNameTryNewErrorReturn::Empty)`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `""`.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_empty_text() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some(String::new()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Empty));
    // generated_file_name_refuses_empty_text
}

/// Contract: the text is `.` → `Err(GeneratedFileNameTryNewErrorReturn::
///   Reserved)`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"."`, a byte the character set admits, so only the
///   reserved decision refuses.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   Reserved)` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_single_dot_text() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some(".".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Reserved));
    // generated_file_name_refuses_the_single_dot_text
}

/// Contract: the text is `..` → `Err(GeneratedFileNameTryNewErrorReturn::
///   Reserved)`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `".."`, bytes the character set admits, so only the
///   reserved decision refuses.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   Reserved)` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_double_dot_text() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
        build_generated_file_name_constructor_params(GeneratedFileNameConstructorParamsOverrides {
            text: Some("..".to_string()),
        });

    // Act
    let result = GeneratedFileName::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedFileNameTryNewErrorReturn::Reserved));
    // generated_file_name_refuses_the_double_dot_text
}

/// Contract: the lowest index `i` whose byte is outside the admitted set →
///   `Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: i,
///   byte })`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"ab/c?"`, holding two invalid bytes at different indexes.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   InvalidCharacter { index: 2, byte: b'/' })` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_lowest_invalid_byte_index() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
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
    // generated_file_name_refuses_the_lowest_invalid_byte_index
}

/// Contract: a byte outside the admitted set → `Err(GeneratedFileNameTryNew
///   ErrorReturn::InvalidCharacter { index: 0, byte })`.
/// Arrange: `build_generated_file_name_constructor_params` with `text`
///   overridden to `"@"`, the byte below `b'A'`.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: b'@' })` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_before_the_uppercase_letters() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
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
    // generated_file_name_refuses_the_byte_before_the_uppercase_letters
}

/// Contract: a byte outside the admitted set → `Err(GeneratedFileNameTryNew
///   ErrorReturn::InvalidCharacter { index: 0, byte })`.
/// Arrange: `text` overridden to `"["`, the byte above `b'Z'`.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: b'[' })` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_after_the_uppercase_letters() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
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
    // generated_file_name_refuses_the_byte_after_the_uppercase_letters
}

/// Contract: a byte outside the admitted set → `Err(GeneratedFileNameTryNew
///   ErrorReturn::InvalidCharacter { index: 0, byte })`.
/// Arrange: `text` overridden to `` "`" ``, the byte below `b'a'`.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: b'`' })` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_before_the_lowercase_letters() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
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
    // generated_file_name_refuses_the_byte_before_the_lowercase_letters
}

/// Contract: a byte outside the admitted set → `Err(GeneratedFileNameTryNew
///   ErrorReturn::InvalidCharacter { index: 0, byte })`.
/// Arrange: `text` overridden to `"{"`, the byte above `b'z'`.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: b'{' })` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_after_the_lowercase_letters() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
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
    // generated_file_name_refuses_the_byte_after_the_lowercase_letters
}

/// Contract: a byte outside the admitted set → `Err(GeneratedFileNameTryNew
///   ErrorReturn::InvalidCharacter { index: 0, byte })`.
/// Arrange: `text` overridden to `"/"`, the byte below `b'0'`.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: b'/' })` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_before_the_digits() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
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
    // generated_file_name_refuses_the_byte_before_the_digits
}

/// Contract: a byte outside the admitted set → `Err(GeneratedFileNameTryNew
///   ErrorReturn::InvalidCharacter { index: 0, byte })`.
/// Arrange: `text` overridden to `":"`, the byte above `b'9'`.
/// Act:     `GeneratedFileName::try_new`.
/// Assert:  the result equals `Err(GeneratedFileNameTryNewErrorReturn::
///   InvalidCharacter { index: 0, byte: b':' })` by `assert_eq!`.
#[test]
fn generated_file_name_refuses_the_byte_after_the_digits() {
    // Arrange
    let params: GeneratedFileNameConstructorParams =
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
    // generated_file_name_refuses_the_byte_after_the_digits
}

/// Contract: `as_str(&self) -> &str` returns the text.
/// Arrange: `build_generated_file_name` with `text` overridden to
///   `"g1_add.vectors"`.
/// Act:     `as_str`.
/// Assert:  the returned text equals `"g1_add.vectors"` by `assert_eq!`.
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
    // generated_file_name_as_str_returns_its_text
}

/// Contract: `params.segments` is empty → `Err(GeneratedDirectoryTryNew
///   ErrorReturn::Empty)`.
/// Arrange: `build_generated_directory_constructor_params` with `segments`
///   overridden to `Vec::new()`.
/// Act:     `GeneratedDirectory::try_new`.
/// Assert:  the result equals `Err(GeneratedDirectoryTryNewErrorReturn::
///   Empty)` by `assert_eq!`.
#[test]
fn generated_directory_refuses_no_segments() {
    // Arrange
    let params: GeneratedDirectoryConstructorParams = build_generated_directory_constructor_params(
        GeneratedDirectoryConstructorParamsOverrides {
            segments: Some(Vec::new()),
        },
    );

    // Act
    let result = GeneratedDirectory::try_new(params);

    // Assert
    assert_eq!(result, Err(GeneratedDirectoryTryNewErrorReturn::Empty));
    // generated_directory_refuses_no_segments
}

/// Contract: `params.segments` is not empty → `Ok(GeneratedDirectory {
///   segments })`, the segments in the params' order.
/// Arrange: `build_generated_directory_constructor_params` with `segments`
///   overridden to two `build_generated_file_name` values over `"suite"` and
///   then `"generated"`, an order that is neither sorted nor the builder's.
/// Act:     `GeneratedDirectory::try_new`.
/// Assert:  `result.map(|directory| directory.segments)` equals `Ok` of the
///   same two names in the same order by `assert_eq!`.
#[test]
fn generated_directory_admits_its_segments_in_order() {
    // Arrange
    let suite = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("suite".to_string()),
    });
    let generated = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("generated".to_string()),
    });
    let params: GeneratedDirectoryConstructorParams = build_generated_directory_constructor_params(
        GeneratedDirectoryConstructorParamsOverrides {
            segments: Some(vec![suite.clone(), generated.clone()]),
        },
    );

    // Act
    let result = GeneratedDirectory::try_new(params);

    // Assert
    assert_eq!(
        result.map(|directory| directory.segments),
        Ok(vec![suite, generated])
    );
    // generated_directory_admits_its_segments_in_order
}

/// Contract: `EVM_GENERATE_DECLARATION`, the declaration `EvmGenerate`'s
///   `IGenerateAdapter` implementation names, holds, in order, the kinds,
///   sizings, and second-group gates the interaction spec lists,
///   `adapter_version` `1`, and `interface_version` `GENERATE_INTERFACE_VERSION`.
/// Arrange: none; the declaration depends on no type parameter of
///   `EvmGenerate`.
/// Act:     read `EVM_GENERATE_DECLARATION`.
/// Assert:  its `kinds` equals, in the spec's order, one
///   `build_vector_kind_declaration` per kind with `kind`, `sizing`, and
///   `requires_second_group_arithmetic` overridden to the spec's values, and
///   its `adapter_version` equals `1` and its `interface_version` equals
///   `GENERATE_INTERFACE_VERSION`, by `assert_eq!`, the kinds differing from
///   each other in sizing and gate so a swapped entry fails.
#[test]
fn evm_generate_declares_its_kinds_with_their_sizing_and_second_group_gates() {
    // Act
    let declaration = EVM_GENERATE_DECLARATION;

    // Assert
    let expected = vec![
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
    assert_eq!(declaration.kinds, expected.as_slice());
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(declaration.interface_version, GENERATE_INTERFACE_VERSION);
    // evm_generate_declares_its_kinds_with_their_sizing_and_second_group_gates
}

/// Contract: an emitted kind with no entry in `self.settings.kinds` →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MissingKindSettings {
///   kind }))`, `kind` the first such kind in declared order, before any
///   function is called.
/// Arrange: `settings.kinds` overridden to one `build_evm_vector_kind_settings`
///   per kind the default list names but the possession and challenge kinds,
///   each with the kind and names the default list holds, and `sizes` at its
///   default, so two emitted kinds lack settings; the settings' root removed
///   before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn
///   ::MissingKindSettings { kind: VectorKind::Possession })))` and the root
///   does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_refuses_settings_missing_an_emitted_kind() {
    // Arrange
    let default = build_evm_generate_settings(Default::default());
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .filter(|entry| entry.kind != VectorKind::Possession && entry.kind != VectorKind::Challenge)
        .map(|entry| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(entry.names),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
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
                    settings: self.overrides.settings.clone(),
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
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Evm(
            EvmGenerateErrorReturn::MissingKindSettings {
                kind: VectorKind::Possession
            }
        ))
    ));
    assert!(!Path::exists(&root));
    // emit_refuses_settings_missing_an_emitted_kind
}

/// Contract: an emitted kind with no entry in `self.sizes` →
///   `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MissingKindSettings {
///   kind }))`, `kind` the first such kind in declared order, before any
///   function is called.
/// Arrange: `sizes` overridden to one `build_vector_kind_settings` per kind
///   the default list names but the challenge and mint-proof kinds, each with
///   the kind and size the default sizes hold, and `settings` at its default,
///   so two emitted kinds lack sizes; the default settings' root removed
///   before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn
///   ::MissingKindSettings { kind: VectorKind::Challenge })))` and the root
///   does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_refuses_sizes_missing_an_emitted_kind() {
    // Arrange
    let settings = build_evm_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let default_sizes = build_evm_generate_constructor_params(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides::default(),
            )
            .sizes;
            let sizes: Vec<VectorKindSettings> = default_sizes
                .into_iter()
                .filter(|entry| {
                    entry.kind != VectorKind::Challenge && entry.kind != VectorKind::MintProof
                })
                .map(|entry| {
                    build_vector_kind_settings(VectorKindSettingsOverrides {
                        kind: Some(entry.kind),
                        size: Some(entry.size),
                    })
                })
                .collect();
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
                    settings: self.overrides.settings.clone(),
                    sizes: Some(sizes),
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
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Evm(
            EvmGenerateErrorReturn::MissingKindSettings {
                kind: VectorKind::Challenge
            }
        ))
    ));
    assert!(!Path::exists(&root));
    // emit_refuses_sizes_missing_an_emitted_kind
}

/// Contract: under `IdentityScope::Asset`, the lowest index `i` whose
///   identity's length is not `ASSET_IDENTITY_HASH_LENGTH` →
///   `Err(EmitErrorReturn::AssetIdentityLength { index: i, actual })`,
///   before any setting is read.
/// Arrange: `scope` overridden to `IdentityScope::Asset`; `identities`
///   `vec![hash.clone(), vec![0x22; ASSET_IDENTITY_HASH_LENGTH - 1],
///   vec![0x33; ASSET_IDENTITY_HASH_LENGTH + 1]]`, two refused lengths at
///   different indexes after an admitted identity; `proof_identity` `hash`;
///   `settings.kinds` missing the possession and challenge kinds as
///   `emit_refuses_settings_missing_an_emitted_kind` arranges, so the
///   refusal's precedence over every setting read shows; the settings' root
///   removed before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::AssetIdentityLength
///   { index: 1, actual }) if actual == ASSET_IDENTITY_HASH_LENGTH - 1)`
///   and the root does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_refuses_an_asset_scope_identity_of_another_length_with_its_index() {
    // Arrange
    let default = build_evm_generate_settings(Default::default());
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .filter(|entry| entry.kind != VectorKind::Possession && entry.kind != VectorKind::Challenge)
        .map(|entry| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(entry.names),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        scope: Some(IdentityScope::Asset),
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let hash = build_asset_identity_hash(Default::default())
                .as_bytes()
                .to_vec();
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![
                hash.clone(),
                vec![0x22; ASSET_IDENTITY_HASH_LENGTH - 1],
                vec![0x33; ASSET_IDENTITY_HASH_LENGTH + 1],
            ];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    scope: self.overrides.scope,
                    settings: self.overrides.settings.clone(),
                    ..Default::default()
                },
            );
            let emit_payload =
                build_emit_payload(&mint, &transfer, &identities, &hash, Default::default());

            // Act
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::AssetIdentityLength { index: 1, actual })
            if actual == ASSET_IDENTITY_HASH_LENGTH - 1
    ));
    assert!(!Path::exists(&root));
    // emit_refuses_an_asset_scope_identity_of_another_length_with_its_index
}

/// Contract: under `IdentityScope::Asset`, an identity `AssetIdentityHash::
///   try_new` refuses → `Err(EmitErrorReturn::AssetIdentityHash { index,
///   error })`, `error` unchanged, before any setting is read.
/// Arrange: `scope` overridden to `IdentityScope::Asset`; `identities`
///   `vec![hash.clone(), vec![0x00; ASSET_IDENTITY_HASH_LENGTH]]`, an
///   all-zero value of the admitted length after an admitted identity;
///   `proof_identity` `hash`; `settings.kinds` missing the possession and
///   challenge kinds as `emit_refuses_settings_missing_an_emitted_kind`
///   arranges, so the refusal's precedence over every setting read shows;
///   the settings' root removed before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::AssetIdentityHash
///   { index: 1, error: AssetIdentityHashTryNewErrorReturn::AllZero }))`
///   and the root does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_returns_an_asset_scope_identitys_hash_refusal_with_its_index() {
    // Arrange
    let default = build_evm_generate_settings(Default::default());
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .filter(|entry| entry.kind != VectorKind::Possession && entry.kind != VectorKind::Challenge)
        .map(|entry| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(entry.names),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        scope: Some(IdentityScope::Asset),
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let hash = build_asset_identity_hash(Default::default())
                .as_bytes()
                .to_vec();
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![hash.clone(), vec![0x00; ASSET_IDENTITY_HASH_LENGTH]];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    scope: self.overrides.scope,
                    settings: self.overrides.settings.clone(),
                    ..Default::default()
                },
            );
            let emit_payload =
                build_emit_payload(&mint, &transfer, &identities, &hash, Default::default());

            // Act
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::AssetIdentityHash {
            index: 1,
            error: AssetIdentityHashTryNewErrorReturn::AllZero
        })
    ));
    assert!(!Path::exists(&root));
    // emit_returns_an_asset_scope_identitys_hash_refusal_with_its_index
}

/// Contract: under `IdentityScope::Asset`, a proof identity whose length is
///   not `ASSET_IDENTITY_HASH_LENGTH` → `Err(EmitErrorReturn::
///   AssetProofIdentityLength { actual })`, before any setting is read.
/// Arrange: `scope` overridden to `IdentityScope::Asset`; `identities`
///   `vec![hash.clone()]`; `proof_identity` `vec![0x22;
///   ASSET_IDENTITY_HASH_LENGTH + 1]`; `settings.kinds` missing the
///   possession and challenge kinds as
///   `emit_refuses_settings_missing_an_emitted_kind` arranges, so the
///   refusal's precedence over every setting read shows; the settings' root
///   removed before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::AssetProofIdentityLength
///   { actual }) if actual == ASSET_IDENTITY_HASH_LENGTH + 1)` and the root
///   does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_refuses_an_asset_scope_proof_identity_of_another_length() {
    // Arrange
    let default = build_evm_generate_settings(Default::default());
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .filter(|entry| entry.kind != VectorKind::Possession && entry.kind != VectorKind::Challenge)
        .map(|entry| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(entry.names),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        scope: Some(IdentityScope::Asset),
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let hash = build_asset_identity_hash(Default::default())
                .as_bytes()
                .to_vec();
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![hash.clone()];
            let proof_identity = vec![0x22; ASSET_IDENTITY_HASH_LENGTH + 1];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    scope: self.overrides.scope,
                    settings: self.overrides.settings.clone(),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &proof_identity,
                Default::default(),
            );

            // Act
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::AssetProofIdentityLength { actual })
            if actual == ASSET_IDENTITY_HASH_LENGTH + 1
    ));
    assert!(!Path::exists(&root));
    // emit_refuses_an_asset_scope_proof_identity_of_another_length
}

/// Contract: under `IdentityScope::Asset`, a proof identity
///   `AssetIdentityHash::try_new` refuses → `Err(EmitErrorReturn::
///   AssetProofIdentityHash(error))`, `error` unchanged, before any setting
///   is read.
/// Arrange: `scope` overridden to `IdentityScope::Asset`; `identities`
///   `vec![hash.clone()]`; `proof_identity` `vec![0x00;
///   ASSET_IDENTITY_HASH_LENGTH]`; `settings.kinds` missing the possession
///   and challenge kinds as `emit_refuses_settings_missing_an_emitted_kind`
///   arranges, so the refusal's precedence over every setting read shows;
///   the settings' root removed before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::AssetProofIdentityHash(
///   AssetIdentityHashTryNewErrorReturn::AllZero)))` and the root does not
///   exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_returns_an_asset_scope_proof_identitys_hash_refusal() {
    // Arrange
    let default = build_evm_generate_settings(Default::default());
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .filter(|entry| entry.kind != VectorKind::Possession && entry.kind != VectorKind::Challenge)
        .map(|entry| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(entry.names),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        scope: Some(IdentityScope::Asset),
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let hash = build_asset_identity_hash(Default::default())
                .as_bytes()
                .to_vec();
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![hash.clone()];
            let proof_identity = vec![0x00; ASSET_IDENTITY_HASH_LENGTH];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    scope: self.overrides.scope,
                    settings: self.overrides.settings.clone(),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &proof_identity,
                Default::default(),
            );

            // Act
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::AssetProofIdentityHash(
            AssetIdentityHashTryNewErrorReturn::AllZero
        ))
    ));
    assert!(!Path::exists(&root));
    // emit_returns_an_asset_scope_proof_identitys_hash_refusal
}

/// Contract: under `IdentityScope::Asset`, the configured identities are
///   admitted before the proof identity → the identity's refusal.
/// Arrange: `scope` overridden to `IdentityScope::Asset`; `identities`
///   `vec![vec![0x22; ASSET_IDENTITY_HASH_LENGTH - 1]]` and
///   `proof_identity` `vec![0x00; ASSET_IDENTITY_HASH_LENGTH]`, each
///   refused; the settings' root removed before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::AssetIdentityLength
///   { index: 0, actual }) if actual == ASSET_IDENTITY_HASH_LENGTH - 1)`
///   and the root does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_refuses_an_asset_scope_identity_before_the_proof_identity() {
    // Arrange
    let settings = build_evm_generate_settings(Default::default());
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        scope: Some(IdentityScope::Asset),
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![vec![0x22; ASSET_IDENTITY_HASH_LENGTH - 1]];
            let proof_identity = vec![0x00; ASSET_IDENTITY_HASH_LENGTH];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    scope: self.overrides.scope,
                    settings: self.overrides.settings.clone(),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &proof_identity,
                Default::default(),
            );

            // Act
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::AssetIdentityLength { index: 0, actual })
            if actual == ASSET_IDENTITY_HASH_LENGTH - 1
    ));
    assert!(!Path::exists(&root));
    // emit_refuses_an_asset_scope_identity_before_the_proof_identity
}

/// Contract: under `IdentityScope::Asset`, identities and a proof identity
///   `AssetIdentityHash::try_new` admits → no refusal, the settings read
///   next.
/// Arrange: `scope` overridden to `IdentityScope::Asset`; `identities`
///   `vec![hash.clone(), other_hash]`; `proof_identity` `hash`;
///   `settings.kinds` missing the possession and challenge kinds as
///   `emit_refuses_settings_missing_an_emitted_kind` arranges, so the
///   settings read shows admission passed; the settings' root removed
///   before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn
///   ::MissingKindSettings { kind: VectorKind::Possession })))` and the
///   root does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_admits_every_asset_identity_hash_under_the_asset_scope() {
    // Arrange
    let default = build_evm_generate_settings(Default::default());
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .filter(|entry| entry.kind != VectorKind::Possession && entry.kind != VectorKind::Challenge)
        .map(|entry| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(entry.names),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        scope: Some(IdentityScope::Asset),
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let hash = build_asset_identity_hash(Default::default())
                .as_bytes()
                .to_vec();
            let other_hash =
                build_asset_identity_hash(AssetIdentityHashConstructorParamsOverrides {
                    bytes: Some([0x11; ASSET_IDENTITY_HASH_LENGTH]),
                })
                .as_bytes()
                .to_vec();
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![hash.clone(), other_hash];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    scope: self.overrides.scope,
                    settings: self.overrides.settings.clone(),
                    ..Default::default()
                },
            );
            let emit_payload =
                build_emit_payload(&mint, &transfer, &identities, &hash, Default::default());

            // Act
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Evm(
            EvmGenerateErrorReturn::MissingKindSettings {
                kind: VectorKind::Possession
            }
        ))
    ));
    assert!(!Path::exists(&root));
    // emit_admits_every_asset_identity_hash_under_the_asset_scope
}

/// Contract: under `IdentityScope::Entitlement`, every identity and the
///   proof identity is the canonical identity → no refusal, the settings
///   read next.
/// Arrange: `scope` the builder's default `IdentityScope::Entitlement`;
///   `identities` `vec![vec![0x22; ASSET_IDENTITY_HASH_LENGTH - 1],
///   vec![0x00; ASSET_IDENTITY_HASH_LENGTH]]` and `proof_identity`
///   `vec![0x00; ASSET_IDENTITY_HASH_LENGTH]`, values the asset scope
///   refuses; `settings.kinds` missing the possession and challenge kinds
///   as `emit_refuses_settings_missing_an_emitted_kind` arranges, so the
///   settings read shows admission passed; the settings' root removed
///   before the act.
/// Act:     `emit` in the delivery-proof consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn
///   ::MissingKindSettings { kind: VectorKind::Possession })))` and the
///   root does not exist.
/// Collaborators: the pairing's, hash-to-scalar's, encoding's, chain-forms',
///   kem's, key-agreement's, delivery-proof's, and random-source's mock
///   concretes in their default modes through their factories.
#[test]
fn emit_admits_identities_as_given_under_the_entitlement_scope() {
    // Arrange
    let default = build_evm_generate_settings(Default::default());
    let kinds: Vec<_> = default
        .kinds
        .into_iter()
        .filter(|entry| entry.kind != VectorKind::Possession && entry.kind != VectorKind::Challenge)
        .map(|entry| {
            build_evm_vector_kind_settings(EvmVectorKindSettingsOverrides {
                kind: Some(entry.kind),
                names: Some(entry.names),
            })
        })
        .collect();
    let settings = build_evm_generate_settings(EvmGenerateSettingsOverrides {
        kinds: Some(kinds),
        ..Default::default()
    });
    let root = settings.foundry_root.clone();
    let overrides = EvmGenerateConstructorParamsOverrides {
        settings: Some(settings),
        ..Default::default()
    };
    let _ = std::fs::remove_dir_all(&root);

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
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
        type Output = EmitReturn;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let mint = build_canonical_delivery_context(Default::default());
            let transfer = build_canonical_delivery_context(Default::default());
            let identities = vec![
                vec![0x22; ASSET_IDENTITY_HASH_LENGTH - 1],
                vec![0x00; ASSET_IDENTITY_HASH_LENGTH],
            ];
            let proof_identity = vec![0x00; ASSET_IDENTITY_HASH_LENGTH];
            let subject = build_evm_generate(
                self.pairing,
                self.hashing,
                self.encoder,
                self.random,
                self.kem,
                self.key_agreement,
                &payload.adapter,
                EvmGenerateConstructorParamsOverrides {
                    scope: self.overrides.scope,
                    settings: self.overrides.settings.clone(),
                    ..Default::default()
                },
            );
            let emit_payload = build_emit_payload(
                &mint,
                &transfer,
                &identities,
                &proof_identity,
                Default::default(),
            );

            // Act
            subject.emit(build_emit_params(Default::default()), emit_payload)
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
                        forms: PhantomData::<F>,
                    },
                    Default::default(),
                ),
                build_create_delivery_proof_params(Default::default()),
                build_create_delivery_proof_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the delivery-proof factory admits the selection")
            };
            success.output
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        overrides: &'a EvmGenerateConstructorParamsOverrides,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = EmitReturn;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = EmitReturn;

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
                        overrides: self.overrides,
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
        overrides: &'a EvmGenerateConstructorParamsOverrides,
    }

    impl<'a> IPairingConsumer for PairingConsumer<'a> {
        type Output = EmitReturn;

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
                    overrides: self.overrides,
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
        overrides: &overrides,
    };
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
        build_create_pairing_payload(Default::default()),
    );
    let Ok(success) = created else {
        panic!("the pairing factory ran the consumer")
    };
    let result = success.output;

    // Assert
    assert!(matches!(
        result,
        Err(EmitErrorReturn::Evm(
            EvmGenerateErrorReturn::MissingKindSettings {
                kind: VectorKind::Possession
            }
        ))
    ));
    assert!(!Path::exists(&root));
    // emit_admits_identities_as_given_under_the_entitlement_scope
}
