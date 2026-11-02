#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{
    AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{
    CanonicalFieldValue, ICanonicalField, IDecoderAdapter, IEncoderAdapter, build_from_field_params,
};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityParams, DeriveIdentityPayload, ICredentialKemAdapter,
    IdentityElementComponentsParams, IdentityElementComponentsPayload, IdentityScope, SetupParams,
    SetupPayload, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, VerifierGroupArithmetic, build_create_pairing_params,
};
use proof::{
    DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_REPLACEMENT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_delivery_context,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, build_create_random_source_params, build_fill_bytes_payload,
    create_random_source,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::interface::IProofVectorsCaseConsumer;
use super::mock::{
    ProofVectorContextOverrides, ProofVectorsCaseParamsOverrides, ProofVectorsParamsOverrides,
    ProofVectorsPayloadOverrides, build_proof_vector_context, build_proof_vectors_case_deps,
    build_proof_vectors_case_params, build_proof_vectors_case_payload, build_proof_vectors_deps,
    build_proof_vectors_params, build_proof_vectors_payload, run_proof_vectors_case,
};
use super::proof_vectors;
use crate::delivery::provides::AdmittedKemIdentity;

/// Contract: Computed → `Ok(ProofVectorsSuccessReturn { mint, transfer })`, a
///   record for each index from `0` below `params.mint_count.get()` and below
///   `params.transfer_count.get()`, each list in index order.
/// Arrange: `mint_count` of `2` and `transfer_count` of `3`, counts that
///   differ from each other and from the builder's default, so a list sized
///   from the other count or the default reports otherwise.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `mint` holds `2` records and `transfer` holds `3`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn proof_vectors_emit_the_configured_counts_of_mint_and_transfer_records() {
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
            .expect("the subject's calls succeed");

            // Assert
            assert_eq!(success.mint.len(), 2);
            assert_eq!(success.transfer.len(), 3);
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(Default::default()),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_emit_the_configured_counts_of_mint_and_transfer_records
}

