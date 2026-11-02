#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParams, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams, SecretConstructorParamsOverrides, SuiteIdentifier,
    SuiteIdentifierConstructorParams, build_secret,
};
use encoding::{
    CanonicalFieldValue, CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, ICanonicalField,
    IDecoderAdapter, IEncoderAdapter, build_decode_params, build_encode_params,
    build_from_field_params,
};
use envelope::{
    DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeComponents, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, KeyPairPublicKeysParams, KeyPairPublicKeysPayload, PublicKeysComponents,
};
use hash_to_scalar::{DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter};
use hashing::build_root;
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter,
    IdentityElementComponentsParams, IdentityElementComponentsPayload, IdentityScope,
    ParameterSetComponentsParams, ParameterSetComponentsPayload, SetupParams, SetupPayload,
};
use pairing::{
    CreatePairingParamsOverrides, DecodeG1Params, DecodeG2Params, DecodeScalarParams,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES, VerifierGroupArithmetic,
    build_create_pairing_params,
};
use proof::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, DeliveryProofComponents,
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
    IProofVectorsCaseConsumer, MintProofVector, MintProofVectorDescription,
    MintProofVectorDescriptionConstructorParams, ProofVectorContext, ProofVectorsErrorReturn,
    TransferProofVector, TransferProofVectorDescription,
    TransferProofVectorDescriptionConstructorParams,
};
use super::mock::{
    ProofVectorContextOverrides, ProofVectorsCaseParamsOverrides, ProofVectorsParamsOverrides,
    ProofVectorsPayloadOverrides, build_proof_vector_context, build_proof_vectors_case_deps,
    build_proof_vectors_case_params, build_proof_vectors_case_payload, build_proof_vectors_deps,
    build_proof_vectors_params, build_proof_vectors_payload, run_proof_vectors_case,
};
use super::proof_vectors;
use crate::delivery::provides::MintErrorReturn;

/// Contract: `proof_vectors` over a case with a mint count above one, every
///   adapter real and the operating-system source real → every mint record
///   rebuilt from its bytes alone is a statement and proof `verify` accepts.
/// Arrange: `mint_count` of `2` and `transfer_count` of `1`, the entitlement
///   scope, the contexts, purposes, and identity at their defaults; every
///   case, each member of `PAIRING_CONCRETES` crossed with the selections the
///   other families declare, so both curves and both verifier forms run.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `mint` holds `2` records and each record's rebuild yields
///   `is_valid` true.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_mint_record_is_a_statement_and_proof_the_delivery_proof_verifies() {
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            .expect("the generation succeeds");

            // Assert
            assert_eq!(success.mint.len(), 2);
            for record in &success.mint {
                let context = rebuild_mint_context::<F>(&record.context);
                let statement = AlgebraicMintStatement {
                    context,
                    hpub: decode_g2::<P>(pairing, &record.hpub),
                    identity_element: decode_g1::<P>(pairing, &record.identity_element),
                    buyer_keys: PublicKeysComponents {
                        pk1: decode_g1::<P>(pairing, &record.buyer_pk1),
                        pk2: decode_g2::<P>(pairing, &record.buyer_pk2),
                    },
                    envelope: EnvelopeComponents {
                        c1: decode_g1::<P>(pairing, &record.envelope_c1),
                        c2: decode_g1::<P>(pairing, &record.envelope_c2),
                        d1: decode_g2::<P>(pairing, &record.envelope_d1),
                        d2: decode_g2::<P>(pairing, &record.envelope_d2),
                    },
                };
                let proof = rebuild_mint_proof::<P, D>(pairing, delivery_proof, record);
                let verdict = delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("verify succeeds");
                assert!(verdict.is_valid);
            }
        }
    }

    run_every_case(Consumer);
    // every_mint_record_is_a_statement_and_proof_the_delivery_proof_verifies
}

/// Contract: `proof_vectors` over a case with a transfer count above one,
///   every adapter real and the operating-system source real → every transfer
///   record rebuilt from its bytes alone is a statement and proof `verify`
///   accepts.
/// Arrange: `mint_count` of `1` and `transfer_count` of `2`, the entitlement
///   scope, the contexts, purposes, and identity at their defaults; every
///   case.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `transfer` holds `2` records and each record's rebuild yields
///   `is_valid` true, the old envelope, the new envelope, the seller's keys,
///   and the buyer's keys each taken from the record.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_transfer_record_is_a_statement_and_proof_the_delivery_proof_verifies() {
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            .expect("the generation succeeds");

            // Assert
            assert_eq!(success.transfer.len(), 2);
            for record in &success.transfer {
                let context = rebuild_transfer_context::<F>(&record.context);
                let statement = AlgebraicTransferStatement {
                    context,
                    identity_element: decode_g1::<P>(pairing, &record.identity_element),
                    seller_keys: PublicKeysComponents {
                        pk1: decode_g1::<P>(pairing, &record.seller_pk1),
                        pk2: decode_g2::<P>(pairing, &record.seller_pk2),
                    },
                    buyer_keys: PublicKeysComponents {
                        pk1: decode_g1::<P>(pairing, &record.buyer_pk1),
                        pk2: decode_g2::<P>(pairing, &record.buyer_pk2),
                    },
                    old_envelope: EnvelopeComponents {
                        c1: decode_g1::<P>(pairing, &record.old_envelope_c1),
                        c2: decode_g1::<P>(pairing, &record.old_envelope_c2),
                        d1: decode_g2::<P>(pairing, &record.old_envelope_d1),
                        d2: decode_g2::<P>(pairing, &record.old_envelope_d2),
                    },
                    new_envelope: EnvelopeComponents {
                        c1: decode_g1::<P>(pairing, &record.new_envelope_c1),
                        c2: decode_g1::<P>(pairing, &record.new_envelope_c2),
                        d1: decode_g2::<P>(pairing, &record.new_envelope_d1),
                        d2: decode_g2::<P>(pairing, &record.new_envelope_d2),
                    },
                };
                let proof = rebuild_transfer_proof::<P, D>(pairing, delivery_proof, record);
                let verdict = delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("verify succeeds");
                assert!(verdict.is_valid);
            }
        }
    }

    run_every_case(Consumer);
    // every_transfer_record_is_a_statement_and_proof_the_delivery_proof_verifies
}

