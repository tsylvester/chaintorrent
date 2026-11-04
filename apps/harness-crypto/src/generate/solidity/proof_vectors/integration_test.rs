#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{
    ParameterSetIdentifier, ParameterSetIdentifierConstructorParams,
    ParameterSetIdentifierConstructorParamsOverrides, SuiteIdentifier,
    SuiteIdentifierConstructorParams, SuiteIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier, build_secret, build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, CreateEncodingParams, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, ICanonicalField, IDecoderAdapter, IEncoderAdapter,
    IEncodingContract, ToFieldsParams, build_create_encoding_params, build_decode_params,
    build_encode_params, build_from_field_params,
};
use envelope::{
    CreateKeyAgreementParams, DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeComponents,
    GenerateKeysParams, GenerateKeysPayload, IKeyAgreementAdapter, KeyPairPublicKeysParams,
    KeyPairPublicKeysPayload, PublicKeysComponents,
};
use hash_to_scalar::{
    CreateHashToScalarParams, CreateHashToScalarParamsOverrides,
    DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter, build_create_hash_to_scalar_params,
};
use hashing::{Root, RootConstructorParams, RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParams, DECLARED_KEM_SELECTIONS, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, ParameterSetComponentsParams,
    ParameterSetComponentsPayload, SetupParamsOverrides, SetupPayloadOverrides,
    build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, DecodeG1Params, DecodeG2Params, DecodeScalarParams,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES, PairingConcrete,
    VerifierGroupArithmetic, build_create_pairing_params,
};
use proof::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_GRANT,
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_REPLACEMENT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryContext, DeliveryContextOverrides, DeliveryProofComponents, DeliveryProofConcrete,
    IDeliveryProofAdapter, MintPurpose, MintResponses, MintSecondGroupFirstMessages,
    ProofFromComponentsParams, ProofFromComponentsPayload, TransferPurpose, TransferResponses,
    TransferSecondGroupFirstMessages, VerifyParams, VerifyPayload,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::interface::{
    IProofVectorsCaseConsumer, MintProofVector, ProofVectorsCaseParams, ProofVectorsCasePayload,
    ProofVectorsErrorReturn, TransferProofVector,
};
use super::mock::{
    ProofVectorContextOverrides, ProofVectorsCaseParamsOverrides, ProofVectorsParamsOverrides,
    ProofVectorsPayloadOverrides, build_mint_proof_vector_description, build_proof_vector_context,
    build_proof_vectors_case_deps, build_proof_vectors_case_params,
    build_proof_vectors_case_payload, build_proof_vectors_deps, build_proof_vectors_params,
    build_proof_vectors_payload, build_transfer_proof_vector_description, run_proof_vectors_case,
};
use super::proof_vectors;
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};

fn case_params(
    pairing_concrete: PairingConcrete,
    hash_to_scalar: &CreateHashToScalarParams,
    encoding_selection: &CreateEncodingParams,
    forms_concrete: ChainFormsConcrete,
    kem: &CreateKemParams,
    key_agreement: &CreateKeyAgreementParams,
    delivery_proof: &DeliveryProofConcrete,
    random_kind: RandomSourceKind,
) -> ProofVectorsCaseParams {
    build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        })),
        hash_to_scalar: Some(build_create_hash_to_scalar_params(
            CreateHashToScalarParamsOverrides {
                concrete: Some(hash_to_scalar.concrete),
                identifier: Some(hash_to_scalar.identifier),
            },
        )),
        encoding: Some(build_create_encoding_params(
            CreateEncodingParamsOverrides {
                concrete: Some(encoding_selection.concrete),
                identifier: Some(encoding_selection.identifier),
            },
        )),
        chain_forms: Some(build_create_chain_forms_params(
            CreateChainFormsParamsOverrides {
                concrete: Some(forms_concrete),
                ..Default::default()
            },
        )),
        kem: Some(*kem),
        key_agreement: Some(*key_agreement),
        delivery_proof: Some(build_create_delivery_proof_params(
            CreateDeliveryProofParamsOverrides {
                concrete: Some(*delivery_proof),
                ..Default::default()
            },
        )),
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(random_kind),
            },
        )),
    })
}

