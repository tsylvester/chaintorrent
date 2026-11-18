#![allow(clippy::expect_used)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter, IEncodingContract};
use envelope::{KeyAgreementDeclarationOverrides, build_key_agreement_declaration};
use kem::{KemDeclarationOverrides, build_kem_declaration};
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{DeliveryProofDeclarationOverrides, build_delivery_proof_declaration};

use super::super::render::provides::{
    SolidityConstantNameConstructorParamsOverrides, SolidityConstantValue, SolidityLibraryEntry,
    SolidityLibraryEntryOverrides, SolidityMemberNameConstructorParamsOverrides,
    SolidityStringLiteralConstructorParamsOverrides, build_solidity_constant_name,
    build_solidity_library_entry, build_solidity_member_name, build_solidity_string_literal,
};
use super::constants;
use super::provides::{
    CellValuesDescription, CellValuesDescriptionConstructorParamsOverrides, CellValuesOverrides,
    ConstantsCasePayload, ConstantsErrorReturn, ConstantsParamsOverrides, ConstantsReturn,
    IConstantsCaseConsumer, build_cell_values, build_cell_values_description,
    build_constants_case_deps, build_constants_case_params, build_constants_case_payload,
    build_constants_deps, build_constants_params, build_constants_payload, run_constants_case,
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
///   `KEY_IDENTIFIER_DOMAIN_TAG`, `INTENT_DOMAIN_NAME`, `INTENT_DOMAIN_VERSION`,
///   `INTENT_STRUCT_NAME`, `INTENT_MEMBER_IDENTITY`, `INTENT_MEMBER_NONCE`,
///   `INTENT_MEMBER_EXPIRY`, `INTENT_MEMBER_CALL`, `NONCE_KIND_TYPE_NAME`,
///   `EXPIRY_KIND_TYPE_NAME`, `FIXED_BYTES32_TYPE_NAME`, `UNSIGNED16_TYPE_NAME`,
///   `UNSIGNED32_TYPE_NAME`, `UNSIGNED64_TYPE_NAME`, `TEXT_TYPE_NAME`,
///   `BYTES_TYPE_NAME`, `FIXED_BYTES20_TYPE_NAME`, and `UNSIGNED256_TYPE_NAME`.
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
        "KEY_IDENTIFIER_DOMAIN_TAG",
        "INTENT_DOMAIN_NAME",
        "INTENT_DOMAIN_VERSION",
        "INTENT_STRUCT_NAME",
        "INTENT_MEMBER_IDENTITY",
        "INTENT_MEMBER_NONCE",
        "INTENT_MEMBER_EXPIRY",
        "INTENT_MEMBER_CALL",
        "NONCE_KIND_TYPE_NAME",
        "EXPIRY_KIND_TYPE_NAME",
        "FIXED_BYTES32_TYPE_NAME",
        "UNSIGNED16_TYPE_NAME",
        "UNSIGNED32_TYPE_NAME",
        "UNSIGNED64_TYPE_NAME",
        "TEXT_TYPE_NAME",
        "BYTES_TYPE_NAME",
        "FIXED_BYTES20_TYPE_NAME",
        "UNSIGNED256_TYPE_NAME",
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
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside
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
///   for the interval, the kinds the chain family's mock forms carry and the EVM
///   forms carry alike, so the mint and transfer sequences differ from each other
///   and no name is one the ABI concrete returns.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: the `MINT_FIELDS` and `TRANSFER_FIELDS` entries each equal
///   `build_solidity_library_entry` over their name and `SolidityConstantValue::String`
///   of `build_solidity_string_literal` over the literal text the block assembles
///   for it from the field kinds the descriptions' interaction specs list and the
///   mock encoder's declared names above, by `assert_eq!`.
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
            [
                "mockFixedBytes32",
                "mockUnsigned16",
                "mockUnsigned256",
                "mockFixedBytes20",
                "mockFixedBytes32",
                "mockFixedBytes32",
                "mockUnsigned256",
                "mockUnsigned256",
                "mockUnsigned64",
                "mockUnsigned64",
                "mockUnsigned16",
                "mockFixedBytes20",
                "mockFixedBytes20",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockUnsigned64",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
            ]
            .join(","),
        ),
        (
            "TRANSFER_FIELDS",
            [
                "mockFixedBytes32",
                "mockUnsigned16",
                "mockUnsigned256",
                "mockFixedBytes20",
                "mockFixedBytes32",
                "mockFixedBytes32",
                "mockUnsigned256",
                "mockUnsigned256",
                "mockUnsigned64",
                "mockUnsigned64",
                "mockUnsigned16",
                "mockFixedBytes20",
                "mockFixedBytes20",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockUnsigned64",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
                "mockBytes",
            ]
            .join(","),
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
                            text: Some(sequence),
                        }
                    )
                )),
            })
        );
    }
}
// constants_carry_the_mint_and_transfer_field_sequences