/// Contract: `proof_vectors` over an admitted identity in the asset forms
///   over one `Root` → `Ok`, every record rebuilt and verified.
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
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            .expect("the generation succeeds");

            // Assert
            for record in &success.mint {
                let statement = AlgebraicMintStatement {
                    context: rebuild_mint_context::<F>(&record.context),
                    hpub: decode_g2::<P>(pairing, &record.hpub),
                    identity_element: decode_g1::<P>(pairing, &record.identity_element),
                    buyer_keys: PublicKeysComponents {
                        pk1: decode_g1::<P>(pairing, &record.buyer_pk1),
                        pk2: decode_g2::<P>(pairing, &record.buyer_pk2),
                    },
                    envelope: EnvelopeComponents {
                        c1: decode_g1::<P>(pairing, &record.envelope_c1),
                        c2: decode_g1::<P>(pairing, &record.envelope_c2),
                        d1: decode_g2::<P>(pairing, &record.envelope_d1),
                        d2: decode_g2::<P>(pairing, &record.envelope_d2),
                    },
                };
                let proof = rebuild_mint_proof::<P, D>(pairing, delivery_proof, record);
                let verdict = delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("verify succeeds");
                assert!(verdict.is_valid);
            }
            for record in &success.transfer {
                let statement = AlgebraicTransferStatement {
                    context: rebuild_transfer_context::<F>(&record.context),
                    identity_element: decode_g1::<P>(pairing, &record.identity_element),
                    seller_keys: PublicKeysComponents {
                        pk1: decode_g1::<P>(pairing, &record.seller_pk1),
                        pk2: decode_g2::<P>(pairing, &record.seller_pk2),
                    },
                    buyer_keys: PublicKeysComponents {
                        pk1: decode_g1::<P>(pairing, &record.buyer_pk1),
                        pk2: decode_g2::<P>(pairing, &record.buyer_pk2),
                    },
                    old_envelope: EnvelopeComponents {
                        c1: decode_g1::<P>(pairing, &record.old_envelope_c1),
                        c2: decode_g1::<P>(pairing, &record.old_envelope_c2),
                        d1: decode_g2::<P>(pairing, &record.old_envelope_d1),
                        d2: decode_g2::<P>(pairing, &record.old_envelope_d2),
                    },
                    new_envelope: EnvelopeComponents {
                        c1: decode_g1::<P>(pairing, &record.new_envelope_c1),
                        c2: decode_g1::<P>(pairing, &record.new_envelope_c2),
                        d1: decode_g2::<P>(pairing, &record.new_envelope_d1),
                        d2: decode_g2::<P>(pairing, &record.new_envelope_d2),
                    },
                };
                let proof = rebuild_transfer_proof::<P, D>(pairing, delivery_proof, record);
                let verdict = delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("verify succeeds");
                assert!(verdict.is_valid);
            }
        }
    }

    let root = build_root(hashing::RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for &concrete in PAIRING_CONCRETES {
        for kem in DECLARED_KEM_SELECTIONS {
            for &chain_forms in CHAIN_FORMS_CONCRETES {
                for &key_agreement in DECLARED_KEY_AGREEMENT_SELECTIONS {
                    for &delivery_proof in DELIVERY_PROOF_CONCRETES {
                        for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                            for &hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                                assert!(
                                    run_proof_vectors_case(
                                        &build_proof_vectors_case_deps(
                                            Consumer,
                                            Default::default()
                                        ),
                                        build_proof_vectors_case_params(
                                            ProofVectorsCaseParamsOverrides {
                                                pairing: Some(build_create_pairing_params(
                                                    CreatePairingParamsOverrides {
                                                        concrete: Some(concrete),
                                                        ..Default::default()
                                                    },
                                                )),
                                                hash_to_scalar: Some(hash_to_scalar),
                                                encoding: Some(CreateEncodingParams {
                                                    concrete: encoding_selection.concrete,
                                                    identifier: encoding_selection.identifier,
                                                }),
                                                chain_forms: Some(build_create_chain_forms_params(
                                                    CreateChainFormsParamsOverrides {
                                                        concrete: Some(chain_forms),
                                                        ..Default::default()
                                                    },
                                                ),),
                                                kem: Some(kem::CreateKemParams {
                                                    scope: IdentityScope::Asset,
                                                    ..*kem
                                                }),
                                                key_agreement: Some(key_agreement),
                                                delivery_proof: Some(
                                                    build_create_delivery_proof_params(
                                                        CreateDeliveryProofParamsOverrides {
                                                            concrete: Some(delivery_proof),
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ),
                                                random_source: Some(
                                                    build_create_random_source_params(
                                                        CreateRandomSourceParamsOverrides {
                                                            kind: Some(
                                                                RandomSourceKind::OperatingSystem,
                                                            ),
                                                        },
                                                    )
                                                ),
                                            },
                                        ),
                                        build_proof_vectors_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    )
                                    .is_ok()
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // an_asset_identity_is_proved_under_the_asset_scope
}

/// Contract: each record's `draws`, split at `UNIFORM_BYTES_LENGTH` into the
///   uniforms of its calls in draw order and replayed through `setup` under
///   the admitted identity's setup scope, `derive_identity` over its
///   identity, and `generate_keys` on the case's adapters, reproduce the
///   record's `hpub`, `identity_element`, and key fields.
/// Arrange: `mint_count` and `transfer_count` of `1`, the entitlement scope,
///   each case run under each of the admitted identities over the canonical
///   bytes `b"pkg@1.0.0#1"` and `b"pkg@2.0.0#7"`, so a replay over an identity
///   other than the admitted one reports otherwise.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the replayed parameter set's `hpub`, the replayed identity
///   element's `element`, and each replayed key pair's `pk1` and `pk2`, each
///   encoded through the received pairing, equal the mint record's `hpub`,
///   `identity_element`, `buyer_pk1`, and `buyer_pk2` and the transfer
///   record's `identity_element`, `seller_pk1`, `seller_pk2`, `buyer_pk1`,
///   and `buyer_pk2`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_statement_elements_it_produced() {
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
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
            .expect("the generation succeeds");

            // Assert
            let replay = |draws: &[u8]| {
                let uniform = |index: usize| {
                    build_secret(
                        draws[index * length..(index + 1) * length].to_vec(),
                        SecretConstructorParamsOverrides,
                    )
                };
                let setup = kem
                    .setup(
                        SetupParams {
                            scope: identity.setup_scope,
                        },
                        SetupPayload {
                            master_uniform: uniform(0),
                            u0_uniform: uniform(1),
                            u1_uniform: uniform(2),
                        },
                    )
                    .expect("the replayed setup succeeds");
                let derived = kem
                    .derive_identity(
                        DeriveIdentityParams,
                        DeriveIdentityPayload {
                            parameter_set: &setup.parameter_set,
                            identity: identity.identity,
                        },
                    )
                    .expect("the replayed derivation succeeds");
                let Ok(components) = kem.identity_element_components(
                    IdentityElementComponentsParams,
                    IdentityElementComponentsPayload {
                        identity_element: &derived.identity_element,
                    },
                );
                let element = components.components.element;
                let Ok(components) = kem.parameter_set_components(
                    ParameterSetComponentsParams,
                    ParameterSetComponentsPayload {
                        parameter_set: &setup.parameter_set,
                    },
                );
                let hpub = components.components.hpub;
                let recipient = key_agreement
                    .generate_keys(
                        GenerateKeysParams,
                        GenerateKeysPayload {
                            x_uniform: uniform(4),
                            y_uniform: uniform(5),
                        },
                    )
                    .expect("the replayed recipient keys generate");
                let Ok(recipient_keys) = key_agreement.key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &recipient.key_pair,
                    },
                );
                (hpub, element, recipient_keys.components)
            };
            for record in &success.mint {
                let (hpub, element, recipient_keys) = replay(&record.draws);
                assert_eq!(
                    record.hpub.as_slice(),
                    encode_g2::<P>(pairing, &hpub).as_ref()
                );
                assert_eq!(
                    record.identity_element.as_slice(),
                    encode_g1::<P>(pairing, &element).as_ref()
                );
                assert_eq!(
                    record.buyer_pk1.as_slice(),
                    encode_g1::<P>(pairing, &recipient_keys.pk1).as_ref()
                );
                assert_eq!(
                    record.buyer_pk2.as_slice(),
                    encode_g2::<P>(pairing, &recipient_keys.pk2).as_ref()
                );
            }
            for record in &success.transfer {
                let (_hpub, element, seller_keys) = replay(&record.draws);
                let uniform = |index: usize| {
                    build_secret(
                        record.draws[index * length..(index + 1) * length].to_vec(),
                        SecretConstructorParamsOverrides,
                    )
                };
                let buyer = key_agreement
                    .generate_keys(
                        GenerateKeysParams,
                        GenerateKeysPayload {
                            x_uniform: uniform(6),
                            y_uniform: uniform(7),
                        },
                    )
                    .expect("the replayed buyer keys generate");
                let Ok(buyer_keys) = key_agreement.key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &buyer.key_pair,
                    },
                );
                let buyer_keys = buyer_keys.components;
                assert_eq!(
                    record.identity_element.as_slice(),
                    encode_g1::<P>(pairing, &element).as_ref()
                );
                assert_eq!(
                    record.seller_pk1.as_slice(),
                    encode_g1::<P>(pairing, &seller_keys.pk1).as_ref()
                );
                assert_eq!(
                    record.seller_pk2.as_slice(),
                    encode_g2::<P>(pairing, &seller_keys.pk2).as_ref()
                );
                assert_eq!(
                    record.buyer_pk1.as_slice(),
                    encode_g1::<P>(pairing, &buyer_keys.pk1).as_ref()
                );
                assert_eq!(
                    record.buyer_pk2.as_slice(),
                    encode_g2::<P>(pairing, &buyer_keys.pk2).as_ref()
                );
            }
        }
    }

    for canonical in [b"pkg@1.0.0#1".as_slice(), b"pkg@2.0.0#7".as_slice()] {
        for &concrete in PAIRING_CONCRETES {
            for &kem in DECLARED_KEM_SELECTIONS {
                for &chain_forms in CHAIN_FORMS_CONCRETES {
                    for &key_agreement in DECLARED_KEY_AGREEMENT_SELECTIONS {
                        for &delivery_proof in DELIVERY_PROOF_CONCRETES {
                            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                                for &hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                                    assert!(
                                        run_proof_vectors_case(
                                            &build_proof_vectors_case_deps(
                                                Consumer,
                                                Default::default()
                                            ),
                                            build_proof_vectors_case_params(
                                                ProofVectorsCaseParamsOverrides {
                                                    pairing: Some(build_create_pairing_params(
                                                        CreatePairingParamsOverrides {
                                                            concrete: Some(concrete),
                                                            ..Default::default()
                                                        },
                                                    )),
                                                    hash_to_scalar: Some(hash_to_scalar),
                                                    encoding: Some(CreateEncodingParams {
                                                        concrete: encoding_selection.concrete,
                                                        identifier: encoding_selection.identifier,
                                                    }),
                                                    chain_forms: Some(
                                                        build_create_chain_forms_params(
                                                            CreateChainFormsParamsOverrides {
                                                                concrete: Some(chain_forms),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    kem: Some(kem),
                                                    key_agreement: Some(key_agreement),
                                                    delivery_proof: Some(
                                                        build_create_delivery_proof_params(
                                                            CreateDeliveryProofParamsOverrides {
                                                                concrete: Some(delivery_proof),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    random_source: Some(
                                                        build_create_random_source_params(
                                                            CreateRandomSourceParamsOverrides {
                                                                kind: Some(
                                                                    RandomSourceKind::OperatingSystem,
                                                                ),
                                                            },
                                                        )
                                                    ),
                                                },
                                            ),
                                            build_proof_vectors_case_payload(
                                                canonical,
                                                &build_root(Default::default()),
                                                Default::default(),
                                            ),
                                        )
                                        .is_ok()
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // every_record_draw_replays_to_the_statement_elements_it_produced
}

/// Contract: under a pairing whose declaration names the first-group-only
///   verifier form each record's `second_group_first_messages` holds three
///   second-group point encodings and the record with one byte of them
///   changed is refused by `verify`; under both groups it is empty.
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
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: generator.point,
                },
            );
            let expected = 3 * encoded.bytes.as_ref().len();
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
            .expect("the generation succeeds");

            // Assert
            match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::FirstGroupOnly => {
                    let verify_changed =
                        |record: &MintProofVector, pairing: &P, delivery_proof: &D| {
                            let context = rebuild_mint_context::<F>(&record.context);
                            let statement = AlgebraicMintStatement {
                                context,
                                hpub: decode_g2::<P>(pairing, &record.hpub),
                                identity_element: decode_g1::<P>(pairing, &record.identity_element),
                                buyer_keys: PublicKeysComponents {
                                    pk1: decode_g1::<P>(pairing, &record.buyer_pk1),
                                    pk2: decode_g2::<P>(pairing, &record.buyer_pk2),
                                },
                                envelope: EnvelopeComponents {
                                    c1: decode_g1::<P>(pairing, &record.envelope_c1),
                                    c2: decode_g1::<P>(pairing, &record.envelope_c2),
                                    d1: decode_g2::<P>(pairing, &record.envelope_d1),
                                    d2: decode_g2::<P>(pairing, &record.envelope_d2),
                                },
                            };
                            let mut changed = record.second_group_first_messages.clone();
                            changed[0] ^= 0xFF;
                            let chunk = changed.len() / 3;
                            let decode = |slice: &[u8]| pairing.decode_g2(DecodeG2Params, slice);
                            let (Ok(hpub), Ok(d1), Ok(d2)) = (
                                decode(&changed[0..chunk]),
                                decode(&changed[chunk..2 * chunk]),
                                decode(&changed[2 * chunk..]),
                            ) else {
                                return;
                            };
                            let proof = delivery_proof
                                .proof_from_components(
                                    ProofFromComponentsParams,
                                    ProofFromComponentsPayload {
                                        components: DeliveryProofComponents::MintFirstGroupOnly {
                                            challenge: decode_scalar::<P>(
                                                pairing,
                                                &record.challenge,
                                            ),
                                            responses: MintResponses {
                                                alpha: decode_scalar::<P>(pairing, &record.alpha),
                                                r: decode_scalar::<P>(pairing, &record.r),
                                                rho: decode_scalar::<P>(pairing, &record.rho),
                                                sigma: decode_scalar::<P>(pairing, &record.sigma),
                                            },
                                            first_messages: MintSecondGroupFirstMessages {
                                                hpub: hpub.point,
                                                d1: d1.point,
                                                d2: d2.point,
                                            },
                                        },
                                    },
                                )
                                .expect("the components rebuild the proof")
                                .proof;
                            let verdict = delivery_proof
                                .verify(
                                    VerifyParams,
                                    VerifyPayload {
                                        statement: AlgebraicStatement::Mint(&statement),
                                        proof: &proof,
                                    },
                                )
                                .expect("verify succeeds");
                            assert!(!verdict.is_valid);
                        };
                    for record in &success.mint {
                        assert_eq!(record.second_group_first_messages.len(), expected);
                        verify_changed(record, pairing, delivery_proof);
                    }
                    for record in &success.transfer {
                        assert_eq!(record.second_group_first_messages.len(), expected);
                        let context = rebuild_transfer_context::<F>(&record.context);
                        let statement = AlgebraicTransferStatement {
                            context,
                            identity_element: decode_g1::<P>(pairing, &record.identity_element),
                            seller_keys: PublicKeysComponents {
                                pk1: decode_g1::<P>(pairing, &record.seller_pk1),
                                pk2: decode_g2::<P>(pairing, &record.seller_pk2),
                            },
                            buyer_keys: PublicKeysComponents {
                                pk1: decode_g1::<P>(pairing, &record.buyer_pk1),
                                pk2: decode_g2::<P>(pairing, &record.buyer_pk2),
                            },
                            old_envelope: EnvelopeComponents {
                                c1: decode_g1::<P>(pairing, &record.old_envelope_c1),
                                c2: decode_g1::<P>(pairing, &record.old_envelope_c2),
                                d1: decode_g2::<P>(pairing, &record.old_envelope_d1),
                                d2: decode_g2::<P>(pairing, &record.old_envelope_d2),
                            },
                            new_envelope: EnvelopeComponents {
                                c1: decode_g1::<P>(pairing, &record.new_envelope_c1),
                                c2: decode_g1::<P>(pairing, &record.new_envelope_c2),
                                d1: decode_g2::<P>(pairing, &record.new_envelope_d1),
                                d2: decode_g2::<P>(pairing, &record.new_envelope_d2),
                            },
                        };
                        let mut changed = record.second_group_first_messages.clone();
                        changed[0] ^= 0xFF;
                        let chunk = changed.len() / 3;
                        let decode = |slice: &[u8]| pairing.decode_g2(DecodeG2Params, slice);
                        if let (Ok(pk2), Ok(d1), Ok(d2)) = (
                            decode(&changed[0..chunk]),
                            decode(&changed[chunk..2 * chunk]),
                            decode(&changed[2 * chunk..]),
                        ) {
                            let proof = delivery_proof
                                .proof_from_components(
                                    ProofFromComponentsParams,
                                    ProofFromComponentsPayload {
                                        components:
                                            DeliveryProofComponents::TransferFirstGroupOnly {
                                                challenge: decode_scalar::<P>(
                                                    pairing,
                                                    &record.challenge,
                                                ),
                                                responses: TransferResponses {
                                                    x: decode_scalar::<P>(pairing, &record.x),
                                                    y: decode_scalar::<P>(pairing, &record.y),
                                                    s: decode_scalar::<P>(pairing, &record.s),
                                                    rho: decode_scalar::<P>(pairing, &record.rho),
                                                    sigma: decode_scalar::<P>(
                                                        pairing,
                                                        &record.sigma,
                                                    ),
                                                },
                                                first_messages: TransferSecondGroupFirstMessages {
                                                    pk2: pk2.point,
                                                    d1: d1.point,
                                                    d2: d2.point,
                                                },
                                            },
                                    },
                                )
                                .expect("the components rebuild the proof")
                                .proof;
                            let verdict = delivery_proof
                                .verify(
                                    VerifyParams,
                                    VerifyPayload {
                                        statement: AlgebraicStatement::Transfer(&statement),
                                        proof: &proof,
                                    },
                                )
                                .expect("verify succeeds");
                            assert!(!verdict.is_valid);
                        }
                    }
                }
                VerifierGroupArithmetic::BothGroups => {
                    for record in &success.mint {
                        assert!(record.second_group_first_messages.is_empty());
                    }
                    for record in &success.transfer {
                        assert!(record.second_group_first_messages.is_empty());
                    }
                }
            }
        }
    }

    run_every_case(Consumer);
    // every_record_carries_second_group_first_messages_exactly_in_the_first_group_only_form
}

/// Contract: every first-group point of a record is as long as the
///   `encode_g1` of the first-group generator, every second-group point as
///   long as the `encode_g2` of the second-group generator, and every scalar
///   field 32 bytes.
/// Arrange: `mint_count` and `transfer_count` of `1`; every member of
///   `PAIRING_CONCRETES`, whose encoded lengths differ between the curves.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `identity_element`, `buyer_pk1`, `envelope_c1`,
///   and `envelope_c2` and the transfer record's `identity_element`,
///   `seller_pk1`, `buyer_pk1`, and both envelopes' `c1` and `c2` each hold
///   the length of the first-group generator's encoding; the mint record's
///   `hpub`, `buyer_pk2`, `envelope_d1`, and `envelope_d2` and the transfer
///   record's `seller_pk2`, `buyer_pk2`, and both envelopes' `d1` and `d2`
///   each hold the length of the second-group generator's encoding; the mint
///   record's `challenge`, `alpha`, `r`, `rho`, and `sigma` and the transfer
///   record's `challenge`, `x`, `y`, `s`, `rho`, and `sigma` each hold `32`
///   bytes; the generators' encodings are read through the pairing's
///   generator operations and `encode_g1` and `encode_g2`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_record_encodes_its_elements_in_the_selected_pairings_encodings() {
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            let Ok(generator) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: generator.point,
                },
            );
            let g1_length = encoded.bytes.as_ref().len();
            let Ok(generator) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: generator.point,
                },
            );
            let g2_length = encoded.bytes.as_ref().len();
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
            .expect("the generation succeeds");

            // Assert
            for record in &success.mint {
                assert_eq!(record.identity_element.len(), g1_length);
                assert_eq!(record.buyer_pk1.len(), g1_length);
                assert_eq!(record.envelope_c1.len(), g1_length);
                assert_eq!(record.envelope_c2.len(), g1_length);
                assert_eq!(record.hpub.len(), g2_length);
                assert_eq!(record.buyer_pk2.len(), g2_length);
                assert_eq!(record.envelope_d1.len(), g2_length);
                assert_eq!(record.envelope_d2.len(), g2_length);
                assert_eq!(record.challenge.len(), 32);
                assert_eq!(record.alpha.len(), 32);
                assert_eq!(record.r.len(), 32);
                assert_eq!(record.rho.len(), 32);
                assert_eq!(record.sigma.len(), 32);
            }
            for record in &success.transfer {
                assert_eq!(record.identity_element.len(), g1_length);
                assert_eq!(record.seller_pk1.len(), g1_length);
                assert_eq!(record.buyer_pk1.len(), g1_length);
                assert_eq!(record.old_envelope_c1.len(), g1_length);
                assert_eq!(record.old_envelope_c2.len(), g1_length);
                assert_eq!(record.new_envelope_c1.len(), g1_length);
                assert_eq!(record.new_envelope_c2.len(), g1_length);
                assert_eq!(record.seller_pk2.len(), g2_length);
                assert_eq!(record.buyer_pk2.len(), g2_length);
                assert_eq!(record.old_envelope_d1.len(), g2_length);
                assert_eq!(record.old_envelope_d2.len(), g2_length);
                assert_eq!(record.new_envelope_d1.len(), g2_length);
                assert_eq!(record.new_envelope_d2.len(), g2_length);
                assert_eq!(record.challenge.len(), 32);
                assert_eq!(record.x.len(), 32);
                assert_eq!(record.y.len(), 32);
                assert_eq!(record.s.len(), 32);
                assert_eq!(record.rho.len(), 32);
                assert_eq!(record.sigma.len(), 32);
            }
        }
    }

    run_every_case(Consumer);
    // every_record_encodes_its_elements_in_the_selected_pairings_encodings
}

/// Contract: each record's context holds its typed context's values, each
///   form-typed field the `to_field` of the form its typed context holds.
/// Arrange: `mint_count` and `transfer_count` of `1`, the mint context and
///   the transfer context each overriding every form-typed field with the
///   form `from_field` builds from a value distinct from every other field's
///   in both contexts, of the kind the case's forms declare, and distinct
///   domain values and purposes; every case over `CHAIN_FORMS_CONCRETES`.
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
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides::<F> {
                    suite_identifier: Some(domain::build_suite_identifier(
                        domain::SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x72; 32]),
                            version: Some(2),
                        },
                    )),
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x23; 32]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    entitlement_contract: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x33; 20]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    asset_identity_hash: Some(domain::build_asset_identity_hash(
                        domain::AssetIdentityHashConstructorParamsOverrides {
                            bytes: Some([0x74; 32]),
                        },
                    )),
                    parameter_set_identifier: Some(domain::build_parameter_set_identifier(
                        domain::ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x76; 32]),
                        },
                    )),
                    source_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x43; 32]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    target_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x44; 32]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    old_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(5),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    new_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(6),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    seller: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x63; 20]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    buyer: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x64; 20]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    expiry: Some(1_800_000_001),
                },
            );
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides::<F> {
                    suite_identifier: Some(domain::build_suite_identifier(
                        domain::SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x73; 32]),
                            version: Some(3),
                        },
                    )),
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x24; 32]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    entitlement_contract: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x32; 20]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    asset_identity_hash: Some(domain::build_asset_identity_hash(
                        domain::AssetIdentityHashConstructorParamsOverrides {
                            bytes: Some([0x75; 32]),
                        },
                    )),
                    parameter_set_identifier: Some(domain::build_parameter_set_identifier(
                        domain::ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x77; 32]),
                        },
                    )),
                    source_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x45; 32]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    target_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x46; 32]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    old_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(6),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    new_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(7),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    seller: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x65; 20]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    buyer: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x66; 20]),
                        )
                        .expect("the value is the form's kind")
                        .value,
                    ),
                    expiry: Some(1_900_000_000),
                },
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
            .expect("the generation succeeds");

            // Assert
            assert_eq!(
                success.mint[0].context,
                build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some([0x72; 32]),
                    suite_version: Some(2),
                    chain: Some(CanonicalFieldValue::Unsigned256([0x23; 32])),
                    entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x33; 20])),
                    asset_root: Some([0x74; 32]),
                    parameter_set: Some([0x76; 32]),
                    source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x43; 32])),
                    target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x44; 32])),
                    old_interval: Some(CanonicalFieldValue::Unsigned64(5)),
                    new_interval: Some(CanonicalFieldValue::Unsigned64(6)),
                    purpose: Some(1),
                    seller: Some(CanonicalFieldValue::FixedBytes20([0x63; 20])),
                    buyer: Some(CanonicalFieldValue::FixedBytes20([0x64; 20])),
                    expiry: Some(1_800_000_001),
                })
            );
            assert_eq!(
                success.transfer[0].context,
                build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some([0x73; 32]),
                    suite_version: Some(3),
                    chain: Some(CanonicalFieldValue::Unsigned256([0x24; 32])),
                    entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x32; 20])),
                    asset_root: Some([0x75; 32]),
                    parameter_set: Some([0x77; 32]),
                    source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x45; 32])),
                    target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x46; 32])),
                    old_interval: Some(CanonicalFieldValue::Unsigned64(6)),
                    new_interval: Some(CanonicalFieldValue::Unsigned64(7)),
                    purpose: Some(2),
                    seller: Some(CanonicalFieldValue::FixedBytes20([0x65; 20])),
                    buyer: Some(CanonicalFieldValue::FixedBytes20([0x66; 20])),
                    expiry: Some(1_900_000_000),
                })
            );
        }
    }

    run_every_case(Consumer);
    // every_record_context_holds_the_fields_of_its_typed_context
}