fn run_every_case(kind: RandomSourceKind, run: &dyn Fn(ProofVectorsCaseParams) -> bool) {
    for pairing_concrete in PAIRING_CONCRETES {
        for hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for delivery_proof in DELIVERY_PROOF_CONCRETES {
                                assert!(run(case_params(
                                    *pairing_concrete,
                                    hash_to_scalar,
                                    encoding_selection,
                                    *forms_concrete,
                                    kem_selection,
                                    key_agreement,
                                    delivery_proof,
                                    kind.clone(),
                                )));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn case_payload<'a>(canonical: &'a [u8], root: &'a Root) -> ProofVectorsCasePayload<'a> {
    build_proof_vectors_case_payload(canonical, root, Default::default())
}

fn form_value(kind: CanonicalFieldKind, seed: u8) -> CanonicalFieldValue {
    match kind {
        CanonicalFieldKind::FixedBytes32 => CanonicalFieldValue::FixedBytes32([seed; 32]),
        CanonicalFieldKind::Unsigned16 => CanonicalFieldValue::Unsigned16(u16::from(seed)),
        CanonicalFieldKind::Unsigned32 => CanonicalFieldValue::Unsigned32(u32::from(seed)),
        CanonicalFieldKind::Unsigned64 => CanonicalFieldValue::Unsigned64(u64::from(seed)),
        CanonicalFieldKind::Text => CanonicalFieldValue::Text(format!("seed-{seed}")),
        CanonicalFieldKind::Bytes => CanonicalFieldValue::Bytes(vec![seed; 32]),
        CanonicalFieldKind::FixedBytes20 => CanonicalFieldValue::FixedBytes20([seed; 20]),
        CanonicalFieldKind::Unsigned256 => CanonicalFieldValue::Unsigned256([seed; 32]),
    }
}

/// Contract: `proof_vectors` over a case with a mint count above one, every
///   adapter real and the operating-system source real → every mint record
///   rebuilt from its bytes alone is a statement and proof `verify` accepts.
/// Arrange: `mint_count` of `2` and `transfer_count` of `1`, the entitlement
///   scope, the contexts, purposes, and identity at their defaults; every case,
///   each member of `PAIRING_CONCRETES` crossed with the selections the other
///   families declare, so both curves and both verifier forms run.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `mint` holds `2` records and each record's rebuild yields `is_valid`
///   true.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_mint_record_is_a_statement_and_proof_the_delivery_proof_verifies() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let g1 = |bytes: &[u8]| {
                pairing
                    .decode_g1(DecodeG1Params, bytes)
                    .expect("the first-group element decodes")
                    .point
            };
            let g2 = |bytes: &[u8]| {
                pairing
                    .decode_g2(DecodeG2Params, bytes)
                    .expect("the second-group element decodes")
                    .point
            };
            let scalar = |bytes: &[u8]| {
                pairing
                    .decode_scalar(DecodeScalarParams, bytes)
                    .expect("the scalar decodes")
                    .scalar
            };
            let g2_length = pairing
                .encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: pairing
                            .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                            .expect("the generator is reported")
                            .point,
                    },
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len();
            let verify_mint = |record: &MintProofVector, messages: &[u8]| -> bool {
                let context = DeliveryContext::<F, MintPurpose> {
                    suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                        identifier: record.context.suite_identifier,
                        version: record.context.suite_version,
                    })
                    .expect("the suite identifier rebuilds"),
                    chain: F::ChainIdentifier::from_field(
                        build_from_field_params(Default::default()),
                        record.context.chain.clone(),
                    )
                    .expect("the chain form admits the field")
                    .value,
                    entitlement_contract: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.entitlement_contract.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    asset_root: Root::try_new(RootConstructorParams {
                        bytes: record.context.asset_root,
                    })
                    .expect("the asset root rebuilds"),
                    parameter_set_identifier: ParameterSetIdentifier::try_new(
                        ParameterSetIdentifierConstructorParams {
                            bytes: record.context.parameter_set,
                        },
                    )
                    .expect("the parameter set identifier rebuilds"),
                    source_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.source_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    target_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.target_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    old_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.old_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    new_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.new_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    purpose: MintPurpose::try_from(record.context.purpose)
                        .expect("the purpose code is declared"),
                    seller: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.seller.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    buyer: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.buyer.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    expiry: record.context.expiry,
                };
                let statement = AlgebraicMintStatement {
                    context,
                    hpub: g2(&record.hpub),
                    identity_element: g1(&record.identity_element),
                    buyer_keys: PublicKeysComponents {
                        pk1: g1(&record.buyer_pk1),
                        pk2: g2(&record.buyer_pk2),
                    },
                    envelope: EnvelopeComponents {
                        c1: g1(&record.envelope_c1),
                        c2: g1(&record.envelope_c2),
                        d1: g2(&record.envelope_d1),
                        d2: g2(&record.envelope_d2),
                    },
                };
                let challenge = scalar(&record.challenge);
                let responses = MintResponses {
                    alpha: scalar(&record.alpha),
                    r: scalar(&record.r),
                    rho: scalar(&record.rho),
                    sigma: scalar(&record.sigma),
                };
                let components = if messages.is_empty() {
                    DeliveryProofComponents::MintBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let chunks: Vec<&[u8]> = messages.chunks(g2_length).collect();
                    DeliveryProofComponents::MintFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: MintSecondGroupFirstMessages {
                            hpub: g2(chunks[0]),
                            d1: g2(chunks[1]),
                            d2: g2(chunks[2]),
                        },
                    }
                };
                let proof = delivery_proof
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the components assemble")
                    .proof;
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the check returns")
                    .is_valid
            };
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");

            // Assert
            assert_eq!(success.mint.len(), 2);
            for record in &success.mint {
                assert!(verify_mint(record, &record.second_group_first_messages));
            }
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// every_mint_record_is_a_statement_and_proof_the_delivery_proof_verifies

/// Contract: `proof_vectors` over a case with a transfer count above one,
///   every adapter real and the operating-system source real → every transfer
///   record rebuilt from its bytes alone is a statement and proof `verify`
///   accepts.
/// Arrange: `mint_count` of `1` and `transfer_count` of `2`, the entitlement
///   scope, the contexts, purposes, and identity at their defaults; every case.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `transfer` holds `2` records and each record's rebuild yields
///   `is_valid` true, the old envelope, the new envelope, the seller's keys,
///   and the buyer's keys each taken from the record.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_transfer_record_is_a_statement_and_proof_the_delivery_proof_verifies() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let g1 = |bytes: &[u8]| {
                pairing
                    .decode_g1(DecodeG1Params, bytes)
                    .expect("the first-group element decodes")
                    .point
            };
            let g2 = |bytes: &[u8]| {
                pairing
                    .decode_g2(DecodeG2Params, bytes)
                    .expect("the second-group element decodes")
                    .point
            };
            let scalar = |bytes: &[u8]| {
                pairing
                    .decode_scalar(DecodeScalarParams, bytes)
                    .expect("the scalar decodes")
                    .scalar
            };
            let g2_length = pairing
                .encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: pairing
                            .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                            .expect("the generator is reported")
                            .point,
                    },
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len();
            let verify_transfer = |record: &TransferProofVector, messages: &[u8]| -> bool {
                let context = DeliveryContext::<F, TransferPurpose> {
                    suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                        identifier: record.context.suite_identifier,
                        version: record.context.suite_version,
                    })
                    .expect("the suite identifier rebuilds"),
                    chain: F::ChainIdentifier::from_field(
                        build_from_field_params(Default::default()),
                        record.context.chain.clone(),
                    )
                    .expect("the chain form admits the field")
                    .value,
                    entitlement_contract: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.entitlement_contract.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    asset_root: Root::try_new(RootConstructorParams {
                        bytes: record.context.asset_root,
                    })
                    .expect("the asset root rebuilds"),
                    parameter_set_identifier: ParameterSetIdentifier::try_new(
                        ParameterSetIdentifierConstructorParams {
                            bytes: record.context.parameter_set,
                        },
                    )
                    .expect("the parameter set identifier rebuilds"),
                    source_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.source_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    target_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.target_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    old_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.old_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    new_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.new_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    purpose: TransferPurpose::try_from(record.context.purpose)
                        .expect("the purpose code is declared"),
                    seller: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.seller.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    buyer: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.buyer.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    expiry: record.context.expiry,
                };
                let statement = AlgebraicTransferStatement {
                    context,
                    identity_element: g1(&record.identity_element),
                    seller_keys: PublicKeysComponents {
                        pk1: g1(&record.seller_pk1),
                        pk2: g2(&record.seller_pk2),
                    },
                    buyer_keys: PublicKeysComponents {
                        pk1: g1(&record.buyer_pk1),
                        pk2: g2(&record.buyer_pk2),
                    },
                    old_envelope: EnvelopeComponents {
                        c1: g1(&record.old_envelope_c1),
                        c2: g1(&record.old_envelope_c2),
                        d1: g2(&record.old_envelope_d1),
                        d2: g2(&record.old_envelope_d2),
                    },
                    new_envelope: EnvelopeComponents {
                        c1: g1(&record.new_envelope_c1),
                        c2: g1(&record.new_envelope_c2),
                        d1: g2(&record.new_envelope_d1),
                        d2: g2(&record.new_envelope_d2),
                    },
                };
                let challenge = scalar(&record.challenge);
                let responses = TransferResponses {
                    x: scalar(&record.x),
                    y: scalar(&record.y),
                    s: scalar(&record.s),
                    rho: scalar(&record.rho),
                    sigma: scalar(&record.sigma),
                };
                let components = if messages.is_empty() {
                    DeliveryProofComponents::TransferBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let chunks: Vec<&[u8]> = messages.chunks(g2_length).collect();
                    DeliveryProofComponents::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: TransferSecondGroupFirstMessages {
                            pk2: g2(chunks[0]),
                            d1: g2(chunks[1]),
                            d2: g2(chunks[2]),
                        },
                    }
                };
                let proof = delivery_proof
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the components assemble")
                    .proof;
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the check returns")
                    .is_valid
            };
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(2) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");

            // Assert
            assert_eq!(success.transfer.len(), 2);
            for record in &success.transfer {
                assert!(verify_transfer(record, &record.second_group_first_messages));
            }
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// every_transfer_record_is_a_statement_and_proof_the_delivery_proof_verifies

