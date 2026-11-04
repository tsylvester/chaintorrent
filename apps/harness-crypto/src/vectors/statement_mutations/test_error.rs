#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{
    ParameterSetIdentifierConstructorParamsOverrides, ParameterSetIdentifierTryNewErrorReturn,
    SuiteIdentifierConstructorParamsOverrides, SuiteIdentifierTryNewErrorReturn,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::{
    CreateKeyAgreementParamsOverrides, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
    build_hash_to_scalar_params, build_hash_to_scalar_payload,
};
use hashing::{RootConstructorParamsOverrides, RootTryNewErrorReturn, build_root};
use kem::{
    CreateKemParamsOverrides, DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope,
    SetupErrorReturn, build_create_kem_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, ISampleUniformScalar,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarParams,
    SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryContextOverrides, DeliveryProofConcrete, DeliveryRelation, IDeliveryProofAdapter,
    MintPurpose, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn, TransferPurpose,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_params, build_fill_bytes_payload,
};

use super::interface::{IStatementMutationsCaseConsumer, StatementMutationsErrorReturn};
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

const CANONICAL: &[u8] = b"pkg@1.0.0#1";

fn asset_root() -> hashing::Root {
    build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    })
}

/// Contract: Mint delivery, `mint` fails at its first draw →
///   `Err(StatementMutationsErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_random_source_params` overriding `kind` with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`
///   for the subject's source and the independent source; the expected `error`
///   `MintErrorReturn::FillBytes` of the refusal of the independent source's
///   `fill_bytes`.
/// Act:     `statement_mutations`.
/// Assert:  the result equals `Err(StatementMutationsErrorReturn::Mint(error))`
///   whole.
#[test]
fn statement_mutations_return_the_draw_refusal_of_the_mint_delivery_whole() {
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
            let error = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .err()
                .expect("the refusing source refuses");
            let expected = StatementMutationsErrorReturn::Mint(MintErrorReturn::FillBytes(error));
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
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at setup's master sampling →
///   `Err(StatementMutationsErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`;
///   the expected `error` `MintErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(…))`
///   of the `Err` of `sample_from_uniform_bytes` over the independent
///   source's first draw.
/// Act:     `statement_mutations`.
/// Assert:  the result equals `Err(StatementMutationsErrorReturn::Mint(error))`
///   whole.
#[test]
fn statement_mutations_return_the_sampling_refusal_of_the_mint_delivery_whole() {
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
            let uniform = independent
                .fill_bytes(
                    FillBytesParams,
                    build_fill_bytes_payload(FillBytesPayloadOverrides {
                        length: Some(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH),
                    }),
                )
                .expect("the operating-system source yields")
                .bytes;
            let error = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .err()
            .expect("the refusing pairing refuses the sampling");
            let expected = StatementMutationsErrorReturn::Mint(MintErrorReturn::Setup(
                SetupErrorReturn::MasterScalarSampling(error),
            ));
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
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::SampleScalarWrongLength,
                )),
                ..Default::default()
            })),
            ..Default::default()
        }),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at the identity derivation →
