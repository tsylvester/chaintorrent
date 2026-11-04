#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter,
};
use envelope::{
    CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS, GenerateKeysErrorReturn,
    IKeyAgreementAdapter, KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarConcrete,
    HashToScalarErrorReturn, IHashToScalarAdapter, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kdf::{
    CreateKeyDerivationParamsOverrides, DECLARED_KDF_SELECTIONS, DeriveKeyErrorReturn,
    IKeyDerivationAdapter, KdfConcrete, MockIKeyDerivationAdapterDeriveKeyErrorReturn,
    MockIKeyDerivationAdapterFailureMode, build_create_key_derivation_params,
};
use kem::{
    DECLARED_KEM_SELECTIONS, DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope,
    SetupErrorReturn, SetupParamsOverrides, SetupScope, build_encapsulate_params,
    build_encapsulate_payload, build_setup_params, build_setup_payload,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    PAIRING_CONCRETES, PairingConcrete, SampleUniformScalarErrorReturn,
    build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_TRANSFER, DeliveryContextOverrides, DeliveryProofConcrete,
    IDeliveryProofAdapter, MintPurpose, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};
use workflows::{WrapPieceGroupKeyErrorReturn, build_derivation_context};

use super::cross_holder::provides::CrossHolderErrorReturn;
use super::cross_set::provides::CrossSetErrorReturn;
use super::interface::{IVectorsCaseConsumer, VectorsCaseParams, VectorsErrorReturn};
use super::mock::{
    VectorsCaseParamsOverrides, VectorsParamsOverrides, VectorsPayloadOverrides,
    build_vectors_case_deps, build_vectors_case_params, build_vectors_case_payload,
    build_vectors_deps, build_vectors_params, build_vectors_payload, run_vectors_case,
};
use super::statement_mutations::provides::StatementMutationsErrorReturn;
use super::vectors;
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};
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

fn operating_system() -> random::CreateRandomSourceParams {
    build_create_random_source_params(CreateRandomSourceParamsOverrides {
        kind: Some(RandomSourceKind::OperatingSystem),
    })
}

fn selections(
    pairing_concrete: PairingConcrete,
    h2s_selection: &hash_to_scalar::CreateHashToScalarParams,
    encoding_selection: &encoding::CreateEncodingParams,
    forms_concrete: &chain::ChainFormsConcrete,
    kem_selection: &kem::CreateKemParams,
    key_agreement_selection: &envelope::CreateKeyAgreementParams,
    proof_concrete: &proof::DeliveryProofConcrete,
    kdf_selection: &kdf::CreateKeyDerivationParams,
) -> VectorsCaseParams {
    build_vectors_case_params(VectorsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        })),
        hash_to_scalar: Some(*h2s_selection),
        encoding: Some(CreateEncodingParams {
            concrete: encoding_selection.concrete,
            identifier: encoding_selection.identifier,
        }),
        key_derivation: Some(*kdf_selection),
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
        random_source: Some(operating_system()),
    })
}

/// Contract: `vectors` with a distinct count per set → each set's success
///   return's `vectors` holds its count, `non_convertibility`'s holding none
///   under the asset scope.
/// Arrange: the counts `1`, `2`, `3`, `4`, `5`, `6`, and `7` in set order;
///   every member of `PAIRING_CONCRETES` with each selection and scope of
///   `DECLARED_KEM_SELECTIONS`.
/// Act:     `vectors`, once per case, in the case consumer's method.
/// Assert:  each set's `vectors` holds its count, except
///   `non_convertibility`'s, which holds `3` under the entitlement scope and
///   is empty under the asset scope; each case's `Result` `is_ok()`.
/// Boundary: `vectors` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_set_holds_its_configured_count_of_records() {
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
            match identity.setup_scope {
                SetupScope::Entitlement => {
                    assert_eq!(success.non_convertibility.vectors.len(), 3);
                }
                SetupScope::Asset { .. } => {
                    assert!(success.non_convertibility.vectors.is_empty());
                }
            }
            assert_eq!(success.malformed_capsules.vectors.len(), 4);
            assert_eq!(success.cross_set.vectors.len(), 5);
            assert_eq!(success.statement_mutations.vectors.len(), 6);
            assert_eq!(success.replay.vectors.len(), 7);
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer, Default::default()),
                build_vectors_case_params(VectorsCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    random_source: Some(operating_system()),
                    ..Default::default()
                }),
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
    }
}

