#![allow(clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter,
};
use envelope::{KeyAgreementDeclarationOverrides, build_key_agreement_declaration};
use kem::{KemDeclarationOverrides, build_kem_declaration};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, CreatePairingParamsOverrides,
    DecodeG1Params, DecodeG2Params, DecodeScalarErrorReturn, DecodeScalarParams, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, IsIdentityG1Params, IsIdentityG1Payload, IsIdentityG2Params,
    IsIdentityG2Payload, NegG1Params, NegG1Payload, NegG2Params, NegG2Payload, PAIRING_CONCRETES,
    PairingConcrete, PrecompileEncoding, TargetGroupEncodingIdentifier,
    build_create_pairing_params,
};
use proof::{DeliveryProofDeclarationOverrides, build_delivery_proof_declaration};

use super::super::render::provides::{
    SolidityConstantNameConstructorParamsOverrides, SolidityConstantValue, SolidityLibraryEntry,
    SolidityLibraryEntryOverrides, SolidityStringLiteralConstructorParamsOverrides,
    build_solidity_constant_name, build_solidity_library_entry, build_solidity_string_literal,
};
use super::constants;
use super::provides::{
    ConstantsCaseParams, ConstantsCaseParamsOverrides, ConstantsCasePayload,
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

fn case_params(
    pairing_concrete: PairingConcrete,
    chain_forms_concrete: ChainFormsConcrete,
) -> ConstantsCaseParams {
    let selection = &DECLARED_ENCODING_SELECTIONS[0];
    build_constants_case_params(ConstantsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        })),
        encoding: Some(CreateEncodingParams {
            concrete: selection.concrete,
            identifier: selection.identifier,
        }),
        chain_forms: Some(build_create_chain_forms_params(
            CreateChainFormsParamsOverrides {
                concrete: Some(chain_forms_concrete),
                ..Default::default()
            },
        )),
    })
}

/// Contract: given the KEM, key-agreement, and delivery-proof declarations carrying
///   the tags each family's real concrete declares, `constants` returns `Ok` whose
///   entries `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`,
///   `CHALLENGE_TAG`, and `WEIGHT_TAG` are `Bytes` holding the KEM declaration's
///   `identity_tag`, the key agreement's `possession_g1_tag` and
///   `possession_g2_tag`, and the delivery proof's `challenge_tag` and `weight_tag`,
///   each equal to the literal its owning node states.
/// Arrange: the declarations from `build_kem_declaration`,
///   `build_key_agreement_declaration`, and `build_delivery_proof_declaration`
///   overriding each tag with the literal its owning node, `kem/bb1_depth_one`,
///   `envelope/pairing_elgamal`, or `proof/schnorr_fs`, states for the real
///   concrete's tag, the tags differing from each other and from every builder
///   default; every concrete of both declared sets is the variation.
/// Act:    `run_constants_case` per member of each declared set, the consumer's
///   method calling `constants::<P, E, F>`.
/// Assert: the entries `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`,
///   `CHALLENGE_TAG`, and `WEIGHT_TAG` equal `SolidityLibraryEntry` values from
///   `build_solidity_library_entry` overriding `name` and `value` with `Bytes` of
///   the literal its owning node states for that tag.
/// Boundary: `constants` over the real `SolidityConstantName::try_new` and the real
///   pairing and forms concretes.
/// Mocked:  nothing, the declarations being builder data.
#[test]
fn the_entries_carry_each_declarations_domain_tag_under_its_own_name() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ();

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
            // Arrange
            let kem = build_kem_declaration(KemDeclarationOverrides {
                identity_tag: Some(b"ChainTorrent-v1-kem-identity"),
                ..Default::default()
            });
            let key_agreement = build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
                possession_g1_tag: Some(b"ChainTorrent-v1-envelope-possession-g1"),
                possession_g2_tag: Some(b"ChainTorrent-v1-envelope-possession-g2"),
                ..Default::default()
            });
            let delivery_proof =
                build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
                    challenge_tag: Some(b"ChainTorrent-v1-proof-challenge"),
                    weight_tag: Some(b"ChainTorrent-v1-proof-weight"),
                    ..Default::default()
                });

            // Act
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            // Assert
            let entries = result.expect("a supported version admits").entries;
            for (text, tag) in [
                ("IDENTITY_TAG", &b"ChainTorrent-v1-kem-identity"[..]),
                (
                    "POSSESSION_G1_TAG",
                    &b"ChainTorrent-v1-envelope-possession-g1"[..],
                ),
                (
                    "POSSESSION_G2_TAG",
                    &b"ChainTorrent-v1-envelope-possession-g2"[..],
                ),
                ("CHALLENGE_TAG", &b"ChainTorrent-v1-proof-challenge"[..]),
                ("WEIGHT_TAG", &b"ChainTorrent-v1-proof-weight"[..]),
            ] {
                assert_eq!(
                    entry_named(&entries, text),
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
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            run_constants_case(
                &build_constants_case_deps(Consumer, Default::default()),
                case_params(*pairing_concrete, *chain_forms_concrete),
                build_constants_case_payload(Default::default()),
            )
            .expect("every factory ran");
        }
    }
}
// the_entries_carry_each_declarations_domain_tag_under_its_own_name

