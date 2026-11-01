#![allow(clippy::expect_used)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{KeyAgreementDeclarationOverrides, build_key_agreement_declaration};
use kem::{KemDeclarationOverrides, build_kem_declaration};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryProofDeclarationOverrides, build_delivery_proof_declaration};

use super::super::render::provides::{
    SolidityConstantNameConstructorParamsOverrides, SolidityConstantValue, SolidityLibraryEntry,
    SolidityLibraryEntryOverrides, SolidityStringLiteralConstructorParamsOverrides,
    SolidityUint256ConstructorParamsOverrides, build_solidity_constant_name,
    build_solidity_library_entry, build_solidity_string_literal, build_solidity_uint256,
};
use super::constants;
use super::provides::{
    ConstantsCasePayload, ConstantsErrorReturn, ConstantsParamsOverrides, ConstantsReturn,
    IConstantsCaseConsumer, build_constants_case_deps, build_constants_case_params,
    build_constants_case_payload, build_constants_deps, build_constants_params,
    build_constants_payload, run_constants_case,
};

fn entry_named<'a>(entries: &'a [SolidityLibraryEntry], text: &str) -> &'a SolidityLibraryEntry {
    entries
        .iter()
        .find(|entry| {
            entry.name
                == build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                })
        })
        .expect("an entry under the name exists")
}

/// Contract: given `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE`,
///   `constants` returns `Err(ConstantsErrorReturn::UnsupportedStatementVersion
///   { version: params.statement_version })`.
/// Arrange: `build_constants_params` overriding `statement_version` with the literal
///   `0`, the one variation from the default `DELIVERY_STATEMENT_VERSION_ONE`; the
///   pairing and the chain forms are the family mock concretes through the runner.
/// Act:    `run_constants_case` runs the case runner, whose case consumer's single
///   call is `constants::<P, E, F>` over the deps, params, and payload.
/// Assert: `assert_eq!` of the return with
///   `Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: 0 })`.
#[test]
fn constants_refuse_a_statement_version_no_description_serves() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ConstantsReturn;

        fn consume_constants_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            encoder: &E,
            _forms: &F,
            _payload: &ConstantsCasePayload,
        ) -> Self::Output {
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(ConstantsParamsOverrides {
                    statement_version: Some(0),
                }),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    let result = created.expect("every factory ran").output;
    assert_eq!(
        result,
        Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: 0 })
    );
}
// constants_refuse_a_statement_version_no_description_serves

/// Contract: a supported statement version → `Ok` whose entries are named, in order,
///   `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`,
///   `WEIGHT_TAG`, `DELIVERY_STATEMENT_VERSION`, `PURPOSE_MINT`, `PURPOSE_TRANSFER`,
///   `PURPOSE_GRANT`, `PURPOSE_REPLACEMENT`, `MINT_FIELDS`, `TRANSFER_FIELDS`,
///   `SCALAR_FIELD_ORDER`, `G1_GENERATOR`, `G2_GENERATOR`, and `SECOND_GROUP_ARITHMETIC`.
/// Arrange: every collaborator at its defaults, `build_constants_params` at its
///   default `DELIVERY_STATEMENT_VERSION_ONE`; the entry names are distinct, so a
///   reordered, dropped, or renamed entry fails.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: `assert_eq!` of the names of the success's `entries`, in order, with the
///   names the contract lists, each built by `build_solidity_constant_name` over its
///   text as a literal.
#[test]
fn constants_return_the_entries_in_declared_order() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ConstantsReturn;

        fn consume_constants_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            encoder: &E,
            _forms: &F,
            _payload: &ConstantsCasePayload,
        ) -> Self::Output {
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("every factory ran")
        .output
        .expect("a supported version admits");
    let names: Vec<_> = result
        .entries
        .iter()
        .map(|entry| entry.name.clone())
        .collect();
    let expected: Vec<_> = [
        "IDENTITY_TAG",
        "POSSESSION_G1_TAG",
        "POSSESSION_G2_TAG",
        "CHALLENGE_TAG",
        "WEIGHT_TAG",
        "DELIVERY_STATEMENT_VERSION",
        "PURPOSE_MINT",
        "PURPOSE_TRANSFER",
        "PURPOSE_GRANT",
        "PURPOSE_REPLACEMENT",
        "MINT_FIELDS",
        "TRANSFER_FIELDS",
        "SCALAR_FIELD_ORDER",
        "G1_GENERATOR",
        "G2_GENERATOR",
        "SECOND_GROUP_ARITHMETIC",
    ]
    .iter()
    .map(|text| {
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some(text.to_string()),
        })
    })
    .collect();
    assert_eq!(names, expected);
}
// constants_return_the_entries_in_declared_order

