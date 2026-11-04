#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{Secret, SecretConstructorParams};
use encoding::{
    CanonicalFieldValue, CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, ICanonicalField,
    IDecoderAdapter, IEncoderAdapter, ToFieldParams,
};
use envelope::{
    CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeComponents,
    EnvelopeFromComponentsParams, EnvelopeFromComponentsPayload, GenerateKeysErrorReturn,
    GenerateKeysParams, GenerateKeysPayload, IKeyAgreementAdapter, KeyAgreementConcrete,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    UnwrapParams, UnwrapPayload, WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
    build_domain_tag, expected_uniform_bytes,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CredentialFromComponentsParams, CredentialFromComponentsPayload, DECLARED_KEM_SELECTIONS,
    DeriveIdentityErrorReturn, DeriveIdentityPayloadOverrides, ICredentialKemAdapter,
    IdentityScope, IsValidParams, IsValidPayload, KemIdentity, SetupErrorReturn,
    SetupParamsOverrides, SetupPayloadOverrides, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_parameter_set_components_params,
    build_parameter_set_components_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    AddG1Params, AddG1Payload, CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload,
    EncodeG2Params, EncodeG2Payload, EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams,
    G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic,
    IPairingReference, ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides,
    DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, VerifyParams, VerifyPayload,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use super::interface::{IReplayCaseConsumer, ReplayCaseParams, ReplayErrorReturn};
use super::mock::{
    ReplayCaseParamsOverrides, ReplayParamsOverrides, ReplayPayloadOverrides,
    build_replay_case_deps, build_replay_case_params, build_replay_case_payload, build_replay_deps,
    build_replay_params, build_replay_payload, run_replay_case,
};
use super::replay;
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};
use crate::{VectorCountConstructorParamsOverrides, build_vector_count};

const CANONICAL_A: &[u8] = b"pkg@1.0.0#1";
const CANONICAL_B: &[u8] = b"pkg@2.0.0#7";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

fn case_params(overrides: ReplayCaseParamsOverrides) -> ReplayCaseParams {
    build_replay_case_params(overrides)
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
) -> ReplayCaseParams {
    case_params(ReplayCaseParamsOverrides {
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

fn field<X: ICanonicalField>(form: &X) -> CanonicalFieldValue {
    let Ok(canonical) = X::to_field(ToFieldParams, form);
    canonical.field
}

fn uniform_at<P: IPairingArithmetic>(draws: &[u8], index: usize) -> Secret<Vec<u8>> {
    let chunk = draws
        .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
        .nth(index)
        .expect("twelve draws at the declared length");
    let Ok(uniform) = Secret::try_new(SecretConstructorParams {
        value: chunk.to_vec(),
    });
    uniform
}

fn sampled<P: IPairingArithmetic>(uniform: Secret<Vec<u8>>) -> P::Scalar {
    P::Scalar::sample_from_uniform_bytes(
        SampleUniformScalarParams,
        SampleUniformScalarPayload { uniform },
    )
    .expect("a uniform of the declared length samples")
    .scalar
    .expose()
    .clone()
}

/// Contract: `replay` over a case with a count above one, every adapter real
///   and the operating-system source real → both settlements of every record
///   verify and the replayed proof is refused.
/// Arrange: `count` of `2`; every case.
/// Act:     `replay`.
/// Assert:  `vectors` holds `2` records; for each, `transfer_is_valid` and
///   `next_is_valid` are `true` and `replay_is_valid` is `false`.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn each_settlement_verifies_and_the_replayed_proof_is_refused() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(2),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 2);
            for record in &success.vectors {
                assert!(record.transfer_is_valid);
                assert!(record.next_is_valid);
                assert!(!record.replay_is_valid);
            }
        }
    }
    all_cases(
        |pairing_concrete,
         h2s_selection,
         encoding_selection,
         forms_concrete,
         kem_selection,
         key_agreement_selection,
         proof_concrete| {
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    pairing_concrete,
                    h2s_selection,
                    encoding_selection,
                    forms_concrete,
                    kem_selection,
                    key_agreement_selection,
                    proof_concrete,
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        },
    );
}

