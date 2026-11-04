#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{GroupIndexConstructorParamsOverrides, Secret, build_group_index};
use encoding::{
    CanonicalFieldValue, ICanonicalField, IDecoderAdapter, IEncoderAdapter, ToFieldParams,
    build_from_field_params,
};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kdf::IKeyDerivationAdapter;
use kem::{
    CapsuleComponents, CreateKemParamsOverrides, DeriveIdentityPayloadOverrides,
    EncapsulatePayloadOverrides, ICredentialKemAdapter, IdentityScope, KemIdentity,
    SetupParamsOverrides, SetupPayloadOverrides, SetupScope, build_capsule_components_params,
    build_capsule_components_payload, build_create_kem_params, build_derive_identity_params,
    build_derive_identity_payload, build_encapsulate_params, build_encapsulate_payload,
    build_identity_element_components_params, build_identity_element_components_payload,
    build_is_valid_params, build_is_valid_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
};
use proof::{
    DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_REPLACEMENT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, IDeliveryProofAdapter, MintPurpose,
    TransferPurpose, build_delivery_context,
};
use random::{
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter, build_fill_bytes_payload,
};
use workflows::{
    DerivationContextConstructorParamsOverrides, PIECE_GROUP_KEY_LENGTH,
    PieceGroupKeyConstructorParamsOverrides, build_derivation_context, build_piece_group_key,
    build_wrap_piece_group_key_deps, build_wrap_piece_group_key_params,
    build_wrap_piece_group_key_payload, wrap_piece_group_key,
};

use super::interface::IVectorsCaseConsumer;
use super::mock::{
    VectorsCaseParamsOverrides, VectorsParamsOverrides, VectorsPayloadOverrides,
    build_vectors_case_deps, build_vectors_case_params, build_vectors_case_payload,
    build_vectors_deps, build_vectors_params, build_vectors_payload, run_vectors_case,
};
use super::vectors;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::{VectorCountConstructorParamsOverrides, build_vector_count};

const CANONICAL: &[u8] = b"pkg@1.0.0#1";
const OTHER_CANONICAL: &[u8] = b"pkg@2.0.0#7";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

fn other_asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x45; 32]),
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

fn draw_bytes(source: &dyn IRandomSourceAdapter, length: usize) -> Secret<Vec<u8>> {
    source
        .fill_bytes(
            FillBytesParams,
            build_fill_bytes_payload(FillBytesPayloadOverrides {
                length: Some(length),
            }),
        )
        .expect("the independent source fills")
        .bytes
}

fn draw_uniform<P: IPairingArithmetic>(source: &dyn IRandomSourceAdapter) -> Secret<Vec<u8>> {
    draw_bytes(
        source,
        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
    )
}

fn default_contexts<F: IChainForms>() -> (
    proof::DeliveryContext<F, MintPurpose>,
    proof::DeliveryContext<F, TransferPurpose>,
    proof::DeliveryContext<F, TransferPurpose>,
) {
    let mint = build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
    let transfer =
        build_delivery_context::<F, TransferPurpose>(DELIVERY_PURPOSE_TRANSFER, Default::default());
    let next = build_delivery_context::<F, TransferPurpose>(
        DELIVERY_PURPOSE_TRANSFER,
        DeliveryContextOverrides {
            chain: Some(transfer.chain.clone()),
            entitlement_contract: Some(transfer.entitlement_contract.clone()),
            asset_root: Some(transfer.asset_root),
            parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
            source_entitlement: Some(transfer.source_entitlement.clone()),
            target_entitlement: Some(transfer.target_entitlement.clone()),
            old_interval: Some(transfer.new_interval.clone()),
            seller: Some(transfer.buyer.clone()),
            ..Default::default()
        },
    );
    (mint, transfer, next)
}

fn expected_element<P: IPairingArithmetic, K: ICredentialKemAdapter<Pairing = P>>(
    kem: &K,
    scope: SetupScope<'_>,
    kem_identity: KemIdentity<'_>,
    master: Secret<Vec<u8>>,
    u0: Secret<Vec<u8>>,
    u1: Secret<Vec<u8>>,
) -> (K::ParameterSet, K::IdentityElement) {
    let setup = kem
        .setup(
            build_setup_params(SetupParamsOverrides { scope: Some(scope) }),
            build_setup_payload::<P::Scalar>(SetupPayloadOverrides {
                master_uniform: Some(master),
                u0_uniform: Some(u0),
                u1_uniform: Some(u1),
            }),
        )
        .expect("the replay's setup succeeds");
    let derived = kem
        .derive_identity(
            build_derive_identity_params(Default::default()),
            build_derive_identity_payload(
                &setup.parameter_set,
                DeriveIdentityPayloadOverrides {
                    identity: Some(kem_identity),
                },
            ),
        )
        .expect("the replay's identity derivation succeeds");
    (setup.parameter_set, derived.identity_element)
}