/// Contract: each declared tag is read from its declaration → the entries
///   `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, and
///   `WEIGHT_TAG` hold `SolidityConstantValue::Bytes` of the KEM declaration's
///   `identity_tag`, the key agreement's `possession_g1_tag` and `possession_g2_tag`,
///   and the delivery proof's `challenge_tag` and `weight_tag`.
/// Arrange: `build_kem_declaration`, `build_key_agreement_declaration`, and
///   `build_delivery_proof_declaration` each overriding its tags with literals that
///   differ from each other and from every builder default, so an entry read from
///   another field, from a default, or under another name fails.
/// Act:    `constants::<P, E, F>` over the deps, params, and that payload, inside
///   the consumer's method.
/// Assert: for each name the contract lists, the entry read by its name equals
///   `build_solidity_library_entry` over that name and `SolidityConstantValue::Bytes`
///   of the literal arranged for it, by `assert_eq!`.
#[test]
fn constants_carry_each_declarations_domain_tag_under_its_own_name() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ConstantsReturn;

        fn consume_constants_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            encoder: &E,
            _forms: &F,
            _payload: &ConstantsCasePayload,
        ) -> Self::Output {
            let kem = build_kem_declaration(KemDeclarationOverrides {
                identity_tag: Some(b"tag/identity"),
                ..Default::default()
            });
            let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
                possession_g1_tag: Some(b"tag/possession-g1"),
                possession_g2_tag: Some(b"tag/possession-g2"),
                ..Default::default()
            });
            let delivery_proof =
                build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
                    challenge_tag: Some(b"tag/challenge"),
                    weight_tag: Some(b"tag/weight"),
                    ..Default::default()
                });
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("every factory ran")
        .output
        .expect("a supported version admits");
    for (text, tag) in [
        ("IDENTITY_TAG", &b"tag/identity"[..]),
        ("POSSESSION_G1_TAG", &b"tag/possession-g1"[..]),
        ("POSSESSION_G2_TAG", &b"tag/possession-g2"[..]),
        ("CHALLENGE_TAG", &b"tag/challenge"[..]),
        ("WEIGHT_TAG", &b"tag/weight"[..]),
    ] {
        assert_eq!(
            entry_named(&result.entries, text),
            &build_solidity_library_entry(SolidityLibraryEntryOverrides {
                name: Some(build_solidity_constant_name(
                    SolidityConstantNameConstructorParamsOverrides {
                        text: Some(text.to_string()),
                    }
                )),
                value: Some(SolidityConstantValue::Bytes(tag.to_vec())),
            })
        );
    }
}
// constants_carry_each_declarations_domain_tag_under_its_own_name