/// Contract: the next statement's seller keys are the transfer's buyer keys,
///   its old envelope the transfer's new envelope, its identity element the
///   transfer's, and each statement's context the typed context it was built
///   over.
/// Arrange: `count` of `1`; every case; the expected context values read from
///   each typed context the block built: each form's `to_field`, the domain
///   values' `identifier()`, `version()`, and `as_bytes()`, the purpose's
///   `code()`, and the expiry.
/// Act:     `replay`.
/// Assert:  the encodings of the next statement's `seller_keys`,
///   `old_envelope`, and `identity_element` equal those of the transfer
///   statement's `buyer_keys`, `new_envelope`, and `identity_element`; each
///   statement's context's form-typed fields read back through `to_field` to
///   the expected values, its `suite_identifier` has the expected
///   `identifier()` and `version()`, its `asset_root` and
///   `parameter_set_identifier` the expected `as_bytes()`, its purpose the
///   expected `code()`, and its `expiry` the expected value.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn the_next_statement_follows_the_transfer_it_replays_against() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert_eq!(
                encode_g1(pairing, &record.next_statement.seller_keys.pk1),
                encode_g1(pairing, &record.transfer_statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(pairing, &record.next_statement.seller_keys.pk2),
                encode_g2(pairing, &record.transfer_statement.buyer_keys.pk2)
            );
            assert_eq!(
                encode_g1(pairing, &record.next_statement.old_envelope.c1),
                encode_g1(pairing, &record.transfer_statement.new_envelope.c1)
            );
            assert_eq!(
                encode_g1(pairing, &record.next_statement.old_envelope.c2),
                encode_g1(pairing, &record.transfer_statement.new_envelope.c2)
            );
            assert_eq!(
                encode_g2(pairing, &record.next_statement.old_envelope.d1),
                encode_g2(pairing, &record.transfer_statement.new_envelope.d1)
            );
            assert_eq!(
                encode_g2(pairing, &record.next_statement.old_envelope.d2),
                encode_g2(pairing, &record.transfer_statement.new_envelope.d2)
            );
            assert_eq!(
                encode_g1(pairing, &record.next_statement.identity_element),
                encode_g1(pairing, &record.transfer_statement.identity_element)
            );
            let transfer_statement = &record.transfer_statement;
            assert_eq!(
                field(&transfer_statement.context.chain),
                field(&transfer_context.chain)
            );
            assert_eq!(
                field(&transfer_statement.context.entitlement_contract),
                field(&transfer_context.entitlement_contract)
            );
            assert_eq!(
                transfer_statement.context.asset_root.as_bytes(),
                transfer_context.asset_root.as_bytes()
            );
            assert_eq!(
                transfer_statement
                    .context
                    .parameter_set_identifier
                    .as_bytes(),
                transfer_context.parameter_set_identifier.as_bytes()
            );
            assert_eq!(
                field(&transfer_statement.context.source_entitlement),
                field(&transfer_context.source_entitlement)
            );
            assert_eq!(
                field(&transfer_statement.context.target_entitlement),
                field(&transfer_context.target_entitlement)
            );
            assert_eq!(
                field(&transfer_statement.context.old_interval),
                field(&transfer_context.old_interval)
            );
            assert_eq!(
                field(&transfer_statement.context.new_interval),
                field(&transfer_context.new_interval)
            );
            assert_eq!(
                transfer_statement.context.suite_identifier.identifier(),
                transfer_context.suite_identifier.identifier()
            );
            assert_eq!(
                transfer_statement.context.suite_identifier.version(),
                transfer_context.suite_identifier.version()
            );
            assert_eq!(
                transfer_statement.context.purpose.code(),
                transfer_context.purpose.code()
            );
            assert_eq!(
                field(&transfer_statement.context.seller),
                field(&transfer_context.seller)
            );
            assert_eq!(
                field(&transfer_statement.context.buyer),
                field(&transfer_context.buyer)
            );
            assert_eq!(transfer_statement.context.expiry, transfer_context.expiry);
            let next_statement = &record.next_statement;
            assert_eq!(
                field(&next_statement.context.chain),
                field(&next_context.chain)
            );
            assert_eq!(
                field(&next_statement.context.entitlement_contract),
                field(&next_context.entitlement_contract)
            );
            assert_eq!(
                next_statement.context.asset_root.as_bytes(),
                next_context.asset_root.as_bytes()
            );
            assert_eq!(
                next_statement.context.parameter_set_identifier.as_bytes(),
                next_context.parameter_set_identifier.as_bytes()
            );
            assert_eq!(
                field(&next_statement.context.source_entitlement),
                field(&next_context.source_entitlement)
            );
            assert_eq!(
                field(&next_statement.context.target_entitlement),
                field(&next_context.target_entitlement)
            );
            assert_eq!(
                field(&next_statement.context.old_interval),
                field(&next_context.old_interval)
            );
            assert_eq!(
                field(&next_statement.context.new_interval),
                field(&next_context.new_interval)
            );
            assert_eq!(
                next_statement.context.suite_identifier.identifier(),
                next_context.suite_identifier.identifier()
            );
            assert_eq!(
                next_statement.context.suite_identifier.version(),
                next_context.suite_identifier.version()
            );
            assert_eq!(
                next_statement.context.purpose.code(),
                next_context.purpose.code()
            );
            assert_eq!(
                field(&next_statement.context.seller),
                field(&next_context.seller)
            );
            assert_eq!(
                field(&next_statement.context.buyer),
                field(&next_context.buyer)
            );
            assert_eq!(next_statement.context.expiry, next_context.expiry);
        }
    }
    all_cases(
        |pairing_concrete,
         h2s_selection,
         encoding_selection,
         forms_concrete,
         kem_selection,
         key_agreement_selection,
         proof_concrete| {
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    pairing_concrete,
                    h2s_selection,
                    encoding_selection,
                    forms_concrete,
                    kem_selection,
                    key_agreement_selection,
                    proof_concrete,
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        },
    );
}

