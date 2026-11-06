#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryRelation, IDeliveryProofAdapter,
    MintPurpose, TransferPurpose, build_delivery_context,
};
use random::IRandomSourceAdapter;

use super::interface::{VerifierConsumer, VerifierConsumerReturn};
use super::mock::build_verifier_consumer;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::measure::provides::{
    IMeasureCaseConsumer, MeasureErrorReturn, build_measure_case_deps, build_measure_case_params,
    build_measure_case_payload, run_measure_case,
};
use crate::verifier::provides::{
    CreateVerifierParamsOverrides, CreateVerifierReturn, CreateVerifierSuccessReturn,
    IVerifierAdapter, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VerifierConcrete, VerifyOnChainErrorReturn,
    build_create_verifier_deps, build_create_verifier_params, build_create_verifier_payload,
    create_verifier,
};

/// Contract: Refused → the refusal `measure` returns is the output whole,
///   neither converted nor summarized.
/// Arrange: `build_create_verifier_params` overriding `concrete` with
///   `VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused)`.
/// Act:     `create_verifier`, once, inside the consumer.
/// Assert:  the output matches
///   `Ok(CreateVerifierSuccessReturn { output:
///   Err(MeasureErrorReturn::OnChain { relation: DeliveryRelation::Mint,
///   error: VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused) }) })`, the
///   pattern naming every field because `MeasureErrorReturn` omits `PartialEq`.
#[test]
fn consume_verifier_returns_measures_refusal_whole() {
    // Arrange
    struct CaseConsumer;

    impl IMeasureCaseConsumer for CaseConsumer {
        type Output = CreateVerifierReturn<VerifierConsumerReturn>;

        fn consume_measure_case<P, H, E, F, K, A, D, V>(
            &self,
            pairing: &P,
            _hash_to_scalar: &H,
            _encoder: &E,
            _forms: &F,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            _verifier: &V,
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
            let consumer: VerifierConsumer<'_, P, K, A, D> = build_verifier_consumer(
                pairing,
                kem,
                key_agreement,
                delivery_proof,
                random,
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    DELIVERY_PURPOSE_TRANSFER,
                    Default::default(),
                ),
                identity,
                Default::default(),
            );
            create_verifier(
                &build_create_verifier_deps(pairing, delivery_proof, consumer, Default::default()),
                build_create_verifier_params(CreateVerifierParamsOverrides {
                    concrete: Some(VerifierConcrete::Mock(
                        MockIVerifierAdapterFailureMode::RpcRefused,
                    )),
                    ..Default::default()
                }),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    let deps = build_measure_case_deps(CaseConsumer, Default::default());

    // Act
    let result = run_measure_case(
        &deps,
        build_measure_case_params(Default::default()),
        build_measure_case_payload(canonical, &root, Default::default()),
    );

    // Assert
    let output = result.expect("the case composes").output;
    assert!(matches!(
        output,
        Ok(CreateVerifierSuccessReturn {
            output: Err(MeasureErrorReturn::OnChain {
                relation: DeliveryRelation::Mint,
                error: VerifyOnChainErrorReturn::Mock(
                    MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused
                ),
            }),
        })
    ));
    // consume_verifier_returns_measures_refusal_whole
}
