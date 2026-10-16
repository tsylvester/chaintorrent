#![allow(clippy::expect_used)]

use super::super::render::provides::{
    SolidityConstantName, SolidityConstantNameConstructorParamsOverrides, SolidityConstantValue,
    SolidityLibraryEntryOverrides, SolidityStringLiteralConstructorParamsOverrides,
    SolidityUint256ConstructorParamsOverrides, build_solidity_constant_name,
    build_solidity_library_entry, build_solidity_string_literal, build_solidity_uint256,
};
use super::constants;
use super::provides::{
    ConstantsErrorReturn, ConstantsParamsOverrides, ConstantsReturn, build_constants_deps,
    build_constants_params, build_constants_payload,
};
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsReturn, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use envelope::{KeyAgreementDeclarationOverrides, build_key_agreement_declaration};
use kem::{KemDeclarationOverrides, build_kem_declaration};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, IPairingArithmetic,
    IPairingConsumer, IPairingReference, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use proof::{DeliveryProofDeclarationOverrides, build_delivery_proof_declaration};

/// Contract: `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE`
///   → `Err(ConstantsErrorReturn::UnsupportedStatementVersion { version:
///   params.statement_version })`, decided by equality before any other step.
/// Arrange: `build_constants_params` overriding `statement_version` with the
///   literal `0`, the one variation from the default
///   `DELIVERY_STATEMENT_VERSION_ONE`; the declarations from
///   `build_kem_declaration`, `build_key_agreement_declaration`, and
///   `build_delivery_proof_declaration` at their defaults.
/// Act:     `constants::<P, F>` over the deps, params, and payload, inside the
///   inner consumer's method.
/// Assert:  `assert_eq!` of the return with `Err(
///   ConstantsErrorReturn::UnsupportedStatementVersion { version: 0 })`;
///   outside, both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn constants_refuse_a_statement_version_no_description_serves() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<ConstantsReturn>;

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
                    let params = build_constants_params(ConstantsParamsOverrides {
                        statement_version: Some(0),
                    });
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

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let result = chained
        .expect("the chain-forms factory ran the consumer")
        .output;

    // Assert
    assert_eq!(
        result,
        Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: 0 })
    );
    // constants_refuse_a_statement_version_no_description_serves
}

/// Contract: a supported statement version → `Ok` whose entries are named,
///   in order, `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`,
///   `CHALLENGE_TAG`, `WEIGHT_TAG`, `DELIVERY_STATEMENT_VERSION`,
///   `PURPOSE_MINT`, `PURPOSE_TRANSFER`, `PURPOSE_GRANT`,
///   `PURPOSE_REPLACEMENT`, `MINT_FIELDS`, `TRANSFER_FIELDS`,
///   `SCALAR_FIELD_ORDER`, `G1_GENERATOR`, `G2_GENERATOR`, and
///   `SECOND_GROUP_ARITHMETIC`.
/// Arrange: every collaborator at its defaults, `build_constants_params` at
///   its default `DELIVERY_STATEMENT_VERSION_ONE`; the entry names are
///   distinct, so a reordered, dropped, or renamed entry fails.
/// Act:     `constants::<P, F>` over the deps, params, and payload, inside
///   the inner consumer's method.
/// Assert:  `assert_eq!` of the names of the success's `entries`, in order,
///   with the names the contract lists, each built by
///   `build_solidity_constant_name` over its text as a literal; both
///   factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn constants_return_the_entries_in_declared_order() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<ConstantsReturn>;

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

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let result = chained
        .expect("the chain-forms factory ran the consumer")
        .output;

    // Assert
    let entries = result.expect("the declared version is served").entries;
    let names: Vec<SolidityConstantName> = entries.iter().map(|entry| entry.name.clone()).collect();
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
    // constants_return_the_entries_in_declared_order
}

/// Contract: each declared tag is read from its declaration → the entries
///   `IDENTITY_TAG`, `POSSESSION_G1_TAG`, `POSSESSION_G2_TAG`,
///   `CHALLENGE_TAG`, and `WEIGHT_TAG` hold `SolidityConstantValue::Bytes`
///   of the KEM declaration's `identity_tag`, the key agreement's
///   `possession_g1_tag` and `possession_g2_tag`, and the delivery proof's
///   `challenge_tag` and `weight_tag`.
/// Arrange: `build_kem_declaration`, `build_key_agreement_declaration`, and
///   `build_delivery_proof_declaration` each overriding its tags with
///   literals that differ from each other and from every builder default,
///   so an entry read from another field, from a default, or under another
///   name fails.
/// Act:     `constants::<P, F>` over the deps, params, and that payload,
///   inside the inner consumer's method.
/// Assert:  for each name the contract lists, the entry read by its name
///   equals `build_solidity_library_entry` over that name and
///   `SolidityConstantValue::Bytes` of the literal arranged for it, by
///   `assert_eq!`; both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn constants_carry_each_declarations_domain_tag_under_its_own_name() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<ConstantsReturn>;

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

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let result = chained
        .expect("the chain-forms factory ran the consumer")
        .output;

    // Assert
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
    // constants_carry_each_declarations_domain_tag_under_its_own_name
}

