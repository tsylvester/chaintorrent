#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter,
};
use envelope::{
    CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS, GenerateKeysErrorReturn,
    IKeyAgreementAdapter, KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params, build_generate_keys_params,
    build_generate_keys_payload, build_key_pair_public_keys_params,
    build_key_pair_public_keys_payload,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
    build_domain_tag, expected_uniform_bytes,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides,
    ICredentialKemAdapter, IdentityScope, KemIdentity, SetupErrorReturn, SetupParamsOverrides,
    SetupPayloadOverrides, build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_parameter_set_components_params, build_parameter_set_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES,
    PairingConcrete, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryProofComponents,
    DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, VerifyParams, VerifyPayload,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::interface::{
    IStatementMutationsCaseConsumer, MINT_MUTATION_FIELDS, MintMutationField,
    StatementMutationsCaseParams, StatementMutationsErrorReturn, TRANSFER_MUTATION_FIELDS,
    TransferMutationField,
};
use super::mock::{
    StatementMutationsCaseParamsOverrides, StatementMutationsParamsOverrides,
    StatementMutationsPayloadOverrides, build_statement_mutations_case_deps,
    build_statement_mutations_case_params, build_statement_mutations_case_payload,
    build_statement_mutations_deps, build_statement_mutations_params,
    build_statement_mutations_payload, run_statement_mutations_case,
};
use super::statement_mutations;
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};
use crate::{VectorCountConstructorParamsOverrides, build_vector_count};

const CANONICAL_A: &[u8] = b"pkg@1.0.0#1";
const CANONICAL_B: &[u8] = b"pkg@2.0.0#7";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

fn case_params(overrides: StatementMutationsCaseParamsOverrides) -> StatementMutationsCaseParams {
    build_statement_mutations_case_params(overrides)
}

fn all_cases(
    mut params: impl FnMut(
        PairingConcrete,
        &hash_to_scalar::CreateHashToScalarParams,
        &encoding::CreateEncodingParams,
        &chain::ChainFormsConcrete,
        &kem::CreateKemParams,
        &envelope::CreateKeyAgreementParams,
        &proof::DeliveryProofConcrete,
    ),
) {
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                params(
                                    *pairing_concrete,
                                    h2s_selection,
                                    encoding_selection,
                                    forms_concrete,
                                    kem_selection,
                                    key_agreement_selection,
                                    proof_concrete,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn selections(
    pairing_concrete: PairingConcrete,
    h2s_selection: &hash_to_scalar::CreateHashToScalarParams,
    encoding_selection: &encoding::CreateEncodingParams,
    forms_concrete: &chain::ChainFormsConcrete,
    kem_selection: &kem::CreateKemParams,
    key_agreement_selection: &envelope::CreateKeyAgreementParams,
    proof_concrete: &proof::DeliveryProofConcrete,
) -> StatementMutationsCaseParams {
    case_params(StatementMutationsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        })),
        hash_to_scalar: Some(*h2s_selection),
        encoding: Some(CreateEncodingParams {
            concrete: encoding_selection.concrete,
            identifier: encoding_selection.identifier,
        }),
        chain_forms: Some(build_create_chain_forms_params(
            CreateChainFormsParamsOverrides {
                concrete: Some(*forms_concrete),
                ..Default::default()
            },
        )),
        kem: Some(*kem_selection),
        key_agreement: Some(*key_agreement_selection),
        delivery_proof: Some(build_create_delivery_proof_params(
            CreateDeliveryProofParamsOverrides {
                concrete: Some(*proof_concrete),
                ..Default::default()
            },
        )),
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::OperatingSystem),
            },
        )),
    })
}

fn encode_g1<P: IPairingArithmetic>(pairing: &P, point: &P::G1) -> Vec<u8> {
    pairing
        .encode_g1(
            EncodeG1Params,
            EncodeG1Payload {
                point: point.clone(),
            },
        )
        .expect("the first-group encoding is infallible")
        .bytes
        .as_ref()
        .to_vec()
}

fn encode_g2<P: IPairingArithmetic>(pairing: &P, point: &P::G2) -> Vec<u8> {
    pairing
        .encode_g2(
            EncodeG2Params,
            EncodeG2Payload {
                point: point.clone(),
            },
        )
        .expect("the second-group encoding is infallible")
        .bytes
        .as_ref()
        .to_vec()
}

fn encode_scalar<P: IPairingArithmetic>(pairing: &P, scalar: &P::Scalar) -> Vec<u8> {
    pairing
        .encode_scalar(
            EncodeScalarParams,
            EncodeScalarPayload {
                scalar: scalar.clone(),
            },
        )
        .expect("the scalar encoding is infallible")
        .bytes
        .expose()
        .as_ref()
        .to_vec()
}