/// Contract: the record's `draws`, split at `UNIFORM_BYTES_LENGTH` and replayed
///   through `setup` under the admitted identity's setup scope,
///   `derive_identity` over its identity, and `generate_keys` on the case's
///   adapters, reproduce the statements' `identity_element` and keys.
/// Arrange: `count` of `1`, the entitlement scope, each case run under each of
///   the admitted identities over the canonical bytes `b"pkg@1.0.0#1"` and
///   `b"pkg@2.0.0#7"`.
/// Act:     `replay`.
/// Assert:  the replayed identity element's `element` and each replayed key
///   pair's `pk1` and `pk2`, each encoded through the received pairing, equal
///   both statements' `identity_element`, the transfer statement's
///   `seller_keys` and `buyer_keys`, and the next statement's `buyer_keys`; the
///   `encode_scalar` of the replayed identity element's `scalar` equals the
///   `encode_scalar` of the scalar `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes` over the tag `build_domain_tag` builds from
///   `K::DECLARATION.identity_tag`, the canonical bytes, and
///   `UNIFORM_BYTES_LENGTH`, and the `encode_g1` of its `element` equals the
///   `encode_g1` of the `add_g1` of the replayed parameter set's `u0` and the
///   `mul_g1` of its `u1` by that scalar; and each replayed key pair's `pk1`
///   and `pk2` encode equal to the `encode_g1` of the `mul_g1` of
///   `g1_generator` and the `encode_g2` of the `mul_g2` of `g2_generator` by
///   the scalars `sample_from_uniform_bytes` returns for that key pair's `x`
///   and `y` uniforms.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_statements_it_produced() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(uniform_at::<P>(&record.draws, 0)),
                        u0_uniform: Some(uniform_at::<P>(&record.draws, 1)),
                        u1_uniform: Some(uniform_at::<P>(&record.draws, 2)),
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
            let replayed_element = kem
                .identity_element_components(
                    build_identity_element_components_params(Default::default()),
                    build_identity_element_components_payload(
                        &derived.identity_element,
                        Default::default(),
                    ),
                )
                .expect("identity_element_components over the derived element succeeds");
            assert_eq!(
                encode_g1(pairing, &replayed_element.components.element),
                encode_g1(pairing, &record.transfer_statement.identity_element)
            );
            assert_eq!(
                encode_g1(pairing, &replayed_element.components.element),
                encode_g1(pairing, &record.next_statement.identity_element)
            );
            let holder = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: uniform_at::<P>(&record.draws, 4),
                        y_uniform: uniform_at::<P>(&record.draws, 5),
                    },
                )
                .expect("generate_keys over the replayed uniforms succeeds");
            let holder_public = key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &holder.key_pair,
                    },
                )
                .expect("a key pair's public keys are infallible");
            assert_eq!(
                encode_g1(pairing, &holder_public.components.pk1),
                encode_g1(pairing, &record.transfer_statement.seller_keys.pk1)
            );
            assert_eq!(
                encode_g2(pairing, &holder_public.components.pk2),
                encode_g2(pairing, &record.transfer_statement.seller_keys.pk2)
            );
            let buyer = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: uniform_at::<P>(&record.draws, 6),
                        y_uniform: uniform_at::<P>(&record.draws, 7),
                    },
                )
                .expect("generate_keys over the replayed uniforms succeeds");
            let buyer_public = key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &buyer.key_pair,
                    },
                )
                .expect("a key pair's public keys are infallible");
            assert_eq!(
                encode_g1(pairing, &buyer_public.components.pk1),
                encode_g1(pairing, &record.transfer_statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(pairing, &buyer_public.components.pk2),
                encode_g2(pairing, &record.transfer_statement.buyer_keys.pk2)
            );
            let next_buyer = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: uniform_at::<P>(&record.draws, 9),
                        y_uniform: uniform_at::<P>(&record.draws, 10),
                    },
                )
                .expect("generate_keys over the replayed uniforms succeeds");
            let next_buyer_public = key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &next_buyer.key_pair,
                    },
                )
                .expect("a key pair's public keys are infallible");
            assert_eq!(
                encode_g1(pairing, &next_buyer_public.components.pk1),
                encode_g1(pairing, &record.next_statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(pairing, &next_buyer_public.components.pk2),
                encode_g2(pairing, &record.next_statement.buyer_keys.pk2)
            );
            let KemIdentity::Entitlement { canonical } = identity.identity else {
                return;
            };
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            let expected_scalar = sampled::<P>({
                let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                    value: expected_uniform_bytes(
                        &tag,
                        canonical,
                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    ),
                });
                uniform
            });
            assert_eq!(
                encode_scalar(pairing, &replayed_element.components.scalar),
                encode_scalar(pairing, &expected_scalar)
            );
            let parameter_set = kem
                .parameter_set_components(
                    build_parameter_set_components_params(Default::default()),
                    build_parameter_set_components_payload(
                        &setup.parameter_set,
                        Default::default(),
                    ),
                )
                .expect("parameter_set_components over the replayed set succeeds");
            let scaled = pairing
                .mul_g1(
                    MulG1Params,
                    MulG1Payload {
                        point: parameter_set.components.u1.clone(),
                        scalar: expected_scalar.clone(),
                    },
                )
                .expect("mul_g1 is infallible");
            let expected_element = pairing
                .add_g1(
                    AddG1Params,
                    AddG1Payload {
                        left: parameter_set.components.u0.clone(),
                        right: scaled.product,
                    },
                )
                .expect("add_g1 is infallible");
            assert_eq!(
                encode_g1(pairing, &replayed_element.components.element),
                encode_g1(pairing, &expected_element.sum)
            );
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let holder_x = sampled::<P>(uniform_at::<P>(&record.draws, 4));
            let holder_y = sampled::<P>(uniform_at::<P>(&record.draws, 5));
            assert_eq!(
                encode_g1(pairing, &holder_public.components.pk1),
                encode_g1(
                    pairing,
                    &pairing
                        .mul_g1(
                            MulG1Params,
                            MulG1Payload {
                                point: g1.point.clone(),
                                scalar: holder_x,
                            },
                        )
                        .expect("mul_g1 is infallible")
                        .product,
                )
            );
            assert_eq!(
                encode_g2(pairing, &holder_public.components.pk2),
                encode_g2(
                    pairing,
                    &pairing
                        .mul_g2(
                            MulG2Params,
                            MulG2Payload {
                                point: g2.point.clone(),
                                scalar: holder_y,
                            },
                        )
                        .expect("mul_g2 is infallible")
                        .product,
                )
            );
            let buyer_x = sampled::<P>(uniform_at::<P>(&record.draws, 6));
            let buyer_y = sampled::<P>(uniform_at::<P>(&record.draws, 7));
            assert_eq!(
                encode_g1(pairing, &buyer_public.components.pk1),
                encode_g1(
                    pairing,
                    &pairing
                        .mul_g1(
                            MulG1Params,
                            MulG1Payload {
                                point: g1.point.clone(),
                                scalar: buyer_x,
                            },
                        )
                        .expect("mul_g1 is infallible")
                        .product,
                )
            );
            assert_eq!(
                encode_g2(pairing, &buyer_public.components.pk2),
                encode_g2(
                    pairing,
                    &pairing
                        .mul_g2(
                            MulG2Params,
                            MulG2Payload {
                                point: g2.point.clone(),
                                scalar: buyer_y,
                            },
                        )
                        .expect("mul_g2 is infallible")
                        .product,
                )
            );
            let next_buyer_x = sampled::<P>(uniform_at::<P>(&record.draws, 9));
            let next_buyer_y = sampled::<P>(uniform_at::<P>(&record.draws, 10));
            assert_eq!(
                encode_g1(pairing, &next_buyer_public.components.pk1),
                encode_g1(
                    pairing,
                    &pairing
                        .mul_g1(
                            MulG1Params,
                            MulG1Payload {
                                point: g1.point.clone(),
                                scalar: next_buyer_x,
                            },
                        )
                        .expect("mul_g1 is infallible")
                        .product,
                )
            );
            assert_eq!(
                encode_g2(pairing, &next_buyer_public.components.pk2),
                encode_g2(
                    pairing,
                    &pairing
                        .mul_g2(
                            MulG2Params,
                            MulG2Payload {
                                point: g2.point.clone(),
                                scalar: next_buyer_y,
                            },
                        )
                        .expect("mul_g2 is infallible")
                        .product,
                )
            );
        }
    }
    all_cases(
        |pairing_concrete,
         h2s_selection,
         encoding_selection,
         forms_concrete,
         kem_selection,
         key_agreement_selection,
         proof_concrete| {
            if !matches!(kem_selection.scope, IdentityScope::Entitlement) {
                return;
            }
            for canonical in [CANONICAL_A, CANONICAL_B] {
                let result = run_replay_case(
                    &build_replay_case_deps(Consumer, Default::default()),
                    selections(
                        pairing_concrete,
                        h2s_selection,
                        encoding_selection,
                        forms_concrete,
                        kem_selection,
                        key_agreement_selection,
                        proof_concrete,
                    ),
                    build_replay_case_payload(canonical, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        },
    );
}

/// Contract: each settlement's new envelope opens under its recipient's key
///   pair to a credential valid for the entitlement under the record's
///   parameter set.
/// Arrange: `count` of `1`; every case; the parameter set, identity element,
///   and the buyer's and next buyer's key pairs replayed from the record's
///   draws through `setup`, `derive_identity`, and `generate_keys`.
/// Act:     `replay`.
/// Assert:  for each statement,
///   `envelope_from_components(EnvelopeFromComponentsParams,
///   EnvelopeFromComponentsPayload { components })` over its `new_envelope`,
///   `unwrap` under its recipient's replayed key pair, and
///   `credential_from_components` over the unwrapped components yield a
///   credential whose `is_valid(IsValidParams, IsValidPayload {
///   parameter_set, identity_element, credential })` under the replayed
///   parameter set and identity element is `true`.
/// Boundary: `replay`, `envelope_from_components`, `unwrap`,
///   `credential_from_components`, and `is_valid`, through the chain of real
///   functions the private surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_delivered_envelope_opens_to_a_credential_valid_for_the_entitlement() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            let setup = kem
                .setup(
                    build_setup_params(SetupParamsOverrides {
                        scope: Some(identity.setup_scope),
                    }),
                    build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                        master_uniform: Some(uniform_at::<P>(&record.draws, 0)),
                        u0_uniform: Some(uniform_at::<P>(&record.draws, 1)),
                        u1_uniform: Some(uniform_at::<P>(&record.draws, 2)),
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
            let buyer = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: uniform_at::<P>(&record.draws, 6),
                        y_uniform: uniform_at::<P>(&record.draws, 7),
                    },
                )
                .expect("generate_keys over the replayed uniforms succeeds");
            let next_buyer = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: uniform_at::<P>(&record.draws, 9),
                        y_uniform: uniform_at::<P>(&record.draws, 10),
                    },
                )
                .expect("generate_keys over the replayed uniforms succeeds");
            let transfer_envelope = key_agreement
                .envelope_from_components(
                    EnvelopeFromComponentsParams,
                    EnvelopeFromComponentsPayload {
                        components: EnvelopeComponents {
                            c1: record.transfer_statement.new_envelope.c1.clone(),
                            c2: record.transfer_statement.new_envelope.c2.clone(),
                            d1: record.transfer_statement.new_envelope.d1.clone(),
                            d2: record.transfer_statement.new_envelope.d2.clone(),
                        },
                    },
                )
                .expect("the envelope's components rebuild the envelope");
            let opened = key_agreement
                .unwrap(
                    UnwrapParams,
                    UnwrapPayload {
                        key_pair: &buyer.key_pair,
                        envelope: &transfer_envelope.envelope,
                    },
                )
                .expect("the delivered envelope opens under the buyer's keys");
            let credential = kem
                .credential_from_components(
                    CredentialFromComponentsParams,
                    CredentialFromComponentsPayload {
                        components: opened.credential,
                    },
                )
                .expect("the opened components rebuild the credential");
            let valid = kem
                .is_valid(
                    IsValidParams,
                    IsValidPayload {
                        parameter_set: &setup.parameter_set,
                        identity_element: &derived.identity_element,
                        credential: &credential.credential,
                    },
                )
                .expect("is_valid is infallible");
            assert!(valid.is_valid);
            let next_envelope = key_agreement
                .envelope_from_components(
                    EnvelopeFromComponentsParams,
                    EnvelopeFromComponentsPayload {
                        components: EnvelopeComponents {
                            c1: record.next_statement.new_envelope.c1.clone(),
                            c2: record.next_statement.new_envelope.c2.clone(),
                            d1: record.next_statement.new_envelope.d1.clone(),
                            d2: record.next_statement.new_envelope.d2.clone(),
                        },
                    },
                )
                .expect("the envelope's components rebuild the envelope");
            let next_opened = key_agreement
                .unwrap(
                    UnwrapParams,
                    UnwrapPayload {
                        key_pair: &next_buyer.key_pair,
                        envelope: &next_envelope.envelope,
                    },
                )
                .expect("the delivered envelope opens under the next buyer's keys");
            let next_credential = kem
                .credential_from_components(
                    CredentialFromComponentsParams,
                    CredentialFromComponentsPayload {
                        components: next_opened.credential,
                    },
                )
                .expect("the opened components rebuild the credential");
            let next_valid = kem
                .is_valid(
                    IsValidParams,
                    IsValidPayload {
                        parameter_set: &setup.parameter_set,
                        identity_element: &derived.identity_element,
                        credential: &next_credential.credential,
                    },
                )
                .expect("is_valid is infallible");
            assert!(next_valid.is_valid);
        }
    }
    all_cases(
        |pairing_concrete,
         h2s_selection,
         encoding_selection,
         forms_concrete,
         kem_selection,
         key_agreement_selection,
         proof_concrete| {
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    pairing_concrete,
                    h2s_selection,
                    encoding_selection,
                    forms_concrete,
                    kem_selection,
                    key_agreement_selection,
                    proof_concrete,
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        },
    );
}

