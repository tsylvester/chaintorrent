#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, SuiteIdentifierConstructorParamsOverrides,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{
    CanonicalFieldValue, ICanonicalField, IDecoderAdapter, IEncoderAdapter, build_from_field_params,
};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IdentityScope,
    SetupParamsOverrides, SetupPayloadOverrides, SetupScope, build_create_kem_params,
    build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1PayloadOverrides, EncodeG2Params,
    EncodeG2PayloadOverrides, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, PAIRING_CONCRETES, VerifierGroupArithmetic,
    build_create_pairing_params, build_encode_g1_payload, build_encode_g2_payload,
};
use proof::{
    DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_REPLACEMENT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_delivery_context,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, FillBytesParams, FillBytesPayloadOverrides,
    build_create_random_source_params, build_fill_bytes_payload, create_random_source,
};

use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::interface::{
    ConsumeProofVectorsCaseParams, ConsumeProofVectorsCasePayload, IProofVectorsCaseConsumer,
};
use super::mock::{
    ProofVectorContextOverrides, ProofVectorsCaseParamsOverrides, ProofVectorsParamsOverrides,
    ProofVectorsPayloadOverrides, build_proof_vector_context, build_proof_vectors_case_deps,
    build_proof_vectors_case_params, build_proof_vectors_case_payload, build_proof_vectors_deps,
    build_proof_vectors_params, build_proof_vectors_payload, run_proof_vectors_case,
};
use super::proof_vectors;

/// Contract: Computed → `Ok(ProofVectorsSuccessReturn { mint, transfer })`, a
///   record for each index from `0` below `params.mint_count.get()` and below
///   `params.transfer_count.get()`, each list in index order.
/// Arrange: `mint_count` of `2` and `transfer_count` of `3`, counts that differ
///   from each other and from the builder's default, so a list sized from the
///   other count or the default reports otherwise.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: `mint` holds `2` records and `transfer` holds `3`.
/// Collaborators: every family's mock concrete in its default mode through its
///   factory.
#[test]
fn proof_vectors_emit_the_configured_counts_of_mint_and_transfer_records() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
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
            .expect("the run produces the records");

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
/// Assert: the mint record's `draws` hold `6` times `<P::Scalar as
///   ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes and the transfer
///   record's `draws` hold `9` times that length.
/// Collaborators: every family's mock concrete in its default mode through its
///   factory.
#[test]
fn proof_vectors_draw_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let uniform_length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
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
            assert_eq!(success.mint[0].draws.len(), 6 * uniform_length);
            assert_eq!(success.transfer[0].draws.len(), 9 * uniform_length);
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
/// Collaborators: every family's mock concrete in its default mode through its
///   factory; the independent source the randomness family's mock concrete at
///   the default selection.
#[test]
fn proof_vectors_record_the_draws_in_the_sequence_the_source_yields() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the source");
            let uniform_length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let mut calls = Vec::with_capacity(30);
            for _ in 0..30 {
                calls.push(
                    independent
                        .adapter
                        .fill_bytes(
                            FillBytesParams,
                            build_fill_bytes_payload(FillBytesPayloadOverrides {
                                length: Some(uniform_length),
                            }),
                        )
                        .expect("the source yields")
                        .bytes
                        .expose()
                        .clone(),
                );
            }
            let expected = |offset: usize, count: usize| calls[offset..offset + count].concat();
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
            .expect("the run produces the records");

            // Assert
            assert_eq!(success.mint[0].draws, expected(0, 6));
            assert_eq!(success.mint[1].draws, expected(6, 6));
            assert_eq!(success.transfer[0].draws, expected(12, 9));
            assert_eq!(success.transfer[1].draws, expected(21, 9));
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
///   `DELIVERY_PURPOSE_REPLACEMENT` and `DELIVERY_PURPOSE_GRANT`, and each
///   context's fields overridden so every form-typed field and every domain
///   value is distinct within and between the two contexts.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: each record's `context` equals `build_proof_vector_context` over the
///   values its typed context was built with, the mint's `purpose` `4` and the
///   transfer's `3`.
/// Collaborators: every family's mock concrete in its default mode through its
///   factory.
#[test]
fn proof_vectors_place_each_contexts_configured_values_in_its_records() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let chain_identifier = |bytes: [u8; 32]| {
                F::ChainIdentifier::from_field(
                    build_from_field_params(Default::default()),
                    CanonicalFieldValue::Unsigned256(bytes),
                )
                .expect("the chain identifier form admits the field")
                .value
            };
            let identity_form = |bytes: [u8; 20]| {
                F::Identity::from_field(
                    build_from_field_params(Default::default()),
                    CanonicalFieldValue::FixedBytes20(bytes),
                )
                .expect("the identity form admits the field")
                .value
            };
            let entitlement = |bytes: [u8; 32]| {
                F::Entitlement::from_field(
                    build_from_field_params(Default::default()),
                    CanonicalFieldValue::Unsigned256(bytes),
                )
                .expect("the entitlement form admits the field")
                .value
            };
            let interval = |value: u64| {
                F::Interval::from_field(
                    build_from_field_params(Default::default()),
                    CanonicalFieldValue::Unsigned64(value),
                )
                .expect("the interval form admits the field")
                .value
            };
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_REPLACEMENT,
                DeliveryContextOverrides {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x72; 32]),
                            version: Some(2),
                        },
                    )),
                    chain: Some(chain_identifier([0x23; 32])),
                    entitlement_contract: Some(identity_form([0x33; 20])),
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x74; 32]),
                    })),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x76; 32]),
                        },
                    )),
                    source_entitlement: Some(entitlement([0x43; 32])),
                    target_entitlement: Some(entitlement([0x44; 32])),
                    old_interval: Some(interval(5)),
                    new_interval: Some(interval(6)),
                    seller: Some(identity_form([0x63; 20])),
                    buyer: Some(identity_form([0x64; 20])),
                    expiry: Some(1_800_000_001),
                },
            );
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                DeliveryContextOverrides {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x73; 32]),
                            version: Some(3),
                        },
                    )),
                    chain: Some(chain_identifier([0x24; 32])),
                    entitlement_contract: Some(identity_form([0x32; 20])),
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x75; 32]),
                    })),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x77; 32]),
                        },
                    )),
                    source_entitlement: Some(entitlement([0x45; 32])),
                    target_entitlement: Some(entitlement([0x46; 32])),
                    old_interval: Some(interval(6)),
                    new_interval: Some(interval(7)),
                    seller: Some(identity_form([0x65; 20])),
                    buyer: Some(identity_form([0x66; 20])),
                    expiry: Some(1_900_000_000),
                },
            );
            let expected_mint = build_proof_vector_context(ProofVectorContextOverrides {
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
            });
            let expected_transfer = build_proof_vector_context(ProofVectorContextOverrides {
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
            });

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
            assert_eq!(success.mint[0].context, expected_mint);
            assert_eq!(success.transfer[0].context, expected_transfer);
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