/// Contract: `proof_vectors` over an admitted identity in the asset forms over
///   one `Root` → `Ok`, every record rebuilt and verified.
/// Arrange: the KEM selection under `IdentityScope::Asset`, the root
///   `build_root` builds from the bytes `[0x44; 32]`, `mint_count` and
///   `transfer_count` of `1`; every case that declares the asset scope.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result is `Ok` and the mint record's and the transfer record's
///   rebuilds each yield `is_valid` true.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn an_asset_identity_is_proved_under_the_asset_scope() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let g1 = |bytes: &[u8]| {
                pairing
                    .decode_g1(DecodeG1Params, bytes)
                    .expect("the first-group element decodes")
                    .point
            };
            let g2 = |bytes: &[u8]| {
                pairing
                    .decode_g2(DecodeG2Params, bytes)
                    .expect("the second-group element decodes")
                    .point
            };
            let scalar = |bytes: &[u8]| {
                pairing
                    .decode_scalar(DecodeScalarParams, bytes)
                    .expect("the scalar decodes")
                    .scalar
            };
            let g2_length = pairing
                .encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: pairing
                            .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                            .expect("the generator is reported")
                            .point,
                    },
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len();
            let verify_mint = |record: &MintProofVector, messages: &[u8]| -> bool {
                let context = DeliveryContext::<F, MintPurpose> {
                    suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                        identifier: record.context.suite_identifier,
                        version: record.context.suite_version,
                    })
                    .expect("the suite identifier rebuilds"),
                    chain: F::ChainIdentifier::from_field(
                        build_from_field_params(Default::default()),
                        record.context.chain.clone(),
                    )
                    .expect("the chain form admits the field")
                    .value,
                    entitlement_contract: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.entitlement_contract.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    asset_root: Root::try_new(RootConstructorParams {
                        bytes: record.context.asset_root,
                    })
                    .expect("the asset root rebuilds"),
                    parameter_set_identifier: ParameterSetIdentifier::try_new(
                        ParameterSetIdentifierConstructorParams {
                            bytes: record.context.parameter_set,
                        },
                    )
                    .expect("the parameter set identifier rebuilds"),
                    source_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.source_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    target_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.target_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    old_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.old_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    new_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.new_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    purpose: MintPurpose::try_from(record.context.purpose)
                        .expect("the purpose code is declared"),
                    seller: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.seller.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    buyer: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.buyer.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    expiry: record.context.expiry,
                };
                let statement = AlgebraicMintStatement {
                    context,
                    hpub: g2(&record.hpub),
                    identity_element: g1(&record.identity_element),
                    buyer_keys: PublicKeysComponents {
                        pk1: g1(&record.buyer_pk1),
                        pk2: g2(&record.buyer_pk2),
                    },
                    envelope: EnvelopeComponents {
                        c1: g1(&record.envelope_c1),
                        c2: g1(&record.envelope_c2),
                        d1: g2(&record.envelope_d1),
                        d2: g2(&record.envelope_d2),
                    },
                };
                let challenge = scalar(&record.challenge);
                let responses = MintResponses {
                    alpha: scalar(&record.alpha),
                    r: scalar(&record.r),
                    rho: scalar(&record.rho),
                    sigma: scalar(&record.sigma),
                };
                let components = if messages.is_empty() {
                    DeliveryProofComponents::MintBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let chunks: Vec<&[u8]> = messages.chunks(g2_length).collect();
                    DeliveryProofComponents::MintFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: MintSecondGroupFirstMessages {
                            hpub: g2(chunks[0]),
                            d1: g2(chunks[1]),
                            d2: g2(chunks[2]),
                        },
                    }
                };
                let proof = delivery_proof
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the components assemble")
                    .proof;
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the check returns")
                    .is_valid
            };
            let verify_transfer = |record: &TransferProofVector, messages: &[u8]| -> bool {
                let context = DeliveryContext::<F, TransferPurpose> {
                    suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                        identifier: record.context.suite_identifier,
                        version: record.context.suite_version,
                    })
                    .expect("the suite identifier rebuilds"),
                    chain: F::ChainIdentifier::from_field(
                        build_from_field_params(Default::default()),
                        record.context.chain.clone(),
                    )
                    .expect("the chain form admits the field")
                    .value,
                    entitlement_contract: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.entitlement_contract.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    asset_root: Root::try_new(RootConstructorParams {
                        bytes: record.context.asset_root,
                    })
                    .expect("the asset root rebuilds"),
                    parameter_set_identifier: ParameterSetIdentifier::try_new(
                        ParameterSetIdentifierConstructorParams {
                            bytes: record.context.parameter_set,
                        },
                    )
                    .expect("the parameter set identifier rebuilds"),
                    source_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.source_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    target_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.target_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    old_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.old_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    new_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.new_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    purpose: TransferPurpose::try_from(record.context.purpose)
                        .expect("the purpose code is declared"),
                    seller: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.seller.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    buyer: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.buyer.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    expiry: record.context.expiry,
                };
                let statement = AlgebraicTransferStatement {
                    context,
                    identity_element: g1(&record.identity_element),
                    seller_keys: PublicKeysComponents {
                        pk1: g1(&record.seller_pk1),
                        pk2: g2(&record.seller_pk2),
                    },
                    buyer_keys: PublicKeysComponents {
                        pk1: g1(&record.buyer_pk1),
                        pk2: g2(&record.buyer_pk2),
                    },
                    old_envelope: EnvelopeComponents {
                        c1: g1(&record.old_envelope_c1),
                        c2: g1(&record.old_envelope_c2),
                        d1: g2(&record.old_envelope_d1),
                        d2: g2(&record.old_envelope_d2),
                    },
                    new_envelope: EnvelopeComponents {
                        c1: g1(&record.new_envelope_c1),
                        c2: g1(&record.new_envelope_c2),
                        d1: g2(&record.new_envelope_d1),
                        d2: g2(&record.new_envelope_d2),
                    },
                };
                let challenge = scalar(&record.challenge);
                let responses = TransferResponses {
                    x: scalar(&record.x),
                    y: scalar(&record.y),
                    s: scalar(&record.s),
                    rho: scalar(&record.rho),
                    sigma: scalar(&record.sigma),
                };
                let components = if messages.is_empty() {
                    DeliveryProofComponents::TransferBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let chunks: Vec<&[u8]> = messages.chunks(g2_length).collect();
                    DeliveryProofComponents::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: TransferSecondGroupFirstMessages {
                            pk2: g2(chunks[0]),
                            d1: g2(chunks[1]),
                            d2: g2(chunks[2]),
                        },
                    }
                };
                let proof = delivery_proof
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the components assemble")
                    .proof;
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the check returns")
                    .is_valid
            };
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let result = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            let success = result.expect("the asset identity is proved");
            assert!(verify_mint(
                &success.mint[0],
                &success.mint[0].second_group_first_messages
            ));
            assert!(verify_transfer(
                &success.transfer[0],
                &success.transfer[0].second_group_first_messages
            ));
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        if params.kem.scope != IdentityScope::Asset {
            return true;
        }
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(
                b"pkg@1.0.0#1",
                &build_root(RootConstructorParamsOverrides {
                    bytes: Some([0x44; 32]),
                }),
            ),
        )
        .is_ok()
    });
}
// an_asset_identity_is_proved_under_the_asset_scope