/// Contract: the lists hold the configured counts, each mint record's
///   `draws` hold the uniforms of its calls and each transfer record's
///   `draws` those of its calls, each `UNIFORM_BYTES_LENGTH` long, and no two
///   records of either list share `draws`.
/// Arrange: `mint_count` of `2` and `transfer_count` of `3`; every member of
///   `PAIRING_CONCRETES`, whose sampling bounds may differ.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `mint` holds `2` records whose `draws` each hold `6` times
///   `UNIFORM_BYTES_LENGTH` bytes, `transfer` holds `3` records whose `draws`
///   each hold `9` times that length, and no two records of either list
///   share `draws`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            .expect("the generation succeeds");

            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(success.mint.len(), 2);
            assert_eq!(success.transfer.len(), 3);
            for (index, record) in success.mint.iter().enumerate() {
                assert_eq!(record.draws.len(), 6 * length);
                for other in success.mint.iter().skip(index + 1) {
                    assert_ne!(record.draws, other.draws);
                }
            }
            for (index, record) in success.transfer.iter().enumerate() {
                assert_eq!(record.draws.len(), 9 * length);
                for other in success.transfer.iter().skip(index + 1) {
                    assert_ne!(record.draws, other.draws);
                }
            }
        }
    }

    run_every_case(Consumer);
    // every_run_emits_the_configured_count_of_records_with_distinct_draws
}

