#![allow(clippy::expect_used)]

use super::super::render::provides::{
    SolidityConstantName, SolidityConstantNameConstructorParamsOverrides, SolidityConstantValue,
    SolidityLibraryEntry, SolidityLibraryEntryOverrides,
    SolidityStringLiteralConstructorParamsOverrides, SolidityUint256ConstructorParamsOverrides,
    build_solidity_constant_name, build_solidity_library_entry, build_solidity_string_literal,
    build_solidity_uint256,
};
use super::constants;
use super::provides::{
    ConstantsReturn, build_constants_deps, build_constants_params, build_constants_payload,
};
use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use envelope::{KeyAgreementDeclarationOverrides, build_key_agreement_declaration};
use kem::{KemDeclarationOverrides, build_kem_declaration};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, ConsumePairingParams,
    ConsumePairingPayload, CreatePairingParamsOverrides, CreatePairingPayload, DecodeG1Params,
    DecodeG2Params, DecodeScalarErrorReturn, DecodeScalarParams, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, IsIdentityG1Params, IsIdentityG1Payload,
    IsIdentityG2Params, IsIdentityG2Payload, NegG1Params, NegG1Payload, NegG2Params, NegG2Payload,
    PAIRING_CONCRETES, PrecompileEncoding, ScalarFieldOrderParams, ScalarFieldOrderPayload,
    TargetGroupEncodingIdentifier, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use proof::{DeliveryProofDeclarationOverrides, build_delivery_proof_declaration};

/// Contract: entry `the_entries_carry_each_declarations_domain_tag_under_its_
///   own_name`; the KEM, key-agreement, and delivery-proof declarations carry
///   the tags each family's real concrete declares → `Ok` whose entries
///   `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`, `CHALLENGE_TAG`,
///   and `WEIGHT_TAG` are `Bytes` holding the KEM declaration's
///   `identity_tag`, the key agreement's `possession_g1_tag` and
///   `possession_g2_tag`, and the delivery proof's `challenge_tag` and
///   `weight_tag`, each equal to the literal its owning node states.
/// Arrange: the declarations from `build_kem_declaration`,
///   `build_key_agreement_declaration`, and `build_delivery_proof_declaration`
///   overriding each tag with the literal `kem/bb1_depth_one`,
///   `envelope/pairing_elgamal`, or `proof/schnorr_fs` states for the real
///   concrete's tag, the tags differing from each other and from every
///   builder default; every member of `PAIRING_CONCRETES` and of
///   `CHAIN_FORMS_CONCRETES` is the variation.
/// Act:     `constants::<P, F>` in the inner consumer.
/// Assert:  each of the five entries equals the `SolidityLibraryEntry`
///   `build_solidity_library_entry` builds over the entry's name and `Bytes`
///   of the tag's literal; every factory result `is_ok()`.
/// Boundary: `constants` over the real `SolidityConstantName::try_new` and the
///   real pairing and forms concretes.
/// Mocked:  nothing; the declarations are builder data.
#[test]
fn the_entries_carry_each_declarations_domain_tag_under_its_own_name() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = Vec<ConstantsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ConstantsReturn;

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(KemDeclarationOverrides {
                        identity_tag: Some(b"ChainTorrent-v1-kem-identity"),
                        ..Default::default()
                    });
                    let key_agreement =
                        build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
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
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );

                    // Act
                    constants::<P, F>(&deps, params, payload)
                }
            }

            let mut results = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
                results.push(
                    created
                        .expect("the chain-forms factory ran the consumer")
                        .output,
                );
            }
            results
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
        let results = created
            .expect("the pairing factory ran the consumer")
            .output;

        // Assert
        for result in results {
            let entries = result.expect("the declared version is served").entries;
            for (name, tag) in [
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
                let expected = build_solidity_library_entry(SolidityLibraryEntryOverrides {
                    name: Some(build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some(name.to_string()),
                        },
                    )),
                    value: Some(SolidityConstantValue::Bytes(tag.to_vec())),
                });
                let entry = entries
                    .iter()
                    .find(|entry| entry.name == expected.name)
                    .expect("the entry is present under its own name");
                assert_eq!(entry, &expected);
            }
        }
    }
    // the_entries_carry_each_declarations_domain_tag_under_its_own_name
}