///   `Err(StatementMutationsErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: the KEM selection under the entitlement scope and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`;
///   the expected `error` `MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(…))`
///   of the `Err` of `hash_to_scalar` over `build_hash_to_scalar_params` and
///   `build_hash_to_scalar_payload` at their defaults on an adapter built
///   under the same mode.
/// Act:     `statement_mutations`.
/// Assert:  the result equals `Err(StatementMutationsErrorReturn::Mint(error))`
///   whole.
#[test]
fn statement_mutations_return_the_hash_to_scalar_refusal_of_the_mint_delivery_derivation_whole() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
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
            let error = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the refusing hash-to-scalar refuses");
            let expected = StatementMutationsErrorReturn::Mint(MintErrorReturn::DeriveIdentity(
                DeriveIdentityErrorReturn::HashToScalar(error),
            ));
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
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
            hash_to_scalar: Some(build_create_hash_to_scalar_params(
                CreateHashToScalarParamsOverrides {
                    concrete: Some(HashToScalarConcrete::Mock(
                        MockIHashToScalarAdapterFailureMode::SamplingRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery under an asset identity, `mint` fails at setup on
///   the root → `Err(StatementMutationsErrorReturn::Mint(error))`, `error`
///   unchanged.
/// Arrange: the KEM selection under the asset scope, the root `build_root`
///   builds from `[0x44; 32]`, and the hash-to-scalar selection of the
///   derivation block; the expected `error`
///   `MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(…))` of the
///   derivation block's refusal.
/// Act:     `statement_mutations`.
/// Assert:  the result equals `Err(StatementMutationsErrorReturn::Mint(error))`
///   whole.
#[test]
fn statement_mutations_return_the_hash_to_scalar_refusal_of_the_mint_delivery_asset_setup_whole() {
    struct Consumer;

    impl IStatementMutationsCaseConsumer for Consumer {
        type Output = ();

        fn consume_statement_mutations_case<P, H, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &H,
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
            let error = hash_to_scalar
                .hash_to_scalar(
                    build_hash_to_scalar_params(Default::default()),
                    build_hash_to_scalar_payload(Default::default()),
                )
                .err()
                .expect("the refusing hash-to-scalar refuses");
            let expected = StatementMutationsErrorReturn::Mint(MintErrorReturn::Setup(
                SetupErrorReturn::HashToScalar(error),
            ));
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
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
            kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                scope: Some(IdentityScope::Asset),
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
            ..Default::default()
        }),
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at the recipient's key generation →
///   `Err(StatementMutationsErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`.
/// Act:     `statement_mutations`.
/// Assert:  the result equals the whole
///   `Err(StatementMutationsErrorReturn::Mint(MintErrorReturn::GenerateKeys(…)))`.
#[test]
fn statement_mutations_return_the_nonce_draw_refusal_of_the_mint_delivery_key_generation_whole() {
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
            let expected = StatementMutationsErrorReturn::Mint(MintErrorReturn::GenerateKeys(
                GenerateKeysErrorReturn::Mock(
                    MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                ),
            ));
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
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
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
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at the wrap to the recipient's keys →
///   `Err(StatementMutationsErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_key_agreement_params` overriding `concrete` with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`.
/// Act:     `statement_mutations`.
/// Assert:  the result equals the whole
///   `Err(StatementMutationsErrorReturn::Mint(MintErrorReturn::WrapTo(…)))`.
#[test]
fn statement_mutations_return_the_coin_draw_refusal_of_the_mint_delivery_wrap_whole() {
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
            let expected = StatementMutationsErrorReturn::Mint(MintErrorReturn::WrapTo(
                WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused),
            ));
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
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
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
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Mint delivery, `mint` fails at its proof →
///   `Err(StatementMutationsErrorReturn::Mint(error))`, `error` unchanged.
/// Arrange: `build_create_delivery_proof_params` overriding `concrete` with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NonceDrawRefused)`.
/// Act:     `statement_mutations`.
/// Assert:  the result equals the whole
///   `Err(StatementMutationsErrorReturn::Mint(MintErrorReturn::ProveMint(…)))`.
#[test]
fn statement_mutations_return_the_nonce_draw_refusal_of_the_mint_delivery_prover_whole() {
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
            let expected = StatementMutationsErrorReturn::Mint(MintErrorReturn::ProveMint(
                ProveMintErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                ),
            ));
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
    let result = run_statement_mutations_case(
        &build_statement_mutations_case_deps(Consumer, Default::default()),
        build_statement_mutations_case_params(StatementMutationsCaseParamsOverrides {
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
        build_statement_mutations_case_payload(CANONICAL, &asset_root(), Default::default()),
    );
    assert!(result.is_ok());
}

/// Contract: Domain value mutation, `SuiteIdentifier::try_new` refuses the
///   mutated identifier → `Err(StatementMutationsErrorReturn::MutatedSuiteIdentifier
///   { relation, error })`, `error` unchanged.
/// Arrange: the typed mint context overriding `suite_identifier` with
///   `build_suite_identifier` over an identifier of thirty-one `0x00` bytes
///   and a last byte `0x01` and version `1`, which the constructor admits and
///   whose mutation is all zero.
/// Act:     `statement_mutations`.
/// Assert:  the result equals
///   `Err(StatementMutationsErrorReturn::MutatedSuiteIdentifier { relation:
///   DeliveryRelation::Mint, error: SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier })`
///   whole.
#[test]
fn statement_mutations_return_the_refusal_of_a_mutated_suite_identifier_whole() {
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
            let mut identifier = [0x00u8; 32];
            identifier[31] = 0x01;
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides {
                    suite_identifier: Some(build_suite_identifier(
                        SuiteIdentifierConstructorParamsOverrides {
                            identifier: Some(identifier),
                            version: Some(1),
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
            assert_eq!(
                returned.err(),
                Some(StatementMutationsErrorReturn::MutatedSuiteIdentifier {
                    relation: DeliveryRelation::Mint,
                    error: SuiteIdentifierTryNewErrorReturn::AllZeroIdentifier,
                })
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

/// Contract: Domain value mutation, `Root::try_new` refuses the mutated root →
///   `Err(StatementMutationsErrorReturn::MutatedAssetRoot { relation, error })`,
///   `error` unchanged.
/// Arrange: the typed mint context overriding `asset_root` with `build_root`
///   over thirty-one `0x00` bytes and a last byte `0x01`.
/// Act:     `statement_mutations`.
/// Assert:  the result equals
///   `Err(StatementMutationsErrorReturn::MutatedAssetRoot { relation:
///   DeliveryRelation::Mint, error: RootTryNewErrorReturn::AllZero })` whole.
#[test]
fn statement_mutations_return_the_refusal_of_a_mutated_asset_root_whole() {
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
            let mut bytes = [0x00u8; 32];
            bytes[31] = 0x01;
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides {
                    asset_root: Some(build_root(RootConstructorParamsOverrides {
                        bytes: Some(bytes),
                    })),
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
            assert_eq!(
                returned.err(),
                Some(StatementMutationsErrorReturn::MutatedAssetRoot {
                    relation: DeliveryRelation::Mint,
                    error: RootTryNewErrorReturn::AllZero,
                })
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

/// Contract: Domain value mutation, `ParameterSetIdentifier::try_new` refuses
///   the mutated identifier → `Err(StatementMutationsErrorReturn::MutatedParameterSetIdentifier
///   { relation, error })`, `error` unchanged.
/// Arrange: the typed mint context overriding `parameter_set_identifier` with
///   `build_parameter_set_identifier` over thirty-one `0x00` bytes and a last
///   byte `0x01`.
/// Act:     `statement_mutations`.
/// Assert:  the result equals
///   `Err(StatementMutationsErrorReturn::MutatedParameterSetIdentifier {
///   relation: DeliveryRelation::Mint, error: ParameterSetIdentifierTryNewErrorReturn::AllZero })`
///   whole.
#[test]
fn statement_mutations_return_the_refusal_of_a_mutated_parameter_set_identifier_whole() {
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
            let mut bytes = [0x00u8; 32];
            bytes[31] = 0x01;
            let mint_context = build_delivery_context::<F, MintPurpose>(
                DELIVERY_PURPOSE_MINT,
                DeliveryContextOverrides {
                    parameter_set_identifier: Some(build_parameter_set_identifier(
                        ParameterSetIdentifierConstructorParamsOverrides { bytes: Some(bytes) },
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
            assert_eq!(
                returned.err(),
                Some(
                    StatementMutationsErrorReturn::MutatedParameterSetIdentifier {
                        relation: DeliveryRelation::Mint,
                        error: ParameterSetIdentifierTryNewErrorReturn::AllZero,
                    }
                )
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