/// Contract: a mint record from `proof_vectors`, encoded under
///   `MintProofVectorDescription` by each concrete of
///   `DECLARED_ENCODING_SELECTIONS` and decoded under the same description,
///   equals itself.
/// Arrange: `mint_count` and `transfer_count` of `1`; every case, each
///   encoding selection crossed with each member of `PAIRING_CONCRETES`, the
///   description built over the case's forms.
/// Act:    `encode` of the encoder the consumer received, over the record,
///   with `build_encode_params`.
/// Assert: `decode` of the returned bytes, with `build_decode_params`,
///   returns a described record equal to the mint record, its form-typed
///   context fields holding the values their forms rebuilt.
/// Boundary: the description's `to_fields` and `fields_to_value` through the
///   real encoder, after `proof_vectors`.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn a_mint_record_survives_its_description_round_trip() {
    #[derive(Clone)]
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            .expect("the generation succeeds");
            let description = MintProofVectorDescription::<F>::try_new(
                MintProofVectorDescriptionConstructorParams,
            )
            .expect("the description constructs");

            // Act
            let encoded = encoder
                .encode(build_encode_params(&description), &success.mint[0])
                .expect("the record encodes");
            let decoded = decoder
                .decode::<MintProofVectorDescription<F>>(
                    build_decode_params(&description),
                    &encoded.bytes,
                )
                .expect("the bytes decode");

            // Assert
            assert_eq!(decoded.described, success.mint[0]);
        }
    }

    run_every_case(Consumer);
    // a_mint_record_survives_its_description_round_trip
}