/// Contract: entry
///   `the_entries_carry_the_statement_version_and_each_purpose_code_in_
///   declared_order`; the params carry `DELIVERY_STATEMENT_VERSION_ONE` →
///   `Ok` whose entry `DELIVERY_STATEMENT_VERSION` is `Uint16` holding the
///   value of `DELIVERY_STATEMENT_VERSION_ONE` and whose entries
///   `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and
///   `PURPOSE_REPLACEMENT` are `Uint16` holding the codes `DeliveryPurpose`
///   declares for `Mint`, `Transfer`, `Grant`, and `Replacement`, in
///   `DELIVERY_PURPOSES` order.
/// Arrange: `build_constants_params` at its default,
///   `DELIVERY_STATEMENT_VERSION_ONE`; the declarations at their defaults;
///   every member of `PAIRING_CONCRETES` and of `CHAIN_FORMS_CONCRETES` is
///   the variation, the codes differing from each other so that swapped
///   codes or a reordering fail.
/// Act:     `constants::<P, F>` in the inner consumer.
/// Assert:  the `DELIVERY_STATEMENT_VERSION` entry equals the built entry of
///   `Uint16(1)`, and `entries[6..10]` equals the built `PURPOSE_MINT`,
///   `PURPOSE_TRANSFER`, `PURPOSE_GRANT`, and `PURPOSE_REPLACEMENT` entries
///   of `Uint16(1)`, `Uint16(2)`, `Uint16(3)`, and `Uint16(4)` in order;
///   every factory result `is_ok()`.
/// Boundary: `constants` over the real pairing and forms concretes.
/// Mocked:  nothing.
#[test]
fn the_entries_carry_the_statement_version_and_each_purpose_code_in_declared_order() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = Vec<ConstantsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ConstantsReturn;

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(Default::default());
                    let key_agreement = build_key_agreement_declaration(Default::default());
                    let delivery_proof = build_delivery_proof_declaration(Default::default());
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );

                    // Act
                    constants::<P, F>(&deps, params, payload)
                }
            }

            let mut results = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
                results.push(
                    created
                        .expect("the chain-forms factory ran the consumer")
                        .output,
                );
            }
            results
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
        let results = created
            .expect("the pairing factory ran the consumer")
            .output;

        // Assert
        for result in results {
            let entries = result.expect("the declared version is served").entries;
            let entry = |name: &str| {
                build_solidity_library_entry(SolidityLibraryEntryOverrides {
                    name: Some(build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some(name.to_string()),
                        },
                    )),
                    value: None,
                })
            };
            let version_name = entry("DELIVERY_STATEMENT_VERSION").name;
            let version = entries
                .iter()
                .find(|entry| entry.name == version_name)
                .expect("the entry is present under its own name");
            assert_eq!(
                version.value,
                SolidityConstantValue::Uint16(1),
                "the value `proof/mint_statement` states for DELIVERY_STATEMENT_VERSION_ONE"
            );
            let expected_purposes: Vec<SolidityLibraryEntry> = [
                ("PURPOSE_MINT", 1),
                ("PURPOSE_TRANSFER", 2),
                ("PURPOSE_GRANT", 3),
                ("PURPOSE_REPLACEMENT", 4),
            ]
            .iter()
            .map(|(name, code)| {
                let mut expected = entry(name);
                expected.value = SolidityConstantValue::Uint16(*code);
                expected
            })
            .collect();
            assert_eq!(entries[6..10], expected_purposes[..]);
        }
    }
    // the_entries_carry_the_statement_version_and_each_purpose_code_in_declared_order
}