/// Contract: the pairing's `scalar_field_order` → `cell_values.scalar_field_order`
///   holds the order's 32 big-endian bytes.
/// Arrange: every collaborator at its defaults, the pairing the family's mock
///   concrete, whose order is `MOCK_GROUP_ORDER`, `65521`, as 32 big-endian bytes:
///   thirty bytes of `0x00`, then `0xFF`, `0xF1`; the order's nonzero low bytes
///   separate a constant, a truncated, and a little-endian order from the return.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: `cell_values.scalar_field_order` equals the 32 bytes above, the record
///   compared with `build_cell_values` overriding that member to the literal, by
///   `assert_eq!`.
#[test]
fn the_record_carries_the_scalar_field_order_as_32_big_endian_bytes() {
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
        result.cell_values,
        build_cell_values(CellValuesOverrides {
            scalar_field_order: Some(big_endian),
            ..Default::default()
        })
    );
}
// the_record_carries_the_scalar_field_order_as_32_big_endian_bytes

/// Contract: the pairing's generators, encoded by `encode_g1` and `encode_g2` →
///   `cell_values.g1_generator` and `cell_values.g2_generator` hold the encodings'
///   bytes.
/// Arrange: every collaborator at its defaults, the pairing the family's mock
///   concrete, whose first-group generator is residue `1` and second-group
///   generator residue `2`, each encoded as eight big-endian bytes; the encodings
///   differ, so a member holding the other group's generator fails.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: `cell_values.g1_generator` equals seven bytes of `0x00` then `0x01`, and
///   `cell_values.g2_generator` equals seven bytes of `0x00` then `0x02`, the record
///   compared with `build_cell_values` overriding those members to the literals,
///   by `assert_eq!`.
#[test]
fn the_record_carries_the_generators_precompile_encodings() {
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
    assert_eq!(
        result.cell_values,
        build_cell_values(CellValuesOverrides {
            g1_generator: Some(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]),
            g2_generator: Some(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]),
            ..Default::default()
        })
    );
}
// the_record_carries_the_generators_precompile_encodings

/// Contract: the received pairing's declaration → `cell_values.curve_name` is
///   `"Mock"`, `cell_values.precompile_encoding` is `"Mock"`, and
///   `cell_values.second_group_arithmetic` is `false`, each a literal written in
///   the test.
/// Arrange: every collaborator at its defaults, the pairing the family's mock
///   concrete, whose declaration declares the curve `PairingCurve::Mock` and the
///   precompile encoding `Mock` under the verifier form `FirstGroupOnly`.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside the
///   consumer's method.
/// Assert: `assert_eq!` of `cell_values.curve_name` with `"Mock"`, of
///   `cell_values.precompile_encoding` with `"Mock"`, and of
///   `cell_values.second_group_arithmetic` with `false`, each member compared
///   with `build_cell_values` overriding it to the literal.
#[test]
fn the_record_carries_the_pairings_declared_names_and_verifier_form() {
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
    assert_eq!(
        result.cell_values,
        build_cell_values(CellValuesOverrides {
            curve_name: Some("Mock".to_string()),
            precompile_encoding: Some("Mock".to_string()),
            second_group_arithmetic: Some(false),
            ..Default::default()
        })
    );
}
// the_record_carries_the_pairings_declared_names_and_verifier_form