/// Contract: a transfer record from `proof_vectors`, encoded under
///   `TransferProofVectorDescription` by each concrete of
///   `DECLARED_ENCODING_SELECTIONS` and decoded under the same description,
///   equals itself.
/// Arrange: as the mint block.
/// Act:    `encode` of the encoder the consumer received, over the record,
///   with `build_encode_params`.
/// Assert: `decode` of the returned bytes returns a described record equal
///   to the transfer record.
/// Boundary: the description's `to_fields` and `fields_to_value` through the
///   real encoder, after `proof_vectors`.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn a_transfer_record_survives_its_description_round_trip() {
    #[derive(Clone)]
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            .expect("the generation succeeds");
            let description = TransferProofVectorDescription::<F>::try_new(
                TransferProofVectorDescriptionConstructorParams,
            )
            .expect("the description constructs");

            // Act
            let encoded = encoder
                .encode(build_encode_params(&description), &success.transfer[0])
                .expect("the record encodes");
            let decoded = decoder
                .decode::<TransferProofVectorDescription<F>>(
                    build_decode_params(&description),
                    &encoded.bytes,
                )
                .expect("the bytes decode");

            // Assert
            assert_eq!(decoded.described, success.transfer[0]);
        }
    }

    run_every_case(Consumer);
    // a_transfer_record_survives_its_description_round_trip
}

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
/// Assert: the result equals `Err(ProofVectorsErrorReturn::Mint(
///   MintErrorReturn::FillBytes(error)))` whole, `error` the literal
///   arranged.
/// Boundary: `proof_vectors`, `mint`, and the chain of real functions the
///   private surface names.
/// Mocked: the randomness family's mock concrete under `FillBytesRefused`,
///   selected through `create_random_source`; nothing else.
#[test]
fn a_refused_mint_draw_reaches_the_caller_through_the_mint_variant() {
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            let returned = proof_vectors::<P, K, A, D>(
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
            );

            // Assert
            assert_eq!(
                returned,
                Err(ProofVectorsErrorReturn::Mint(MintErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                )))
            );
        }
    }

    for &concrete in PAIRING_CONCRETES {
        for &kem in DECLARED_KEM_SELECTIONS {
            for &chain_forms in CHAIN_FORMS_CONCRETES {
                for &key_agreement in DECLARED_KEY_AGREEMENT_SELECTIONS {
                    for &delivery_proof in DELIVERY_PROOF_CONCRETES {
                        for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                            for &hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                                assert!(
                                    run_proof_vectors_case(
                                        &build_proof_vectors_case_deps(
                                            Consumer,
                                            Default::default()
                                        ),
                                        build_proof_vectors_case_params(
                                            ProofVectorsCaseParamsOverrides {
                                                pairing: Some(build_create_pairing_params(
                                                    CreatePairingParamsOverrides {
                                                        concrete: Some(concrete),
                                                        ..Default::default()
                                                    },
                                                )),
                                                hash_to_scalar: Some(hash_to_scalar),
                                                encoding: Some(CreateEncodingParams {
                                                    concrete: encoding_selection.concrete,
                                                    identifier: encoding_selection.identifier,
                                                }),
                                                chain_forms: Some(build_create_chain_forms_params(
                                                    CreateChainFormsParamsOverrides {
                                                        concrete: Some(chain_forms),
                                                        ..Default::default()
                                                    },
                                                ),),
                                                kem: Some(kem),
                                                key_agreement: Some(key_agreement),
                                                delivery_proof: Some(
                                                    build_create_delivery_proof_params(
                                                        CreateDeliveryProofParamsOverrides {
                                                            concrete: Some(delivery_proof),
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ),
                                                random_source: Some(
                                                    build_create_random_source_params(
                                                        CreateRandomSourceParamsOverrides {
                                                            kind: Some(RandomSourceKind::Mock(
                                                                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                                                            )),
                                                        },
                                                    )
                                                ),
                                            },
                                        ),
                                        build_proof_vectors_case_payload(
                                            b"pkg@1.0.0#1",
                                            &build_root(Default::default()),
                                            Default::default(),
                                        ),
                                    )
                                    .is_ok()
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // a_refused_mint_draw_reaches_the_caller_through_the_mint_variant
}

/// Contract: `member_names()` declares one name for each value `to_fields`
///   emits, for each description over the forms of each member of
///   `CHAIN_FORMS_CONCRETES`.
/// Arrange: `mint_count` and `transfer_count` of `1`; every case, the mint
///   description and the transfer description each in turn.
/// Act:    `member_names()` on the description, in the case consumer's
///   method after `proof_vectors`.
/// Assert: the names are `Ok` and their count equals the count of the values
///   `to_fields` over the record from `proof_vectors` emits.
/// Boundary: the description's `member_names` and `to_fields`, with the
///   `SolidityMemberName` constructor `member_names` calls, after
///   `proof_vectors`.
/// Mocked: nothing; the operating-system generator runs real.
#[test]
fn each_record_kind_declares_one_name_per_field_it_emits() {
    #[derive(Clone)]
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
            identity: crate::delivery::provides::AdmittedKemIdentity<'_>,
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
            .expect("the generation succeeds");

            // Act
            let mint_description = MintProofVectorDescription::<F>::try_new(
                MintProofVectorDescriptionConstructorParams,
            )
            .expect("the description constructs");
            let transfer_description = TransferProofVectorDescription::<F>::try_new(
                TransferProofVectorDescriptionConstructorParams,
            )
            .expect("the description constructs");
            let mint_names = mint_description
                .member_names()
                .expect("the mint names admit");
            let transfer_names = transfer_description
                .member_names()
                .expect("the transfer names admit");
            let mint_fields = encoding::IEncodingContract::to_fields(
                &mint_description,
                encoding::ToFieldsParams,
                &success.mint[0],
            )
            .expect("the record lists its fields")
            .fields;
            let transfer_fields = encoding::IEncodingContract::to_fields(
                &transfer_description,
                encoding::ToFieldsParams,
                &success.transfer[0],
            )
            .expect("the record lists its fields")
            .fields;

            // Assert
            assert_eq!(mint_names.len(), mint_fields.values.len());
            assert_eq!(transfer_names.len(), transfer_fields.values.len());
        }
    }

    run_every_case(Consumer);
    // each_record_kind_declares_one_name_per_field_it_emits
}

fn run_every_case<C: IProofVectorsCaseConsumer + Clone>(consumer: C) {
    for &concrete in PAIRING_CONCRETES {
        for &kem in DECLARED_KEM_SELECTIONS {
            for &chain_forms in CHAIN_FORMS_CONCRETES {
                for &key_agreement in DECLARED_KEY_AGREEMENT_SELECTIONS {
                    for &delivery_proof in DELIVERY_PROOF_CONCRETES {
                        for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                            for &hash_to_scalar in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                                assert!(
                                    run_proof_vectors_case(
                                        &build_proof_vectors_case_deps(
                                            consumer.clone(),
                                            Default::default()
                                        ),
                                        build_proof_vectors_case_params(
                                            ProofVectorsCaseParamsOverrides {
                                                pairing: Some(build_create_pairing_params(
                                                    CreatePairingParamsOverrides {
                                                        concrete: Some(concrete),
                                                        ..Default::default()
                                                    },
                                                )),
                                                hash_to_scalar: Some(hash_to_scalar),
                                                encoding: Some(CreateEncodingParams {
                                                    concrete: encoding_selection.concrete,
                                                    identifier: encoding_selection.identifier,
                                                }),
                                                chain_forms: Some(build_create_chain_forms_params(
                                                    CreateChainFormsParamsOverrides {
                                                        concrete: Some(chain_forms),
                                                        ..Default::default()
                                                    },
                                                ),),
                                                kem: Some(kem),
                                                key_agreement: Some(key_agreement),
                                                delivery_proof: Some(
                                                    build_create_delivery_proof_params(
                                                        CreateDeliveryProofParamsOverrides {
                                                            concrete: Some(delivery_proof),
                                                            ..Default::default()
                                                        },
                                                    ),
                                                ),
                                                random_source: Some(
                                                    build_create_random_source_params(
                                                        CreateRandomSourceParamsOverrides {
                                                            kind: Some(
                                                                RandomSourceKind::OperatingSystem,
                                                            ),
                                                        },
                                                    )
                                                ),
                                            },
                                        ),
                                        build_proof_vectors_case_payload(
                                            b"pkg@1.0.0#1",
                                            &build_root(Default::default()),
                                            Default::default(),
                                        ),
                                    )
                                    .is_ok()
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn decode_g1<P: IPairingArithmetic>(pairing: &P, bytes: &[u8]) -> P::G1 {
    pairing
        .decode_g1(DecodeG1Params, bytes)
        .expect("the record's encoding decodes")
        .point
}

fn decode_g2<P: IPairingArithmetic>(pairing: &P, bytes: &[u8]) -> P::G2 {
    pairing
        .decode_g2(DecodeG2Params, bytes)
        .expect("the record's encoding decodes")
        .point
}

fn decode_scalar<P: IPairingArithmetic>(pairing: &P, bytes: &[u8]) -> P::Scalar {
    pairing
        .decode_scalar(DecodeScalarParams, bytes)
        .expect("the record's encoding decodes")
        .scalar
}

fn encode_g1<P: IPairingArithmetic>(pairing: &P, point: &P::G1) -> P::EncodedG1 {
    let Ok(encoded) = pairing.encode_g1(
        EncodeG1Params,
        EncodeG1Payload {
            point: point.clone(),
        },
    );
    encoded.bytes
}

fn encode_g2<P: IPairingArithmetic>(pairing: &P, point: &P::G2) -> P::EncodedG2 {
    let Ok(encoded) = pairing.encode_g2(
        EncodeG2Params,
        EncodeG2Payload {
            point: point.clone(),
        },
    );
    encoded.bytes
}

fn rebuild_mint_context<F: IChainForms>(
    context: &ProofVectorContext,
) -> proof::DeliveryContext<F, MintPurpose> {
    let identifier = context.suite_identifier;
    let version = context.suite_version;
    let asset_root = context.asset_root;
    let parameter_set = context.parameter_set;
    let purpose = context.purpose;
    let expiry = context.expiry;
    proof::DeliveryContext {
        suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier,
            version,
        })
        .expect("the record's suite identifier admits"),
        chain: <F::ChainIdentifier as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.chain.clone(),
        )
        .expect("the record's chain value is the form's kind")
        .value,
        entitlement_contract: <F::Identity as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.entitlement_contract.clone(),
        )
        .expect("the record's contract value is the form's kind")
        .value,
        asset_identity_hash: AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
            bytes: asset_root,
        })
        .expect("the record's asset root admits"),
        parameter_set_identifier: ParameterSetIdentifier::try_new(
            ParameterSetIdentifierConstructorParams {
                bytes: parameter_set,
            },
        )
        .expect("the record's parameter set admits"),
        source_entitlement: <F::Entitlement as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.source_entitlement.clone(),
        )
        .expect("the record's source entitlement is the form's kind")
        .value,
        target_entitlement: <F::Entitlement as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.target_entitlement.clone(),
        )
        .expect("the record's target entitlement is the form's kind")
        .value,
        old_interval: <F::Interval as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.old_interval.clone(),
        )
        .expect("the record's old interval is the form's kind")
        .value,
        new_interval: <F::Interval as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.new_interval.clone(),
        )
        .expect("the record's new interval is the form's kind")
        .value,
        purpose: MintPurpose::try_from(purpose).expect("the record's purpose is declared"),
        seller: <F::Identity as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.seller.clone(),
        )
        .expect("the record's seller is the form's kind")
        .value,
        buyer: <F::Identity as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.buyer.clone(),
        )
        .expect("the record's buyer is the form's kind")
        .value,
        expiry,
    }
}