/// Contract: the statement version the params carry and each purpose's code
///   → `DELIVERY_STATEMENT_VERSION` holds `SolidityConstantValue::Uint16`
///   of the version, and `PURPOSE_MINT`, `PURPOSE_TRANSFER`,
///   `PURPOSE_GRANT`, and `PURPOSE_REPLACEMENT` hold `Uint16` of the codes
///   `DeliveryPurpose::Mint`, `Transfer`, `Grant`, and `Replacement`
///   declare.
/// Arrange: every collaborator at its defaults, the params at
///   `DELIVERY_STATEMENT_VERSION_ONE`; the purposes' codes differ from each
///   other, so swapped codes fail.
/// Act:     `constants::<P, F>` over the deps, params, and payload, inside
///   the inner consumer's method.
/// Assert:  each entry the contract lists equals
///   `build_solidity_library_entry` over its name and `Uint16` of the
///   literal the private integration block
///   `the_entries_carry_the_statement_version_and_each_purpose_code_in_
///   declared_order` states for it, by `assert_eq!`; both factories' `Ok`
///   arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn constants_carry_the_statement_version_and_each_purpose_code() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<ConstantsReturn>;

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

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let result = chained
        .expect("the chain-forms factory ran the consumer")
        .output;

    // Assert
    let entries = result.expect("the declared version is served").entries;
    for (name, code) in [
        ("DELIVERY_STATEMENT_VERSION", 1),
        ("PURPOSE_MINT", 1),
        ("PURPOSE_TRANSFER", 2),
        ("PURPOSE_GRANT", 3),
        ("PURPOSE_REPLACEMENT", 4),
    ] {
        let expected = build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Uint16(code)),
        });
        let entry = entries
            .iter()
            .find(|entry| entry.name == expected.name)
            .expect("the entry is present under its own name");
        assert_eq!(entry, &expected);
    }
    // constants_carry_the_statement_version_and_each_purpose_code
}

/// Contract: the ABI type names of `MintStatementDescription::FIELDS` and
///   `TransferStatementDescription::FIELDS` joined by `,` in field order →
///   `MINT_FIELDS` and `TRANSFER_FIELDS` hold `SolidityConstantValue::
///   String` of the joined text.
/// Arrange: every collaborator at its defaults; the chain forms' kinds,
///   `FixedBytes20` for the identity, `Unsigned256` for the entitlement and
///   the chain identifier, and `Unsigned64` for the interval, the kinds the
///   chain family's mock forms carry and the EVM forms carry alike, so the
///   mint and transfer sequences differ from each other.
/// Act:     `constants::<P, F>` over the deps, params, and payload, inside
///   the inner consumer's method.
/// Assert:  the `MINT_FIELDS` and `TRANSFER_FIELDS` entries each equal
///   `build_solidity_library_entry` over their name and
///   `SolidityConstantValue::String` of `build_solidity_string_literal`
///   over the literal text the private integration block
///   `the_entries_carry_the_mint_and_transfer_field_sequences_over_the_
///   chain_forms` assembles for it, by `assert_eq!`; both factories' `Ok`
///   arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn constants_carry_the_mint_and_transfer_field_sequences() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<ConstantsReturn>;

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

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let result = chained
        .expect("the chain-forms factory ran the consumer")
        .output;

    // Assert
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
                build_solidity_string_literal(SolidityStringLiteralConstructorParamsOverrides {
                    text: Some(sequence),
                }),
            )),
        });
        let entry = entries
            .iter()
            .find(|entry| entry.name == expected.name)
            .expect("the entry is present under its own name");
        assert_eq!(entry, &expected);
    }
    // constants_carry_the_mint_and_transfer_field_sequences
}