/// Contract: `statement_mutations` over a case with a count above one, every
///   adapter real and the operating-system source real → both baselines of
///   every record verify, every record holds one mutation per field in each
///   constant's order, and every mutation is refused.
/// Arrange: `count` of `2`; every case.
/// Act:     `statement_mutations`.
/// Assert:  `vectors` holds `2` records; for each, `mint_is_valid` and
///   `transfer_is_valid` are `true`, the `field`s of `mint_mutations` equal
///   `MINT_MUTATION_FIELDS` and those of `transfer_mutations` equal
///   `TRANSFER_MUTATION_FIELDS` in order, and every mutation's `is_valid` is
///   `false`.
/// Boundary: `statement_mutations` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_baseline_statement_verifies_and_every_mutation_is_refused() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(2),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.mint_is_valid);
                assert!(record.transfer_is_valid);
                assert_eq!(
                    record
                        .mint_mutations
                        .iter()
                        .map(|mutation| mutation.field)
                        .collect::<Vec<_>>(),
                    MINT_MUTATION_FIELDS.to_vec()
                );
                assert_eq!(
                    record
                        .transfer_mutations
                        .iter()
                        .map(|mutation| mutation.field)
                        .collect::<Vec<_>>(),
                    TRANSFER_MUTATION_FIELDS.to_vec()
                );
                for mutation in &record.mint_mutations {
                    assert!(!mutation.is_valid);
                }
                for mutation in &record.transfer_mutations {
                    assert!(!mutation.is_valid);
                }
            }
        }
    }
    all_cases(|pairing_concrete, h2s, enc, forms, kem, ka, proof| {
        let result = run_statement_mutations_case(
            &build_statement_mutations_case_deps(Consumer, Default::default()),
            selections(pairing_concrete, h2s, enc, forms, kem, ka, proof),
            build_statement_mutations_case_payload(CANONICAL_A, &asset_root(), Default::default()),
        );
        assert!(result.is_ok());
    });
}