fn identity_element_bytes<P: IPairingArithmetic, K: ICredentialKemAdapter<Pairing = P>>(
    pairing: &P,
    kem: &K,
    identity_element: &K::IdentityElement,
) -> Vec<u8> {
    let components = kem
        .identity_element_components(
            build_identity_element_components_params(Default::default()),
            build_identity_element_components_payload(identity_element, Default::default()),
        )
        .expect("the identity element's components are infallible")
        .components;
    encode_g1(pairing, &components.element)
}

/// Contract: Computed → `Ok(VectorsSuccessReturn { .. })`, each set's success
///   return in its own field, each holding its own configured count.
/// Arrange: the counts `1`, `2`, `3`, `4`, `5`, `6`, and `7` for `cross_holder`,
///   `rerandomization`, `non_convertibility`, `malformed_capsules`, `cross_set`,
///   `statement_mutations`, and `replay`, so a count handed to another set
///   reports otherwise.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  each set's `vectors` holds its count.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_returns_each_set_with_its_configured_count() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            let count = |count: u32| {
                build_vector_count(VectorCountConstructorParamsOverrides { count: Some(count) })
            };
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(VectorsParamsOverrides {
                    cross_holder: Some(count(1)),
                    rerandomization: Some(count(2)),
                    non_convertibility: Some(count(3)),
                    malformed_capsules: Some(count(4)),
                    cross_set: Some(count(5)),
                    statement_mutations: Some(count(6)),
                    replay: Some(count(7)),
                }),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.cross_holder.vectors.len(), 1);
            assert_eq!(success.rerandomization.vectors.len(), 2);
            assert_eq!(success.non_convertibility.vectors.len(), 3);
            assert_eq!(success.malformed_capsules.vectors.len(), 4);
            assert_eq!(success.cross_set.vectors.len(), 5);
            assert_eq!(success.statement_mutations.vectors.len(), 6);
            assert_eq!(success.replay.vectors.len(), 7);
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Ordering → the sets are called in order over the one source, each
///   set's draws following the previous set's.
/// Arrange: the expected draws from the independent source in call order: `6`
///   uniforms for `cross_holder`, `5` for `rerandomization`, `4` for
///   `non_convertibility`, `4` for `malformed_capsules`, the piece-group key's
///   `PIECE_GROUP_KEY_LENGTH` bytes and `10` uniforms for `cross_set`, `10`
///   uniforms for `statement_mutations`, and `12` for `replay`, so a transposed
///   set or a second source reports otherwise.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  each set's record's `draws` equal its expected draws.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_draws_every_set_from_the_one_source_in_call_order() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let uniforms = |count: usize| -> Vec<u8> {
                let mut draws = Vec::new();
                for _ in 0..count {
                    draws.extend_from_slice(draw_uniform::<P>(independent).expose().as_ref());
                }
                draws
            };
            let cross_holder_draws = uniforms(6);
            let rerandomization_draws = uniforms(5);
            let non_convertibility_draws = uniforms(4);
            let malformed_capsules_draws = uniforms(4);
            let mut cross_set_draws = draw_bytes(independent, PIECE_GROUP_KEY_LENGTH)
                .expose()
                .clone();
            cross_set_draws.extend_from_slice(&uniforms(10));
            let statement_mutations_draws = uniforms(10);
            let replay_draws = uniforms(12);
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            let count = |count: u32| {
                build_vector_count(VectorCountConstructorParamsOverrides { count: Some(count) })
            };
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(VectorsParamsOverrides {
                    cross_holder: Some(count(1)),
                    rerandomization: Some(count(1)),
                    non_convertibility: Some(count(1)),
                    malformed_capsules: Some(count(1)),
                    cross_set: Some(count(1)),
                    statement_mutations: Some(count(1)),
                    replay: Some(count(1)),
                }),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(success.cross_holder.vectors[0].draws, cross_holder_draws);
            assert_eq!(
                success.rerandomization.vectors[0].draws,
                rerandomization_draws
            );
            assert_eq!(
                success.non_convertibility.vectors[0].draws,
                non_convertibility_draws
            );
            assert_eq!(
                success.malformed_capsules.vectors[0].draws,
                malformed_capsules_draws
            );
            assert_eq!(success.cross_set.vectors[0].draws, cross_set_draws);
            assert_eq!(
                success.statement_mutations.vectors[0].draws,
                statement_mutations_draws
            );
            assert_eq!(success.replay.vectors[0].draws, replay_draws);
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: each set call → the admitted `payload.identity` reaches every set
///   unchanged.
/// Arrange: the canonical bytes `b"pkg@3.0.0#2"`, other than every default, so
///   the admitted identity holds them; for each of `cross_holder`,
///   `rerandomization`, `non_convertibility`, `cross_set`'s two members,
///   `statement_mutations`, and `replay`, the expected identity element from
///   the received KEM's `setup` under `SetupScope::Entitlement` over that set's
///   master, `u0`, and `u1` uniforms at their places in the expected draws,
///   then `derive_identity` over the entitlement identity and
///   `identity_element_components`.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the `encode_g1` of the `element` of each set's identity element
///   equals that of its expected element, and `is_valid` over
///   `non_convertibility`'s record credential returns `true`.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_hands_every_set_the_configured_identity() {
    const IDENTITY: &[u8] = b"pkg@3.0.0#2";

    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let draw = || draw_uniform::<P>(independent);
            let (_, cross_holder_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = (draw(), draw(), draw());
            let (_, rerandomization_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = (draw(), draw());
            let (non_convertibility_set, non_convertibility_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = draw();
            let (_, _malformed_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = draw();
            let _ = draw_bytes(independent, PIECE_GROUP_KEY_LENGTH);
            let (_, first_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = (draw(), draw());
            let (_, second_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = (draw(), draw());
            let (_, statement_mutations_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = (draw(), draw(), draw(), draw(), draw(), draw(), draw());
            let (_, replay_element) = expected_element(
                kem,
                SetupScope::Entitlement,
                KemIdentity::Entitlement {
                    canonical: IDENTITY,
                },
                draw(),
                draw(),
                draw(),
            );
            let _ = (
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
            );
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            let count = |count: u32| {
                build_vector_count(VectorCountConstructorParamsOverrides { count: Some(count) })
            };
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(VectorsParamsOverrides {
                    cross_holder: Some(count(1)),
                    rerandomization: Some(count(1)),
                    non_convertibility: Some(count(1)),
                    malformed_capsules: Some(count(1)),
                    cross_set: Some(count(1)),
                    statement_mutations: Some(count(1)),
                    replay: Some(count(1)),
                }),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.cross_holder.vectors[0].identity_element,
                ),
                identity_element_bytes(pairing, kem, &cross_holder_element)
            );
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.rerandomization.vectors[0].identity_element,
                ),
                identity_element_bytes(pairing, kem, &rerandomization_element)
            );
            assert!(
                kem.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &non_convertibility_set,
                        &non_convertibility_element,
                        &success.non_convertibility.vectors[0].credential,
                        Default::default(),
                    ),
                )
                .expect("the credential check is infallible")
                .is_valid
            );
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.cross_set.vectors[0].first.identity_element,
                ),
                identity_element_bytes(pairing, kem, &first_element)
            );
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.cross_set.vectors[0].second.identity_element,
                ),
                identity_element_bytes(pairing, kem, &second_element)
            );
            assert_eq!(
                encode_g1(
                    pairing,
                    &success.statement_mutations.vectors[0]
                        .mint_statement
                        .identity_element,
                ),
                identity_element_bytes(pairing, kem, &statement_mutations_element)
            );
            assert_eq!(
                encode_g1(
                    pairing,
                    &success.replay.vectors[0]
                        .transfer_statement
                        .identity_element,
                ),
                identity_element_bytes(pairing, kem, &replay_element)
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            IDENTITY,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Non-convertibility set → `payload.other_identity` reaches
///   `non_convertibility` as its other identity.
/// Arrange: the canonical bytes `b"pkg@1.0.0#1"` and the other canonical bytes
///   `b"pkg@2.0.0#7"`, distinct from them, so the identity handed in the other
///   identity's place reports otherwise.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  `non_convertibility`'s record's `is_valid` is `true` and its
///   `other_is_valid` is `false`.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_hands_non_convertibility_the_other_identity() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            let record = &success.non_convertibility.vectors[0];
            assert!(record.is_valid);
            assert!(!record.other_is_valid);
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Cross-set set → `payload.context` reaches `cross_set`.
/// Arrange: the context `build_derivation_context` returns overriding
///   `group_index` with `build_group_index` over an index other than its
///   default; the expected piece-group key over `cross_set`'s key draw, and for
///   the first member the received KEM's `setup` and `encapsulate` over its
///   uniforms, then `wrap_piece_group_key` over the received key-derivation
///   adapter and encoder, the encapsulated value, that context, and the
///   expected key.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the `as_bytes()` of `cross_set`'s record's `first.wrapped` equal
///   those of the expected wrapped key.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_hands_cross_set_the_configured_derivation_context() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let draw = || draw_uniform::<P>(independent);
            for _ in 0..19 {
                let _ = draw();
            }
            let context = build_derivation_context(DerivationContextConstructorParamsOverrides {
                group_index: Some(build_group_index(GroupIndexConstructorParamsOverrides {
                    value: Some(7),
                })),
                ..Default::default()
            });
            let replay_key = build_piece_group_key(PieceGroupKeyConstructorParamsOverrides {
                secret: Some(draw_bytes(independent, PIECE_GROUP_KEY_LENGTH)),
            });
            let first_setup = kem
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
                .expect("the replay's setup succeeds");
            let _ = draw();
            let first_encapsulated = kem
                .encapsulate(
                    build_encapsulate_params(Default::default()),
                    build_encapsulate_payload::<P::Scalar, _>(
                        &first_setup.parameter_set,
                        EncapsulatePayloadOverrides {
                            uniform: Some(draw()),
                        },
                    ),
                )
                .expect("the replay's encapsulation succeeds");
            for _ in 0..5 {
                let _ = draw();
            }
            let expected_wrapped = wrap_piece_group_key(
                &build_wrap_piece_group_key_deps(kdf, encoder, Default::default()),
                build_wrap_piece_group_key_params(Default::default()),
                build_wrap_piece_group_key_payload(
                    &first_encapsulated.encapsulated,
                    &context,
                    &replay_key,
                    Default::default(),
                ),
            )
            .expect("the replay's wrap succeeds");
            let (mint, transfer, next) = default_contexts::<F>();
            let count = |count: u32| {
                build_vector_count(VectorCountConstructorParamsOverrides { count: Some(count) })
            };
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(VectorsParamsOverrides {
                    cross_holder: Some(count(1)),
                    rerandomization: Some(count(1)),
                    non_convertibility: Some(count(1)),
                    malformed_capsules: Some(count(1)),
                    cross_set: Some(count(1)),
                    statement_mutations: Some(count(1)),
                    replay: Some(count(1)),
                }),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert_eq!(
                success.cross_set.vectors[0].first.wrapped.as_bytes(),
                expected_wrapped.wrapped.as_bytes()
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Statement-mutation and replay sets → the mint context reaches
///   `statement_mutations` alone, the transfer context both, and the next
///   context `replay` alone.
/// Arrange: the mint context overriding `buyer` with
///   `CanonicalFieldValue::FixedBytes20([0x71; 20])` and the next context
///   overriding `buyer` with `CanonicalFieldValue::FixedBytes20([0x63; 20])`,
///   so the three contexts' buyers differ.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  the `to_field` of each statement's context `buyer` equals the
///   context the set was handed.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_hands_each_delivery_set_its_contexts() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let buyer = |value: [u8; 20]| -> F::Identity {
                F::Identity::from_field(
                    build_from_field_params(Default::default()),
                    CanonicalFieldValue::FixedBytes20(value),
                )
                .expect("the value has the kind the identity form declares")
                .value
            };
            let context = build_derivation_context(Default::default());
            let mint = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides {
                    buyer: Some(buyer([0x71; 20])),
                    ..Default::default()
                },
            );
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            let next = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                DeliveryContextOverrides {
                    chain: Some(transfer.chain.clone()),
                    entitlement_contract: Some(transfer.entitlement_contract.clone()),
                    asset_root: Some(transfer.asset_root),
                    parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
                    source_entitlement: Some(transfer.source_entitlement.clone()),
                    target_entitlement: Some(transfer.target_entitlement.clone()),
                    old_interval: Some(transfer.new_interval.clone()),
                    seller: Some(transfer.buyer.clone()),
                    buyer: Some(buyer([0x63; 20])),
                    ..Default::default()
                },
            );
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            let to_field = |value: &F::Identity| {
                F::Identity::to_field(ToFieldParams, value)
                    .expect("the form field's conversion is infallible")
                    .field
            };
            let mint_record = &success.statement_mutations.vectors[0];
            assert_eq!(
                to_field(&mint_record.mint_statement.context.buyer),
                CanonicalFieldValue::FixedBytes20([0x71; 20])
            );
            assert_eq!(
                to_field(&mint_record.transfer_statement.context.buyer),
                to_field(&transfer.buyer)
            );
            let replay_record = &success.replay.vectors[0];
            assert_eq!(
                to_field(&replay_record.transfer_statement.context.buyer),
                to_field(&transfer.buyer)
            );
            assert_eq!(
                to_field(&replay_record.next_statement.context.buyer),
                CanonicalFieldValue::FixedBytes20([0x63; 20])
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: Statement-mutation and replay sets → the mint context's purpose
///   reaches `statement_mutations` alone and the transfer context's both.
/// Arrange: the mint context built with `DELIVERY_PURPOSE_REPLACEMENT`, and
///   the transfer and next contexts with `DELIVERY_PURPOSE_GRANT`.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  each statement's context purpose's `code()` equals its context's.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder.
#[test]
fn vectors_hands_each_delivery_set_its_purposes() {
    struct Consumer;

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let context = build_derivation_context(Default::default());
            let mint = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_REPLACEMENT,
                Default::default(),
            );
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                Default::default(),
            );
            let next = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_GRANT,
                DeliveryContextOverrides {
                    chain: Some(transfer.chain.clone()),
                    entitlement_contract: Some(transfer.entitlement_contract.clone()),
                    asset_root: Some(transfer.asset_root),
                    parameter_set_identifier: Some(transfer.parameter_set_identifier.clone()),
                    source_entitlement: Some(transfer.source_entitlement.clone()),
                    target_entitlement: Some(transfer.target_entitlement.clone()),
                    old_interval: Some(transfer.new_interval.clone()),
                    seller: Some(transfer.buyer.clone()),
                    ..Default::default()
                },
            );
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(Default::default()),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            let mint_record = &success.statement_mutations.vectors[0];
            assert_eq!(
                mint_record.mint_statement.context.purpose.code(),
                DELIVERY_PURPOSE_REPLACEMENT.code()
            );
            assert_eq!(
                mint_record.transfer_statement.context.purpose.code(),
                DELIVERY_PURPOSE_GRANT.code()
            );
            let replay_record = &success.replay.vectors[0];
            assert_eq!(
                replay_record.transfer_statement.context.purpose.code(),
                DELIVERY_PURPOSE_GRANT.code()
            );
            assert_eq!(
                replay_record.next_statement.context.purpose.code(),
                DELIVERY_PURPOSE_GRANT.code()
            );
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer, Default::default()),
        build_vectors_case_params(Default::default()),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &asset_root(),
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}

