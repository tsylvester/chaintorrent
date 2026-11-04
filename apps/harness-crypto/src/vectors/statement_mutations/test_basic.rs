#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, Secret,
    SuiteIdentifierConstructorParamsOverrides, build_parameter_set_identifier,
    build_suite_identifier,
};
use encoding::{
    CanonicalFieldValue, ICanonicalField, IDecoderAdapter, IEncoderAdapter, ToFieldParams,
    build_from_field_params,
};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IdentityScope,
    SetupParamsOverrides, SetupPayloadOverrides, build_create_kem_params,
    build_derive_identity_params, build_derive_identity_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_setup_params, build_setup_payload,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, SampleUniformScalarParams, SampleUniformScalarPayload,
};
use proof::{
    AlgebraicStatement, DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_REPLACEMENT, DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides,
    DeliveryProofComponents, IDeliveryProofAdapter, MintPurpose, ProofComponentsParams,
    ProofComponentsPayload, TransferPurpose, VerifyParams, VerifyPayload, build_delivery_context,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, build_fill_bytes_payload,
};

use super::interface::{IStatementMutationsCaseConsumer, MintMutationField, TransferMutationField};
use super::mock::{
    StatementMutationsCaseParamsOverrides, StatementMutationsParamsOverrides,
    StatementMutationsPayloadOverrides, build_statement_mutations_case_deps,
    build_statement_mutations_case_params, build_statement_mutations_case_payload,
    build_statement_mutations_deps, build_statement_mutations_params,
    build_statement_mutations_payload, run_statement_mutations_case,
};
use super::statement_mutations;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::{VectorCountConstructorParamsOverrides, build_vector_count};

const CANONICAL: &[u8] = b"pkg@1.0.0#1";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
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

fn drawn_uniform<P: IPairingArithmetic>(source: &dyn IRandomSourceAdapter) -> Secret<Vec<u8>> {
    source
        .fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(FillBytesPayloadOverrides {
                length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
            }),
        )
        .expect("the independent source yields its draws")
        .bytes
}

fn replayed_offset<P: IPairingArithmetic>(source: &dyn IRandomSourceAdapter) -> P::Scalar {
    let mut uniforms = (0..9)
        .map(|_| drawn_uniform::<P>(source))
        .collect::<Vec<_>>();
    uniforms.push(drawn_uniform::<P>(source));
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload {
            uniform: uniforms.swap_remove(9),
        },
    )
    .expect("a uniform of the declared length samples")
    .scalar
    .expose()
    .clone()
}