/// Contract: the statement version the params carry and each purpose's code →
///   `DELIVERY_STATEMENT_VERSION` holds `SolidityConstantValue::Uint16` of the
///   version, and `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and
///   `PURPOSE_REPLACEMENT` hold `Uint16` of the codes `DeliveryPurpose::Mint`,
///   `Transfer`, `Grant`, and `Replacement` declare.
/// Arrange: every collaborator at its defaults, the params at
///   `DELIVERY_STATEMENT_VERSION_ONE`; the purposes' codes differ from each other,
///   so swapped codes fail.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: each entry the contract lists equals `build_solidity_library_entry` over
///   their name and `Uint16` of the literal `proof/mint_statement` states for it,
///   by `assert_eq!`.
#[test]
fn constants_carry_the_statement_version_and_each_purpose_code() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ConstantsReturn;

        fn consume_constants_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            encoder: &E,
            _forms: &F,
            _payload: &ConstantsCasePayload,
        ) -> Self::Output {
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("every factory ran")
        .output
        .expect("a supported version admits");
    for (text, code) in [
        ("DELIVERY_STATEMENT_VERSION", 1u16),
        ("PURPOSE_MINT", 1u16),
        ("PURPOSE_TRANSFER", 2u16),
        ("PURPOSE_GRANT", 3u16),
        ("PURPOSE_REPLACEMENT", 4u16),
    ] {
        assert_eq!(
            entry_named(&result.entries, text),
            &build_solidity_library_entry(SolidityLibraryEntryOverrides {
                name: Some(build_solidity_constant_name(
                    SolidityConstantNameConstructorParamsOverrides {
                        text: Some(text.to_string()),
                    }
                )),
                value: Some(SolidityConstantValue::Uint16(code)),
            })
        );
    }
}
// constants_carry_the_statement_version_and_each_purpose_code

/// Contract: the type names the encoder returns for the kinds of
///   `MintStatementDescription::FIELDS` and `TransferStatementDescription::FIELDS`
///   joined by `,` in field order → `MINT_FIELDS` and `TRANSFER_FIELDS` hold
///   `SolidityConstantValue::String` of the joined text.
/// Arrange: every collaborator at its defaults, the encoder the family's mock
///   concrete, whose declared names are `mockFixedBytes32`, `mockUnsigned16`,
///   `mockUnsigned32`, `mockUnsigned64`, `mockText`, `mockBytes`, `mockFixedBytes20`,
///   and `mockUnsigned256`; the chain forms' kinds, `FixedBytes20` for the identity,
///   `Unsigned256` for the entitlement and the chain identifier, and `Unsigned64`
///   for the interval, so the mint and transfer sequences differ from each other
///   and no name is one the ABI concrete returns.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: the `MINT_FIELDS` and `TRANSFER_FIELDS` entries each equal
///   `build_solidity_library_entry` over their name and
///   `SolidityConstantValue::String` of `build_solidity_string_literal` over the
///   literal text the block assembles for it from the field kinds the descriptions'
///   interaction specs list and the mock encoder's declared names above, by
///   `assert_eq!`.
#[test]
fn constants_carry_the_mint_and_transfer_field_sequences() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ConstantsReturn;

        fn consume_constants_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            encoder: &E,
            _forms: &F,
            _payload: &ConstantsCasePayload,
        ) -> Self::Output {
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("every factory ran")
        .output
        .expect("a supported version admits");
    for (text, sequence) in [
        (
            "MINT_FIELDS",
            "mockFixedBytes32,mockUnsigned16,mockUnsigned256,mockFixedBytes20,mockFixedBytes32,mockFixedBytes32,mockUnsigned256,mockUnsigned256,mockUnsigned64,mockUnsigned64,mockUnsigned16,mockFixedBytes20,mockFixedBytes20,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockUnsigned64,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes",
        ),
        (
            "TRANSFER_FIELDS",
            "mockFixedBytes32,mockUnsigned16,mockUnsigned256,mockFixedBytes20,mockFixedBytes32,mockFixedBytes32,mockUnsigned256,mockUnsigned256,mockUnsigned64,mockUnsigned64,mockUnsigned16,mockFixedBytes20,mockFixedBytes20,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockUnsigned64,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes,mockBytes",
        ),
    ] {
        assert_eq!(
            entry_named(&result.entries, text),
            &build_solidity_library_entry(SolidityLibraryEntryOverrides {
                name: Some(build_solidity_constant_name(
                    SolidityConstantNameConstructorParamsOverrides {
                        text: Some(text.to_string()),
                    }
                )),
                value: Some(SolidityConstantValue::String(
                    build_solidity_string_literal(
                        SolidityStringLiteralConstructorParamsOverrides {
                            text: Some(sequence.to_string()),
                        }
                    )
                )),
            })
        );
    }
}
// constants_carry_the_mint_and_transfer_field_sequences