/// Contract: each set call → every set runs under the admitted identity's
///   asset scope.
/// Arrange: `build_vectors_case_params` with the KEM's asset-scope selection,
///   so the admitted identities are the roots `build_root` builds from
///   `[0x44; 32]` and `[0x45; 32]` under `SetupScope::Asset`; the expected
///   draws as the call-order block builds them, with `non_convertibility`
///   drawing none; the expected identity element of each set that derives one
///   from `setup` under `SetupScope::Asset` over the root `[0x44; 32]` and that
///   set's master, `u0`, and `u1` uniforms at their places in those draws.
/// Act:     `vectors`, once, in the case consumer's method.
/// Assert:  `non_convertibility`'s `vectors` is empty and every other set's
///   holds `1` record; each set's identity element equals its expected
///   element; and `capsule_components` over `malformed_capsules`'s record's
///   `capsule` returns the `CapsuleComponents::Asset` arm.
/// Collaborators: every family's mock concrete, reached through its factory at
///   the defaults of its params builder except the KEM's asset-scope selection.
#[test]
fn vectors_hands_every_set_the_asset_scoped_identity() {
    let root = asset_root();

    struct Consumer {
        root: hashing::Root,
    }

    impl IVectorsCaseConsumer for Consumer {
        type Output = ();

        fn consume_vectors_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            independent: &dyn IRandomSourceAdapter,
            kdf: &dyn IKeyDerivationAdapter,
            identity: AdmittedKemIdentity<'_>,
            other_identity: AdmittedKemIdentity<'_>,
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
            let scope = SetupScope::Asset { root: &self.root };
            let kem_identity = KemIdentity::Asset { root: &self.root };
            let draw = || draw_uniform::<P>(independent);
            let (_, cross_holder_element) =
                expected_element(kem, scope, kem_identity, draw(), draw(), draw());
            let _ = (draw(), draw(), draw());
            let (_, rerandomization_element) =
                expected_element(kem, scope, kem_identity, draw(), draw(), draw());
            let _ = (draw(), draw());
            let (_, _malformed_element) =
                expected_element(kem, scope, kem_identity, draw(), draw(), draw());
            let _ = draw();
            let _ = draw_bytes(independent, PIECE_GROUP_KEY_LENGTH);
            let (_, first_element) =
                expected_element(kem, scope, kem_identity, draw(), draw(), draw());
            let _ = (draw(), draw());
            let (_, second_element) =
                expected_element(kem, scope, kem_identity, draw(), draw(), draw());
            let _ = (draw(), draw());
            let (_, statement_mutations_element) =
                expected_element(kem, scope, kem_identity, draw(), draw(), draw());
            let _ = (draw(), draw(), draw(), draw(), draw(), draw(), draw());
            let (_, replay_element) =
                expected_element(kem, scope, kem_identity, draw(), draw(), draw());
            let _ = (
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
                draw(),
            );
            let context = build_derivation_context(Default::default());
            let (mint, transfer, next) = default_contexts::<F>();
            let count = |count: u32| {
                build_vector_count(VectorCountConstructorParamsOverrides { count: Some(count) })
            };
            // Act
            let returned = vectors::<P, K, A, D, E>(
                &build_vectors_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    kdf,
                    encoder,
                    Default::default(),
                ),
                build_vectors_params(VectorsParamsOverrides {
                    cross_holder: Some(count(1)),
                    rerandomization: Some(count(1)),
                    non_convertibility: Some(count(1)),
                    malformed_capsules: Some(count(1)),
                    cross_set: Some(count(1)),
                    statement_mutations: Some(count(1)),
                    replay: Some(count(1)),
                }),
                build_vectors_payload(
                    &context,
                    &mint,
                    &transfer,
                    &next,
                    VectorsPayloadOverrides {
                        identity: Some(identity),
                        other_identity: Some(other_identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            // Assert
            assert!(success.non_convertibility.vectors.is_empty());
            assert_eq!(success.cross_holder.vectors.len(), 1);
            assert_eq!(success.rerandomization.vectors.len(), 1);
            assert_eq!(success.malformed_capsules.vectors.len(), 1);
            assert_eq!(success.cross_set.vectors.len(), 1);
            assert_eq!(success.statement_mutations.vectors.len(), 1);
            assert_eq!(success.replay.vectors.len(), 1);
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.cross_holder.vectors[0].identity_element,
                ),
                identity_element_bytes(pairing, kem, &cross_holder_element)
            );
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.rerandomization.vectors[0].identity_element,
                ),
                identity_element_bytes(pairing, kem, &rerandomization_element)
            );
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.cross_set.vectors[0].first.identity_element,
                ),
                identity_element_bytes(pairing, kem, &first_element)
            );
            assert_eq!(
                identity_element_bytes(
                    pairing,
                    kem,
                    &success.cross_set.vectors[0].second.identity_element,
                ),
                identity_element_bytes(pairing, kem, &second_element)
            );
            assert_eq!(
                encode_g1(
                    pairing,
                    &success.statement_mutations.vectors[0]
                        .mint_statement
                        .identity_element,
                ),
                identity_element_bytes(pairing, kem, &statement_mutations_element)
            );
            assert_eq!(
                encode_g1(
                    pairing,
                    &success.replay.vectors[0]
                        .transfer_statement
                        .identity_element,
                ),
                identity_element_bytes(pairing, kem, &replay_element)
            );
            let components = kem
                .capsule_components(
                    build_capsule_components_params(Default::default()),
                    build_capsule_components_payload(
                        &success.malformed_capsules.vectors[0].capsule,
                        Default::default(),
                    ),
                )
                .expect("the capsule's components are infallible")
                .components;
            assert!(matches!(components, CapsuleComponents::Asset { .. }));
        }
    }
    let result = run_vectors_case(
        &build_vectors_case_deps(Consumer { root }, Default::default()),
        build_vectors_case_params(VectorsCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_vectors_case_payload(
            CANONICAL,
            OTHER_CANONICAL,
            &root,
            &other_asset_root(),
            Default::default(),
        ),
    );
    assert!(result.is_ok());
}
