#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::IChainForms;
use encoding::{IDecoderAdapter, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference};
use proof::{IDeliveryProofAdapter, MintPurpose, TransferPurpose, build_delivery_context};
use random::IRandomSourceAdapter;

use super::interface::{GenerateConsumer, GenerateConsumerReturn};
use super::mock::build_generate_consumer;
use crate::delivery::provides::AdmittedKemIdentity;
use crate::generate::provides::{
    CreateGenerateParamsOverrides, CreateGenerateReturn, EmitErrorReturn, GenerateConcrete,
    ICreateGenerateCaseConsumer, MockIGenerateAdapterEmitErrorReturn,
    MockIGenerateAdapterFailureMode, build_create_generate_case_deps,
    build_create_generate_case_params, build_create_generate_case_payload,
    build_create_generate_deps, build_create_generate_params, build_create_generate_payload,
    create_generate, run_create_generate_case,
};

/// Contract: Refused → the refusal `emit` returns is the output whole, neither
///   converted nor summarized.
/// Arrange: `build_create_generate_params` overriding `concrete` with
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::WriteRefused)`.
/// Act:     `create_generate`, once, inside the consumer.
/// Assert:  the output matches
///   `Err(EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::WriteRefused))`,
///   the pattern naming every field because `EmitErrorReturn` omits `PartialEq`.
#[test]
fn consume_generate_returns_the_concretes_refusal_whole() {
    // Arrange
    struct CaseConsumer;

    impl ICreateGenerateCaseConsumer for CaseConsumer {
        type Output = CreateGenerateReturn<GenerateConsumerReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            _decoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let proof_identity = identities[0];
            let consumer: GenerateConsumer<'_, F> = build_generate_consumer(
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default()),
                build_delivery_context::<F, TransferPurpose>(
                    TransferPurpose::Transfer,
                    Default::default(),
                ),
                identities,
                proof_identity,
                Default::default(),
            );
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    consumer,
                    Default::default(),
                ),
                build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::WriteRefused,
                    )),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [b"example-package@1.0.0", b"example-package@2.0.0"];
    let root_default = build_root(Default::default());
    let root_marked = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots: [&Root; 2] = [&root_default, &root_marked];
    let deps = build_create_generate_case_deps(CaseConsumer, Default::default());

    // Act
    let result = run_create_generate_case(
        &deps,
        build_create_generate_case_params(Default::default()),
        build_create_generate_case_payload(&canonicals, &roots, Default::default()),
    );

    // Assert
    let output = result
        .expect("the case composes")
        .output
        .expect("the factory admits the mock concrete and runs the consumer")
        .output;
    assert!(matches!(
        output,
        Err(EmitErrorReturn::Mock(
            MockIGenerateAdapterEmitErrorReturn::WriteRefused
        ))
    ));
    // consume_generate_returns_the_concretes_refusal_whole
}