/// Contract: Computed → `Ok(StatementMutationsSuccessReturn { vectors })`, a
///   record for each index from `0` below `params.count.get()`.
/// Arrange: `count` of `3`, a count other than the builder's default.
/// Act:     `statement_mutations`.
/// Assert:  `vectors` holds `3` records.
#[test]
fn statement_mutations_emits_the_configured_count_of_records() {
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
            let params = build_statement_mutations_params(StatementMutationsParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(3),
                })),
            });
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
                params,
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
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Draw → every uniform has the length the pairing's scalar sampling
///   bound declares, and a record's `draws` are the mint delivery's draws, the
///   transfer delivery's draws, and the offset uniform concatenated.
/// Arrange: `count` of `1`.
/// Act:     `statement_mutations`.
/// Assert:  the record's `draws` hold `10` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes.
#[test]
fn statement_mutations_draws_each_record_ten_uniforms_at_the_declared_length() {
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
            let params = build_statement_mutations_params(StatementMutationsParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
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
                params,
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
            assert_eq!(
                success.vectors[0].draws.len(),
                10 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Ordering → the records in index order, each record's draws in
///   draw order.
/// Arrange: `count` of `2`; the expected draws from the independent source,
///   ten consecutive calls for each record in turn, so a transposed record or
///   draw reports otherwise.
/// Act:     `statement_mutations`.
/// Assert:  each record's `draws` equal its expected draws.
#[test]
fn statement_mutations_record_the_draws_in_the_sequence_the_source_yields() {
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
            independent: &dyn IRandomSourceAdapter,
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
            let params = build_statement_mutations_params(StatementMutationsParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(2),
                })),
            });
            let mut expected: Vec<Vec<u8>> = Vec::new();
            for _ in 0..2 {
                let mut draws = Vec::new();
                for _ in 0..10 {
                    draws.extend_from_slice(drawn_uniform::<P>(independent).expose());
                }
                expected.push(draws);
            }
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
                params,
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
            assert_eq!(success.vectors[0].draws, expected[0]);
            assert_eq!(success.vectors[1].draws, expected[1]);
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Statement mutation and Proof-scalar mutation for the mint →
///   `mint_mutations` holds one mutation per field of `MINT_MUTATION_FIELDS`,
///   in that order.
/// Arrange: `count` of `1`.
/// Act:     `statement_mutations`.
/// Assert:  the `field` of each of the record's `mint_mutations`, in order,
///   equals every mint mutation field written out.
#[test]
fn statement_mutations_mutate_every_mint_field_and_proof_scalar_in_order() {
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
            let fields: Vec<MintMutationField> = success.vectors[0]
                .mint_mutations
                .iter()
                .map(|mutation| mutation.field)
                .collect();
            assert_eq!(
                fields,
                vec![
                    MintMutationField::SuiteIdentifier,
                    MintMutationField::Chain,
                    MintMutationField::EntitlementContract,
                    MintMutationField::AssetRoot,
                    MintMutationField::ParameterSetIdentifier,
                    MintMutationField::SourceEntitlement,
                    MintMutationField::TargetEntitlement,
                    MintMutationField::OldInterval,
                    MintMutationField::NewInterval,
                    MintMutationField::Purpose,
                    MintMutationField::Seller,
                    MintMutationField::Buyer,
                    MintMutationField::Expiry,
                    MintMutationField::Hpub,
                    MintMutationField::IdentityElement,
                    MintMutationField::BuyerPk1,
                    MintMutationField::BuyerPk2,
                    MintMutationField::EnvelopeC1,
                    MintMutationField::EnvelopeC2,
                    MintMutationField::EnvelopeD1,
                    MintMutationField::EnvelopeD2,
                    MintMutationField::Challenge,
                    MintMutationField::Alpha,
                    MintMutationField::R,
                    MintMutationField::Rho,
                    MintMutationField::Sigma,
                ]
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Statement mutation and Proof-scalar mutation for the transfer →
///   `transfer_mutations` holds one mutation per field of
///   `TRANSFER_MUTATION_FIELDS`, in that order.
/// Arrange: `count` of `1`.
/// Act:     `statement_mutations`.
/// Assert:  the `field` of each of the record's `transfer_mutations`, in
///   order, equals every transfer mutation field written out.
#[test]
fn statement_mutations_mutate_every_transfer_field_and_proof_scalar_in_order() {
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
            let fields: Vec<TransferMutationField> = success.vectors[0]
                .transfer_mutations
                .iter()
                .map(|mutation| mutation.field)
                .collect();
            assert_eq!(
                fields,
                vec![
                    TransferMutationField::SuiteIdentifier,
                    TransferMutationField::Chain,
                    TransferMutationField::EntitlementContract,
                    TransferMutationField::AssetRoot,
                    TransferMutationField::ParameterSetIdentifier,
                    TransferMutationField::SourceEntitlement,
                    TransferMutationField::TargetEntitlement,
                    TransferMutationField::OldInterval,
                    TransferMutationField::NewInterval,
                    TransferMutationField::Purpose,
                    TransferMutationField::Seller,
                    TransferMutationField::Buyer,
                    TransferMutationField::Expiry,
                    TransferMutationField::IdentityElement,
                    TransferMutationField::SellerPk1,
                    TransferMutationField::SellerPk2,
                    TransferMutationField::BuyerPk1,
                    TransferMutationField::BuyerPk2,
                    TransferMutationField::OldEnvelopeC1,
                    TransferMutationField::OldEnvelopeC2,
                    TransferMutationField::OldEnvelopeD1,
                    TransferMutationField::OldEnvelopeD2,
                    TransferMutationField::NewEnvelopeC1,
                    TransferMutationField::NewEnvelopeC2,
                    TransferMutationField::NewEnvelopeD1,
                    TransferMutationField::NewEnvelopeD2,
                    TransferMutationField::Challenge,
                    TransferMutationField::X,
                    TransferMutationField::Y,
                    TransferMutationField::S,
                    TransferMutationField::Rho,
                    TransferMutationField::Sigma,
                ]
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Verdict → each baseline's `is_valid` and each mutation's
///   `is_valid` are the verdict `verify` returns over the pair.
/// Arrange: `count` of `1`; the baselines verify on the family's mock
///   concrete, and every mutation differs from its baseline in one field or
///   one scalar, so a verdict read from anything but `verify` over the pair
///   reports otherwise.
/// Act:     `statement_mutations`.
/// Assert:  `mint_is_valid` and `transfer_is_valid` are `true`; each
///   mutation's `is_valid` equals the `is_valid` `verify` returns over its
///   statement and proof.
#[test]
fn statement_mutations_record_the_baselines_as_verified_and_every_mutation_as_refused() {
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
                assert_eq!(mutation.is_valid, verdict);
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
                assert_eq!(mutation.is_valid, verdict);
            }
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Canonical value mutation → a form-typed field is rebuilt from
///   its canonical value with the lowest bit of its last byte flipped, or for
///   an integer kind its lowest bit flipped.
/// Arrange: `count` of `1`; the typed mint context overriding `chain` with the
///   form `from_field` builds over `Unsigned256([0x2A; 32])`, `seller` with the
///   form over `FixedBytes20([0x61; 20])`, and `old_interval` with the form
///   over `Unsigned64(6)`.
/// Act:     `statement_mutations`.
/// Assert:  the `to_field` of the named fields of the `Chain`, `Seller`, and
///   `OldInterval` mutations' contexts equal the flipped values.
#[test]
fn statement_mutations_flip_the_lowest_bit_of_each_form_typed_field() {
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
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides {
                    chain: Some(
                        F::ChainIdentifier::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned256([0x2A; 32]),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    seller: Some(
                        F::Identity::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::FixedBytes20([0x61; 20]),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    old_interval: Some(
                        F::Interval::from_field(
                            build_from_field_params(Default::default()),
                            CanonicalFieldValue::Unsigned64(6),
                        )
                        .expect("the value has the kind the form declares")
                        .value,
                    ),
                    ..Default::default()
                },
            );
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
            let mut expected_chain = [0x2Au8; 32];
            expected_chain[31] = 0x2B;
            let chain_mutation = &record.mint_mutations[1];
            assert_eq!(chain_mutation.field, MintMutationField::Chain);
            assert_eq!(
                F::ChainIdentifier::to_field(
                    ToFieldParams,
                    &chain_mutation.statement.context.chain
                )
                .expect("the form reads back its value")
                .field,
                CanonicalFieldValue::Unsigned256(expected_chain)
            );
            let mut expected_seller = [0x61u8; 20];
            expected_seller[19] = 0x60;
            let seller_mutation = &record.mint_mutations[10];
            assert_eq!(seller_mutation.field, MintMutationField::Seller);
            assert_eq!(
                F::Identity::to_field(ToFieldParams, &seller_mutation.statement.context.seller)
                    .expect("the form reads back its value")
                    .field,
                CanonicalFieldValue::FixedBytes20(expected_seller)
            );
            let interval_mutation = &record.mint_mutations[7];
            assert_eq!(interval_mutation.field, MintMutationField::OldInterval);
            assert_eq!(
                F::Interval::to_field(
                    ToFieldParams,
                    &interval_mutation.statement.context.old_interval
                )
                .expect("the form reads back its value")
                .field,
                CanonicalFieldValue::Unsigned64(7)
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Domain value mutation → the suite identifier, the asset root,
///   and the parameter-set identifier are rebuilt with the lowest bit of
///   their last byte flipped, the version unchanged.
/// Arrange: `count` of `1`; the typed mint context overriding
///   `suite_identifier` with `build_suite_identifier` over identifier
///   `[0x72; 32]` and version `2`, `asset_root` with `build_root` over
///   `[0x74; 32]`, and `parameter_set_identifier` with
///   `build_parameter_set_identifier` over `[0x76; 32]`.
/// Act:     `statement_mutations`.
/// Assert:  the `SuiteIdentifier`, `AssetRoot`, and `ParameterSetIdentifier`
///   mutations' context fields carry the flipped values.
#[test]
fn statement_mutations_flip_the_lowest_bit_of_each_domain_value() {
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
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some([0x72; 32]),
                            version: Some(2),
                        },
                    )),
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some([0x74; 32]),
                    })),
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides {
                            bytes: Some([0x76; 32]),
                        },
                    )),
                    ..Default::default()
                },
            );
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
            let mut expected_identifier = [0x72u8; 32];
            expected_identifier[31] = 0x73;
            let suite_mutation = &record.mint_mutations[0];
            assert_eq!(suite_mutation.field, MintMutationField::SuiteIdentifier);
            assert_eq!(
                suite_mutation
                    .statement
                    .context
                    .suite_identifier
                    .identifier(),
                &expected_identifier
            );
            assert_eq!(
                suite_mutation.statement.context.suite_identifier.version(),
                2
            );
            let mut expected_root = [0x74u8; 32];
            expected_root[31] = 0x75;
            let root_mutation = &record.mint_mutations[3];
            assert_eq!(root_mutation.field, MintMutationField::AssetRoot);
            assert_eq!(
                root_mutation.statement.context.asset_root.as_bytes(),
                &expected_root
            );
            let mut expected_parameter_set = [0x76u8; 32];
            expected_parameter_set[31] = 0x77;
            let parameter_set_mutation = &record.mint_mutations[4];
            assert_eq!(
                parameter_set_mutation.field,
                MintMutationField::ParameterSetIdentifier
            );
            assert_eq!(
                parameter_set_mutation
                    .statement
                    .context
                    .parameter_set_identifier
                    .as_bytes(),
                &expected_parameter_set
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Domain value mutation, the purpose → `Mint` becomes
///   `Replacement` and `Transfer` becomes `Grant`, and the other way.
/// Arrange: `count` of `1`, run once with the typed contexts built at the
///   default purposes, and once built with `DELIVERY_PURPOSE_REPLACEMENT` and
///   `DELIVERY_PURPOSE_GRANT`.
/// Act:     `statement_mutations`, once per run.
/// Assert:  the `code()` of the `Purpose` mutations' contexts' purposes equal
///   the other purpose of each relation in each run.
#[test]
fn statement_mutations_swap_each_purpose_for_the_other_of_its_relation() {
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
            let first_mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let first_transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let second_mint = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_REPLACEMENT,
                Default::default(),
            );
            let second_transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                Default::default(),
            );
            let params = build_statement_mutations_params(StatementMutationsParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let first = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_statement_mutations_payload(
                    &first_mint,
                    &first_transfer,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let second = statement_mutations(
                &build_statement_mutations_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_statement_mutations_payload(
                    &second_mint,
                    &second_transfer,
                    StatementMutationsPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let first_record = &first.expect("the first run's calls succeed").vectors[0];
            let mint_purpose = &first_record.mint_mutations[9];
            assert_eq!(mint_purpose.field, MintMutationField::Purpose);
            assert_eq!(
                mint_purpose.statement.context.purpose.code(),
                DELIVERY_PURPOSE_REPLACEMENT.code()
            );
            let transfer_purpose = &first_record.transfer_mutations[9];
            assert_eq!(transfer_purpose.field, TransferMutationField::Purpose);
            assert_eq!(
                transfer_purpose.statement.context.purpose.code(),
                DELIVERY_PURPOSE_GRANT.code()
            );
            let second_record = &second.expect("the second run's calls succeed").vectors[0];
            let mint_purpose = &second_record.mint_mutations[9];
            assert_eq!(
                mint_purpose.statement.context.purpose.code(),
                DELIVERY_PURPOSE_MINT.code()
            );
            let transfer_purpose = &second_record.transfer_mutations[9];
            assert_eq!(
                transfer_purpose.statement.context.purpose.code(),
                DELIVERY_PURPOSE_TRANSFER.code()
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Domain value mutation, the expiry → `expiry ^ 1`.
/// Arrange: `count` of `1`; the typed mint context overriding `expiry` with
///   `1_800_000_000` and the typed transfer context with `1_900_000_001`.
/// Act:     `statement_mutations`.
/// Assert:  the `Expiry` mutations' contexts' expiries equal `1_800_000_001`
///   for the mint and `1_900_000_000` for the transfer.
#[test]
fn statement_mutations_flip_the_lowest_bit_of_the_expiry() {
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
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides {
                    expiry: Some(1_800_000_000),
                    ..Default::default()
                },
            );
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    expiry: Some(1_900_000_001),
                    ..Default::default()
                },
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
            let mint_expiry = &record.mint_mutations[12];
            assert_eq!(mint_expiry.field, MintMutationField::Expiry);
            assert_eq!(mint_expiry.statement.context.expiry, 1_800_000_001);
            let transfer_expiry = &record.transfer_mutations[12];
            assert_eq!(transfer_expiry.field, TransferMutationField::Expiry);
            assert_eq!(transfer_expiry.statement.context.expiry, 1_900_000_000);
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Point mutation → a first-group point becomes its sum with the
///   first-group generator multiplied by the offset, and a second-group point
///   its sum with the second-group generator multiplied by the offset.
/// Arrange: `count` of `1`; the replayed offset from the independent source's
///   tenth draw; the expected mutated `identity_element` the `encode_g1` of
///   `add_g1` of the baseline's `identity_element` and `mul_g1` of the
///   first-group generator by the replayed offset, and the expected mutated
///   `hpub` the `encode_g2` of the like sum in the second group over the
///   baseline's `hpub`.
/// Act:     `statement_mutations`.
/// Assert:  the `encode_g1` of the `IdentityElement` mint mutation's
///   statement's `identity_element` and the `encode_g2` of the `Hpub`
///   mutation's statement's `hpub` equal the expected bytes.
#[test]
fn statement_mutations_offset_each_point_by_the_generator_times_the_offset() {
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
            independent: &dyn IRandomSourceAdapter,
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
            let offset = replayed_offset::<P>(independent);
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
            let g1_generator = pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload)
                .expect("the first-group generator is infallible")
                .point;
            let g2_generator = pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload)
                .expect("the second-group generator is infallible")
                .point;
            let g1_offset = pairing
                .mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: g1_generator,
                        scalar: offset.clone(),
                    },
                )
                .expect("the first-group offset is infallible")
                .product;
            let g2_offset = pairing
                .mul_g2(
                    MulG2Params,
                    MulG2Payload {
                        point: g2_generator,
                        scalar: offset,
                    },
                )
                .expect("the second-group offset is infallible")
                .product;
            let expected_identity_element = pairing
                .add_g1(
                    AddG1Params,
                    AddG1Payload {
                        left: record.mint_statement.identity_element.clone(),
                        right: g1_offset,
                    },
                )
                .expect("the first-group sum is infallible")
                .sum;
            let expected_hpub = pairing
                .add_g2(
                    AddG2Params,
                    AddG2Payload {
                        left: record.mint_statement.hpub.clone(),
                        right: g2_offset,
                    },
                )
                .expect("the second-group sum is infallible")
                .sum;
            let identity_mutation = &record.mint_mutations[14];
            assert_eq!(identity_mutation.field, MintMutationField::IdentityElement);
            assert_eq!(
                encode_g1::<P>(pairing, &identity_mutation.statement.identity_element),
                encode_g1::<P>(pairing, &expected_identity_element)
            );
            let hpub_mutation = &record.mint_mutations[13];
            assert_eq!(hpub_mutation.field, MintMutationField::Hpub);
            assert_eq!(
                encode_g2::<P>(pairing, &hpub_mutation.statement.hpub),
                encode_g2::<P>(pairing, &expected_hpub)
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Proof-scalar mutation → the scalar the field names becomes its
///   sum with the offset and every other component is kept.
/// Arrange: `count` of `1`; the replayed offset from the independent source's
///   tenth draw; the components of the record's baseline proofs from
///   `proof_components`, and the expected `challenge` of each `Challenge`
///   mutation the `encode_scalar` of `add_scalar` of the baseline's challenge
///   and the replayed offset.
/// Act:     `statement_mutations`.
/// Assert:  the `encode_scalar` of the challenge of each `Challenge`
///   mutation's proof components equals the expected bytes, and each of its
///   responses encodes to the baseline's response of the same name.
#[test]
fn statement_mutations_offset_each_proof_scalar_by_the_offset() {
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
            independent: &dyn IRandomSourceAdapter,
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
            let offset = replayed_offset::<P>(independent);
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
            let mint_components = delivery_proof
                .proof_components(
                    ProofComponentsParams,
                    ProofComponentsPayload {
                        proof: &record.mint_proof,
                    },
                )
                .expect("the mint baseline's components are infallible")
                .components;
            let transfer_components = delivery_proof
                .proof_components(
                    ProofComponentsParams,
                    ProofComponentsPayload {
                        proof: &record.transfer_proof,
                    },
                )
                .expect("the transfer baseline's components are infallible")
                .components;
            let (mint_challenge, mint_responses) = match mint_components {
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
            let (transfer_challenge, transfer_responses) = match transfer_components {
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
            let expected_mint_challenge = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: mint_challenge.clone(),
                        right: offset.clone(),
                    },
                )
                .expect("the scalar sum is infallible")
                .sum;
            let expected_transfer_challenge = pairing
                .add_scalar(
                    AddScalarParams,
                    AddScalarPayload {
                        left: transfer_challenge.clone(),
                        right: offset,
                    },
                )
                .expect("the scalar sum is infallible")
                .sum;
            let mint_challenge_mutation = &record.mint_mutations[21];
            assert_eq!(mint_challenge_mutation.field, MintMutationField::Challenge);
            let mint_mutated = delivery_proof
                .proof_components(
                    ProofComponentsParams,
                    ProofComponentsPayload {
                        proof: &mint_challenge_mutation.proof,
                    },
                )
                .expect("the mint mutation's components are infallible")
                .components;
            let (mutated_challenge, mutated_responses) = match mint_mutated {
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
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_challenge),
                encode_scalar::<P>(pairing, &expected_mint_challenge)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.alpha),
                encode_scalar::<P>(pairing, &mint_responses.alpha)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.r),
                encode_scalar::<P>(pairing, &mint_responses.r)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.rho),
                encode_scalar::<P>(pairing, &mint_responses.rho)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.sigma),
                encode_scalar::<P>(pairing, &mint_responses.sigma)
            );
            let transfer_challenge_mutation = &record.transfer_mutations[26];
            assert_eq!(
                transfer_challenge_mutation.field,
                TransferMutationField::Challenge
            );
            let transfer_mutated = delivery_proof
                .proof_components(
                    ProofComponentsParams,
                    ProofComponentsPayload {
                        proof: &transfer_challenge_mutation.proof,
                    },
                )
                .expect("the transfer mutation's components are infallible")
                .components;
            let (mutated_challenge, mutated_responses) = match transfer_mutated {
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
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_challenge),
                encode_scalar::<P>(pairing, &expected_transfer_challenge)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.x),
                encode_scalar::<P>(pairing, &transfer_responses.x)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.y),
                encode_scalar::<P>(pairing, &transfer_responses.y)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.s),
                encode_scalar::<P>(pairing, &transfer_responses.s)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.rho),
                encode_scalar::<P>(pairing, &transfer_responses.rho)
            );
            assert_eq!(
                encode_scalar::<P>(pairing, &mutated_responses.sigma),
                encode_scalar::<P>(pairing, &transfer_responses.sigma)
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(Default::default()),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery under an asset identity → the admitted identity's
///   setup scope `SetupScope::Asset { root }` and identity
///   `KemIdentity::Asset { root }` reach the mint delivery unchanged, and the
///   baseline statements carry the identity element it derived.
/// Arrange: the KEM selection under the asset scope, the root `build_root`
///   builds from `[0x44; 32]`, `count` of `1`; the expected element from the
///   received KEM over the independent source's first three draws.
/// Act:     `statement_mutations`.
/// Assert:  the `encode_g1` of the baseline mint statement's
///   `identity_element` equals the `encode_g1` of the expected element's
///   `element`.
#[test]
fn statement_mutations_carry_the_identity_element_of_an_asset_identity() {
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
            independent: &dyn IRandomSourceAdapter,
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
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(drawn_uniform::<P>(independent)),
                        u0_uniform: Some(drawn_uniform::<P>(independent)),
                        u1_uniform: Some(drawn_uniform::<P>(independent)),
                    }),
                )
                .expect("setup over the replayed uniforms succeeds");
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
            let expected_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("the identity element's components are infallible")
                .components;
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
            assert_eq!(
                encode_g1::<P>(pairing, &success.vectors[0].mint_statement.identity_element),
                encode_g1::<P>(pairing, &expected_element.element)
            );
        }
    }
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}