/// Contract: each record's `draws`, split at `UNIFORM_BYTES_LENGTH` into the
///   uniforms of its calls in draw order and replayed through `setup` under the
///   admitted identity's setup scope, `derive_identity` over its identity, and
///   `generate_keys` on the case's adapters, reproduce the record's `hpub`,
///   `identity_element`, and key fields.
/// Arrange: `mint_count` and `transfer_count` of `1`, the entitlement scope,
///   each case run under each of the admitted identities over the canonical
///   bytes `b"pkg@1.0.0#1"` and `b"pkg@2.0.0#7"`, so a replay over an identity
///   other than the admitted one reports otherwise.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the replayed parameter set's `hpub`, the replayed identity
///   element's `element`, and each replayed key pair's `pk1` and `pk2`, each
///   encoded through the received pairing, equal the mint record's `hpub`,
///   `identity_element`, `buyer_pk1`, and `buyer_pk2` and the transfer record's
///   `identity_element`, `seller_pk1`, `seller_pk2`, `buyer_pk1`, and
///   `buyer_pk2`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_statement_elements_it_produced() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw =
                |draws: &[u8], index: usize| draws[index * uniform..(index + 1) * uniform].to_vec();
            let g1_bytes = |point: &P::G1| {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group element encodes")
                    .bytes
            };
            let g2_bytes = |point: &P::G2| {
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the second-group element encodes")
                    .bytes
            };
            let replay_element = |draws: &[u8]| {
                let parameter_set = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(identity.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                            master_uniform: Some(build_secret(draw(draws, 0), Default::default())),
                            u0_uniform: Some(build_secret(draw(draws, 1), Default::default())),
                            u1_uniform: Some(build_secret(draw(draws, 2), Default::default())),
                        }),
                    )
                    .expect("the setup replays")
                    .parameter_set;
                let identity_element = kem
                    .derive_identity(
                        build_derive_identity_params(Default::default()),
                        build_derive_identity_payload(
                            &parameter_set,
                            DeriveIdentityPayloadOverrides {
                                identity: Some(identity.identity),
                            },
                        ),
                    )
                    .expect("the identity derives")
                    .identity_element;
                (
                    parameter_set,
                    kem.identity_element_components(
                        build_identity_element_components_params(Default::default()),
                        build_identity_element_components_payload(
                            &identity_element,
                            Default::default(),
                        ),
                    )
                    .expect("the element opens")
                    .components
                    .element,
                )
            };
            let replay_keys = |x: Vec<u8>, y: Vec<u8>| {
                let generated = key_agreement
                    .generate_keys(
                        GenerateKeysParams,
                        GenerateKeysPayload {
                            x_uniform: build_secret(x, Default::default()),
                            y_uniform: build_secret(y, Default::default()),
                        },
                    )
                    .expect("the keys generate");
                key_agreement
                    .key_pair_public_keys(
                        KeyPairPublicKeysParams,
                        KeyPairPublicKeysPayload {
                            key_pair: &generated.key_pair,
                        },
                    )
                    .expect("the keys open")
                    .components
            };
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");

            // Assert
            let mint_record = &success.mint[0];
            let (parameter_set, element) = replay_element(&mint_record.draws);
            let parameter_set_components = kem
                .parameter_set_components(
                    ParameterSetComponentsParams,
                    ParameterSetComponentsPayload {
                        parameter_set: &parameter_set,
                    },
                )
                .expect("the parameter set opens")
                .components;
            assert_eq!(
                mint_record.hpub.as_slice(),
                g2_bytes(&parameter_set_components.hpub).as_ref()
            );
            assert_eq!(
                mint_record.identity_element.as_slice(),
                g1_bytes(&element).as_ref()
            );
            let buyer_keys = replay_keys(draw(&mint_record.draws, 4), draw(&mint_record.draws, 5));
            assert_eq!(
                mint_record.buyer_pk1.as_slice(),
                g1_bytes(&buyer_keys.pk1).as_ref()
            );
            assert_eq!(
                mint_record.buyer_pk2.as_slice(),
                g2_bytes(&buyer_keys.pk2).as_ref()
            );

            let transfer_record = &success.transfer[0];
            let (_parameter_set, element) = replay_element(&transfer_record.draws);
            assert_eq!(
                transfer_record.identity_element.as_slice(),
                g1_bytes(&element).as_ref()
            );
            let seller_keys = replay_keys(
                draw(&transfer_record.draws, 4),
                draw(&transfer_record.draws, 5),
            );
            assert_eq!(
                transfer_record.seller_pk1.as_slice(),
                g1_bytes(&seller_keys.pk1).as_ref()
            );
            assert_eq!(
                transfer_record.seller_pk2.as_slice(),
                g2_bytes(&seller_keys.pk2).as_ref()
            );
            let buyer_keys = replay_keys(
                draw(&transfer_record.draws, 6),
                draw(&transfer_record.draws, 7),
            );
            assert_eq!(
                transfer_record.buyer_pk1.as_slice(),
                g1_bytes(&buyer_keys.pk1).as_ref()
            );
            assert_eq!(
                transfer_record.buyer_pk2.as_slice(),
                g2_bytes(&buyer_keys.pk2).as_ref()
            );
        }
    }

    for canonical in [b"pkg@1.0.0#1".as_slice(), b"pkg@2.0.0#7".as_slice()] {
        run_every_case(RandomSourceKind::OperatingSystem, &|params| {
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                params,
                case_payload(canonical, &build_root(Default::default())),
            )
            .is_ok()
        });
    }
}
// every_record_draw_replays_to_the_statement_elements_it_produced