/// Contract: the list holds the configured count, each record's `draws` hold
///   its twelve uniforms at their length, and no two records share `draws`.
/// Arrange: `count` of `3`; every member of `PAIRING_CONCRETES`.
/// Act:     `replay`.
/// Assert:  `vectors` holds `3` records whose `draws` each hold `12` times
///   `UNIFORM_BYTES_LENGTH` bytes, and no two records share `draws`.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(3),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            assert_eq!(success.vectors.len(), 3);
            for record in &success.vectors {
                assert_eq!(
                    record.draws.len(),
                    12 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                );
            }
            assert_ne!(success.vectors[0].draws, success.vectors[1].draws);
            assert_ne!(success.vectors[0].draws, success.vectors[2].draws);
            assert_ne!(success.vectors[1].draws, success.vectors[2].draws);
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        let result = run_replay_case(
            &build_replay_case_deps(Consumer, Default::default()),
            selections(
                *pairing_concrete,
                &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                &DECLARED_ENCODING_SELECTIONS[0],
                &CHAIN_FORMS_CONCRETES[0],
                &DECLARED_KEM_SELECTIONS[0],
                &DECLARED_KEY_AGREEMENT_SELECTIONS[0],
                &DELIVERY_PROOF_CONCRETES[0],
            ),
            build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
        );
        assert!(result.is_ok());
    }
}

