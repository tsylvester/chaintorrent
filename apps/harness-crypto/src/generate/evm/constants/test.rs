#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::super::render::provides::{
    SolidityConstantNameConstructorParamsOverrides, SolidityConstantValue, SolidityLibraryEntry,
    SolidityStringLiteralConstructorParamsOverrides, SolidityUint256ConstructorParamsOverrides,
    build_solidity_constant_name, build_solidity_string_literal, build_solidity_uint256,
};
use super::constants;
use super::interface::{
    ConstantsDeps, ConstantsErrorReturn, ConstantsParams, ConstantsPayload, ConstantsReturn,
};
use super::mock::{ConstantsParamsOverrides, build_constants_params};
use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsDeps,
    CreateChainFormsPayload, IChainForms, IChainFormsConsumer, build_create_chain_forms_params,
    create_chain_forms,
};
use envelope::{
    KeyAgreementDeclaration, KeyAgreementDeclarationOverrides, build_key_agreement_declaration,
};
use hex::decode;
use kem::{KemDeclaration, KemDeclarationOverrides, build_kem_declaration};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference, PairingConcrete,
    build_create_pairing_params, create_pairing,
};
use proof::{
    DeliveryProofDeclaration, DeliveryProofDeclarationOverrides, build_delivery_proof_declaration,
};

const G1_GENERATOR_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000001\
     0000000000000000000000000000000000000000000000000000000000000002";
const G2_GENERATOR_HEX: &str = "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
     1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
     090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
     12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";

struct ConstantsPairingProbe {
    params: ConstantsParams,
    kem: KemDeclaration,
    key_agreement: KeyAgreementDeclaration,
    delivery_proof: DeliveryProofDeclaration,
}

impl IPairingConsumer for ConstantsPairingProbe {
    type Output = ConstantsReturn;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let Ok(success) = create_chain_forms(
            &CreateChainFormsDeps {
                consumer: ConstantsFormsProbe {
                    pairing: &payload.adapter,
                    probe: self,
                },
            },
            build_create_chain_forms_params(Default::default()),
            CreateChainFormsPayload,
        ) else {
            panic!("the EVM forms concrete is admitted")
        };
        success.output
    }
}

struct ConstantsFormsProbe<'a, P: IPairingReference> {
    pairing: &'a P,
    probe: &'a ConstantsPairingProbe,
}

impl<P: IPairingReference> IChainFormsConsumer for ConstantsFormsProbe<'_, P> {
    type Output = ConstantsReturn;

    fn consume_chain_forms<F: IChainForms>(
        &self,
        _params: ConsumeChainFormsParams,
        _payload: ConsumeChainFormsPayload<F>,
    ) -> Self::Output {
        constants::<P, F>(
            &ConstantsDeps {
                pairing: self.pairing,
            },
            self.probe.params,
            ConstantsPayload {
                kem: &self.probe.kem,
                key_agreement: &self.probe.key_agreement,
                delivery_proof: &self.probe.delivery_proof,
            },
        )
    }
}

fn run_constants(probe: ConstantsPairingProbe, concrete: PairingConcrete) -> ConstantsReturn {
    let Ok(success) = create_pairing(
        &CreatePairingDeps { consumer: probe },
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(concrete),
            ..Default::default()
        }),
        CreatePairingPayload,
    ) else {
        panic!("the pairing concrete is admitted")
    };
    success.output
}

fn entry_named<'a>(entries: &'a [SolidityLibraryEntry], name: &str) -> &'a SolidityLibraryEntry {
    let name = build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
        text: Some(name.to_string()),
    });
    entries
        .iter()
        .find(|entry| entry.name == name)
        .expect("the entry is present")
}