/// Contract: each name in `CELL_VALUES_MEMBER_NAMES` that `SolidityMemberName::try_new`
///   admits → `Ok` of the names in order, and their count equals the length of
///   `CellValuesDescription::FIELDS`.
/// Arrange: `build_cell_values_description` over
///   `CellValuesDescriptionConstructorParamsOverrides`.
/// Act:    `member_names()`.
/// Assert: the result equals `Ok` of the names `curveName`, `precompileEncoding`,
///   `scalarFieldOrder`, `g1Generator`, `g2Generator`, and `secondGroupArithmetic`,
///   each through `build_solidity_member_name`, and its length equals
///   `CellValuesDescription::FIELDS.len()`.
#[test]
fn cell_values_description_declares_the_records_member_names() {
    // Arrange
    let description =
        build_cell_values_description(CellValuesDescriptionConstructorParamsOverrides);

    // Act
    let result = description.member_names();

    // Assert
    assert_eq!(
        result.as_ref().map(Vec::len),
        Ok(CellValuesDescription::FIELDS.len())
    );
    assert_eq!(
        result,
        Ok(vec![
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("curveName".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("precompileEncoding".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("scalarFieldOrder".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("g1Generator".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("g2Generator".to_string()),
            }),
            build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
                text: Some("secondGroupArithmetic".to_string()),
            }),
        ])
    );
}
// cell_values_description_declares_the_records_member_names

/// Contract: each signed-intent value is read from the handed forms' `DECLARATION`
///   → `KEY_IDENTIFIER_DOMAIN_TAG` holds `SolidityConstantValue::Bytes` of the
///   declaration's `key_identifier_domain_tag`, and `INTENT_DOMAIN_NAME`,
///   `INTENT_DOMAIN_VERSION`, `INTENT_STRUCT_NAME`, `INTENT_MEMBER_IDENTITY`,
///   `INTENT_MEMBER_NONCE`, `INTENT_MEMBER_EXPIRY`, and `INTENT_MEMBER_CALL`
///   hold `SolidityConstantValue::String` of the declaration's
///   `intent_domain_name`, `intent_domain_version`, `intent_struct_name`, and
///   `intent_member_names`' `identity`, `nonce`, `expiry`, and `call`.
/// Arrange: every collaborator at its defaults, the chain forms the family's
///   mock concrete, whose declaration carries
///   `b"chaintorrent/evm/key-identifier/v1"`, `"ChainTorrent"`, `"1"`,
///   `"Intent"`, and the member names `identity`, `nonce`, `expiry`, and `call`,
///   each differing from the domain tags, the purpose names, and every type
///   name, so an entry read from another field or a literal fails.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside
///   the consumer's method.
/// Assert: for each name the contract lists, the entry read by its name equals
///   `build_solidity_library_entry` over that name and
///   `SolidityConstantValue::Bytes` of `F::DECLARATION.key_identifier_domain_tag`
///   or `SolidityConstantValue::String` of `build_solidity_string_literal` over
///   the field of the same name of `F::DECLARATION`, by `assert_eq!`; the asserts
///   run inside the consumer's method, where `F::DECLARATION` is in scope.
#[test]
fn constants_carry_the_signed_intent_values_the_forms_declare() {
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
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            // Assert — inside the method, where `F::DECLARATION` is in scope
            let entries = &result.as_ref().expect("a supported version admits").entries;
            assert_eq!(
                entry_named(entries, "KEY_IDENTIFIER_DOMAIN_TAG"),
                &build_solidity_library_entry(SolidityLibraryEntryOverrides {
                    name: Some(build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some("KEY_IDENTIFIER_DOMAIN_TAG".to_string()),
                        }
                    )),
                    value: Some(SolidityConstantValue::Bytes(
                        F::DECLARATION.key_identifier_domain_tag.to_vec(),
                    )),
                })
            );
            for (text, field) in [
                ("INTENT_DOMAIN_NAME", F::DECLARATION.intent_domain_name),
                (
                    "INTENT_DOMAIN_VERSION",
                    F::DECLARATION.intent_domain_version,
                ),
                ("INTENT_STRUCT_NAME", F::DECLARATION.intent_struct_name),
                (
                    "INTENT_MEMBER_IDENTITY",
                    F::DECLARATION.intent_member_names.identity,
                ),
                (
                    "INTENT_MEMBER_NONCE",
                    F::DECLARATION.intent_member_names.nonce,
                ),
                (
                    "INTENT_MEMBER_EXPIRY",
                    F::DECLARATION.intent_member_names.expiry,
                ),
                (
                    "INTENT_MEMBER_CALL",
                    F::DECLARATION.intent_member_names.call,
                ),
            ] {
                assert_eq!(
                    entry_named(entries, text),
                    &build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some(text.to_string()),
                            }
                        )),
                        value: Some(SolidityConstantValue::String(
                            build_solidity_string_literal(
                                SolidityStringLiteralConstructorParamsOverrides {
                                    text: Some(field.to_string()),
                                }
                            )
                        )),
                    })
                );
            }
            result
        }
    }

    // Act
    let created = run_constants_case(
        &build_constants_case_deps(Consumer, Default::default()),
        build_constants_case_params(Default::default()),
        build_constants_case_payload(Default::default()),
    );

    // Assert
    created
        .expect("every factory ran")
        .output
        .expect("a supported version admits");
}
// constants_carry_the_signed_intent_values_the_forms_declare