/// Contract: Mint record and transfer record → every uniform has the length
///   the pairing's scalar sampling bound declares, and a record's `draws` are
///   its deliveries' uniforms concatenated in draw order; a mint record holds
///   the mint delivery's master, `u0`, `u1`, issuance, recipient `x`, and
///   recipient `y` uniforms, and a transfer record those and the transfer
///   delivery's buyer `x`, buyer `y`, and rerandomization uniforms.
/// Arrange: `mint_count` and `transfer_count` of `1`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `draws` hold `6` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes and the
///   transfer record's `draws` hold `9` times that length.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn proof_vectors_draw_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
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
            .expect("the subject's calls succeed");

            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(success.mint[0].draws.len(), 6 * length);
            assert_eq!(success.transfer[0].draws.len(), 9 * length);
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(Default::default()),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_draw_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the mint records, then the transfer records; within a
///   record its deliveries' draws in the order master, `u0`, `u1`, issuance,
///   recipient `x` and `y`, and for a transfer buyer `x` and `y` and
///   rerandomization.
/// Arrange: `mint_count` of `2` and `transfer_count` of `2`; the expected
///   draws from the independent source, consecutive calls grouped for each
///   mint record in turn and then each transfer record in turn, so a
///   transposed record or draw reports otherwise.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: each record's `draws` equal its expected draws.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection, and the independent source a
///   second concrete of the same selection.
#[test]
fn proof_vectors_record_the_draws_in_the_sequence_the_source_yields() {
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
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the source")
            .adapter;
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw = |source: &dyn IRandomSourceAdapter| {
                source
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the mock source fills")
                    .bytes
                    .expose()
                    .clone()
            };
            let expected_mint: Vec<Vec<u8>> = (0..2)
                .map(|_| (0..6).flat_map(|_| draw(&*independent)).collect())
                .collect();
            let expected_transfer: Vec<Vec<u8>> = (0..2)
                .map(|_| (0..9).flat_map(|_| draw(&*independent)).collect())
                .collect();
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
            .expect("the subject's calls succeed");

            // Assert
            for (record, expected) in success.mint.iter().zip(expected_mint.iter()) {
                assert_eq!(record.draws.as_slice(), expected.as_slice());
            }
            for (record, expected) in success.transfer.iter().zip(expected_transfer.iter()) {
                assert_eq!(record.draws.as_slice(), expected.as_slice());
            }
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(Default::default()),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_record_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Record context → a `ProofVectorContext` read from the delivery
///   statement's typed context: `suite_identifier` a copy of `identifier()`,
///   `suite_version` its `version()`, each form-typed value the field its
///   form's `to_field` returns, `asset_root` and `parameter_set` copies of
///   their `as_bytes()`, `purpose` the purpose's `code()`, and `expiry` the
///   context's.
/// Arrange: `mint_count` and `transfer_count` of `1`, the purposes
///   `DELIVERY_PURPOSE_REPLACEMENT` and `DELIVERY_PURPOSE_GRANT`, the mint
///   context and the transfer context each overriding every form-typed field
///   and every domain value, every value distinct within and between the two
///   contexts.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `context` equals `build_proof_vector_context`
///   over the mint context's values with `purpose` `4`, and the transfer
///   record's `context` equals it over the transfer context's values with
///   `purpose` `3`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn proof_vectors_place_each_contexts_configured_values_in_its_records() {
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
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_REPLACEMENT,
                DeliveryContextOverrides::<F> {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x72; 32]),
                            version: Some(2),
                        },
                    )),
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x23; 32]),
                        )
                        .expect("the value is the chain identifier's kind")
                        .value,
                    ),
                    entitlement_contract: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x33; 20]),
                        )
                        .expect("the value is the identity's kind")
                        .value,
                    ),
                    asset_identity_hash: Some(build_asset_identity_hash(
                        AssetIdentityHashConstructorParamsOverrides {
                            bytes: Some([0x74; 32]),
                        },
                    )),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x76; 32]),
                        },
                    )),
                    source_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x43; 32]),
                        )
                        .expect("the value is the entitlement's kind")
                        .value,
                    ),
                    target_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x44; 32]),
                        )
                        .expect("the value is the entitlement's kind")
                        .value,
                    ),
                    old_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(5),
                        )
                        .expect("the value is the interval's kind")
                        .value,
                    ),
                    new_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(6),
                        )
                        .expect("the value is the interval's kind")
                        .value,
                    ),
                    seller: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x63; 20]),
                        )
                        .expect("the value is the identity's kind")
                        .value,
                    ),
                    buyer: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x64; 20]),
                        )
                        .expect("the value is the identity's kind")
                        .value,
                    ),
                    expiry: Some(1_800_000_001),
                },
            );
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                DeliveryContextOverrides::<F> {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x73; 32]),
                            version: Some(3),
                        },
                    )),
                    chain: Some(
                        <F::ChainIdentifier as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x24; 32]),
                        )
                        .expect("the value is the chain identifier's kind")
                        .value,
                    ),
                    entitlement_contract: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x32; 20]),
                        )
                        .expect("the value is the identity's kind")
                        .value,
                    ),
                    asset_identity_hash: Some(build_asset_identity_hash(
                        AssetIdentityHashConstructorParamsOverrides {
                            bytes: Some([0x75; 32]),
                        },
                    )),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x77; 32]),
                        },
                    )),
                    source_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x45; 32]),
                        )
                        .expect("the value is the entitlement's kind")
                        .value,
                    ),
                    target_entitlement: Some(
                        <F::Entitlement as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x46; 32]),
                        )
                        .expect("the value is the entitlement's kind")
                        .value,
                    ),
                    old_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(6),
                        )
                        .expect("the value is the interval's kind")
                        .value,
                    ),
                    new_interval: Some(
                        <F::Interval as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(7),
                        )
                        .expect("the value is the interval's kind")
                        .value,
                    ),
                    seller: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x65; 20]),
                        )
                        .expect("the value is the identity's kind")
                        .value,
                    ),
                    buyer: Some(
                        <F::Identity as ICanonicalField>::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x66; 20]),
                        )
                        .expect("the value is the identity's kind")
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
            .expect("the subject's calls succeed");

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
                    purpose: Some(4),
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
                    purpose: Some(3),
                    seller: Some(CanonicalFieldValue::FixedBytes20([0x65; 20])),
                    buyer: Some(CanonicalFieldValue::FixedBytes20([0x66; 20])),
                    expiry: Some(1_900_000_000),
                })
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(Default::default()),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_place_each_contexts_configured_values_in_its_records
}