/// Contract: under a pairing whose declaration names the first-group-only
///   verifier form each record's `second_group_first_messages` holds three
///   second-group point encodings and the record with one byte of them changed
///   is refused by `verify`; under both groups it is empty.
/// Arrange: `mint_count` and `transfer_count` of `1`; every member of
///   `PAIRING_CONCRETES`, which holds both forms.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: under the first-group-only form each record's
///   `second_group_first_messages` is as long as `3` times the `encode_g2` of
///   the second-group generator and its rebuild with one byte changed yields
///   `is_valid` false; under both groups it equals `Vec::new()`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_record_carries_second_group_first_messages_exactly_in_the_first_group_only_form() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let g1 = |bytes: &[u8]| {
                pairing
                    .decode_g1(DecodeG1Params, bytes)
                    .expect("the first-group element decodes")
                    .point
            };
            let g2 = |bytes: &[u8]| {
                pairing
                    .decode_g2(DecodeG2Params, bytes)
                    .expect("the second-group element decodes")
                    .point
            };
            let scalar = |bytes: &[u8]| {
                pairing
                    .decode_scalar(DecodeScalarParams, bytes)
                    .expect("the scalar decodes")
                    .scalar
            };
            let g2_length = pairing
                .encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: pairing
                            .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                            .expect("the generator is reported")
                            .point,
                    },
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len();
            let verify_mint = |record: &MintProofVector, messages: &[u8]| -> bool {
                let context = DeliveryContext::<F, MintPurpose> {
                    suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                        identifier: record.context.suite_identifier,
                        version: record.context.suite_version,
                    })
                    .expect("the suite identifier rebuilds"),
                    chain: F::ChainIdentifier::from_field(
                        build_from_field_params(Default::default()),
                        record.context.chain.clone(),
                    )
                    .expect("the chain form admits the field")
                    .value,
                    entitlement_contract: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.entitlement_contract.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    asset_root: Root::try_new(RootConstructorParams {
                        bytes: record.context.asset_root,
                    })
                    .expect("the asset root rebuilds"),
                    parameter_set_identifier: ParameterSetIdentifier::try_new(
                        ParameterSetIdentifierConstructorParams {
                            bytes: record.context.parameter_set,
                        },
                    )
                    .expect("the parameter set identifier rebuilds"),
                    source_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.source_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    target_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.target_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    old_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.old_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    new_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.new_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    purpose: MintPurpose::try_from(record.context.purpose)
                        .expect("the purpose code is declared"),
                    seller: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.seller.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    buyer: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.buyer.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    expiry: record.context.expiry,
                };
                let statement = AlgebraicMintStatement {
                    context,
                    hpub: g2(&record.hpub),
                    identity_element: g1(&record.identity_element),
                    buyer_keys: PublicKeysComponents {
                        pk1: g1(&record.buyer_pk1),
                        pk2: g2(&record.buyer_pk2),
                    },
                    envelope: EnvelopeComponents {
                        c1: g1(&record.envelope_c1),
                        c2: g1(&record.envelope_c2),
                        d1: g2(&record.envelope_d1),
                        d2: g2(&record.envelope_d2),
                    },
                };
                let challenge = scalar(&record.challenge);
                let responses = MintResponses {
                    alpha: scalar(&record.alpha),
                    r: scalar(&record.r),
                    rho: scalar(&record.rho),
                    sigma: scalar(&record.sigma),
                };
                let components = if messages.is_empty() {
                    DeliveryProofComponents::MintBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let mut points = Vec::new();
                    for chunk in messages.chunks(g2_length) {
                        match pairing.decode_g2(DecodeG2Params, chunk) {
                            Ok(success) => points.push(success.point),
                            Err(_error) => return false,
                        }
                    }
                    if points.len() != 3 {
                        return false;
                    }
                    DeliveryProofComponents::MintFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: MintSecondGroupFirstMessages {
                            hpub: points[0].clone(),
                            d1: points[1].clone(),
                            d2: points[2].clone(),
                        },
                    }
                };
                let proof = delivery_proof
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the components assemble")
                    .proof;
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the check returns")
                    .is_valid
            };
            let verify_transfer = |record: &TransferProofVector, messages: &[u8]| -> bool {
                let context = DeliveryContext::<F, TransferPurpose> {
                    suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                        identifier: record.context.suite_identifier,
                        version: record.context.suite_version,
                    })
                    .expect("the suite identifier rebuilds"),
                    chain: F::ChainIdentifier::from_field(
                        build_from_field_params(Default::default()),
                        record.context.chain.clone(),
                    )
                    .expect("the chain form admits the field")
                    .value,
                    entitlement_contract: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.entitlement_contract.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    asset_root: Root::try_new(RootConstructorParams {
                        bytes: record.context.asset_root,
                    })
                    .expect("the asset root rebuilds"),
                    parameter_set_identifier: ParameterSetIdentifier::try_new(
                        ParameterSetIdentifierConstructorParams {
                            bytes: record.context.parameter_set,
                        },
                    )
                    .expect("the parameter set identifier rebuilds"),
                    source_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.source_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    target_entitlement: F::Entitlement::from_field(
                        build_from_field_params(Default::default()),
                        record.context.target_entitlement.clone(),
                    )
                    .expect("the entitlement form admits the field")
                    .value,
                    old_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.old_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    new_interval: F::Interval::from_field(
                        build_from_field_params(Default::default()),
                        record.context.new_interval.clone(),
                    )
                    .expect("the interval form admits the field")
                    .value,
                    purpose: TransferPurpose::try_from(record.context.purpose)
                        .expect("the purpose code is declared"),
                    seller: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.seller.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    buyer: F::Identity::from_field(
                        build_from_field_params(Default::default()),
                        record.context.buyer.clone(),
                    )
                    .expect("the identity form admits the field")
                    .value,
                    expiry: record.context.expiry,
                };
                let statement = AlgebraicTransferStatement {
                    context,
                    identity_element: g1(&record.identity_element),
                    seller_keys: PublicKeysComponents {
                        pk1: g1(&record.seller_pk1),
                        pk2: g2(&record.seller_pk2),
                    },
                    buyer_keys: PublicKeysComponents {
                        pk1: g1(&record.buyer_pk1),
                        pk2: g2(&record.buyer_pk2),
                    },
                    old_envelope: EnvelopeComponents {
                        c1: g1(&record.old_envelope_c1),
                        c2: g1(&record.old_envelope_c2),
                        d1: g2(&record.old_envelope_d1),
                        d2: g2(&record.old_envelope_d2),
                    },
                    new_envelope: EnvelopeComponents {
                        c1: g1(&record.new_envelope_c1),
                        c2: g1(&record.new_envelope_c2),
                        d1: g2(&record.new_envelope_d1),
                        d2: g2(&record.new_envelope_d2),
                    },
                };
                let challenge = scalar(&record.challenge);
                let responses = TransferResponses {
                    x: scalar(&record.x),
                    y: scalar(&record.y),
                    s: scalar(&record.s),
                    rho: scalar(&record.rho),
                    sigma: scalar(&record.sigma),
                };
                let components = if messages.is_empty() {
                    DeliveryProofComponents::TransferBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let mut points = Vec::new();
                    for chunk in messages.chunks(g2_length) {
                        match pairing.decode_g2(DecodeG2Params, chunk) {
                            Ok(success) => points.push(success.point),
                            Err(_error) => return false,
                        }
                    }
                    if points.len() != 3 {
                        return false;
                    }
                    DeliveryProofComponents::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: TransferSecondGroupFirstMessages {
                            pk2: points[0].clone(),
                            d1: points[1].clone(),
                            d2: points[2].clone(),
                        },
                    }
                };
                let proof = delivery_proof
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the components assemble")
                    .proof;
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the check returns")
                    .is_valid
            };
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");

            // Assert
            match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::FirstGroupOnly => {
                    for record in &success.mint {
                        assert_eq!(record.second_group_first_messages.len(), 3 * g2_length);
                        let mut changed = record.second_group_first_messages.clone();
                        *changed.last_mut().expect("the encodings are nonempty") ^= 0x01;
                        assert!(!verify_mint(record, &changed));
                    }
                    for record in &success.transfer {
                        assert_eq!(record.second_group_first_messages.len(), 3 * g2_length);
                        let mut changed = record.second_group_first_messages.clone();
                        *changed.last_mut().expect("the encodings are nonempty") ^= 0x01;
                        assert!(!verify_transfer(record, &changed));
                    }
                }
                VerifierGroupArithmetic::BothGroups => {
                    for record in &success.mint {
                        assert_eq!(record.second_group_first_messages, Vec::<u8>::new());
                    }
                    for record in &success.transfer {
                        assert_eq!(record.second_group_first_messages, Vec::<u8>::new());
                    }
                }
            }
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// every_record_carries_second_group_first_messages_exactly_in_the_first_group_only_form

