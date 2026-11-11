#![allow(clippy::expect_used)]

use chain::IChainForms;
use envelope::{
    CreateKeyAgreementParamsOverrides, GenerateKeysErrorReturn, IKeyAgreementAdapter,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_params,
};
use hashing::build_root;
use kem::{
    CreateKemParamsOverrides, DeriveIdentityErrorReturn, ICredentialKemAdapter, IdentityScope,
    SetupErrorReturn, build_create_kem_params,
};
use pairing::{
    IPairingArithmetic, IPairingReference, ISampleUniformScalar, SampleUniformScalarErrorReturn,
};
use proof::{
    CreateDeliveryProofParamsOverrides, DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::interface::{AdmittedKemIdentity, IMintCaseConsumer, MintErrorReturn};
use super::mint;
use super::mock::{
    MintCaseParamsOverrides, MintPayloadOverrides, build_mint_case_deps, build_mint_case_params,
    build_mint_case_payload, build_mint_deps, build_mint_params, build_mint_payload, run_mint_case,
};

/// Contract: Draw, `fill_bytes` refuses → `Err(MintErrorReturn::FillBytes(error))`.
/// Arrange: `create_random_source` with `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`.
/// Act:     `mint`.
/// Assert:  the result equals `Err(MintErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory.
#[test]
fn mint_returns_a_refused_draw_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(MintCaseParamsOverrides {
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::Mock(
                            MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                        )),
                    },
                )),
                ..Default::default()
            }),
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_a_refused_draw_whole
}

/// Contract: Recipient, `generate_keys` refuses →
///   `Err(MintErrorReturn::GenerateKeys(error))`.
/// Arrange: `create_key_agreement` with `KeyAgreementConcrete::Mock(
///   MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`.
/// Act:     `mint`.
/// Assert:  the result equals `Err(MintErrorReturn::GenerateKeys(
///   GenerateKeysErrorReturn::Mock(
///   MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused)))`
///   whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn mint_returns_a_refused_key_generation_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::GenerateKeys(
                    GenerateKeysErrorReturn::Mock(
                        MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
                    )
                ))
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(MintCaseParamsOverrides {
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
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_a_refused_key_generation_whole
}

/// Contract: Wrap, `wrap_to` refuses → `Err(MintErrorReturn::WrapTo(error))`.
/// Arrange: `create_key_agreement` with `KeyAgreementConcrete::Mock(
///   MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`.
/// Act:     `mint`.
/// Assert:  the result equals `Err(MintErrorReturn::WrapTo(
///   WrapToErrorReturn::Mock(
///   MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused)))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn mint_returns_a_refused_wrap_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(
                    MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused
                )))
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(MintCaseParamsOverrides {
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
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_a_refused_wrap_whole
}

/// Contract: Proof, `prove_mint` refuses →
///   `Err(MintErrorReturn::ProveMint(error))`.
/// Arrange: `create_delivery_proof` with `DeliveryProofConcrete::Mock(
///   MockIDeliveryProofAdapterFailureMode::NonceDrawRefused)`.
/// Act:     `mint`.
/// Assert:  the result equals `Err(MintErrorReturn::ProveMint(
///   ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)))`
///   whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn mint_returns_a_refused_proof_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                )))
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(MintCaseParamsOverrides {
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
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_a_refused_proof_whole
}

/// Contract: Derivation, `derive_identity` refuses →
///   `Err(MintErrorReturn::DeriveIdentity(error))`.
/// Arrange: `create_hash_to_scalar` with `HashToScalarConcrete::Mock(
///   MockIHashToScalarAdapterFailureMode::SamplingRefused)` and the
///   entitlement scope.
/// Act:     `mint`.
/// Assert:  the result equals `Err(MintErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::HashToScalar(HashToScalarErrorReturn::
///   Sampling(SampleUniformScalarErrorReturn::WrongLength { expected:
///   UNIFORM_BYTES_LENGTH, actual: 0 }))))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn mint_returns_a_derivation_refusal_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            actual: 0,
                        }
                    ))
                ))
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(MintCaseParamsOverrides {
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
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_a_derivation_refusal_whole
}

/// Contract: Setup, `setup` refuses → `Err(MintErrorReturn::Setup(error))`.
/// Arrange: the same hash-to-scalar selection, an asset root from
///   `build_root(Default::default())` handed to `build_mint_case_payload` as
///   `identity_hash`, and the KEM selection under the asset scope, so the
///   runner produces the identity the asset scope forms.
/// Act:     `mint`.
/// Assert:  the result equals `Err(MintErrorReturn::Setup(SetupErrorReturn::
///   HashToScalar(HashToScalarErrorReturn::Sampling(
///   SampleUniformScalarErrorReturn::WrongLength { expected:
///   UNIFORM_BYTES_LENGTH, actual: 0 }))))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory; the subject's randomness source, the randomness
///   family's mock concrete at the default selection.
#[test]
fn mint_returns_a_setup_refusal_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(
                    HashToScalarErrorReturn::Sampling(
                        SampleUniformScalarErrorReturn::WrongLength {
                            expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                            actual: 0,
                        }
                    )
                )))
            );
        }
    }

    assert!(
        run_mint_case(
            &build_mint_case_deps(Consumer, Default::default()),
            build_mint_case_params(MintCaseParamsOverrides {
                hash_to_scalar: Some(build_create_hash_to_scalar_params(
                    CreateHashToScalarParamsOverrides {
                        concrete: Some(HashToScalarConcrete::Mock(
                            MockIHashToScalarAdapterFailureMode::SamplingRefused,
                        )),
                        ..Default::default()
                    },
                )),
                kem: Some(build_create_kem_params(CreateKemParamsOverrides {
                    scope: Some(IdentityScope::Asset),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            build_mint_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // mint_returns_a_setup_refusal_whole
}