fn rebuild_transfer_context<F: IChainForms>(
    context: &ProofVectorContext,
) -> proof::DeliveryContext<F, TransferPurpose> {
    let identifier = context.suite_identifier;
    let version = context.suite_version;
    let asset_root = context.asset_root;
    let parameter_set = context.parameter_set;
    let purpose = context.purpose;
    let expiry = context.expiry;
    proof::DeliveryContext {
        suite_identifier: SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
            identifier,
            version,
        })
        .expect("the record's suite identifier admits"),
        chain: <F::ChainIdentifier as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.chain.clone(),
        )
        .expect("the record's chain value is the form's kind")
        .value,
        entitlement_contract: <F::Identity as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.entitlement_contract.clone(),
        )
        .expect("the record's contract value is the form's kind")
        .value,
        asset_identity_hash: AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
            bytes: asset_root,
        })
        .expect("the record's asset root admits"),
        parameter_set_identifier: ParameterSetIdentifier::try_new(
            ParameterSetIdentifierConstructorParams {
                bytes: parameter_set,
            },
        )
        .expect("the record's parameter set admits"),
        source_entitlement: <F::Entitlement as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.source_entitlement.clone(),
        )
        .expect("the record's source entitlement is the form's kind")
        .value,
        target_entitlement: <F::Entitlement as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.target_entitlement.clone(),
        )
        .expect("the record's target entitlement is the form's kind")
        .value,
        old_interval: <F::Interval as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.old_interval.clone(),
        )
        .expect("the record's old interval is the form's kind")
        .value,
        new_interval: <F::Interval as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.new_interval.clone(),
        )
        .expect("the record's new interval is the form's kind")
        .value,
        purpose: TransferPurpose::try_from(purpose).expect("the record's purpose is declared"),
        seller: <F::Identity as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.seller.clone(),
        )
        .expect("the record's seller is the form's kind")
        .value,
        buyer: <F::Identity as ICanonicalField>::from_field(
            build_from_field_params(Default::default()),
            context.buyer.clone(),
        )
        .expect("the record's buyer is the form's kind")
        .value,
        expiry,
    }
}