/// Contract: given the params carry `DELIVERY_STATEMENT_VERSION_ONE`, `constants`
///   returns `Ok` whose entry `DELIVERY_STATEMENT_VERSION` is `Uint16` holding the
///   value of `DELIVERY_STATEMENT_VERSION_ONE` and whose entries `PURPOSE_MINT`,
///   `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and `PURPOSE_REPLACEMENT` are `Uint16`
///   holding the codes `DeliveryPurpose` declares for `Mint`, `Transfer`, `Grant`,
///   and `Replacement`, in `DELIVERY_PURPOSES` order.
/// Arrange: `build_constants_params` at its default, `DELIVERY_STATEMENT_VERSION_ONE`;
///   the declarations at their defaults; every concrete of both declared sets is
///   the variation, the codes differing from each other so that swapped codes or a
///   reordering fail.
/// Act:    `run_constants_case` per member of each declared set, the consumer's
///   method calling `constants::<P, E, F>`.
/// Assert: the entry `DELIVERY_STATEMENT_VERSION` equals `Uint16` of the literal
///   `proof/mint_statement` states for `DELIVERY_STATEMENT_VERSION_ONE`, and the
///   entries `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and
///   `PURPOSE_REPLACEMENT` equal `Uint16` of the literal codes `proof/mint_statement`
///   states for `Mint`, `Transfer`, `Grant`, and `Replacement`, their positions in
///   the returned entries following `DELIVERY_PURPOSES` order.
/// Boundary: `constants` over the real pairing and forms concretes.
/// Mocked:  nothing.
#[test]
fn the_entries_carry_the_statement_version_and_each_purpose_code_in_declared_order() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ();

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
            // Arrange
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());

            // Act
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            // Assert
            let entries = result.expect("a supported version admits").entries;
            for (index, (text, code)) in [
                ("DELIVERY_STATEMENT_VERSION", 1u16),
                ("PURPOSE_MINT", 1u16),
                ("PURPOSE_TRANSFER", 2u16),
                ("PURPOSE_GRANT", 3u16),
                ("PURPOSE_REPLACEMENT", 4u16),
            ]
            .iter()
            .enumerate()
            {
                assert_eq!(
                    &entries[5 + index],
                    &build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(build_solidity_constant_name(
                            SolidityConstantNameConstructorParamsOverrides {
                                text: Some(text.to_string()),
                            }
                        )),
                        value: Some(SolidityConstantValue::Uint16(*code)),
                    })
                );
            }
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            run_constants_case(
                &build_constants_case_deps(Consumer, Default::default()),
                case_params(*pairing_concrete, *chain_forms_concrete),
                build_constants_case_payload(Default::default()),
            )
            .expect("every factory ran");
        }
    }
}
// the_entries_carry_the_statement_version_and_each_purpose_code_in_declared_order