/// Contract: entry
///   `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_
///   chain_forms`; the forms the chain family hands the consumer → `Ok`
///   whose entries `MINT_FIELDS` and `TRANSFER_FIELDS` are `String`
///   holding the ABI type names of `MintStatementDescription::FIELDS` and
///   `TransferStatementDescription::FIELDS` joined by `,` in field order,
///   a fixed twenty-byte kind as `bytes20`; this block proves the
///   absorbed `KIND` outcomes of the four `chain/create_chain_forms`
///   round-trip entries the spec dispositions.
/// Arrange: the defaults; every member of `PAIRING_CONCRETES` and of
///   `CHAIN_FORMS_CONCRETES` is the variation, each sequence compared with
///   its own literal.
/// Act:     `constants::<P, F>` in the inner consumer.
/// Assert:  each entry equals the `SolidityLibraryEntry` built over its
///   name and `String` of `build_solidity_string_literal` over the literal
///   ABI type names assembled from the field kinds `proof/mint_statement`
///   and `proof/transfer_statement` state, the EVM forms' kinds, and the
///   table `render` states; every factory result `is_ok()`.
/// Boundary: `constants` over the real `SolidityAbiType::try_new` and
///   `as_str`, `SolidityStringLiteral::try_new`, and the real forms
///   concrete.
/// Mocked:  nothing.
#[test]
fn the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = Vec<ConstantsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ConstantsReturn;

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(Default::default());
                    let key_agreement = build_key_agreement_declaration(Default::default());
                    let delivery_proof = build_delivery_proof_declaration(Default::default());
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );

                    // Act
                    constants::<P, F>(&deps, params, payload)
                }
            }

            let mut results = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
                results.push(
                    created
                        .expect("the chain-forms factory ran the consumer")
                        .output,
                );
            }
            results
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
        let results = created
            .expect("the pairing factory ran the consumer")
            .output;

        // Assert
        for result in results {
            let entries = result.expect("the declared version is served").entries;
            for (name, sequence) in [
                (
                    "MINT_FIELDS",
                    "bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,\
                    uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,\
                    bytes,uint64,bytes,bytes,bytes,bytes,bytes"
                        .to_string(),
                ),
                (
                    "TRANSFER_FIELDS",
                    "bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,\
                    uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,\
                    bytes,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,\
                    bytes,bytes,bytes"
                        .to_string(),
                ),
            ] {
                let expected = build_solidity_library_entry(SolidityLibraryEntryOverrides {
                    name: Some(build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some(name.to_string()),
                        },
                    )),
                    value: Some(SolidityConstantValue::String(
                        build_solidity_string_literal(
                            SolidityStringLiteralConstructorParamsOverrides {
                                text: Some(sequence),
                            },
                        ),
                    )),
                });
                let entry = entries
                    .iter()
                    .find(|entry| entry.name == expected.name)
                    .expect("the entry is present under its own name");
                assert_eq!(entry, &expected);
            }
        }
    }
    // the_entries_carry_the_mint_and_transfer_field_sequences_over_the_chain_forms
}

/// Contract: entry `the_entries_carry_the_scalar_field_order_as_32_big_
///   endian_bytes`; a pairing of `PAIRING_CONCRETES` → `Ok` whose entry
///   `SCALAR_FIELD_ORDER` is `Uint256` whose 32 bytes the same pairing's
///   `decode_scalar` refuses with `NonCanonical` and whose bytes less one
///   it admits.
/// Arrange: the defaults; every member of `PAIRING_CONCRETES` and of
///   `CHAIN_FORMS_CONCRETES` is the variation, so a constant, a truncated,
///   a little-endian, or an off-by-one order fails.
/// Act:     `constants::<P, F>` in the inner consumer.
/// Assert:  in place in the inner consumer, the `SCALAR_FIELD_ORDER` entry
///   equals the built entry of `Uint256` over the order's 32 big-endian
///   bytes the received pairing's `scalar_field_order` returns;
///   `decode_scalar` over the bytes returns `Err(
///   DecodeScalarErrorReturn::NonCanonical)` whole and over the bytes less
///   one returns `Ok`; every factory result `is_ok()`.
/// Boundary: `constants` over the real `SolidityUint256::try_new`, with the
///   real `scalar_field_order` and `decode_scalar` as the oracle.
/// Mocked:  nothing.
#[test]
fn the_entries_carry_the_scalar_field_order_as_32_big_endian_bytes() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingArithmetic + IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingArithmetic + IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(Default::default());
                    let key_agreement = build_key_agreement_declaration(Default::default());
                    let delivery_proof = build_delivery_proof_declaration(Default::default());
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );

                    // Act
                    let result = constants::<P, F>(&deps, params, payload);

                    // Assert
                    let Ok(order) = self
                        .pairing
                        .scalar_field_order(ScalarFieldOrderParams, ScalarFieldOrderPayload);
                    let entries = result.expect("the declared version is served").entries;
                    let name = build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some("SCALAR_FIELD_ORDER".to_string()),
                        },
                    );
                    let big_endian: [u8; 32] = order
                        .bytes
                        .as_slice()
                        .try_into()
                        .expect("the family's order is 32 bytes");
                    let expected = build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name.clone()),
                        value: Some(SolidityConstantValue::Uint256(build_solidity_uint256(
                            SolidityUint256ConstructorParamsOverrides {
                                big_endian: Some(big_endian),
                            },
                        ))),
                    });
                    let entry = entries
                        .iter()
                        .find(|entry| entry.name == name)
                        .expect("the entry is present under its own name");
                    assert_eq!(entry, &expected);
                    assert_eq!(
                        self.pairing
                            .decode_scalar(DecodeScalarParams, &order.bytes)
                            .err(),
                        Some(DecodeScalarErrorReturn::NonCanonical)
                    );
                    let mut less_one = order.bytes.clone();
                    for byte in less_one.iter_mut().rev() {
                        let (next, borrow) = byte.overflowing_sub(1);
                        *byte = next;
                        if !borrow {
                            break;
                        }
                    }
                    assert!(
                        self.pairing
                            .decode_scalar(DecodeScalarParams, &less_one)
                            .is_ok()
                    );
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // the_entries_carry_the_scalar_field_order_as_32_big_endian_bytes
}