/// Contract: every statement mutation differs from the baseline in the field
///   it names alone, by that field's mutation, and carries the baseline
///   proof's components; every proof-scalar mutation carries the baseline
///   statement and the baseline proof's components but for the scalar it
///   names, which is the baseline's plus the record's offset.
/// Arrange: `count` of `1`; every case; the record's offset replayed from its
///   tenth uniform through `sample_from_uniform_bytes`.
/// Act:     `statement_mutations`.
/// Assert:  for every statement mutation, each field of its statement
///   encodes, or reads back through `to_field`, to the baseline's but the one
///   it names, which equals the expected value, and the components of its
///   proof encode to the baseline proof's; for every proof-scalar mutation,
///   each field of its statement equals the baseline's and each component of
///   its proof encodes to the baseline proof's but the one it names.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_mutation_replaces_its_field_alone() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            let offset_uniform =
                &record.draws[9 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH..];
            let offset = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: domain::Secret::try_new(domain::SecretConstructorParams {
                        value: offset_uniform.to_vec(),
                    })
                    .expect("a draw of the declared length is a valid uniform"),
                },
            )
            .expect("a uniform of the declared length samples")
            .scalar
            .expose()
            .clone();
            let mutated_scalar = |baseline: &P::Scalar| -> P::Scalar {
                pairing
                    .add_scalar(
                        AddScalarParams,
                        AddScalarPayload {
                            left: baseline.clone(),
                            right: offset.clone(),
                        },
                    )
                    .expect("the scalar sum is infallible")
                    .sum
            };
            let mint_baseline = delivery_proof
                .proof_components(
                    proof::ProofComponentsParams,
                    proof::ProofComponentsPayload {
                        proof: &record.mint_proof,
                    },
                )
                .expect("the mint baseline's components are infallible")
                .components;
            let transfer_baseline = delivery_proof
                .proof_components(
                    proof::ProofComponentsParams,
                    proof::ProofComponentsPayload {
                        proof: &record.transfer_proof,
                    },
                )
                .expect("the transfer baseline's components are infallible")
                .components;
            let (baseline_mint_challenge, baseline_mint_responses) = match mint_baseline {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => (challenge, responses),
                _ => panic!("a mint proof carries mint components"),
            };
            let (baseline_transfer_challenge, baseline_transfer_responses) = match transfer_baseline
            {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                }
                | DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    ..
                } => (challenge, responses),
                _ => panic!("a transfer proof carries transfer components"),
            };
            let mut expected_suite = *record.mint_statement.context.suite_identifier.identifier();
            expected_suite[31] ^= 0x01;
            let mut expected_root = *record.mint_statement.context.asset_root.as_bytes();
            expected_root[31] ^= 0x01;
            let mut expected_param = *record
                .mint_statement
                .context
                .parameter_set_identifier
                .as_bytes();
            expected_param[31] ^= 0x01;
            for (index, mutation) in record.mint_mutations.iter().enumerate() {
                let field = mutation.field;
                // The mutated statement equals the baseline in every field
                // but the one it names.
                if field == MintMutationField::SuiteIdentifier {
                    assert_eq!(
                        mutation.statement.context.suite_identifier.identifier(),
                        &expected_suite
                    );
                    assert_eq!(
                        mutation.statement.context.suite_identifier.version(),
                        record.mint_statement.context.suite_identifier.version()
                    );
                } else {
                    assert_eq!(
                        mutation.statement.context.suite_identifier.identifier(),
                        record.mint_statement.context.suite_identifier.identifier()
                    );
                    assert_eq!(
                        mutation.statement.context.suite_identifier.version(),
                        record.mint_statement.context.suite_identifier.version()
                    );
                }
                assert_eq!(
                    mutation.statement.context.asset_root,
                    if field == MintMutationField::AssetRoot {
                        hashing::Root::try_new(hashing::RootConstructorParams {
                            bytes: expected_root,
                        })
                        .expect("the mutated root is admitted")
                    } else {
                        record.mint_statement.context.asset_root
                    }
                );
                assert_eq!(
                    mutation
                        .statement
                        .context
                        .parameter_set_identifier
                        .as_bytes(),
                    if field == MintMutationField::ParameterSetIdentifier {
                        &expected_param
                    } else {
                        record
                            .mint_statement
                            .context
                            .parameter_set_identifier
                            .as_bytes()
                    }
                );
                assert_eq!(
                    mutation.statement.context.expiry,
                    if field == MintMutationField::Expiry {
                        record.mint_statement.context.expiry ^ 1
                    } else {
                        record.mint_statement.context.expiry
                    }
                );
                assert_eq!(
                    mutation.statement.context.purpose.code(),
                    if field == MintMutationField::Purpose {
                        match record.mint_statement.context.purpose {
                            MintPurpose::Mint => MintPurpose::Replacement.code(),
                            MintPurpose::Replacement => MintPurpose::Mint.code(),
                        }
                    } else {
                        record.mint_statement.context.purpose.code()
                    }
                );
                if field == MintMutationField::Hpub {
                    let g2 = pairing
                        .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                        .expect("generator")
                        .point;
                    let off = pairing
                        .mul_g2(
                            MulG2Params,
                            MulG2Payload {
                                point: g2,
                                scalar: offset.clone(),
                            },
                        )
                        .expect("offset")
                        .product;
                    let expected = pairing
                        .add_g2(
                            AddG2Params,
                            AddG2Payload {
                                left: record.mint_statement.hpub.clone(),
                                right: off,
                            },
                        )
                        .expect("sum")
                        .sum;
                    assert_eq!(
                        encode_g2::<P>(pairing, &mutation.statement.hpub),
                        encode_g2::<P>(pairing, &expected)
                    );
                }
                assert_eq!(
                    encode_g1::<P>(pairing, &mutation.statement.identity_element),
                    if field == MintMutationField::IdentityElement {
                        let g1 = pairing
                            .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                            .expect("generator")
                            .point;
                        let off = pairing
                            .mul_g1(
                                MulG1Params,
                                MulG1Payload {
                                    point: g1,
                                    scalar: offset.clone(),
                                },
                            )
                            .expect("offset")
                            .product;
                        let expected = pairing
                            .add_g1(
                                AddG1Params,
                                AddG1Payload {
                                    left: record.mint_statement.identity_element.clone(),
                                    right: off,
                                },
                            )
                            .expect("sum")
                            .sum;
                        encode_g1::<P>(pairing, &expected)
                    } else {
                        encode_g1::<P>(pairing, &record.mint_statement.identity_element)
                    }
                );
                // Every statement mutation's proof carries the baseline's
                // components.
                let components = delivery_proof
                    .proof_components(
                        proof::ProofComponentsParams,
                        proof::ProofComponentsPayload {
                            proof: &mutation.proof,
                        },
                    )
                    .expect("the mutation's components are infallible")
                    .components;
                let (challenge, responses) = match components {
                    DeliveryProofComponents::MintBothGroups {
                        challenge,
                        responses,
                    }
                    | DeliveryProofComponents::MintFirstGroupOnly {
                        challenge,
                        responses,
                        ..
                    } => (challenge, responses),
                    _ => panic!("a mint mutation's proof carries mint components"),
                };
                if index < 21 {
                    assert_eq!(
                        encode_scalar::<P>(pairing, &challenge),
                        encode_scalar::<P>(pairing, &baseline_mint_challenge)
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.alpha),
                        encode_scalar::<P>(pairing, &baseline_mint_responses.alpha)
                    );
                } else {
                    let expected = |name: MintMutationField, baseline: &P::Scalar| -> P::Scalar {
                        if name == field {
                            mutated_scalar(baseline)
                        } else {
                            baseline.clone()
                        }
                    };
                    assert_eq!(
                        encode_scalar::<P>(pairing, &challenge),
                        encode_scalar::<P>(
                            pairing,
                            &expected(MintMutationField::Challenge, &baseline_mint_challenge)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.alpha),
                        encode_scalar::<P>(
                            pairing,
                            &expected(MintMutationField::Alpha, &baseline_mint_responses.alpha)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.r),
                        encode_scalar::<P>(
                            pairing,
                            &expected(MintMutationField::R, &baseline_mint_responses.r)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.rho),
                        encode_scalar::<P>(
                            pairing,
                            &expected(MintMutationField::Rho, &baseline_mint_responses.rho)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.sigma),
                        encode_scalar::<P>(
                            pairing,
                            &expected(MintMutationField::Sigma, &baseline_mint_responses.sigma)
                        )
                    );
                    assert_eq!(
                        encode_g1::<P>(pairing, &mutation.statement.identity_element),
                        encode_g1::<P>(pairing, &record.mint_statement.identity_element)
                    );
                    assert_eq!(
                        encode_g2::<P>(pairing, &mutation.statement.hpub),
                        encode_g2::<P>(pairing, &record.mint_statement.hpub)
                    );
                }
            }
            for (index, mutation) in record.transfer_mutations.iter().enumerate() {
                let field = mutation.field;
                let components = delivery_proof
                    .proof_components(
                        proof::ProofComponentsParams,
                        proof::ProofComponentsPayload {
                            proof: &mutation.proof,
                        },
                    )
                    .expect("the mutation's components are infallible")
                    .components;
                let (challenge, responses) = match components {
                    DeliveryProofComponents::TransferBothGroups {
                        challenge,
                        responses,
                    }
                    | DeliveryProofComponents::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        ..
                    } => (challenge, responses),
                    _ => panic!("a transfer mutation's proof carries transfer components"),
                };
                if index < 26 {
                    assert_eq!(
                        encode_scalar::<P>(pairing, &challenge),
                        encode_scalar::<P>(pairing, &baseline_transfer_challenge)
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.x),
                        encode_scalar::<P>(pairing, &baseline_transfer_responses.x)
                    );
                    assert_eq!(
                        mutation.statement.context.expiry,
                        if field == TransferMutationField::Expiry {
                            record.transfer_statement.context.expiry ^ 1
                        } else {
                            record.transfer_statement.context.expiry
                        }
                    );
                    assert_eq!(
                        mutation.statement.context.purpose.code(),
                        if field == TransferMutationField::Purpose {
                            match record.transfer_statement.context.purpose {
                                TransferPurpose::Grant => TransferPurpose::Transfer.code(),
                                TransferPurpose::Transfer => TransferPurpose::Grant.code(),
                            }
                        } else {
                            record.transfer_statement.context.purpose.code()
                        }
                    );
                    assert_eq!(
                        encode_g1::<P>(pairing, &mutation.statement.identity_element),
                        if field == TransferMutationField::IdentityElement {
                            let g1 = pairing
                                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                                .expect("generator")
                                .point;
                            let off = pairing
                                .mul_g1(
                                    MulG1Params,
                                    MulG1Payload {
                                        point: g1,
                                        scalar: offset.clone(),
                                    },
                                )
                                .expect("offset")
                                .product;
                            let expected = pairing
                                .add_g1(
                                    AddG1Params,
                                    AddG1Payload {
                                        left: record.transfer_statement.identity_element.clone(),
                                        right: off,
                                    },
                                )
                                .expect("sum")
                                .sum;
                            encode_g1::<P>(pairing, &expected)
                        } else {
                            encode_g1::<P>(pairing, &record.transfer_statement.identity_element)
                        }
                    );
                } else {
                    let expected =
                        |name: TransferMutationField, baseline: &P::Scalar| -> P::Scalar {
                            if name == field {
                                mutated_scalar(baseline)
                            } else {
                                baseline.clone()
                            }
                        };
                    assert_eq!(
                        encode_scalar::<P>(pairing, &challenge),
                        encode_scalar::<P>(
                            pairing,
                            &expected(
                                TransferMutationField::Challenge,
                                &baseline_transfer_challenge
                            )
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.x),
                        encode_scalar::<P>(
                            pairing,
                            &expected(TransferMutationField::X, &baseline_transfer_responses.x)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.y),
                        encode_scalar::<P>(
                            pairing,
                            &expected(TransferMutationField::Y, &baseline_transfer_responses.y)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.s),
                        encode_scalar::<P>(
                            pairing,
                            &expected(TransferMutationField::S, &baseline_transfer_responses.s)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.rho),
                        encode_scalar::<P>(
                            pairing,
                            &expected(TransferMutationField::Rho, &baseline_transfer_responses.rho)
                        )
                    );
                    assert_eq!(
                        encode_scalar::<P>(pairing, &responses.sigma),
                        encode_scalar::<P>(
                            pairing,
                            &expected(
                                TransferMutationField::Sigma,
                                &baseline_transfer_responses.sigma
                            )
                        )
                    );
                }
            }
        }
    }
    all_cases(|pairing_concrete, h2s, enc, forms, kem, ka, proof| {
        let result = run_statement_mutations_case(
            &build_statement_mutations_case_deps(Consumer, Default::default()),
            selections(pairing_concrete, h2s, enc, forms, kem, ka, proof),
            build_statement_mutations_case_payload(CANONICAL_A, &asset_root(), Default::default()),
        );
        assert!(result.is_ok());
    });
}