/// Contract: given the forms the chain family hands the consumer and the ABI
///   concrete the encoding family hands it, `constants` returns `Ok` whose entries
///   `MINT_FIELDS` and `TRANSFER_FIELDS` are `String` holding the type names the
///   ABI concrete returns for the kinds of `MintStatementDescription::FIELDS` and
///   `TransferStatementDescription::FIELDS` joined by `,` in field order, a fixed
///   twenty-byte kind as `bytes20`; this block proves the absorbed `KIND` outcomes
///   of the four `chain/create_chain_forms` round-trip entries the spec
///   dispositions.
/// Arrange: the defaults; every concrete of both declared sets is the variation,
///   each sequence compared with its own literal.
/// Act:    `run_constants_case` per member of each declared set, the consumer's
///   method calling `constants::<P, E, F>`.
/// Assert: the entry `MINT_FIELDS` equals `String` of `build_solidity_string_literal`
///   over the literal ABI type names the block assembles from the field kinds
///   `proof/mint_statement`'s interaction spec lists, the forms' kinds, and the
///   ABI table `encoding/abi` states, joined by `,`, and the entry
///   `TRANSFER_FIELDS` equals `String` of the literal assembled likewise from
///   `proof/transfer_statement`'s spec.
/// Boundary: `constants` over the real encoder's `type_name`,
///   `SolidityAbiType::try_new` and `as_str`, `SolidityStringLiteral::try_new`, and
///   the real forms concrete.
/// Mocked:  nothing.
#[test]
fn the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ();

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
            // Arrange
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());

            // Act
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            // Assert
            let entries = result.expect("a supported version admits").entries;
            for (text, sequence) in [
                (
                    "MINT_FIELDS",
                    [
                        "bytes32", "uint16", "uint256", "bytes20", "bytes32", "bytes32", "uint256",
                        "uint256", "uint64", "uint64", "uint16", "bytes20", "bytes20", "bytes",
                        "bytes", "bytes", "bytes", "bytes", "bytes", "uint64", "bytes", "bytes",
                        "bytes", "bytes", "bytes",
                    ]
                    .join(","),
                ),
                (
                    "TRANSFER_FIELDS",
                    [
                        "bytes32", "uint16", "uint256", "bytes20", "bytes32", "bytes32", "uint256",
                        "uint256", "uint64", "uint64", "uint16", "bytes20", "bytes20", "bytes",
                        "bytes", "bytes", "bytes", "bytes", "bytes", "bytes", "bytes", "bytes",
                        "bytes", "bytes", "bytes", "uint64", "bytes", "bytes", "bytes", "bytes",
                        "bytes", "bytes",
                    ]
                    .join(","),
                ),
            ] {
                assert_eq!(
                    entry_named(&entries, text),
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
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            run_constants_case(
                &build_constants_case_deps(Consumer, Default::default()),
                case_params(*pairing_concrete, *chain_forms_concrete),
                build_constants_case_payload(Default::default()),
            )
            .expect("every factory ran");
        }
    }
}
// the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms

/// Contract: given a pairing of `PAIRING_CONCRETES`, `constants` returns `Ok` whose
///   member `cell_values.scalar_field_order` holds 32 bytes the same pairing's
///   `decode_scalar` refuses with `NonCanonical` and whose bytes less one it
///   admits.
/// Arrange: the defaults; every concrete of `PAIRING_CONCRETES` is the variation,
///   so a constant, a truncated, a little-endian, or an off-by-one order fails.
/// Act:    `run_constants_case` per member of `PAIRING_CONCRETES`, the consumer's
///   method calling `constants::<P, E, F>`.
/// Assert: in place in the consumer's method, the received pairing's
///   `decode_scalar` over the member's bytes returns
///   `Err(DecodeScalarErrorReturn::NonCanonical)` whole, and over the member's
///   bytes less one returns `Ok`.
/// Boundary: `constants`, with the real `decode_scalar` as the oracle.
/// Mocked:  nothing.
#[test]
fn the_record_carries_the_scalar_field_order_as_32_big_endian_bytes() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ();

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
            // Arrange
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());

            // Act
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            // Assert
            let scalar_field_order = result
                .expect("a supported version admits")
                .cell_values
                .scalar_field_order;
            assert_eq!(
                pairing
                    .decode_scalar(DecodeScalarParams, &scalar_field_order)
                    .err(),
                Some(DecodeScalarErrorReturn::NonCanonical)
            );
            let mut less_one = scalar_field_order;
            less_one[31] -= 1;
            assert!(pairing.decode_scalar(DecodeScalarParams, &less_one).is_ok());
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            run_constants_case(
                &build_constants_case_deps(Consumer, Default::default()),
                case_params(*pairing_concrete, *chain_forms_concrete),
                build_constants_case_payload(Default::default()),
            )
            .expect("every factory ran");
        }
    }
}
// the_record_carries_the_scalar_field_order_as_32_big_endian_bytes