/// Contract: entry `the_entries_carry_the_generators_precompile_encodings`;
///   a pairing of `PAIRING_CONCRETES` → `Ok` whose entries `G1_GENERATOR`
///   and `G2_GENERATOR` are `Bytes` that the same pairing's `decode_g1`
///   and `decode_g2` admit, each decoded point's sum with the negation of
///   that group's generator being the identity.
/// Arrange: the defaults; every member of `PAIRING_CONCRETES` and of
///   `CHAIN_FORMS_CONCRETES` is the variation, so an entry holding the
///   other group's generator or another point fails at the decoder.
/// Act:     `constants::<P, F>` in the inner consumer.
/// Assert:  in place in the inner consumer, each entry is `Bytes`; the
///   received pairing's `decode_g1` and `decode_g2` over the entries'
///   bytes return `Ok`; the sum of each decoded point with the negation
///   of that group's generator satisfies `is_identity_g1` or
///   `is_identity_g2`; every factory result `is_ok()`.
/// Boundary: `constants` over the real pairing, with the decoders and
///   group operations as the oracle.
/// Mocked:  nothing.
#[test]
fn the_entries_carry_the_generators_precompile_encodings() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingArithmetic + IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingArithmetic + IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(Default::default());
                    let key_agreement = build_key_agreement_declaration(Default::default());
                    let delivery_proof = build_delivery_proof_declaration(Default::default());
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );
                    let g1_name = build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some("G1_GENERATOR".to_string()),
                        },
                    );
                    let g2_name = build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some("G2_GENERATOR".to_string()),
                        },
                    );

                    // Act
                    let result = constants::<P, F>(&deps, params, payload);

                    // Assert
                    let entries = result.expect("the declared version is served").entries;
                    let g1_bytes = entries
                        .iter()
                        .find_map(|entry| match &entry.value {
                            SolidityConstantValue::Bytes(bytes) if entry.name == g1_name => {
                                Some(bytes)
                            }
                            _ => None,
                        })
                        .expect("G1_GENERATOR is a Bytes entry");
                    let g2_bytes = entries
                        .iter()
                        .find_map(|entry| match &entry.value {
                            SolidityConstantValue::Bytes(bytes) if entry.name == g2_name => {
                                Some(bytes)
                            }
                            _ => None,
                        })
                        .expect("G2_GENERATOR is a Bytes entry");
                    let decoded_g1 = self
                        .pairing
                        .decode_g1(DecodeG1Params, g1_bytes)
                        .expect("the G1_GENERATOR bytes decode");
                    let Ok(generator_g1) = self
                        .pairing
                        .g1_generator(G1GeneratorParams, G1GeneratorPayload);
                    let Ok(negated_g1) = self.pairing.neg_g1(
                        NegG1Params,
                        NegG1Payload {
                            point: generator_g1.point,
                        },
                    );
                    let Ok(sum_g1) = self.pairing.add_g1(
                        AddG1Params,
                        AddG1Payload {
                            left: decoded_g1.point,
                            right: negated_g1.negation,
                        },
                    );
                    let Ok(identity_g1) = self.pairing.is_identity_g1(
                        IsIdentityG1Params,
                        IsIdentityG1Payload { point: sum_g1.sum },
                    );
                    assert!(identity_g1.is_identity);
                    let decoded_g2 = self
                        .pairing
                        .decode_g2(DecodeG2Params, g2_bytes)
                        .expect("the G2_GENERATOR bytes decode");
                    let Ok(generator_g2) = self
                        .pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload);
                    let Ok(negated_g2) = self.pairing.neg_g2(
                        NegG2Params,
                        NegG2Payload {
                            point: generator_g2.point,
                        },
                    );
                    let Ok(sum_g2) = self.pairing.add_g2(
                        AddG2Params,
                        AddG2Payload {
                            left: decoded_g2.point,
                            right: negated_g2.negation,
                        },
                    );
                    let Ok(identity_g2) = self.pairing.is_identity_g2(
                        IsIdentityG2Params,
                        IsIdentityG2Payload { point: sum_g2.sum },
                    );
                    assert!(identity_g2.is_identity);
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // the_entries_carry_the_generators_precompile_encodings
}