/// Contract: the record's `draws`, split at `UNIFORM_BYTES_LENGTH` and
///   replayed through `setup`, `derive_identity`, `generate_keys`, and
///   `sample_from_uniform_bytes` on the case's adapters, reproduce the
///   statements' `hpub`, `identity_element`, and keys and the `Hpub`
///   mutation's offset.
/// Arrange: `count` of `1`, the entitlement scope, each case run under each
///   of the admitted identities over the canonical bytes `b"pkg@1.0.0#1"` and
///   `b"pkg@2.0.0#7"`.
/// Act:     `statement_mutations`.
/// Assert:  the replayed parameter set's `hpub`, identity element's
///   `element`, and key pairs' `pk1`/`pk2` equal the baselines'; the `Hpub`
///   mutation's `hpub` encodes to the expected offset; the identity
///   element's `scalar` encodes to the scalar sampled over the tag built
///   from `K::DECLARATION.identity_tag`, the canonical bytes, and
///   `UNIFORM_BYTES_LENGTH`.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_statements_and_the_offset_it_produced() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let uniforms: Vec<domain::Secret<Vec<u8>>> = record
                .draws
                .chunks_exact(length)
                .map(|chunk| {
                    domain::Secret::try_new(domain::SecretConstructorParams {
                        value: chunk.to_vec(),
                    })
                    .expect("a draw of the declared length is a valid uniform")
                })
                .collect();
            let clone_uniform = |index: usize| -> domain::Secret<Vec<u8>> {
                domain::Secret::try_new(domain::SecretConstructorParams {
                    value: uniforms[index].expose().clone(),
                })
                .expect("a draw of the declared length is a valid uniform")
            };
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(clone_uniform(0)),
                        u0_uniform: Some(clone_uniform(1)),
                        u1_uniform: Some(clone_uniform(2)),
                    }),
                )
                .expect("setup over the replayed uniforms succeeds");
            let parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("the parameter set's components are infallible")
                .components;
            let derived = kem
                .derive_identity(
                    build_derive_identity_params(Default::default()),
                    build_derive_identity_payload(
                        &setup.parameter_set,
                        DeriveIdentityPayloadOverrides {
                            identity: Some(identity.identity),
                        },
                    ),
                )
                .expect("derive_identity over the admitted identity succeeds");
            let identity_components = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
            let replay_scalar = |uniform: &domain::Secret<Vec<u8>>| -> P::Scalar {
                P::Scalar::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload {
                        uniform: domain::Secret::try_new(domain::SecretConstructorParams {
                            value: uniform.expose().clone(),
                        })
                        .expect("uniform"),
                    },
                )
                .expect("a uniform of the declared length samples")
                .scalar
                .expose()
                .clone()
            };
            let g1 = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("generator")
                .point;
            let g2 = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("generator")
                .point;
            let replayed_hpub = encode_g2::<P>(pairing, &parameter_set.hpub);
            assert_eq!(
                replayed_hpub,
                encode_g2::<P>(pairing, &record.mint_statement.hpub)
            );
            assert_eq!(
                encode_g1::<P>(pairing, &identity_components.element),
                encode_g1::<P>(pairing, &record.mint_statement.identity_element)
            );
            assert_eq!(
                encode_g1::<P>(pairing, &identity_components.element),
                encode_g1::<P>(pairing, &record.transfer_statement.identity_element)
            );
            let offset = replay_scalar(&uniforms[9]);
            let canonical = match identity.identity {
                KemIdentity::Entitlement { canonical } => canonical.to_vec(),
                KemIdentity::Asset { root } => root.as_bytes().to_vec(),
            };
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            let offset_scalar_expected = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload {
                    uniform: domain::Secret::try_new(domain::SecretConstructorParams {
                        value: expected_uniform_bytes(
                            &tag,
                            canonical.as_slice(),
                            <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                        )
                        .to_vec(),
                    })
                    .expect("the tag is a valid uniform"),
                },
            )
            .expect("the identity scalar samples")
            .scalar
            .expose()
            .clone();
            assert_eq!(
                encode_scalar::<P>(pairing, &identity_components.scalar),
                encode_scalar::<P>(pairing, &offset_scalar_expected)
            );
            let expected_element = pairing
                .add_g1(
                    AddG1Params,
                    AddG1Payload {
                        left: parameter_set.u0.clone(),
                        right: pairing
                            .mul_g1(
                                MulG1Params,
                                MulG1Payload {
                                    point: parameter_set.u1.clone(),
                                    scalar: offset_scalar_expected.clone(),
                                },
                            )
                            .expect("product")
                            .product,
                    },
                )
                .expect("sum")
                .sum;
            assert_eq!(
                encode_g1::<P>(pairing, &identity_components.element),
                encode_g1::<P>(pairing, &expected_element)
            );
            let buyer_keys = key_agreement
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(
                        pairing,
                        envelope::GenerateKeysPayloadOverrides {
                            x_uniform: Some(clone_uniform(4)),
                            y_uniform: Some(clone_uniform(5)),
                        },
                    ),
                )
                .expect("key generation over the replayed uniforms succeeds");
            let buyer_public_keys = key_agreement
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&buyer_keys.key_pair, Default::default()),
                )
                .expect("the key pair's public keys are infallible")
                .components;
            let x_scalar = replay_scalar(&uniforms[4]);
            let y_scalar = replay_scalar(&uniforms[5]);
            assert_eq!(
                encode_g1::<P>(pairing, &buyer_public_keys.pk1),
                encode_g1::<P>(
                    pairing,
                    &pairing
                        .mul_g1(
                            MulG1Params,
                            MulG1Payload {
                                point: g1.clone(),
                                scalar: x_scalar.clone(),
                            }
                        )
                        .expect("product")
                        .product
                )
            );
            assert_eq!(
                encode_g2::<P>(pairing, &buyer_public_keys.pk2),
                encode_g2::<P>(
                    pairing,
                    &pairing
                        .mul_g2(
                            MulG2Params,
                            MulG2Payload {
                                point: g2.clone(),
                                scalar: y_scalar.clone(),
                            }
                        )
                        .expect("product")
                        .product
                )
            );
            assert_eq!(
                encode_g1::<P>(pairing, &buyer_public_keys.pk1),
                encode_g1::<P>(pairing, &record.mint_statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2::<P>(pairing, &buyer_public_keys.pk2),
                encode_g2::<P>(pairing, &record.mint_statement.buyer_keys.pk2)
            );
            let seller_keys = key_agreement
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(
                        pairing,
                        envelope::GenerateKeysPayloadOverrides {
                            x_uniform: Some(clone_uniform(4)),
                            y_uniform: Some(clone_uniform(5)),
                        },
                    ),
                )
                .expect("key generation over the replayed uniforms succeeds");
            let seller_public_keys = key_agreement
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(&seller_keys.key_pair, Default::default()),
                )
                .expect("the key pair's public keys are infallible")
                .components;
            assert_eq!(
                encode_g1::<P>(pairing, &seller_public_keys.pk1),
                encode_g1::<P>(pairing, &record.transfer_statement.seller_keys.pk1)
            );
            assert_eq!(
                encode_g2::<P>(pairing, &seller_public_keys.pk2),
                encode_g2::<P>(pairing, &record.transfer_statement.seller_keys.pk2)
            );
            let transfer_buyer = key_agreement
                .generate_keys(
                    build_generate_keys_params(Default::default()),
                    build_generate_keys_payload(
                        pairing,
                        envelope::GenerateKeysPayloadOverrides {
                            x_uniform: Some(clone_uniform(6)),
                            y_uniform: Some(clone_uniform(7)),
                        },
                    ),
                )
                .expect("the transfer buyer's key generation succeeds");
            let transfer_buyer_keys = key_agreement
                .key_pair_public_keys(
                    build_key_pair_public_keys_params(Default::default()),
                    build_key_pair_public_keys_payload(
                        &transfer_buyer.key_pair,
                        Default::default(),
                    ),
                )
                .expect("the key pair's public keys are infallible")
                .components;
            assert_eq!(
                encode_g1::<P>(pairing, &transfer_buyer_keys.pk1),
                encode_g1::<P>(pairing, &record.transfer_statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2::<P>(pairing, &transfer_buyer_keys.pk2),
                encode_g2::<P>(pairing, &record.transfer_statement.buyer_keys.pk2)
            );
            let g2_offset = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2,
                        scalar: offset.clone(),
                    },
                )
                .expect("offset")
                .product;
            let expected_hpub = pairing
                .add_g2(
                    AddG2Params,
                    AddG2Payload {
                        left: parameter_set.hpub.clone(),
                        right: g2_offset,
                    },
                )
                .expect("sum")
                .sum;
            let hpub_mutation = record
                .mint_mutations
                .iter()
                .find(|mutation| mutation.field == MintMutationField::Hpub)
                .expect("the Hpub mutation is present");
            assert_eq!(
                encode_g2::<P>(pairing, &hpub_mutation.statement.hpub),
                encode_g2::<P>(pairing, &expected_hpub)
            );
        }
    }
    for canonical in [CANONICAL_A, CANONICAL_B] {
        all_cases(|pairing_concrete, h2s, enc, forms, kem, ka, proof| {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(Consumer, Default::default()),
                selections(pairing_concrete, h2s, enc, forms, kem, ka, proof),
                build_statement_mutations_case_payload(
                    canonical,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
        });
    }
}