/// Contract: `replay` over an admitted asset identity → `Ok`, both settlements
///   verify and the replay is refused.
/// Arrange: the KEM selection under the asset scope, the root `build_root`
///   builds from `[0x44; 32]`, `count` of `1`; every case that declares the
///   asset scope.
/// Act:     `replay`.
/// Assert:  the result is `Ok`, `transfer_is_valid` and `next_is_valid` are
///   `true`, and `replay_is_valid` is `false`.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn an_asset_identity_replays_under_the_asset_scope() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            assert!(record.transfer_is_valid);
            assert!(record.next_is_valid);
            assert!(!record.replay_is_valid);
        }
    }
    all_cases(
        |pairing_concrete,
         h2s_selection,
         encoding_selection,
         forms_concrete,
         kem_selection,
         key_agreement_selection,
         proof_concrete| {
            if !matches!(kem_selection.scope, IdentityScope::Asset) {
                return;
            }
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    pairing_concrete,
                    h2s_selection,
                    encoding_selection,
                    forms_concrete,
                    kem_selection,
                    key_agreement_selection,
                    proof_concrete,
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        },
    );
}

/// Contract: a hash-to-scalar refusal reaches the caller inside the mint
///   delivery's `DeriveIdentity` variant under the entitlement scope and its
///   `Setup` variant under the asset scope, within the function's `Mint`
///   variant, whole.
/// Arrange: `count` of `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, with
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`;
///   the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength
///   { expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
///   actual: 0 })`.
/// Act:     `replay`.
/// Assert:  the result equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error))))`
///   under the entitlement scope and
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(error))))`
///   under the asset scope, whole.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the hash-to-scalar family's mock
///   concrete is selected by configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_whole() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let error =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            let expected = match identity.identity {
                KemIdentity::Entitlement { .. } => ReplayErrorReturn::Mint(
                    MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)),
                ),
                KemIdentity::Asset { .. } => ReplayErrorReturn::Mint(MintErrorReturn::Setup(
                    SetupErrorReturn::HashToScalar(error),
                )),
            };
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(returned.err(), Some(expected));
        }
    }
    let refusing_h2s = build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
        concrete: Some(HashToScalarConcrete::Mock(
            MockIHashToScalarAdapterFailureMode::SamplingRefused,
        )),
        ..Default::default()
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    *pairing_concrete,
                    &refusing_h2s,
                    &DECLARED_ENCODING_SELECTIONS[0],
                    &CHAIN_FORMS_CONCRETES[0],
                    kem_selection,
                    &DECLARED_KEY_AGREEMENT_SELECTIONS[0],
                    &DELIVERY_PROOF_CONCRETES[0],
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        }
    }
}

/// Contract: a nonce-draw refusal of the mint delivery's prover reaches the
///   caller inside the function's `Mint` variant, whole.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, with
///   `build_create_delivery_proof_params` overriding `concrete` with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NonceDrawRefused)`,
///   and, as the counterfactual, each member of `DELIVERY_PROOF_CONCRETES`.
/// Act:     `replay`, once per delivery-proof selection.
/// Assert:  the result under the refusing selection equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the delivery-proof family's mock
///   concrete is selected by configuration.
#[test]
fn a_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected =
                ReplayErrorReturn::Mint(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                )));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(returned.err(), Some(expected));
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_replay_case(
                &build_replay_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                selections(
                    *pairing_concrete,
                    &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                    &DECLARED_ENCODING_SELECTIONS[0],
                    &CHAIN_FORMS_CONCRETES[0],
                    kem_selection,
                    &DECLARED_KEY_AGREEMENT_SELECTIONS[0],
                    &DeliveryProofConcrete::Mock(
                        MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                    ),
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                let result = run_replay_case(
                    &build_replay_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    selections(
                        *pairing_concrete,
                        &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                        &DECLARED_ENCODING_SELECTIONS[0],
                        &CHAIN_FORMS_CONCRETES[0],
                        kem_selection,
                        &DECLARED_KEY_AGREEMENT_SELECTIONS[0],
                        proof_concrete,
                    ),
                    build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: a nonce-draw refusal of the mint delivery's key generation for
///   the holder reaches the caller inside the function's `Mint` variant,
///   whole.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, with
///   `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`,
///   and, as the counterfactual, each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`.
/// Act:     `replay`, once per key-agreement selection.
/// Assert:  the result under the refusing selection equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the key-agreement family's mock
///   concrete is selected by configuration.
#[test]
fn a_key_generation_nonce_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::GenerateKeys(
                GenerateKeysErrorReturn::Mock(
                    MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                ),
            ));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(returned.err(), Some(expected));
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let refusing = build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                concrete: Some(KeyAgreementConcrete::Mock(
                    MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                )),
                ..Default::default()
            });
            let result = run_replay_case(
                &build_replay_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                selections(
                    *pairing_concrete,
                    &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                    &DECLARED_ENCODING_SELECTIONS[0],
                    &CHAIN_FORMS_CONCRETES[0],
                    kem_selection,
                    &refusing,
                    &DELIVERY_PROOF_CONCRETES[0],
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_replay_case(
                    &build_replay_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    selections(
                        *pairing_concrete,
                        &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                        &DECLARED_ENCODING_SELECTIONS[0],
                        &CHAIN_FORMS_CONCRETES[0],
                        kem_selection,
                        key_agreement_selection,
                        &DELIVERY_PROOF_CONCRETES[0],
                    ),
                    build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: a coin-draw refusal of the mint delivery's wrap to the holder's
///   keys reaches the caller inside the function's `Mint` variant, whole.
/// Arrange: `count` of `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, with
///   `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`,
///   and, as the counterfactual, each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`.
/// Act:     `replay`, once per key-agreement selection.
/// Assert:  the result under the refusing selection equals
///   `Err(ReplayErrorReturn::Mint(MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: `replay` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the key-agreement family's mock
///   concrete is selected by configuration.
#[test]
fn a_wrap_coin_draw_refusal_reaches_the_caller_whole() {
    struct Consumer {
        expects_refusal: bool,
    }

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let expected = ReplayErrorReturn::Mint(MintErrorReturn::WrapTo(
                WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused),
            ));
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_replay_params(ReplayParamsOverrides {
                    count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                        count: Some(1),
                    })),
                }),
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            if self.expects_refusal {
                assert_eq!(returned.err(), Some(expected));
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let refusing = build_create_key_agreement_params(CreateKeyAgreementParamsOverrides {
                concrete: Some(KeyAgreementConcrete::Mock(
                    MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                )),
                ..Default::default()
            });
            let result = run_replay_case(
                &build_replay_case_deps(
                    Consumer {
                        expects_refusal: true,
                    },
                    Default::default(),
                ),
                selections(
                    *pairing_concrete,
                    &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                    &DECLARED_ENCODING_SELECTIONS[0],
                    &CHAIN_FORMS_CONCRETES[0],
                    kem_selection,
                    &refusing,
                    &DELIVERY_PROOF_CONCRETES[0],
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_replay_case(
                    &build_replay_case_deps(
                        Consumer {
                            expects_refusal: false,
                        },
                        Default::default(),
                    ),
                    selections(
                        *pairing_concrete,
                        &DECLARED_HASH_TO_SCALAR_SELECTIONS[0],
                        &DECLARED_ENCODING_SELECTIONS[0],
                        &CHAIN_FORMS_CONCRETES[0],
                        kem_selection,
                        key_agreement_selection,
                        &DELIVERY_PROOF_CONCRETES[0],
                    ),
                    build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
                );
                assert!(result.is_ok());
            }
        }
    }
}