/// Contract: entry `the_entries_carry_the_verifier_form_the_pairing_
///   declares`; a pairing of `PAIRING_CONCRETES` → `Ok` whose entry
///   `SECOND_GROUP_ARITHMETIC` is `Bool(false)` for a pairing whose
///   declaration carries `PrecompileEncoding::Eip196Eip197` and
///   `Bool(true)` for `PrecompileEncoding::Eip2537`, the expectation keyed
///   on the precompile encoding and never on the `verifier_group_
///   arithmetic` field the function reads.
/// Arrange: the defaults; every member of `PAIRING_CONCRETES` and of
///   `CHAIN_FORMS_CONCRETES` is the variation, which holds a pairing of
///   each precompile encoding, so a constant fails.
/// Act:     `constants::<P, F>` in the inner consumer.
/// Assert:  in place in the inner consumer, the `SECOND_GROUP_ARITHMETIC`
///   entry equals `Bool(false)` where `P::DECLARATION.precompile_encoding`
///   is `Eip196Eip197` and `Bool(true)` where it is `Eip2537`; every
///   factory result `is_ok()`.
/// Boundary: `constants` over the real pairing and forms concretes.
/// Mocked:  nothing.
#[test]
fn the_entries_carry_the_verifier_form_the_pairing_declares() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingArithmetic + IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingArithmetic + IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ();

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(Default::default());
                    let key_agreement = build_key_agreement_declaration(Default::default());
                    let delivery_proof = build_delivery_proof_declaration(Default::default());
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );

                    // Act
                    let result = constants::<P, F>(&deps, params, payload);

                    // Assert
                    let entries = result.expect("the declared version is served").entries;
                    let name = build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some("SECOND_GROUP_ARITHMETIC".to_string()),
                        },
                    );
                    let expected = build_solidity_library_entry(SolidityLibraryEntryOverrides {
                        name: Some(name.clone()),
                        value: Some(SolidityConstantValue::Bool(
                            P::DECLARATION.precompile_encoding == PrecompileEncoding::Eip2537,
                        )),
                    });
                    let entry = entries
                        .iter()
                        .find(|entry| entry.name == name)
                        .expect("the entry is present under its own name");
                    assert_eq!(entry, &expected);
                }
            }

            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
            }
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
    }
    // the_entries_carry_the_verifier_form_the_pairing_declares
}

/// Contract: entry `the_entries_are_listed_in_declared_order`; a concrete
///   of each declared set → `Ok` whose entry names, in order, are
///   `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`,
///   `CHALLENGE_TAG`, `WEIGHT_TAG`, `DELIVERY_STATEMENT_VERSION`,
///   `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`,
///   `PURPOSE_REPLACEMENT`, `MINT_FIELDS`, `TRANSFER_FIELDS`,
///   `SCALAR_FIELD_ORDER`, `G1_GENERATOR`, `G2_GENERATOR`, and
///   `SECOND_GROUP_ARITHMETIC`.
/// Arrange: the defaults; every member of `PAIRING_CONCRETES` and of
///   `CHAIN_FORMS_CONCRETES` is the variation.
/// Act:     `constants::<P, F>` in the inner consumer.
/// Assert:  the names of the returned entries, read in order, equal the
///   names the contract lists in the order it lists them, each built with
///   `build_solidity_constant_name`; every factory result `is_ok()`.
/// Boundary: `constants` over the real pairing and forms concretes.
/// Mocked:  nothing.
#[test]
fn the_entries_are_listed_in_declared_order() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = Vec<ConstantsReturn>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = ConstantsReturn;

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(Default::default());
                    let key_agreement = build_key_agreement_declaration(Default::default());
                    let delivery_proof = build_delivery_proof_declaration(Default::default());
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );

                    // Act
                    constants::<P, F>(&deps, params, payload)
                }
            }

            let mut results = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
                results.push(
                    created
                        .expect("the chain-forms factory ran the consumer")
                        .output,
                );
            }
            results
        }
    }

    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
        let results = created
            .expect("the pairing factory ran the consumer")
            .output;

        // Assert
        for result in results {
            let entries = result.expect("the declared version is served").entries;
            let names: Vec<SolidityConstantName> =
                entries.iter().map(|entry| entry.name.clone()).collect();
            let expected_names: Vec<SolidityConstantName> = [
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
            assert_eq!(names, expected_names);
        }
    }
    // the_entries_are_listed_in_declared_order
}

