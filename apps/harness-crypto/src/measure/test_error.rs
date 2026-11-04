#![allow(clippy::expect_used)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    CreateDeliveryProofParamsOverrides, DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER,
    DeliveryProofConcrete, DeliveryRelation, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::interface::{IMeasureCaseConsumer, MeasureErrorReturn, MeasureReturn};
use super::measure;
use super::mock::{
    MeasureCaseParamsOverrides, MeasurePayloadOverrides, build_measure_case_deps,
    build_measure_case_params, build_measure_case_payload, build_measure_deps,
    build_measure_params, build_measure_payload, run_measure_case,
};
use crate::delivery::provides::{AdmittedKemIdentity, MintErrorReturn};
use crate::verifier::provides::{
    CreateVerifierParamsOverrides, IVerifierAdapter, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VerifierConcrete, VerifyOnChainErrorReturn,
    build_create_verifier_params,
};

/// Contract: Delivery pair → a refused draw is
///   `Err(MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(..)))` unchanged.
/// Arrange: the source `create_random_source` returns for
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  the result matches `Err(MeasureErrorReturn::Mint(MintErrorReturn::
///   FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter)))`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the source, the
///   randomness family's mock concrete under `FillBytesRefused`, selected
///   through the random-source params.
#[test]
fn measure_returns_a_refused_draw_inside_the_mint_variant() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            measure::<P, K, A, D, V>(
                &build_measure_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    verifier,
                    random,
                    Default::default(),
                ),
                build_measure_params(Default::default()),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
        }
    }

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(MeasureCaseParamsOverrides {
            random_source: Some(build_create_random_source_params(
                CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                },
            )),
            ..Default::default()
        }),
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    assert!(matches!(
        case.output,
        Err(MeasureErrorReturn::Mint(MintErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        )))
    ));
    // measure_returns_a_refused_draw_inside_the_mint_variant
}

/// Contract: Delivery pair → a refused proof nonce is
///   `Err(MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(..)))` unchanged.
/// Arrange: the delivery proof `MockIDeliveryProofAdapterFailureMode::
///   NonceDrawRefused`, selected through `build_create_delivery_proof_params`.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  the result matches `Err(MeasureErrorReturn::Mint(MintErrorReturn::
///   ProveMint(ProveMintErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused))))`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the delivery proof, the
///   delivery-proof family's mock concrete under `NonceDrawRefused`,
///   selected through the delivery-proof params.
#[test]
fn measure_returns_a_refused_nonce_draw_inside_the_mint_variant() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            measure::<P, K, A, D, V>(
                &build_measure_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    verifier,
                    random,
                    Default::default(),
                ),
                build_measure_params(Default::default()),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
        }
    }

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(MeasureCaseParamsOverrides {
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
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    assert!(matches!(
        case.output,
        Err(MeasureErrorReturn::Mint(MintErrorReturn::ProveMint(
            ProveMintErrorReturn::Mock(
                MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
            )
        )))
    ));
    // measure_returns_a_refused_nonce_draw_inside_the_mint_variant
}

/// Contract: Costed pair → the verifier's own refusal is
///   `Err(MeasureErrorReturn::OnChain { relation, error })` unchanged.
/// Arrange: the verifier `VerifierConcrete::Mock(MockIVerifierAdapterFailure
///   Mode::RpcRefused)`, selected through `build_create_verifier_params`.
/// Act:     `measure`, once, in the case consumer's method.
/// Assert:  the result matches `Err(MeasureErrorReturn::OnChain { relation:
///   DeliveryRelation::Mint, error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) })`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder, except the verifier, the
///   verifier family's mock concrete under `RpcRefused`, selected through
///   the verifier params.
#[test]
fn measure_returns_a_refused_chain_read_with_the_relation_it_submitted() {
    struct Consumer;

    impl IMeasureCaseConsumer for Consumer {
        type Output = MeasureReturn;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            verifier: &V,
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
            V: IVerifierAdapter<DeliveryProof = D>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer_context = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            // Act
            measure::<P, K, A, D, V>(
                &build_measure_deps(
                    pairing,
                    kem,
                    key_agreement,
                    delivery_proof,
                    verifier,
                    random,
                    Default::default(),
                ),
                build_measure_params(Default::default()),
                build_measure_payload(
                    &mint_context,
                    &transfer_context,
                    MeasurePayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            )
        }
    }

    let case = run_measure_case(
        &build_measure_case_deps(Consumer, Default::default()),
        build_measure_case_params(MeasureCaseParamsOverrides {
            verifier: Some(build_create_verifier_params(
                CreateVerifierParamsOverrides {
                    concrete: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                },
            )),
            ..Default::default()
        }),
        build_measure_case_payload(
            b"pkg@1.0.0#1",
            &build_root(RootConstructorParamsOverrides {
                bytes: Some([0x44; 32]),
            }),
            Default::default(),
        ),
    )
    .expect("the case runs");
    // Assert
    assert!(matches!(
        case.output,
        Err(MeasureErrorReturn::OnChain {
            relation: DeliveryRelation::Mint,
            error: VerifyOnChainErrorReturn::Mock(
                MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
            )
        })
    ));
    // measure_returns_a_refused_chain_read_with_the_relation_it_submitted
}