/// Contract: given a pairing of `PAIRING_CONCRETES`, `constants` returns `Ok` whose
///   members `cell_values.g1_generator` and `cell_values.g2_generator` are bytes
///   the same pairing's `decode_g1` and `decode_g2` admit, each decoded point's
///   sum with the negation of that group's generator being the identity.
/// Arrange: the defaults; every concrete of `PAIRING_CONCRETES` is the variation,
///   so a member holding the other group's generator or another point fails at
///   the decoder.
/// Act:    `run_constants_case` per member of `PAIRING_CONCRETES`, the consumer's
///   method calling `constants::<P, E, F>`.
/// Assert: in place in the consumer's method, the received pairing's `decode_g1`
///   and `decode_g2` over the members' bytes return `Ok`; the sum of each decoded
///   point with the negation of that group's generator satisfies `is_identity_g1`
///   or `is_identity_g2`.
/// Boundary: `constants` over the real pairing, with the decoders and group
///   operations as the oracle.
/// Mocked:  nothing.
#[test]
fn the_record_carries_the_generators_precompile_encodings() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ();

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
            // Arrange
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());

            // Act
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            // Assert
            let cell_values = result.expect("a supported version admits").cell_values;

            let decoded_g1 = pairing
                .decode_g1(DecodeG1Params, &cell_values.g1_generator)
                .expect("the g1_generator encoding decodes");
            let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(negation) = pairing.neg_g1(
                NegG1Params,
                NegG1Payload {
                    point: generator.point,
                },
            );
            let Ok(sum) = pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: decoded_g1.point,
                    right: negation.negation,
                },
            );
            let Ok(identity) =
                pairing.is_identity_g1(IsIdentityG1Params, IsIdentityG1Payload { point: sum.sum });
            assert!(identity.is_identity);

            let decoded_g2 = pairing
                .decode_g2(DecodeG2Params, &cell_values.g2_generator)
                .expect("the g2_generator encoding decodes");
            let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(negation) = pairing.neg_g2(
                NegG2Params,
                NegG2Payload {
                    point: generator.point,
                },
            );
            let Ok(sum) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: decoded_g2.point,
                    right: negation.negation,
                },
            );
            let Ok(identity) =
                pairing.is_identity_g2(IsIdentityG2Params, IsIdentityG2Payload { point: sum.sum });
            assert!(identity.is_identity);
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            run_constants_case(
                &build_constants_case_deps(Consumer, Default::default()),
                case_params(*pairing_concrete, *chain_forms_concrete),
                build_constants_case_payload(Default::default()),
            )
            .expect("every factory ran");
        }
    }
}
// the_record_carries_the_generators_precompile_encodings

/// Contract: given a pairing of `PAIRING_CONCRETES`, `constants` returns `Ok` whose
///   member `cell_values.second_group_arithmetic` is `false` for a pairing whose
///   declaration carries `PrecompileEncoding::Eip196Eip197` and `true` for
///   `PrecompileEncoding::Eip2537`.
/// Arrange: the defaults; every concrete of `PAIRING_CONCRETES` is the variation,
///   which holds a pairing of each precompile encoding, so a constant fails.
/// Act:    `run_constants_case` per member of `PAIRING_CONCRETES`, the consumer's
///   method calling `constants::<P, E, F>`.
/// Assert: in place in the consumer's method, the member
///   `cell_values.second_group_arithmetic` equals `false` where
///   `P::DECLARATION.precompile_encoding` is `Eip196Eip197` and `true` where it is
///   `Eip2537`, the expectation keyed on the encoding and not on
///   `verifier_group_arithmetic`.
/// Boundary: `constants` over the real pairing and forms concretes.
/// Mocked:  nothing.
#[test]
fn the_record_carries_the_verifier_form_the_pairing_declares() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = ();

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
            // Arrange
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());

            // Act
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            // Assert
            let second_group_arithmetic = result
                .expect("a supported version admits")
                .cell_values
                .second_group_arithmetic;
            let expected = match P::DECLARATION.precompile_encoding {
                PrecompileEncoding::Eip196Eip197 => false,
                PrecompileEncoding::Eip2537 => true,
                PrecompileEncoding::Mock => panic!("PAIRING_CONCRETES names no Mock pairing"),
            };
            assert_eq!(second_group_arithmetic, expected);
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            run_constants_case(
                &build_constants_case_deps(Consumer, Default::default()),
                case_params(*pairing_concrete, *chain_forms_concrete),
                build_constants_case_payload(Default::default()),
            )
            .expect("every factory ran");
        }
    }
}
// the_record_carries_the_verifier_form_the_pairing_declares