/// Contract: absorbed entry `libraries_sharing_an_identifier_return_the_
///   same_scalar_field_order`; any two concretes whose declared
///   target-group encoding identifiers are equal return equal
///   `SCALAR_FIELD_ORDER` entries.
/// Arrange: the defaults; every member of `PAIRING_CONCRETES` and of
///   `CHAIN_FORMS_CONCRETES` is the variation, whose declared set holds
///   libraries that share an identifier and libraries that do not, so a
///   constant order or an order read from the wrong pairing fails.
/// Act:     `constants::<P, F>` in the inner consumer, which returns the
///   pairing's `P::DECLARATION.target_group_encoding` with the
///   `SCALAR_FIELD_ORDER` entry as its output.
/// Assert:  any two outputs whose identifiers are equal hold equal
///   entries; every factory result `is_ok()`.
/// Boundary: `constants` over every real pairing concrete, the comparison
///   made across the runs.
/// Mocked:  nothing.
#[test]
fn libraries_sharing_an_identifier_carry_the_same_scalar_field_order() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = Vec<(TargetGroupEncodingIdentifier, SolidityLibraryEntry)>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            struct InnerConsumer<'a, P: IPairingArithmetic + IPairingReference> {
                pairing: &'a P,
            }

            impl<'a, P: IPairingArithmetic + IPairingReference> IChainFormsConsumer for InnerConsumer<'a, P> {
                type Output = (TargetGroupEncodingIdentifier, SolidityLibraryEntry);

                fn consume_chain_forms<F: IChainForms>(
                    &self,
                    _params: ConsumeChainFormsParams,
                    _payload: ConsumeChainFormsPayload<F>,
                ) -> Self::Output {
                    // Arrange
                    let kem = build_kem_declaration(Default::default());
                    let key_agreement = build_key_agreement_declaration(Default::default());
                    let delivery_proof = build_delivery_proof_declaration(Default::default());
                    let deps = build_constants_deps(self.pairing, Default::default());
                    let params = build_constants_params(Default::default());
                    let payload = build_constants_payload(
                        &kem,
                        &key_agreement,
                        &delivery_proof,
                        Default::default(),
                    );

                    // Act
                    let result = constants::<P, F>(&deps, params, payload);
                    let entries = result.expect("the declared version is served").entries;
                    let name = build_solidity_constant_name(
                        SolidityConstantNameConstructorParamsOverrides {
                            text: Some("SCALAR_FIELD_ORDER".to_string()),
                        },
                    );
                    let entry = entries
                        .iter()
                        .find(|entry| entry.name == name)
                        .expect("the entry is present under its own name")
                        .clone();
                    (P::DECLARATION.target_group_encoding, entry)
                }
            }

            let mut outputs = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        InnerConsumer {
                            pairing: &payload.adapter,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(created.is_ok());
                outputs.push(
                    created
                        .expect("the chain-forms factory ran the consumer")
                        .output,
                );
            }
            outputs
        }
    }

    let mut outputs = Vec::new();
    for concrete in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(OuterConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok());
        outputs.extend(
            created
                .expect("the pairing factory ran the consumer")
                .output,
        );
    }

    // Assert
    for (left_identifier, left_entry) in &outputs {
        for (right_identifier, right_entry) in &outputs {
            if left_identifier == right_identifier {
                assert_eq!(left_entry, right_entry);
            }
        }
    }
    // libraries_sharing_an_identifier_carry_the_same_scalar_field_order
}