/// Contract: `params.statement_version` not equal to
///   `DELIVERY_STATEMENT_VERSION_ONE` is refused, decided by equality before any
///   other step.
/// Arrange: a probe whose params carry statement_version 2.
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks.
/// Assert:  the output is
///   Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: 2 }).
#[test]
fn constants_refuse_a_statement_version_no_description_serves() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(ConstantsParamsOverrides {
            statement_version: Some(2),
        }),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };

    // Act
    let output = run_constants(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    assert_eq!(
        output,
        Err(ConstantsErrorReturn::UnsupportedStatementVersion { version: 2 })
    );
}

/// Contract: the identity mapping's, the proofs of possession's, and the
///   delivery proof's domain tags are each emitted as Bytes read from their
///   families' declarations.
/// Arrange: the KEM declaration's identity_tag "test-identity", the
///   key-agreement declaration's possession_g1_tag "test-possession-g1" and
///   possession_g2_tag "test-possession-g2", and the delivery-proof
///   declaration's challenge_tag "test-challenge" and weight_tag
///   "test-weight".
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks.
/// Assert:  IDENTITY_TAG, POSSESSION_G1_TAG, POSSESSION_G2_TAG, CHALLENGE_TAG,
///   and WEIGHT_TAG hold SolidityConstantValue::Bytes of those byte strings.
#[test]
fn constants_carry_each_family_declarations_domain_tags() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(KemDeclarationOverrides {
            identity_tag: Some(b"test-identity"),
            ..Default::default()
        }),
        key_agreement: build_key_agreement_declaration(KeyAgreementDeclarationOverrides {
            possession_g1_tag: Some(b"test-possession-g1"),
            possession_g2_tag: Some(b"test-possession-g2"),
            ..Default::default()
        }),
        delivery_proof: build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
            challenge_tag: Some(b"test-challenge"),
            weight_tag: Some(b"test-weight"),
            ..Default::default()
        }),
    };

    // Act
    let output = run_constants(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output else {
        panic!("the constants are computed")
    };
    assert_eq!(
        entry_named(&success.entries, "IDENTITY_TAG").value,
        SolidityConstantValue::Bytes(b"test-identity".to_vec())
    );
    assert_eq!(
        entry_named(&success.entries, "POSSESSION_G1_TAG").value,
        SolidityConstantValue::Bytes(b"test-possession-g1".to_vec())
    );
    assert_eq!(
        entry_named(&success.entries, "POSSESSION_G2_TAG").value,
        SolidityConstantValue::Bytes(b"test-possession-g2".to_vec())
    );
    assert_eq!(
        entry_named(&success.entries, "CHALLENGE_TAG").value,
        SolidityConstantValue::Bytes(b"test-challenge".to_vec())
    );
    assert_eq!(
        entry_named(&success.entries, "WEIGHT_TAG").value,
        SolidityConstantValue::Bytes(b"test-weight".to_vec())
    );
}

/// Contract: the delivery-statement version the params carry is emitted as
///   Uint16, and each declared delivery purpose's code is emitted as Uint16
///   under its purpose's name, in the declared purposes' order.
/// Arrange: the default probe.
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks.
/// Assert:  DELIVERY_STATEMENT_VERSION holds Uint16(1) and PURPOSE_MINT,
///   PURPOSE_TRANSFER, PURPOSE_GRANT, and PURPOSE_REPLACEMENT hold Uint16(1),
///   (2), (3), and (4).
#[test]
fn constants_carry_the_statement_version_and_each_purpose_code() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };

    // Act
    let output = run_constants(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output else {
        panic!("the constants are computed")
    };
    assert_eq!(
        entry_named(&success.entries, "DELIVERY_STATEMENT_VERSION").value,
        SolidityConstantValue::Uint16(1)
    );
    assert_eq!(
        entry_named(&success.entries, "PURPOSE_MINT").value,
        SolidityConstantValue::Uint16(1)
    );
    assert_eq!(
        entry_named(&success.entries, "PURPOSE_TRANSFER").value,
        SolidityConstantValue::Uint16(2)
    );
    assert_eq!(
        entry_named(&success.entries, "PURPOSE_GRANT").value,
        SolidityConstantValue::Uint16(3)
    );
    assert_eq!(
        entry_named(&success.entries, "PURPOSE_REPLACEMENT").value,
        SolidityConstantValue::Uint16(4)
    );
}