/// Contract: Mint record, the admitted identity `KemIdentity::Entitlement
///   { canonical }` under `SetupScope::Entitlement` handed to the mint delivery
///   → the mint record's `identity_element` is the `encode_g1` bytes of the
///   derived identity element's `element`.
/// Arrange: `mint_count` and `transfer_count` of `1`, the identity
///   `b"pkg@1.0.0#1"`; the expected element from the received KEM over the
///   independent source's first three draws as `master_uniform`, `u0_uniform`,
///   and `u1_uniform`: `setup` under `SetupScope::Entitlement`,
///   `derive_identity` over `KemIdentity::Entitlement { canonical: &identity }`
///   and the parameter set `setup` returned, `identity_element_components` over
///   the result, and the received pairing's `encode_g1` over `element`, each
///   call through the family's builders.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `identity_element` equals the expected bytes.
/// Collaborators: every family's mock concrete in its default mode through its
///   factory; the independent source the randomness family's mock concrete at
///   the default selection.
#[test]
fn proof_vectors_carry_the_identity_element_of_an_entitlement_identity() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the source");
            let draw = || {
                independent
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the source yields")
                    .bytes
            };
            let parameter_set = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(SetupScope::Entitlement),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("setup yields the parameter set")
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
                .expect("the admitted identity derives")
                .identity_element;
            let element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element opens")
                .components
                .element;
            let expected = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(element),
                        },
                    ),
                )
                .expect("the element encodes")
                .bytes;
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
            assert_eq!(
                success.mint[0].identity_element.as_slice(),
                expected.as_ref()
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

/// Contract: Mint record, the admitted identity `KemIdentity::Asset
///   { root: &root }` under `SetupScope::Asset { root: &root }` handed to the
///   mint delivery → the mint record's `identity_element` is the `encode_g1`
///   bytes of the derived identity element's `element`.
/// Arrange: `build_create_kem_params` overriding `scope` with
///   `IdentityScope::Asset`, the root `build_root` builds from the bytes
///   `[0x44; 32]`, `mint_count` and `transfer_count` of `1`; the expected
///   element as the entitlement block builds it, over that root,
///   `SetupScope::Asset`, and `KemIdentity::Asset`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `identity_element` equals the expected bytes.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the independent source the randomness family's mock
///   concrete at the default selection.
#[test]
fn proof_vectors_carry_the_identity_element_of_an_asset_identity() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            // Arrange
            let independent = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the source");
            let draw = || {
                independent
                    .adapter
                    .fill_bytes(
                        FillBytesParams,
                        build_fill_bytes_payload(FillBytesPayloadOverrides {
                            length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                        }),
                    )
                    .expect("the source yields")
                    .bytes
            };
            let parameter_set = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(draw()),
                        u0_uniform: Some(draw()),
                        u1_uniform: Some(draw()),
                    }),
                )
                .expect("setup yields the parameter set")
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
                .expect("the admitted identity derives")
                .identity_element;
            let element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element opens")
                .components
                .element;
            let expected = pairing
                .encode_g1(
                    EncodeG1Params,
                    build_encode_g1_payload(
                        pairing,
                        EncodeG1PayloadOverrides {
                            point: Some(element),
                        },
                    ),
                )
                .expect("the element encodes")
                .bytes;
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
            assert_eq!(
                success.mint[0].identity_element.as_slice(),
                expected.as_ref()
            );
        }
    }

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
            build_proof_vectors_case_payload(
                b"pkg@1.0.0#1",
                &build_root(RootConstructorParamsOverrides {
                    bytes: Some([0x44; 32]),
                }),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // proof_vectors_carry_the_identity_element_of_an_asset_identity
}