/// Contract: Mint record, the admitted identity `KemIdentity::Entitlement`
///   under `SetupScope::Entitlement` handed to the mint delivery → the mint
///   record's `identity_element` is the `encode_g1` bytes of the derived
///   identity element's `element`.
/// Arrange: `mint_count` and `transfer_count` of `1`, the identity
///   `b"pkg@1.0.0#1"`; the expected element from the received KEM over the
///   independent source's first three draws as `master_uniform`, `u0_uniform`,
///   and `u1_uniform`: `setup` under the admitted identity's setup scope,
///   `derive_identity` over its identity and the parameter set `setup`
///   returned, `identity_element_components` over the result, and the
///   received pairing's `encode_g1` over `element`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `identity_element` equals the expected bytes.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection, and the independent source a
///   second concrete of the same selection.
#[test]
fn proof_vectors_carry_the_identity_element_of_an_entitlement_identity() {
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
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the source")
            .adapter;
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw = |source: &dyn IRandomSourceAdapter| {
                source
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the mock source fills")
                    .bytes
            };
            let master_uniform = draw(&*independent);
            let u0_uniform = draw(&*independent);
            let u1_uniform = draw(&*independent);
            let setup = kem
                .setup(
                    SetupParams {
                        scope: identity.setup_scope,
                    },
                    SetupPayload {
                        master_uniform,
                        u0_uniform,
                        u1_uniform,
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
            let Ok(expected) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: components.components.element,
                },
            );
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
            .expect("the subject's calls succeed");

            // Assert
            assert_eq!(
                success.mint[0].identity_element.as_slice(),
                expected.bytes.as_ref()
            );
        }
    }

    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(Default::default()),
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_carry_the_identity_element_of_an_entitlement_identity
}

/// Contract: Mint record, the admitted identity `KemIdentity::Asset` under
///   `SetupScope::Asset` handed to the mint delivery → the mint record's
///   `identity_element` is the `encode_g1` bytes of the derived identity
///   element's `element`.
/// Arrange: `build_create_kem_params` overriding `scope` with
///   `IdentityScope::Asset`, the root `build_root` builds from the bytes
///   `[0x44; 32]`, `mint_count` and `transfer_count` of `1`; the expected
///   element as the entitlement block builds it, over that root,
///   `SetupScope::Asset`, and `KemIdentity::Asset`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `identity_element` equals the expected bytes.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection, and the independent source a
///   second concrete of the same selection.
#[test]
fn proof_vectors_carry_the_identity_element_of_an_asset_identity() {
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
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the source")
            .adapter;
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw = |source: &dyn IRandomSourceAdapter| {
                source
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(length),
                        }),
                    )
                    .expect("the mock source fills")
                    .bytes
            };
            let master_uniform = draw(&*independent);
            let u0_uniform = draw(&*independent);
            let u1_uniform = draw(&*independent);
            let setup = kem
                .setup(
                    SetupParams {
                        scope: identity.setup_scope,
                    },
                    SetupPayload {
                        master_uniform,
                        u0_uniform,
                        u1_uniform,
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
            let Ok(expected) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: components.components.element,
                },
            );
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
            .expect("the subject's calls succeed");

            // Assert
            assert_eq!(
                success.mint[0].identity_element.as_slice(),
                expected.bytes.as_ref()
            );
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    assert!(
        run_proof_vectors_case(
            &build_proof_vectors_case_deps(Consumer, Default::default()),
            build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(IdentityScope::Asset),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            build_proof_vectors_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
        )
        .is_ok()
    );
    // proof_vectors_carry_the_identity_element_of_an_asset_identity
}