/// Contract: every first-group point of a record is as long as the `encode_g1`
///   of the first-group generator, every second-group point as long as the
///   `encode_g2` of the second-group generator, and every scalar field 32
///   bytes.
/// Arrange: `mint_count` and `transfer_count` of `1`; every member of
///   `PAIRING_CONCRETES`, whose encoded lengths differ between the curves.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `identity_element`, `buyer_pk1`, `envelope_c1`,
///   and `envelope_c2` and the transfer record's `identity_element`,
///   `seller_pk1`, `buyer_pk1`, and both envelopes' `c1` and `c2` each hold the
///   length of the first-group generator's encoding; the mint record's `hpub`,
///   `buyer_pk2`, `envelope_d1`, and `envelope_d2` and the transfer record's
///   `seller_pk2`, `buyer_pk2`, and both envelopes' `d1` and `d2` each hold the
///   length of the second-group generator's encoding; the mint record's
///   `challenge`, `alpha`, `r`, `rho`, and `sigma` and the transfer record's
///   `challenge`, `x`, `y`, `s`, `rho`, and `sigma` each hold `32` bytes; the
///   generators' encodings are read through the pairing's generator operations
///   and `encode_g1` and `encode_g2`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_record_encodes_its_elements_in_the_selected_pairings_encodings() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let g1_length = pairing
                .encode_g1(
                    EncodeG1Params,
                    EncodeG1Payload {
                        point: pairing
                            .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                            .expect("the generator is reported")
                            .point,
                    },
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len();
            let g2_length = pairing
                .encode_g2(
                    EncodeG2Params,
                    EncodeG2Payload {
                        point: pairing
                            .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                            .expect("the generator is reported")
                            .point,
                    },
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len();
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");

            // Assert
            for record in &success.mint {
                for field in [
                    &record.identity_element,
                    &record.buyer_pk1,
                    &record.envelope_c1,
                    &record.envelope_c2,
                ] {
                    assert_eq!(field.len(), g1_length);
                }
                for field in [
                    &record.hpub,
                    &record.buyer_pk2,
                    &record.envelope_d1,
                    &record.envelope_d2,
                ] {
                    assert_eq!(field.len(), g2_length);
                }
                for scalar in [
                    record.challenge,
                    record.alpha,
                    record.r,
                    record.rho,
                    record.sigma,
                ] {
                    assert_eq!(scalar.len(), 32);
                }
            }
            for record in &success.transfer {
                for field in [
                    &record.identity_element,
                    &record.seller_pk1,
                    &record.buyer_pk1,
                    &record.old_envelope_c1,
                    &record.old_envelope_c2,
                    &record.new_envelope_c1,
                    &record.new_envelope_c2,
                ] {
                    assert_eq!(field.len(), g1_length);
                }
                for field in [
                    &record.seller_pk2,
                    &record.buyer_pk2,
                    &record.old_envelope_d1,
                    &record.old_envelope_d2,
                    &record.new_envelope_d1,
                    &record.new_envelope_d2,
                ] {
                    assert_eq!(field.len(), g2_length);
                }
                for scalar in [
                    record.challenge,
                    record.x,
                    record.y,
                    record.s,
                    record.rho,
                    record.sigma,
                ] {
                    assert_eq!(scalar.len(), 32);
                }
            }
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// every_record_encodes_its_elements_in_the_selected_pairings_encodings

/// Contract: each record's context holds its typed context's values, each
///   form-typed field the `to_field` of the form its typed context holds.
/// Arrange: `mint_count` and `transfer_count` of `1`, the mint context and the
///   transfer context each overriding every form-typed field with the form
///   `from_field` builds from a value distinct from every other field's in
///   both contexts, of the kind the case's forms declare, `suite_identifier`,
///   `asset_root`, and `parameter_set_identifier` with distinct values from
///   `build_suite_identifier`, `build_root`, and
///   `build_parameter_set_identifier`, and distinct purposes; every case over
///   `CHAIN_FORMS_CONCRETES`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: each record's `context` equals `build_proof_vector_context`
///   overriding the form-typed fields and `expiry` with the context's values,
///   `purpose` with the purpose's code, and `suite_identifier`,
///   `suite_version`, `asset_root`, and `parameter_set` with the literal
///   values the arrangement built the domain values from.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_record_context_holds_the_fields_of_its_typed_context() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let form = |kind, seed| form_value(kind, seed);
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_REPLACEMENT,
                DeliveryContextOverrides {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x51; 32]),
                            version: Some(2),
                        },
                    )),
                    chain: Some(
                        F::ChainIdentifier::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::ChainIdentifier as ICanonicalField>::KIND, 0x21),
                        )
                        .expect("the chain form admits the field")
                        .value,
                    ),
                    entitlement_contract: Some(
                        F::Identity::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Identity as ICanonicalField>::KIND, 0x22),
                        )
                        .expect("the identity form admits the field")
                        .value,
                    ),
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x52; 32]),
                    })),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x53; 32]),
                        },
                    )),
                    source_entitlement: Some(
                        F::Entitlement::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Entitlement as ICanonicalField>::KIND, 0x23),
                        )
                        .expect("the entitlement form admits the field")
                        .value,
                    ),
                    target_entitlement: Some(
                        F::Entitlement::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Entitlement as ICanonicalField>::KIND, 0x24),
                        )
                        .expect("the entitlement form admits the field")
                        .value,
                    ),
                    old_interval: Some(
                        F::Interval::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Interval as ICanonicalField>::KIND, 0x25),
                        )
                        .expect("the interval form admits the field")
                        .value,
                    ),
                    new_interval: Some(
                        F::Interval::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Interval as ICanonicalField>::KIND, 0x26),
                        )
                        .expect("the interval form admits the field")
                        .value,
                    ),
                    seller: Some(
                        F::Identity::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Identity as ICanonicalField>::KIND, 0x27),
                        )
                        .expect("the identity form admits the field")
                        .value,
                    ),
                    buyer: Some(
                        F::Identity::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Identity as ICanonicalField>::KIND, 0x28),
                        )
                        .expect("the identity form admits the field")
                        .value,
                    ),
                    expiry: Some(1_800_000_001),
                },
            );
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                DeliveryContextOverrides {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x54; 32]),
                            version: Some(3),
                        },
                    )),
                    chain: Some(
                        F::ChainIdentifier::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::ChainIdentifier as ICanonicalField>::KIND, 0x31),
                        )
                        .expect("the chain form admits the field")
                        .value,
                    ),
                    entitlement_contract: Some(
                        F::Identity::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Identity as ICanonicalField>::KIND, 0x32),
                        )
                        .expect("the identity form admits the field")
                        .value,
                    ),
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x55; 32]),
                    })),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x56; 32]),
                        },
                    )),
                    source_entitlement: Some(
                        F::Entitlement::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Entitlement as ICanonicalField>::KIND, 0x33),
                        )
                        .expect("the entitlement form admits the field")
                        .value,
                    ),
                    target_entitlement: Some(
                        F::Entitlement::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Entitlement as ICanonicalField>::KIND, 0x34),
                        )
                        .expect("the entitlement form admits the field")
                        .value,
                    ),
                    old_interval: Some(
                        F::Interval::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Interval as ICanonicalField>::KIND, 0x35),
                        )
                        .expect("the interval form admits the field")
                        .value,
                    ),
                    new_interval: Some(
                        F::Interval::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Interval as ICanonicalField>::KIND, 0x36),
                        )
                        .expect("the interval form admits the field")
                        .value,
                    ),
                    seller: Some(
                        F::Identity::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Identity as ICanonicalField>::KIND, 0x37),
                        )
                        .expect("the identity form admits the field")
                        .value,
                    ),
                    buyer: Some(
                        F::Identity::from_field(
                            build_from_field_params(Default::default()),
                            form(<F::Identity as ICanonicalField>::KIND, 0x38),
                        )
                        .expect("the identity form admits the field")
                        .value,
                    ),
                    expiry: Some(1_900_000_002),
                },
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");

            // Assert
            assert_eq!(
                success.mint[0].context,
                build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some([0x51; 32]),
                    suite_version: Some(2),
                    chain: Some(form(<F::ChainIdentifier as ICanonicalField>::KIND, 0x21)),
                    entitlement_contract: Some(form(<F::Identity as ICanonicalField>::KIND, 0x22)),
                    asset_root: Some([0x52; 32]),
                    parameter_set: Some([0x53; 32]),
                    source_entitlement: Some(form(<F::Entitlement as ICanonicalField>::KIND, 0x23)),
                    target_entitlement: Some(form(<F::Entitlement as ICanonicalField>::KIND, 0x24)),
                    old_interval: Some(form(<F::Interval as ICanonicalField>::KIND, 0x25)),
                    new_interval: Some(form(<F::Interval as ICanonicalField>::KIND, 0x26)),
                    purpose: Some(4),
                    seller: Some(form(<F::Identity as ICanonicalField>::KIND, 0x27)),
                    buyer: Some(form(<F::Identity as ICanonicalField>::KIND, 0x28)),
                    expiry: Some(1_800_000_001),
                }),
            );
            assert_eq!(
                success.transfer[0].context,
                build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some([0x54; 32]),
                    suite_version: Some(3),
                    chain: Some(form(<F::ChainIdentifier as ICanonicalField>::KIND, 0x31)),
                    entitlement_contract: Some(form(<F::Identity as ICanonicalField>::KIND, 0x32)),
                    asset_root: Some([0x55; 32]),
                    parameter_set: Some([0x56; 32]),
                    source_entitlement: Some(form(<F::Entitlement as ICanonicalField>::KIND, 0x33)),
                    target_entitlement: Some(form(<F::Entitlement as ICanonicalField>::KIND, 0x34)),
                    old_interval: Some(form(<F::Interval as ICanonicalField>::KIND, 0x35)),
                    new_interval: Some(form(<F::Interval as ICanonicalField>::KIND, 0x36)),
                    purpose: Some(3),
                    seller: Some(form(<F::Identity as ICanonicalField>::KIND, 0x37)),
                    buyer: Some(form(<F::Identity as ICanonicalField>::KIND, 0x38)),
                    expiry: Some(1_900_000_002),
                }),
            );
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// every_record_context_holds_the_fields_of_its_typed_context