/// Contract: given a concrete of each declared set, `constants` returns `Ok` whose
///   entry names, in order, are `IDENTITY_TAG`, `POSSESSION_G1_TAG`,
///   `POSSESSION_G2_TAG`, `CHALLENGE_TAG`, `WEIGHT_TAG`, `DELIVERY_STATEMENT_VERSION`,
///   `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, `PURPOSE_REPLACEMENT`,
///   `MINT_FIELDS`, and `TRANSFER_FIELDS`.
/// Arrange: the defaults; every concrete of both declared sets is the variation.
/// Act:    `run_constants_case` per member of each declared set, the consumer's
///   method calling `constants::<P, E, F>` and returning the entries as its output.
/// Assert: the names of the returned entries, read in order, equal the names the
///   entry lists in the order it lists them, each built with
///   `build_solidity_constant_name`.
/// Boundary: `constants` over the real pairing and forms concretes.
/// Mocked:  nothing.
#[test]
fn the_entries_are_listed_in_declared_order() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = Vec<SolidityLibraryEntry>;

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
            // Arrange
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());

            // Act
            constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            )
            .expect("a supported version admits")
            .entries
        }
    }

    // Act — over the declared sets
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            let entries = run_constants_case(
                &build_constants_case_deps(Consumer, Default::default()),
                case_params(*pairing_concrete, *chain_forms_concrete),
                build_constants_case_payload(Default::default()),
            )
            .expect("every factory ran")
            .output;

            // Assert
            let names: Vec<_> = entries.iter().map(|entry| entry.name.clone()).collect();
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
    }
}
// the_entries_are_listed_in_declared_order

/// Contract: the absorbed entry
///   `libraries_sharing_an_identifier_return_the_same_scalar_field_order`: given
///   any two concretes whose declared target-group encoding identifiers are equal,
///   the `scalar_field_order` members `constants` returns are equal.
/// Arrange: the defaults; every concrete of `PAIRING_CONCRETES` is the variation,
///   whose declared set holds libraries that share an identifier and libraries
///   that do not, so a constant order or an order read from the wrong pairing
///   fails.
/// Act:    `run_constants_case` per member of `PAIRING_CONCRETES`, the consumer's
///   method calling `constants::<P, E, F>` and returning the pairing's
///   `P::DECLARATION.target_group_encoding` with the `cell_values.scalar_field_order`
///   member as its output.
/// Assert: any two outputs whose identifiers are equal hold equal members.
/// Boundary: `constants` over every real pairing concrete, the comparison made
///   across the runs.
/// Mocked:  nothing.
#[test]
fn libraries_sharing_an_identifier_carry_the_same_scalar_field_order() {
    struct Consumer;

    impl IConstantsCaseConsumer for Consumer {
        type Output = (TargetGroupEncodingIdentifier, [u8; 32]);

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
            // Arrange
            let kem = build_kem_declaration(Default::default());
            let key_agreement = build_key_agreement_declaration(Default::default());
            let delivery_proof = build_delivery_proof_declaration(Default::default());

            // Act
            let result = constants::<P, E, F>(
                &build_constants_deps(pairing, encoder, Default::default()),
                build_constants_params(Default::default()),
                build_constants_payload(&kem, &key_agreement, &delivery_proof, Default::default()),
            );

            (
                P::DECLARATION.target_group_encoding,
                result
                    .expect("a supported version admits")
                    .cell_values
                    .scalar_field_order,
            )
        }
    }

    // Act — over the declared sets
    let mut outputs = Vec::new();
    for pairing_concrete in PAIRING_CONCRETES {
        for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
            outputs.push(
                run_constants_case(
                    &build_constants_case_deps(Consumer, Default::default()),
                    case_params(*pairing_concrete, *chain_forms_concrete),
                    build_constants_case_payload(Default::default()),
                )
                .expect("every factory ran")
                .output,
            );
        }
    }

    // Assert
    for (i, (identifier_a, member_a)) in outputs.iter().enumerate() {
        for (identifier_b, member_b) in outputs.iter().skip(i + 1) {
            if identifier_a == identifier_b {
                assert_eq!(member_a, member_b);
            }
        }
    }
}
// libraries_sharing_an_identifier_carry_the_same_scalar_field_order