/// Contract: `vectors` over every case → every set's records hold the verdicts
///   their function computes over the values this node hands it.
/// Arrange: every count `1`; every member of `PAIRING_CONCRETES` crossed with
///   each member of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, each case of
///   `DECLARED_ENCODING_SELECTIONS`, every member of `CHAIN_FORMS_CONCRETES`,
///   each selection of `DECLARED_KEM_SELECTIONS` under the entitlement scope,
///   each member of `DECLARED_KEY_AGREEMENT_SELECTIONS`, every member of
///   `DELIVERY_PROOF_CONCRETES`, and each member of
///   `DECLARED_KDF_SELECTIONS`, so both curves and both verifier forms run.
/// Act:     `vectors`, once per case, in the case consumer's method.
/// Assert:  `cross_holder`'s record decapsulates to the encapsulated
///   `key_material()` bytes with its issued and successor credentials;
///   `rerandomization`'s `issued_is_valid` and `rerandomized_is_valid` are
///   `true`; `non_convertibility`'s `is_valid` is `true` and `other_is_valid`
///   `false`; `malformed_capsules`'s `is_well_formed` is `true` and
///   `displaced_is_well_formed` `false`; each `cross_set` member's
///   `unwrapped.expose()` equals the record's `piece_group_key.expose()`;
///   `statement_mutations`'s `mint_is_valid` and `transfer_is_valid` are
///   `true` and every mutation's `is_valid` `false`; and `replay`'s
///   `transfer_is_valid` and `next_is_valid` are `true` and `replay_is_valid`
///   `false`.
/// Boundary: `vectors` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_set_holds_the_verdicts_its_function_computes() {
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
            let cross_holder = &success.cross_holder.vectors[0];
            assert_eq!(
                cross_holder.issued_decapsulated.key_material().expose(),
                cross_holder.encapsulated.key_material().expose()
            );
            assert_eq!(
                cross_holder.successor_decapsulated.key_material().expose(),
                cross_holder.encapsulated.key_material().expose()
            );
            let rerandomization = &success.rerandomization.vectors[0];
            assert!(rerandomization.issued_is_valid);
            assert!(rerandomization.rerandomized_is_valid);
            let non_convertibility = &success.non_convertibility.vectors[0];
            assert!(non_convertibility.is_valid);
            assert!(!non_convertibility.other_is_valid);
            let malformed_capsules = &success.malformed_capsules.vectors[0];
            assert!(malformed_capsules.is_well_formed);
            assert!(!malformed_capsules.displaced_is_well_formed);
            let cross_set = &success.cross_set.vectors[0];
            assert_eq!(
                cross_set.first.unwrapped.expose(),
                cross_set.piece_group_key.expose()
            );
            assert_eq!(
                cross_set.second.unwrapped.expose(),
                cross_set.piece_group_key.expose()
            );
            let statement_mutations = &success.statement_mutations.vectors[0];
            assert!(statement_mutations.mint_is_valid);
            assert!(statement_mutations.transfer_is_valid);
            for mutation in &statement_mutations.mint_mutations {
                assert!(!mutation.is_valid);
            }
            for mutation in &statement_mutations.transfer_mutations {
                assert!(!mutation.is_valid);
            }
            let replay = &success.replay.vectors[0];
            assert!(replay.transfer_is_valid);
            assert!(replay.next_is_valid);
            assert!(!replay.replay_is_valid);
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        if kem_selection.scope != IdentityScope::Entitlement {
                            continue;
                        }
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                for kdf_selection in DECLARED_KDF_SELECTIONS {
                                    let result = run_vectors_case(
                                        &build_vectors_case_deps(Consumer, Default::default()),
                                        selections(
                                            *pairing_concrete,
                                            h2s_selection,
                                            encoding_selection,
                                            forms_concrete,
                                            kem_selection,
                                            key_agreement_selection,
                                            proof_concrete,
                                            kdf_selection,
                                        ),
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
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Contract: a hash-to-scalar refusal reaches the caller inside the
///   cross-holder variant, whole.
/// Arrange: every count `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES`, with
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::
///   SamplingRefused)`; the expected `error` the literal
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: <P::Scalar as ISampleUniformScalar>::
///   UNIFORM_BYTES_LENGTH, actual: 0 })`.
/// Act:     `vectors`, once per case, in the case consumer's method.
/// Assert:  the result equals `Err(VectorsErrorReturn::CrossHolder(
///   CrossHolderErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::
///   HashToScalar(error))))` under the entitlement scope and
///   `Err(VectorsErrorReturn::CrossHolder(CrossHolderErrorReturn::Setup(
///   SetupErrorReturn::HashToScalar(error))))` under the asset scope, whole;
///   each case's `Result` `is_ok()`.
/// Boundary: `vectors` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the hash-to-scalar family's mock
///   concrete is selected by configuration.
#[test]
fn a_hash_to_scalar_refusal_reaches_the_caller_inside_the_cross_holder_variant() {
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
            let expected =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            let expected = match identity.setup_scope {
                SetupScope::Entitlement => {
                    VectorsErrorReturn::CrossHolder(CrossHolderErrorReturn::DeriveIdentity(
                        DeriveIdentityErrorReturn::HashToScalar(expected),
                    ))
                }
                SetupScope::Asset { .. } => VectorsErrorReturn::CrossHolder(
                    CrossHolderErrorReturn::Setup(SetupErrorReturn::HashToScalar(expected)),
                ),
            };
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
            // Assert
            assert_eq!(returned.err(), Some(expected));
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer, Default::default()),
                build_vectors_case_params(VectorsCaseParamsOverrides {
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
                    random_source: Some(operating_system()),
                    ..Default::default()
                }),
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
    }
}