/// Contract: `verify` over each statement and proof pair the record names
///   returns the verdict the record holds.
/// Arrange: `count` of `1`; every case.
/// Act:     `replay`.
/// Assert:  `verify` on the received delivery proof over
///   `AlgebraicStatement::Transfer(&transfer_statement)` and `&transfer_proof`
///   returns `is_valid` `true`, over
///   `AlgebraicStatement::Transfer(&next_statement)` and `&next_proof` returns
///   `true`, and over `AlgebraicStatement::Transfer(&next_statement)` and
///   `&transfer_proof` returns `false`, each equal to the record's
///   `transfer_is_valid`, `next_is_valid`, and `replay_is_valid`.
/// Boundary: `replay` and `verify`, through the chain of real functions the
///   private surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_carried_statement_and_proof_verify_to_their_recorded_verdict() {
    struct Consumer;

    impl IReplayCaseConsumer for Consumer {
        type Output = ();

        fn consume_replay_case<P, H, E, F, K, A, D>(
            &self,
            _pairing: &P,
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
            let next_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer_context.chain.clone()),
                    entitlement_contract: Some(transfer_context.entitlement_contract.clone()),
                    asset_root: Some(transfer_context.asset_root),
                    parameter_set_identifier: Some(
                        transfer_context.parameter_set_identifier.clone(),
                    ),
                    source_entitlement: Some(transfer_context.source_entitlement.clone()),
                    target_entitlement: Some(transfer_context.target_entitlement.clone()),
                    old_interval: Some(transfer_context.new_interval.clone()),
                    seller: Some(transfer_context.buyer.clone()),
                    ..Default::default()
                },
            );
            let params = build_replay_params(ReplayParamsOverrides {
                count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
                    count: Some(1),
                })),
            });
            // Act
            let returned = replay(
                &build_replay_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                params,
                build_replay_payload(
                    &mint_context,
                    &transfer_context,
                    &next_context,
                    ReplayPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let success = returned.expect("the subject's calls succeed");
            let record = &success.vectors[0];
            let transfer_verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&record.transfer_statement),
                        proof: &record.transfer_proof,
                    },
                )
                .expect("the reference verifier returns a verdict")
                .is_valid;
            assert_eq!(record.transfer_is_valid, transfer_verdict);
            assert!(transfer_verdict);
            let next_verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&record.next_statement),
                        proof: &record.next_proof,
                    },
                )
                .expect("the reference verifier returns a verdict")
                .is_valid;
            assert_eq!(record.next_is_valid, next_verdict);
            assert!(next_verdict);
            let replay_verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&record.next_statement),
                        proof: &record.transfer_proof,
                    },
                )
                .expect("the reference verifier returns a verdict")
                .is_valid;
            assert_eq!(record.replay_is_valid, replay_verdict);
            assert!(!replay_verdict);
        }
    }
    all_cases(
        |pairing_concrete,
         h2s_selection,
         encoding_selection,
         forms_concrete,
         kem_selection,
         key_agreement_selection,
         proof_concrete| {
            let result = run_replay_case(
                &build_replay_case_deps(Consumer, Default::default()),
                selections(
                    pairing_concrete,
                    h2s_selection,
                    encoding_selection,
                    forms_concrete,
                    kem_selection,
                    key_agreement_selection,
                    proof_concrete,
                ),
                build_replay_case_payload(CANONICAL_A, &asset_root(), Default::default()),
            );
            assert!(result.is_ok());
        },
    );
}