/// Contract: the pairing's `scalar_field_order` → `SCALAR_FIELD_ORDER` holds
///   `SolidityConstantValue::Uint256` of the order's 32 big-endian bytes.
/// Arrange: every collaborator at its defaults, the pairing the family's mock
///   concrete, whose order is `MOCK_GROUP_ORDER`, `65521`, as 32 big-endian bytes:
///   thirty bytes of `0x00`, then `0xFF`, `0xF1`; the order's nonzero low bytes
///   separate a constant, a truncated, and a little-endian order from the return.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: the `SCALAR_FIELD_ORDER` entry equals `build_solidity_library_entry` over
///   its name and `SolidityConstantValue::Uint256` of `build_solidity_uint256` over
///   `big_endian` the 32 bytes above, by `assert_eq!`.
#[test]
fn constants_carry_the_scalar_field_order_as_32_big_endian_bytes() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ConstantsReturn;

        fn consume_constants_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            encoder: &E,
            _forms: &F,
            _payload: &ConstantsCasePayload,
        ) -> Self::Output {
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("every factory ran")
        .output
        .expect("a supported version admits");
    let mut big_endian = [0x00u8; 32];
    big_endian[30] = 0xFF;
    big_endian[31] = 0xF1;
    assert_eq!(
        entry_named(&result.entries, "SCALAR_FIELD_ORDER"),
        &build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some("SCALAR_FIELD_ORDER".to_string()),
                }
            )),
            value: Some(SolidityConstantValue::Uint256(build_solidity_uint256(
                SolidityUint256ConstructorParamsOverrides {
                    big_endian: Some(big_endian),
                }
            ))),
        })
    );
}
// constants_carry_the_scalar_field_order_as_32_big_endian_bytes

/// Contract: the pairing's generators, encoded by `encode_g1` and `encode_g2` →
///   `G1_GENERATOR` and `G2_GENERATOR` hold `SolidityConstantValue::Bytes` of the
///   encodings' bytes.
/// Arrange: every collaborator at its defaults, the pairing the family's mock
///   concrete, whose first-group generator is residue `1` and second-group
///   generator residue `2`, each encoded as eight big-endian bytes; the encodings
///   differ, so an entry holding the other group's generator fails.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: `G1_GENERATOR` equals `build_solidity_library_entry` over its name and
///   `Bytes` of seven bytes of `0x00` then `0x01`, and `G2_GENERATOR` the same over
///   seven bytes of `0x00` then `0x02`, by `assert_eq!`.
#[test]
fn constants_carry_the_generators_encodings() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ConstantsReturn;

        fn consume_constants_case<
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        >(
            &self,
            pairing: &P,
            encoder: &E,
            _forms: &F,
            _payload: &ConstantsCasePayload,
        ) -> Self::Output {
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    let result = created
        .expect("every factory ran")
        .output
        .expect("a supported version admits");
    for (text, bytes) in [
        (
            "G1_GENERATOR",
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
        ),
        (
            "G2_GENERATOR",
            vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
        ),
    ] {
        assert_eq!(
            entry_named(&result.entries, text),
            &build_solidity_library_entry(SolidityLibraryEntryOverrides {
                name: Some(build_solidity_constant_name(
                    SolidityConstantNameConstructorParamsOverrides {
                        text: Some(text.to_string()),
                    }
                )),
                value: Some(SolidityConstantValue::Bytes(bytes)),
            })
        );
    }
}
// constants_carry_the_generators_encodings