/// Contract: each kind's type name is the encoder's → `NONCE_KIND_TYPE_NAME` and
///   `EXPIRY_KIND_TYPE_NAME` hold `SolidityConstantValue::String` of the
///   encoder's type name for `F::DECLARATION.nonce_kind` and
///   `F::DECLARATION.expiry_kind`, and `FIXED_BYTES32_TYPE_NAME`,
///   `UNSIGNED16_TYPE_NAME`, `UNSIGNED32_TYPE_NAME`, `UNSIGNED64_TYPE_NAME`,
///   `TEXT_TYPE_NAME`, `BYTES_TYPE_NAME`, `FIXED_BYTES20_TYPE_NAME`, and
///   `UNSIGNED256_TYPE_NAME` hold `String` of the encoder's type name for each
///   `CanonicalFieldKind` variant in declaration order.
/// Arrange: every collaborator at its defaults, the encoder the family's mock
///   concrete, whose declared names are `mockFixedBytes32`, `mockUnsigned16`,
///   `mockUnsigned32`, `mockUnsigned64`, `mockText`, `mockBytes`,
///   `mockFixedBytes20`, and `mockUnsigned256`, and the chain forms the family's
///   mock concrete, whose `nonce_kind` and `expiry_kind` are `Unsigned64`; the
///   mock names differ from every tag, member name, and ABI name, so a literal
///   or another kind's name fails.
/// Act:    `constants::<P, E, F>` over the deps, params, and payload, inside
///   the consumer's method.
/// Assert: for each name the contract lists, the entry read by its name equals
///   `build_solidity_library_entry` over that name and
///   `SolidityConstantValue::String` of `build_solidity_string_literal` over the
///   mock encoder's name for the kind the contract states, by `assert_eq!`.
#[test]
fn constants_carry_the_nonce_expiry_and_each_kinds_type_names() {
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
    for (text, type_name) in [
        ("NONCE_KIND_TYPE_NAME", "mockUnsigned64"),
        ("EXPIRY_KIND_TYPE_NAME", "mockUnsigned64"),
        ("FIXED_BYTES32_TYPE_NAME", "mockFixedBytes32"),
        ("UNSIGNED16_TYPE_NAME", "mockUnsigned16"),
        ("UNSIGNED32_TYPE_NAME", "mockUnsigned32"),
        ("UNSIGNED64_TYPE_NAME", "mockUnsigned64"),
        ("TEXT_TYPE_NAME", "mockText"),
        ("BYTES_TYPE_NAME", "mockBytes"),
        ("FIXED_BYTES20_TYPE_NAME", "mockFixedBytes20"),
        ("UNSIGNED256_TYPE_NAME", "mockUnsigned256"),
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
                            text: Some(type_name.to_string()),
                        }
                    )
                )),
            })
        );
    }
}
// constants_carry_the_nonce_expiry_and_each_kinds_type_names