fn rebuild_mint_proof<P: IPairingArithmetic, D: IDeliveryProofAdapter<Pairing = P>>(
    pairing: &P,
    delivery_proof: &D,
    record: &MintProofVector,
) -> D::Proof {
    let responses = MintResponses {
        alpha: decode_scalar::<P>(pairing, &record.alpha),
        r: decode_scalar::<P>(pairing, &record.r),
        rho: decode_scalar::<P>(pairing, &record.rho),
        sigma: decode_scalar::<P>(pairing, &record.sigma),
    };
    let challenge = decode_scalar::<P>(pairing, &record.challenge);
    let components = if record.second_group_first_messages.is_empty() {
        DeliveryProofComponents::MintBothGroups {
            challenge,
            responses,
        }
    } else {
        let chunk = record.second_group_first_messages.len() / 3;
        DeliveryProofComponents::MintFirstGroupOnly {
            challenge,
            responses,
            first_messages: MintSecondGroupFirstMessages {
                hpub: decode_g2::<P>(pairing, &record.second_group_first_messages[0..chunk]),
                d1: decode_g2::<P>(
                    pairing,
                    &record.second_group_first_messages[chunk..2 * chunk],
                ),
                d2: decode_g2::<P>(pairing, &record.second_group_first_messages[2 * chunk..]),
            },
        }
    };
    delivery_proof
        .proof_from_components(
            ProofFromComponentsParams,
            ProofFromComponentsPayload { components },
        )
        .expect("the components rebuild the proof")
        .proof
}