/// Contract: the mint and transfer transcripts' field kinds, read from their
///   descriptions over the forms in play, are each emitted as one
///   comma-separated string of ABI type names.
/// Arrange: the default probe; the EVM forms' identity is bytes20, entitlement
///   and chain identifier uint256, and interval uint64.
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks.
/// Assert:  MINT_FIELDS and TRANSFER_FIELDS hold SolidityConstantValue::String
///   of the two field sequences.
#[test]
fn constants_render_the_mint_and_transfer_field_sequences_over_the_evm_forms() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };

    // Act
    let output = run_constants(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output else {
        panic!("the constants are computed")
    };
    assert_eq!(
        entry_named(&success.entries, "MINT_FIELDS").value,
        SolidityConstantValue::String(build_solidity_string_literal(
            SolidityStringLiteralConstructorParamsOverrides {
                text: Some("bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,bytes,bytes".to_string()),
            }
        ))
    );
    assert_eq!(
        entry_named(&success.entries, "TRANSFER_FIELDS").value,
        SolidityConstantValue::String(build_solidity_string_literal(
            SolidityStringLiteralConstructorParamsOverrides {
                text: Some("bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,bytes,bytes,bytes".to_string()),
            }
        ))
    );
}

/// Contract: the scalar field's order, read from the pairing's reference trait
///   as big-endian bytes, is emitted as a 256-bit value.
/// Arrange: the default probe.
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks.
/// Assert:  SCALAR_FIELD_ORDER holds SolidityConstantValue::Uint256 of
///   0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001.
#[test]
fn constants_carry_the_bn254_scalar_field_order_big_endian() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };

    // Act
    let output = run_constants(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output else {
        panic!("the constants are computed")
    };
    assert_eq!(
        entry_named(&success.entries, "SCALAR_FIELD_ORDER").value,
        SolidityConstantValue::Uint256(build_solidity_uint256(
            SolidityUint256ConstructorParamsOverrides {
                big_endian: Some([
                    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81,
                    0x81, 0x58, 0x5d, 0x28, 0x33, 0xe8, 0x48, 0x79, 0xb9, 0x70, 0x91, 0x43, 0xe1,
                    0xf5, 0x93, 0xf0, 0x00, 0x00, 0x01,
                ]),
            }
        ))
    );
}

/// Contract: the scalar field's order, read from the pairing's reference trait
///   as big-endian bytes, is emitted as a 256-bit value.
/// Arrange: the default probe.
/// Act:     constants through the probe over PairingConcrete::Bls12381Arkworks.
/// Assert:  SCALAR_FIELD_ORDER holds SolidityConstantValue::Uint256 of
///   0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001.
#[test]
fn constants_carry_the_bls12_381_scalar_field_order_big_endian() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };

    // Act
    let output = run_constants(probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    let Ok(success) = output else {
        panic!("the constants are computed")
    };
    assert_eq!(
        entry_named(&success.entries, "SCALAR_FIELD_ORDER").value,
        SolidityConstantValue::Uint256(build_solidity_uint256(
            SolidityUint256ConstructorParamsOverrides {
                big_endian: Some([
                    0x73, 0xed, 0xa7, 0x53, 0x29, 0x9d, 0x7d, 0x48, 0x33, 0x39, 0xd8, 0x08, 0x09,
                    0xa1, 0xd8, 0x05, 0x53, 0xbd, 0xa4, 0x02, 0xff, 0xfe, 0x5b, 0xfe, 0xff, 0xff,
                    0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
                ]),
            }
        ))
    );
}