/// Contract: the lists hold the configured counts, each mint record's `draws`
///   hold the uniforms of its calls and each transfer record's `draws` those
///   of its calls, each `UNIFORM_BYTES_LENGTH` long, and no two records of
///   either list share `draws`.
/// Arrange: `mint_count` of `2` and `transfer_count` of `3`; every member of
///   `PAIRING_CONCRETES`, whose sampling bounds may differ.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `mint` holds `2` records whose `draws` each hold `6` times
///   `UNIFORM_BYTES_LENGTH` bytes, `transfer` holds `3` records whose `draws`
///   each hold `9` times that length, and no two records of either list share
///   `draws`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(3) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");

            // Assert
            assert_eq!(success.mint.len(), 2);
            assert_eq!(success.transfer.len(), 3);
            for record in &success.mint {
                assert_eq!(record.draws.len(), 6 * uniform);
            }
            for record in &success.transfer {
                assert_eq!(record.draws.len(), 9 * uniform);
            }
            for (left, record) in success.mint.iter().enumerate() {
                for other in success.mint.iter().skip(left + 1) {
                    assert_ne!(record.draws, other.draws);
                }
            }
            for (left, record) in success.transfer.iter().enumerate() {
                for other in success.transfer.iter().skip(left + 1) {
                    assert_ne!(record.draws, other.draws);
                }
            }
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// every_run_emits_the_configured_count_of_records_with_distinct_draws