/// Contract: a key-derivation refusal reaches the caller inside the cross-set
///   variant, whole.
/// Arrange: every count `1`; each selection and scope of
///   `DECLARED_KEM_SELECTIONS` over every member of `PAIRING_CONCRETES` and
///   each case of `DECLARED_ENCODING_SELECTIONS`, with
///   `build_create_key_derivation_params` overriding `concrete` with
///   `KdfConcrete::Mock(MockIKeyDerivationAdapterFailureMode::
///   KeyMaterialLengthUnrepresentable)`, and, as the counterfactual, each
///   member of `DECLARED_KDF_SELECTIONS`; the expected `length` the length of
///   the `key_material()` of an `encapsulate` on the received KEM.
/// Act:     `vectors`, once per key-derivation selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection equals
///   `Err(VectorsErrorReturn::CrossSet(CrossSetErrorReturn::Wrap(
///   WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::Mock(
///   MockIKeyDerivationAdapterDeriveKeyErrorReturn::
///   KeyMaterialLengthUnrepresentable { length })))))` whole, and the result
///   under each declared selection `is_ok()`.
/// Boundary: `vectors` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the key-derivation family's mock
///   concrete is selected by configuration.
#[test]
fn a_key_derivation_refusal_reaches_the_caller_inside_the_cross_set_variant() {
    struct Consumer {
        refuse: bool,
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
            // Assert
            if self.refuse {
                let setup = kem
                    .setup(
                        build_setup_params(SetupParamsOverrides {
                            scope: Some(identity.setup_scope),
                        }),
                        build_setup_payload::<P::Scalar>(Default::default()),
                    )
                    .expect("the replay's setup succeeds");
                let encapsulated = kem
                    .encapsulate(
                        build_encapsulate_params(Default::default()),
                        build_encapsulate_payload::<P::Scalar, _>(
                            &setup.parameter_set,
                            Default::default(),
                        ),
                    )
                    .expect("the replay's encapsulation succeeds");
                let length = encapsulated.encapsulated.key_material().expose().len();
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::CrossSet(CrossSetErrorReturn::Wrap(
                        WrapPieceGroupKeyErrorReturn::KeyDerivation(DeriveKeyErrorReturn::Mock(
                            MockIKeyDerivationAdapterDeriveKeyErrorReturn::KeyMaterialLengthUnrepresentable {
                                length,
                            },
                        ))
                    )))
                );
            } else {
                assert!(returned.is_ok());
            }
        }
    }
    for pairing_concrete in PAIRING_CONCRETES {
        for encoding_selection in DECLARED_ENCODING_SELECTIONS {
            for kem_selection in DECLARED_KEM_SELECTIONS {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(
                            CreatePairingParamsOverrides {
                                concrete: Some(*pairing_concrete),
                                ..Default::default()
                            },
                        )),
                        encoding: Some(CreateEncodingParams {
                            concrete: encoding_selection.concrete,
                            identifier: encoding_selection.identifier,
                        }),
                        key_derivation: Some(build_create_key_derivation_params(
                            CreateKeyDerivationParamsOverrides {
                                concrete: Some(KdfConcrete::Mock(
                                    MockIKeyDerivationAdapterFailureMode::KeyMaterialLengthUnrepresentable,
                                )),
                                ..Default::default()
                            },
                        )),
                        chain_forms: Some(build_create_chain_forms_params(
                            CreateChainFormsParamsOverrides {
                                concrete: Some(ChainFormsConcrete::Evm),
                                ..Default::default()
                            },
                        )),
                        kem: Some(*kem_selection),
                        random_source: Some(operating_system()),
                        ..Default::default()
                    }),
                    build_vectors_case_payload(
                        CANONICAL,
                        OTHER_CANONICAL,
                        &asset_root(),
                        &other_asset_root(),
                        Default::default(),
                    ),
                );
                assert!(result.is_ok());
                for kdf_selection in DECLARED_KDF_SELECTIONS {
                    let result = run_vectors_case(
                        &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                        build_vectors_case_params(VectorsCaseParamsOverrides {
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing_concrete),
                                    ..Default::default()
                                },
                            )),
                            encoding: Some(CreateEncodingParams {
                                concrete: encoding_selection.concrete,
                                identifier: encoding_selection.identifier,
                            }),
                            key_derivation: Some(*kdf_selection),
                            chain_forms: Some(build_create_chain_forms_params(
                                CreateChainFormsParamsOverrides {
                                    concrete: Some(ChainFormsConcrete::Evm),
                                    ..Default::default()
                                },
                            )),
                            kem: Some(*kem_selection),
                            random_source: Some(operating_system()),
                            ..Default::default()
                        }),
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
            }
        }
    }
}