/// Contract: the first- and second-group generators' precompile encodings are
///   each emitted as Bytes.
/// Arrange: the default probe.
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks.
/// Assert:  G1_GENERATOR holds Bytes of the EIP-196 generator encoding and
///   G2_GENERATOR holds Bytes of the EIP-197 generator encoding.
#[test]
fn constants_carry_the_bn254_generators_precompile_encodings() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };
    let Ok(g1) = decode(G1_GENERATOR_HEX) else {
        panic!("the G1 generator hex decodes")
    };
    let Ok(g2) = decode(G2_GENERATOR_HEX) else {
        panic!("the G2 generator hex decodes")
    };

    // Act
    let output = run_constants(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output else {
        panic!("the constants are computed")
    };
    assert_eq!(
        entry_named(&success.entries, "G1_GENERATOR").value,
        SolidityConstantValue::Bytes(g1)
    );
    assert_eq!(
        entry_named(&success.entries, "G2_GENERATOR").value,
        SolidityConstantValue::Bytes(g2)
    );
}

/// Contract: whether the verifier has second-group arithmetic, read from the
///   pairing's declaration, is emitted as Bool.
/// Arrange: the default probe.
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks and
///   PairingConcrete::Bls12381Arkworks.
/// Assert:  SECOND_GROUP_ARITHMETIC holds Bool(false) on BN254 and Bool(true)
///   on BLS12-381.
#[test]
fn constants_declare_first_group_only_arithmetic_on_bn254_and_both_groups_on_bls12_381() {
    // Arrange
    let bn254_probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };
    let bls12_381_probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };

    // Act
    let bn254_output = run_constants(bn254_probe, PairingConcrete::Bn254Arkworks);
    let bls12_381_output = run_constants(bls12_381_probe, PairingConcrete::Bls12381Arkworks);

    // Assert
    let Ok(bn254_success) = bn254_output else {
        panic!("the constants are computed")
    };
    let Ok(bls12_381_success) = bls12_381_output else {
        panic!("the constants are computed")
    };
    assert_eq!(
        entry_named(&bn254_success.entries, "SECOND_GROUP_ARITHMETIC").value,
        SolidityConstantValue::Bool(false)
    );
    assert_eq!(
        entry_named(&bls12_381_success.entries, "SECOND_GROUP_ARITHMETIC").value,
        SolidityConstantValue::Bool(true)
    );
}

/// Contract: the entries are returned in the order the interaction spec lists
///   them.
/// Arrange: the default probe.
/// Act:     constants through the probe over PairingConcrete::Bn254Arkworks.
/// Assert:  the entries' names, in order, are IDENTITY_TAG, POSSESSION_G1_TAG,
///   POSSESSION_G2_TAG, CHALLENGE_TAG, WEIGHT_TAG, DELIVERY_STATEMENT_VERSION,
///   PURPOSE_MINT, PURPOSE_TRANSFER, PURPOSE_GRANT, PURPOSE_REPLACEMENT,
///   MINT_FIELDS, TRANSFER_FIELDS, SCALAR_FIELD_ORDER, G1_GENERATOR,
///   G2_GENERATOR, and SECOND_GROUP_ARITHMETIC.
#[test]
fn constants_list_their_entries_in_declared_order() {
    // Arrange
    let probe = ConstantsPairingProbe {
        params: build_constants_params(Default::default()),
        kem: build_kem_declaration(Default::default()),
        key_agreement: build_key_agreement_declaration(Default::default()),
        delivery_proof: build_delivery_proof_declaration(Default::default()),
    };
    let expected_names = [
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
    .into_iter()
    .map(|name| {
        build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
            text: Some(name.to_string()),
        })
    })
    .collect::<Vec<_>>();

    // Act
    let output = run_constants(probe, PairingConcrete::Bn254Arkworks);

    // Assert
    let Ok(success) = output else {
        panic!("the constants are computed")
    };
    let names = success
        .entries
        .iter()
        .map(|entry| entry.name.clone())
        .collect::<Vec<_>>();
    assert_eq!(names, expected_names);
}