fn rebuild_transfer_proof<P: IPairingArithmetic, D: IDeliveryProofAdapter<Pairing = P>>(
    pairing: &P,
    delivery_proof: &D,
    record: &TransferProofVector,
) -> D::Proof {
    let responses = TransferResponses {
        x: decode_scalar::<P>(pairing, &record.x),
        y: decode_scalar::<P>(pairing, &record.y),
        s: decode_scalar::<P>(pairing, &record.s),
        rho: decode_scalar::<P>(pairing, &record.rho),
        sigma: decode_scalar::<P>(pairing, &record.sigma),
    };
    let challenge = decode_scalar::<P>(pairing, &record.challenge);
    let components = if record.second_group_first_messages.is_empty() {
        DeliveryProofComponents::TransferBothGroups {
            challenge,
            responses,
        }
    } else {
        let chunk = record.second_group_first_messages.len() / 3;
        DeliveryProofComponents::TransferFirstGroupOnly {
            challenge,
            responses,
            first_messages: TransferSecondGroupFirstMessages {
                pk2: decode_g2::<P>(pairing, &record.second_group_first_messages[0..chunk]),
                d1: decode_g2::<P>(
                    pairing,
                    &record.second_group_first_messages[chunk..2 * chunk],
                ),
                d2: decode_g2::<P>(pairing, &record.second_group_first_messages[2 * chunk..]),
            },
        }
    };
    delivery_proof
        .proof_from_components(
            ProofFromComponentsParams,
            ProofFromComponentsPayload { components },
        )
        .expect("the components rebuild the proof")
        .proof
}