/// Contract: the pairing's `scalar_field_order` → `SCALAR_FIELD_ORDER`
///   holds `SolidityConstantValue::Uint256` of the order's 32 big-endian
///   bytes.
/// Arrange: every collaborator at its defaults, the pairing the family's
///   mock concrete, whose order is `MOCK_GROUP_ORDER`, `65521`, as 32
///   big-endian bytes: thirty bytes of `0x00`, then `0xFF`, `0xF1`; the
///   order's nonzero low bytes separate a constant, a truncated, and a
///   little-endian order from the return.
/// Act:     `constants::<P, F>` over the deps, params, and payload, inside
///   the inner consumer's method.
/// Assert:  the `SCALAR_FIELD_ORDER` entry equals
///   `build_solidity_library_entry` over its name and
///   `SolidityConstantValue::Uint256` of `build_solidity_uint256` over
///   `big_endian` the 32 bytes above, by `assert_eq!`; both factories' `Ok`
///   arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn constants_carry_the_scalar_field_order_as_32_big_endian_bytes() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<ConstantsReturn>;

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

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let result = chained
        .expect("the chain-forms factory ran the consumer")
        .output;

    // Assert
    let entries = result.expect("the declared version is served").entries;
    let mut big_endian = [0u8; 32];
    big_endian[30] = 0xFF;
    big_endian[31] = 0xF1;
    let expected = build_solidity_library_entry(SolidityLibraryEntryOverrides {
        name: Some(build_solidity_constant_name(
            SolidityConstantNameConstructorParamsOverrides {
                text: Some("SCALAR_FIELD_ORDER".to_string()),
            },
        )),
        value: Some(SolidityConstantValue::Uint256(build_solidity_uint256(
            SolidityUint256ConstructorParamsOverrides {
                big_endian: Some(big_endian),
            },
        ))),
    });
    let entry = entries
        .iter()
        .find(|entry| entry.name == expected.name)
        .expect("the entry is present under its own name");
    assert_eq!(entry, &expected);
    // constants_carry_the_scalar_field_order_as_32_big_endian_bytes
}

/// Contract: the pairing's generators, encoded by `encode_g1` and
///   `encode_g2` → `G1_GENERATOR` and `G2_GENERATOR` hold
///   `SolidityConstantValue::Bytes` of the encodings' bytes.
/// Arrange: every collaborator at its defaults, the pairing the family's
///   mock concrete, whose first-group generator is residue `1` and
///   second-group generator residue `2`, each encoded as eight big-endian
///   bytes; the encodings differ, so an entry holding the other group's
///   generator fails.
/// Act:     `constants::<P, F>` over the deps, params, and payload, inside
///   the inner consumer's method.
/// Assert:  `G1_GENERATOR` equals `build_solidity_library_entry` over its
///   name and `Bytes` of seven bytes of `0x00` then `0x01`, and
///   `G2_GENERATOR` the same over seven bytes of `0x00` then `0x02`, by
///   `assert_eq!`; both factories' `Ok` arms.
/// Collaborators: the pairing family's and the chain family's mock concretes
///   through `create_pairing` and `create_chain_forms` under the default
///   params.
#[test]
fn constants_carry_the_generators_encodings() {
    struct OuterConsumer;

    impl IPairingConsumer for OuterConsumer {
        type Output = CreateChainFormsReturn<ConstantsReturn>;

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

            create_chain_forms(
                &build_create_chain_forms_deps(
                    InnerConsumer {
                        pairing: &payload.adapter,
                    },
                    Default::default(),
                ),
                build_create_chain_forms_params(Default::default()),
                build_create_chain_forms_payload(Default::default()),
            )
        }
    }

    let created = create_pairing(
        &build_create_pairing_deps(OuterConsumer, Default::default()),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    );

    assert!(created.is_ok());
    let chained = created
        .expect("the pairing factory ran the consumer")
        .output;
    assert!(chained.is_ok());
    let result = chained
        .expect("the chain-forms factory ran the consumer")
        .output;

    // Assert
    let entries = result.expect("the declared version is served").entries;
    for (name, encoding) in [
        ("G1_GENERATOR", vec![0, 0, 0, 0, 0, 0, 0, 1]),
        ("G2_GENERATOR", vec![0, 0, 0, 0, 0, 0, 0, 2]),
    ] {
        let expected = build_solidity_library_entry(SolidityLibraryEntryOverrides {
            name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            value: Some(SolidityConstantValue::Bytes(encoding)),
        });
        let entry = entries
            .iter()
            .find(|entry| entry.name == expected.name)
            .expect("the entry is present under its own name");
        assert_eq!(entry, &expected);
    }
    // constants_carry_the_generators_encodings
}