/// Contract: Mint proof components, `MintFirstGroupOnly { challenge,
///   responses, first_messages }` → `second_group_first_messages` holds one
///   `encode_g2` encoding for each of `first_messages.hpub`, `d1`, and `d2`.
/// Arrange: `mint_count` and `transfer_count` of `1`; the case varying
///   through `PAIRING_CONCRETES`, whose BN254 concretes declare
///   `VerifierGroupArithmetic::FirstGroupOnly`; the expected length from the
///   received pairing: the length of `encode_g2` over its second-group
///   generator, read through the pairing's generator operation, times `3`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `second_group_first_messages` holds the expected
///   length in bytes.
/// Collaborators: the pairing family's real concretes through `create_pairing`,
///   every other family's mock concrete in its default mode through its
///   factory.
#[test]
fn proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_mint() {
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
            .expect("the subject's calls succeed");

            // Assert
            if P::DECLARATION.verifier_group_arithmetic == VerifierGroupArithmetic::FirstGroupOnly {
                assert_eq!(success.mint[0].second_group_first_messages.len(), expected);
            }
        }
    }

    for &member in PAIRING_CONCRETES {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(member),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                build_proof_vectors_case_payload(
                    b"pkg@1.0.0#1",
                    &build_root(Default::default()),
                    Default::default(),
                ),
            )
            .is_ok()
        );
    }
    // proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_mint
}

/// Contract: Transfer proof components, `TransferFirstGroupOnly { challenge,
///   responses, first_messages }` → `second_group_first_messages` holds one
///   `encode_g2` encoding for each of `first_messages.pk2`, `d1`, and `d2`.
/// Arrange: as the mint block.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the transfer record's `second_group_first_messages` holds the
///   expected length in bytes.
/// Collaborators: the pairing family's real concretes through `create_pairing`,
///   every other family's mock concrete in its default mode through its
///   factory.
#[test]
fn proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_transfer() {
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
            .expect("the subject's calls succeed");

            // Assert
            if P::DECLARATION.verifier_group_arithmetic == VerifierGroupArithmetic::FirstGroupOnly {
                assert_eq!(
                    success.transfer[0].second_group_first_messages.len(),
                    expected
                );
            }
        }
    }

    for &member in PAIRING_CONCRETES {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(member),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                build_proof_vectors_case_payload(
                    b"pkg@1.0.0#1",
                    &build_root(Default::default()),
                    Default::default(),
                ),
            )
            .is_ok()
        );
    }
    // proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_transfer
}

/// Contract: Mint proof components, `MintBothGroups { challenge, responses }`
///   → `second_group_first_messages` empty.
/// Arrange: `mint_count` and `transfer_count` of `1`; the case varying
///   through the members of `PAIRING_CONCRETES`, whose BLS12-381 concretes
///   declare `VerifierGroupArithmetic::BothGroups`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `second_group_first_messages` equals
///   `Vec::new()`.
/// Collaborators: the pairing family's real concretes through `create_pairing`,
///   every other family's mock concrete in its default mode through its
///   factory.
#[test]
fn proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_mint() {
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
            .expect("the subject's calls succeed");

            // Assert
            if P::DECLARATION.verifier_group_arithmetic == VerifierGroupArithmetic::BothGroups {
                assert_eq!(
                    success.mint[0].second_group_first_messages,
                    Vec::<u8>::new()
                );
            }
        }
    }

    for &member in PAIRING_CONCRETES {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(member),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                build_proof_vectors_case_payload(
                    b"pkg@1.0.0#1",
                    &build_root(Default::default()),
                    Default::default(),
                ),
            )
            .is_ok()
        );
    }
    // proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_mint
}

/// Contract: Transfer proof components, `TransferBothGroups { challenge,
///   responses }` → `second_group_first_messages` empty.
/// Arrange: as the mint block.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the transfer record's `second_group_first_messages` equals
///   `Vec::new()`.
/// Collaborators: the pairing family's real concretes through `create_pairing`,
///   every other family's mock concrete in its default mode through its
///   factory.
#[test]
fn proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_transfer() {
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
            .expect("the subject's calls succeed");

            // Assert
            if P::DECLARATION.verifier_group_arithmetic == VerifierGroupArithmetic::BothGroups {
                assert_eq!(
                    success.transfer[0].second_group_first_messages,
                    Vec::<u8>::new()
                );
            }
        }
    }

    for &member in PAIRING_CONCRETES {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(member),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                build_proof_vectors_case_payload(
                    b"pkg@1.0.0#1",
                    &build_root(Default::default()),
                    Default::default(),
                ),
            )
            .is_ok()
        );
    }
    // proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_transfer
}