/// Contract: a mint record from `proof_vectors`, encoded under
///   `MintProofVectorDescription` by each concrete of
///   `DECLARED_ENCODING_SELECTIONS` and decoded under the same description,
///   equals itself.
/// Arrange: `mint_count` and `transfer_count` of `1`; every case, each
///   encoding selection crossed with each member of `PAIRING_CONCRETES`, the
///   description built over the case's forms.
/// Act:    `encode` of the encoder the consumer received, over the record,
///   with `build_encode_params`.
/// Assert: `decode` of the returned bytes, with `build_decode_params`, returns
///   a described record equal to the mint record, its form-typed context
///   fields holding the values their forms rebuilt.
/// Boundary: the description's `to_fields` and `fields_to_value` through the
///   real encoder, after `proof_vectors`.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn a_mint_record_survives_its_description_round_trip() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let description = build_mint_proof_vector_description::<F>(Default::default());
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");
            let bytes = encoder
                .encode(build_encode_params(&description), &success.mint[0])
                .expect("the record encodes")
                .bytes;
            let described = decoder
                .decode(build_decode_params(&description), &bytes)
                .expect("the bytes decode")
                .described;

            // Assert
            assert_eq!(described, success.mint[0]);
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// a_mint_record_survives_its_description_round_trip

/// Contract: a transfer record from `proof_vectors`, encoded under
///   `TransferProofVectorDescription` by each concrete of
///   `DECLARED_ENCODING_SELECTIONS` and decoded under the same description,
///   equals itself.
/// Arrange: as the mint block.
/// Act:    `encode` of the encoder the consumer received, over the record,
///   with `build_encode_params`.
/// Assert: `decode` of the returned bytes returns a described record equal to
///   the transfer record.
/// Boundary: the description's `to_fields` and `fields_to_value` through the
///   real encoder, after `proof_vectors`.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn a_transfer_record_survives_its_description_round_trip() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let description = build_transfer_proof_vector_description::<F>(Default::default());
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");
            let bytes = encoder
                .encode(build_encode_params(&description), &success.transfer[0])
                .expect("the record encodes")
                .bytes;
            let described = decoder
                .decode(build_decode_params(&description), &bytes)
                .expect("the bytes decode")
                .described;

            // Assert
            assert_eq!(described, success.transfer[0]);
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// a_transfer_record_survives_its_description_round_trip

/// Contract: `proof_vectors` over a case whose source refuses its draws →
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::FillBytes(error)))`,
///   the mint delivery's refusal whole.
/// Arrange: `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, a selection by configuration, every other adapter at
///   the case's selection; `mint_count` and `transfer_count` of `1`; every
///   member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS`; the expected `error` the literal
///   `FillBytesErrorReturn::MockIRandomSourceAdapter`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the result equals
///   `Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::FillBytes(error)))`
///   whole, `error` the literal arranged.
/// Boundary: `proof_vectors`, `mint`, and the chain of real functions the
///   private surface names.
/// Mocked: the randomness family's mock concrete under `FillBytesRefused`,
///   selected through `create_random_source`; nothing else.
#[test]
fn a_refused_mint_draw_reaches_the_caller_through_the_mint_variant() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let result = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );

            // Assert
            assert_eq!(
                result,
                Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))),
            );
        }
    }

    run_every_case(
        RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused),
        &|params| {
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                params,
                case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
            )
            .is_ok()
        },
    );
}
// a_refused_mint_draw_reaches_the_caller_through_the_mint_variant

/// Contract: `member_names()` declares one name for each value `to_fields`
///   emits, for each description over the forms of each member of
///   `CHAIN_FORMS_CONCRETES`.
/// Arrange: `mint_count` and `transfer_count` of `1`; every case, the mint
///   description and the transfer description each in turn.
/// Act:    `member_names()` on the description, in the case consumer's method
///   after `proof_vectors`.
/// Assert: the names are `Ok` and their count equals the count of the values
///   `to_fields` over the record from `proof_vectors` emits.
/// Boundary: the description's `member_names` and `to_fields`, with the
///   `SolidityMemberName` constructor `member_names` calls, after
///   `proof_vectors`.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn each_record_kind_declares_one_name_per_field_it_emits() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _decoder: &E,
            _kem: &K,
            _key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            H: ?Sized + IHashToScalarAdapter<P::Scalar>,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_description = build_mint_proof_vector_description::<F>(Default::default());
            let transfer_description =
                build_transfer_proof_vector_description::<F>(Default::default());
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );

            // Act
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    pairing,
                    _kem,
                    _key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                    transfer_count: Some(build_vector_count(
                        VectorCountConstructorParamsOverrides { count: Some(1) },
                    )),
                }),
                build_proof_vectors_payload(
                    &mint_context,
                    &transfer_context,
                    ProofVectorsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
            .expect("the run produces the records");
            let mint_names = mint_description
                .member_names()
                .expect("the mint names are admitted");
            let mint_fields = mint_description
                .to_fields(ToFieldsParams, &success.mint[0])
                .expect("the mint record maps")
                .fields;
            let transfer_names = transfer_description
                .member_names()
                .expect("the transfer names are admitted");
            let transfer_fields = transfer_description
                .to_fields(ToFieldsParams, &success.transfer[0])
                .expect("the transfer record maps")
                .fields;

            // Assert
            assert_eq!(mint_names.len(), mint_fields.values.len());
            assert_eq!(transfer_names.len(), transfer_fields.values.len());
        }
    }

    run_every_case(RandomSourceKind::OperatingSystem, &|params| {
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            params,
            case_payload(b"pkg@1.0.0#1", &build_root(Default::default())),
        )
        .is_ok()
    });
}
// each_record_kind_declares_one_name_per_field_it_emits