/// Contract: the list holds the configured count, each record's `draws` hold
///   its ten uniforms at their length, and no two records share `draws`.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES`.
/// Act:     `statement_mutations`.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `10` times
///   `UNIFORM_BYTES_LENGTH` bytes, and no two records share `draws`.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(3),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 3);
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            for (index, record) in success.vectors.iter().enumerate() {
                assert_eq!(record.draws.len(), 10 * length);
                for other in &success.vectors[index + 1..] {
                    assert_ne!(record.draws, other.draws);
                }
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        let result = run_statement_mutations_case(
            &build_statement_mutations_case_deps(Consumer, Default::default()),
            case_params(StatementMutationsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    },
                )),
                ..Default::default()
            }),
            build_statement_mutations_case_payload(CANONICAL_A, &asset_root(), Default::default()),
        );
        assert!(result.is_ok());
    }
}

/// Contract: `statement_mutations` over an admitted asset identity → `Ok`,
///   both baselines verify and every mutation is refused.
/// Arrange: the KEM selection under the asset scope, the root `build_root`
///   builds from `[0x44; 32]`, `count` of `1`; every case that declares the
///   asset scope.
/// Act:     `statement_mutations`.
/// Assert:  the result is `Ok`, `mint_is_valid` and `transfer_is_valid` are
///   `true`, and every mutation's `is_valid` is `false`.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn an_asset_identity_is_mutated_under_the_asset_scope() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert!(record.mint_is_valid);
            assert!(record.transfer_is_valid);
            for mutation in &record.mint_mutations {
                assert!(!mutation.is_valid);
            }
            for mutation in &record.transfer_mutations {
                assert!(!mutation.is_valid);
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            if kem_selection.scope != IdentityScope::Asset {
                continue;
            }
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(Consumer, Default::default()),
                case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL_A,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: a hash-to-scalar refusal reaches the caller inside the mint
///   delivery's `DeriveIdentity` variant under the entitlement scope and its
///   `Setup` variant under the asset scope, within the function's `Mint`
///   variant, whole.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, with
///   the hash-to-scalar mock under `SamplingRefused`.
/// Act:     `statement_mutations`.
/// Assert:  the result equals the whole `Err(…)` under each scope.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  the hash-to-scalar family's mock concrete, selected by
///   configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let error =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            let expected = match identity.setup_scope {
                kem::SetupScope::Entitlement => StatementMutationsErrorReturn::Mint(
                    MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)),
                ),
                kem::SetupScope::Asset { root: _ } => StatementMutationsErrorReturn::Mint(
                    MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)),
                ),
            };
            // Act
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(returned.err(), Some(expected));
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(Consumer, Default::default()),
                case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(HashToScalarConcrete::Mock(
                                MockIHashToScalarAdapterFailureMode::SamplingRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    kem: Some(*kem_selection),
                    ..Default::default()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL_A,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: a nonce-draw refusal of the mint delivery's prover reaches the
///   caller inside the function's `Mint` variant, whole.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, the delivery-proof
///   mock under `NonceDrawRefused`, and each member of
///   `DELIVERY_PROOF_CONCRETES` as the counterfactual.
/// Act:     `statement_mutations`.
/// Assert:  the result under the refusing selection equals the whole
///   `Err(…)`, and under each declared selection `is_ok()`.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  the delivery-proof family's mock concrete, selected by
///   configuration.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
                            MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                        ))
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    delivery_proof: Some(build_create_delivery_proof_params(
                        CreateDeliveryProofParamsOverrides {
                            concrete: Some(DeliveryProofConcrete::Mock(
                                MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL_A,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        delivery_proof: Some(build_create_delivery_proof_params(
                            CreateDeliveryProofParamsOverrides {
                                concrete: Some(*proof_concrete),
                                ..Default::default()
                            },
                        )),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                        ..Default::default()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL_A,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: a nonce-draw refusal of the mint delivery's recipient key
///   generation reaches the caller inside the function's `Mint` variant,
///   whole.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, the key-agreement mock
///   under `NonceDrawRefused`, and each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS` as the counterfactual.
/// Act:     `statement_mutations`.
/// Assert:  the result under the refusing selection equals the whole
///   `Err(…)`, and under each declared selection `is_ok()`.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  the key-agreement family's mock concrete, selected by
///   configuration.
#[test]
fn a_key_generation_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(
                            MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
                        ))
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL_A,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for ka_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        key_agreement: Some(*ka_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                        ..Default::default()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL_A,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: a coin-draw refusal of the mint delivery's wrap to the
///   recipient's keys reaches the caller inside the function's `Mint`
///   variant, whole.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, the key-agreement mock
///   under `CoinDrawRefused`, and each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS` as the counterfactual.
/// Act:     `statement_mutations`.
/// Assert:  the result under the refusing selection equals the whole
///   `Err(…)`, and under each declared selection `is_ok()`.
/// Boundary: `statement_mutations` and the chain of real functions the
///   private surface names.
/// Mocked:  the key-agreement family's mock concrete, selected by
///   configuration.
#[test]
fn a_wrap_coin_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(
                    returned.err(),
                    Some(StatementMutationsErrorReturn::Mint(
                        MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(
                            MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused
                        ))
                    ))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_statement_mutations_case(
                &build_statement_mutations_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                case_params(StatementMutationsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }),
                build_statement_mutations_case_payload(
                    CANONICAL_A,
                    &asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for ka_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_statement_mutations_case(
                    &build_statement_mutations_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    case_params(StatementMutationsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        key_agreement: Some(*ka_selection),
                        random_source: Some(build_create_random_source_params(
                            CreateRandomSourceParamsOverrides {
                                kind: Some(RandomSourceKind::OperatingSystem),
                            },
                        )),
                        ..Default::default()
                    }),
                    build_statement_mutations_case_payload(
                        CANONICAL_A,
                        &asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: `verify` over each statement and proof the record carries
///   returns the verdict the record holds.
/// Arrange: `count` of `1`; every case.
/// Act:     `statement_mutations`.
/// Assert:  `verify` over each baseline returns `is_valid` `true`, and over
///   each mutation's pair returns the mutation's `is_valid`, `false`.
/// Boundary: `statement_mutations` and `verify`, through the chain of real
///   functions the private surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_carried_statement_and_proof_verify_to_their_recorded_verdict() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
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
            let returned = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_statement_mutations_params(StatementMutationsParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_statement_mutations_payload(
                    &mint_context,
                    &transfer_context,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert!(
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&record.mint_statement),
                            proof: &record.mint_proof,
                        },
                    )
                    .expect("verify over the mint baseline succeeds")
                    .is_valid
            );
            assert!(
                delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&record.transfer_statement),
                            proof: &record.transfer_proof,
                        },
                    )
                    .expect("verify over the transfer baseline succeeds")
                    .is_valid
            );
            for mutation in &record.mint_mutations {
                let verdict = delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&mutation.statement),
                            proof: &mutation.proof,
                        },
                    )
                    .expect("verify over a mint mutation succeeds")
                    .is_valid;
                assert_eq!(verdict, mutation.is_valid);
                assert!(!verdict);
            }
            for mutation in &record.transfer_mutations {
                let verdict = delivery_proof
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&mutation.statement),
                            proof: &mutation.proof,
                        },
                    )
                    .expect("verify over a transfer mutation succeeds")
                    .is_valid;
                assert_eq!(verdict, mutation.is_valid);
                assert!(!verdict);
            }
        }
    }
    all_cases(|pairing_concrete, h2s, enc, forms, kem, ka, proof| {
        let result = run_statement_mutations_case(
            &build_statement_mutations_case_deps(Consumer, Default::default()),
            selections(pairing_concrete, h2s, enc, forms, kem, ka, proof),
            build_statement_mutations_case_payload(CANONICAL_A, &asset_root(), Default::default()),
        );
        assert!(result.is_ok());
    });
}