/// Contract: Mint proof components, `MintFirstGroupOnly { challenge, responses,
///   first_messages }` → `second_group_first_messages` holds one `encode_g2`
///   encoding for each of `first_messages.hpub`, `d1`, and `d2`.
/// Arrange: `mint_count` and `transfer_count` of `1`; the case varying through
///   `PAIRING_CONCRETES`, whose BN254 concretes declare `VerifierGroupArithmetic::
///   FirstGroupOnly`; the expected length from the received pairing: the
///   length of `encode_g2` over its second-group generator, read through the
///   pairing's generator operation, times `3`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `second_group_first_messages` holds the expected
///   length in bytes.
/// Collaborators: the selected pairing concrete through its factory, every
///   other family's mock concrete in its default mode.
#[test]
fn proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_mint() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            if P::DECLARATION.verifier_group_arithmetic != VerifierGroupArithmetic::FirstGroupOnly {
                return;
            }

            // Arrange
            let expected = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(
                                pairing
                                    .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                                    .expect("the generator is reported")
                                    .point,
                            ),
                        },
                    ),
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len()
                * 3;
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
            assert_eq!(success.mint[0].second_group_first_messages.len(), expected);
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(concrete),
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
/// Collaborators: the selected pairing concrete through its factory, every
///   other family's mock concrete in its default mode.
#[test]
fn proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_transfer() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            if P::DECLARATION.verifier_group_arithmetic != VerifierGroupArithmetic::FirstGroupOnly {
                return;
            }

            // Arrange
            let expected = pairing
                .encode_g2(
                    EncodeG2Params,
                    build_encode_g2_payload(
                        pairing,
                        EncodeG2PayloadOverrides {
                            point: Some(
                                pairing
                                    .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                                    .expect("the generator is reported")
                                    .point,
                            ),
                        },
                    ),
                )
                .expect("the generator encodes")
                .bytes
                .as_ref()
                .len()
                * 3;
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
            assert_eq!(
                success.transfer[0].second_group_first_messages.len(),
                expected
            );
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(concrete),
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
/// Arrange: `mint_count` and `transfer_count` of `1`; the case varying through
///   the member of `PAIRING_CONCRETES` whose declaration names
///   `VerifierGroupArithmetic::BothGroups`.
/// Act:    `proof_vectors`, once, in the case consumer's method.
/// Assert: the mint record's `second_group_first_messages` equals `Vec::new()`.
/// Collaborators: the selected pairing concrete through its factory, every
///   other family's mock concrete in its default mode.
#[test]
fn proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_mint() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            if P::DECLARATION.verifier_group_arithmetic != VerifierGroupArithmetic::BothGroups {
                return;
            }

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
            .expect("the run produces the records");

            // Assert
            assert_eq!(
                success.mint[0].second_group_first_messages,
                Vec::<u8>::new()
            );
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(concrete),
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
/// Collaborators: the selected pairing concrete through its factory, every
///   other family's mock concrete in its default mode.
#[test]
fn proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_transfer() {
    struct Consumer;

    impl IProofVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_proof_vectors_case<P, H, E, F, K, A, D>(
            &self,
            _params: ConsumeProofVectorsCaseParams,
            payload: ConsumeProofVectorsCasePayload<'_, P, H, E, K, A, D>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let delivery_proof = payload.delivery_proof;
            let random = payload.random;
            let identity = payload.identity;
            if P::DECLARATION.verifier_group_arithmetic != VerifierGroupArithmetic::BothGroups {
                return;
            }

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
            .expect("the run produces the records");

            // Assert
            assert_eq!(
                success.transfer[0].second_group_first_messages,
                Vec::<u8>::new()
            );
        }
    }

    for concrete in PAIRING_CONCRETES.iter().copied() {
        assert!(
            run_proof_vectors_case(
                &build_proof_vectors_case_deps(Consumer, Default::default()),
                build_proof_vectors_case_params(ProofVectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(concrete),
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