/// Contract: a nonce-draw refusal of key generation reaches the caller inside
///   the statement-mutation variant, whole.
/// Arrange: every count `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, with
///   `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   NonceDrawRefused)`, and, as the counterfactual, each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`.
/// Act:     `vectors`, once per key-agreement selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection equals
///   `Err(VectorsErrorReturn::StatementMutations(StatementMutationsErrorReturn
///   ::Mint(MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(
///   MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused)))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: `vectors` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the key-agreement family's mock
///   concrete is selected by configuration.
#[test]
fn a_key_generation_nonce_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant() {
    struct Consumer {
        refuse: bool,
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
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::StatementMutations(
                        StatementMutationsErrorReturn::Mint(MintErrorReturn::GenerateKeys(
                            GenerateKeysErrorReturn::Mock(
                                MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                            )
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
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: Some(operating_system()),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                build_vectors_case_payload(
                    CANONICAL,
                    OTHER_CANONICAL,
                    &asset_root(),
                    &other_asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        key_agreement: Some(*key_agreement_selection),
                        kem: Some(*kem_selection),
                        random_source: Some(operating_system()),
                        ..Default::default()
                    }),
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
        }
    }
}

/// Contract: a coin-draw refusal of the wrap reaches the caller inside the
///   statement-mutation variant, whole.
/// Arrange: every count `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, with
///   `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::
///   CoinDrawRefused)`, and, as the counterfactual, each member of
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`.
/// Act:     `vectors`, once per key-agreement selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection equals
///   `Err(VectorsErrorReturn::StatementMutations(StatementMutationsErrorReturn
///   ::Mint(MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(
///   MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused)))))` whole,
///   and the result under each declared selection `is_ok()`.
/// Boundary: `vectors` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the key-agreement family's mock
///   concrete is selected by configuration.
#[test]
fn a_wrap_coin_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant() {
    struct Consumer {
        refuse: bool,
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
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::StatementMutations(
                        StatementMutationsErrorReturn::Mint(MintErrorReturn::WrapTo(
                            WrapToErrorReturn::Mock(
                                MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused,
                            )
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
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: Some(operating_system()),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                build_vectors_case_payload(
                    CANONICAL,
                    OTHER_CANONICAL,
                    &asset_root(),
                    &other_asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        key_agreement: Some(*key_agreement_selection),
                        kem: Some(*kem_selection),
                        random_source: Some(operating_system()),
                        ..Default::default()
                    }),
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
        }
    }
}

/// Contract: a nonce-draw refusal of the mint prover reaches the caller inside
///   the statement-mutation variant, whole.
/// Arrange: every count `1`; every member of `PAIRING_CONCRETES` with each
///   selection and scope of `DECLARED_KEM_SELECTIONS`, with
///   `build_create_delivery_proof_params` overriding `concrete` with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::
///   NonceDrawRefused)`, and, as the counterfactual, each member of
///   `DELIVERY_PROOF_CONCRETES`.
/// Act:     `vectors`, once per delivery-proof selection, in the case
///   consumer's method.
/// Assert:  the result under the refusing selection equals
///   `Err(VectorsErrorReturn::StatementMutations(StatementMutationsErrorReturn
///   ::Mint(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)))))`
///   whole, and the result under each declared selection `is_ok()`.
/// Boundary: `vectors` and the chain of real functions the private surface
///   names.
/// Mocked:  nothing at the outer edge; the delivery-proof family's mock
///   concrete is selected by configuration.
#[test]
fn a_mint_nonce_draw_refusal_reaches_the_caller_inside_the_statement_mutations_variant() {
    struct Consumer {
        refuse: bool,
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
            // Assert
            if self.refuse {
                assert_eq!(
                    returned.err(),
                    Some(VectorsErrorReturn::StatementMutations(
                        StatementMutationsErrorReturn::Mint(MintErrorReturn::ProveMint(
                            ProveMintErrorReturn::Mock(
                                MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                            )
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
            let refusing = build_vectors_case_params(VectorsCaseParamsOverrides {
                pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                })),
                delivery_proof: Some(build_create_delivery_proof_params(
                    CreateDeliveryProofParamsOverrides {
                        concrete: Some(DeliveryProofConcrete::Mock(
                            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(*kem_selection),
                random_source: Some(operating_system()),
                ..Default::default()
            });
            let result = run_vectors_case(
                &build_vectors_case_deps(Consumer { refuse: true }, Default::default()),
                refusing,
                build_vectors_case_payload(
                    CANONICAL,
                    OTHER_CANONICAL,
                    &asset_root(),
                    &other_asset_root(),
                    Default::default(),
                ),
            );
            assert!(result.is_ok());
            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                let result = run_vectors_case(
                    &build_vectors_case_deps(Consumer { refuse: false }, Default::default()),
                    build_vectors_case_params(VectorsCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        delivery_proof: Some(build_create_delivery_proof_params(
                            CreateDeliveryProofParamsOverrides {
                                concrete: Some(*proof_concrete),
                                ..Default::default()
                            },
                        )),
                        kem: Some(*kem_selection),
                        random_source: Some(operating_system()),
                        ..Default::default()
                    }),
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
        }
    }
}
