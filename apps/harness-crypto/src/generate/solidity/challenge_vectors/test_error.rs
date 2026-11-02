#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{AssetIdentityHash, AssetIdentityHashConstructorParams};
use encoding::{ICanonicalField, IDecoderAdapter, IEncoderAdapter, build_from_field_params};
use hash_to_scalar::{DomainTagTryNewErrorReturn, IHashToScalarAdapter};
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarErrorReturn,
    build_create_pairing_params,
};
use proof::{
    DeliveryContext, DeliveryContextOverrides, DeliveryProofDeclarationOverrides, MintPurpose,
    TransferPurpose, build_delivery_context, build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::challenge_vectors;
use super::interface::{
    ChallengeVectorsCasePayload, ChallengeVectorsErrorReturn, ChallengeVectorsParams,
    ChallengeVectorsReturn, IChallengeVectorsCaseConsumer,
};
use super::mock::{
    ChallengeVectorsCaseParamsOverrides, ChallengeVectorsCasePayloadOverrides,
    ChallengeVectorsDepsOverrides, ChallengeVectorsParamsOverrides,
    ChallengeVectorsPayloadOverrides, build_challenge_vectors_case_deps,
    build_challenge_vectors_case_params, build_challenge_vectors_case_payload,
    build_challenge_vectors_deps, build_challenge_vectors_params, build_challenge_vectors_payload,
    run_challenge_vectors_case,
};

fn typed_context<F: IChainForms, Purpose>(
    context: &super::interface::CanonicalDeliveryContext,
    purpose: Purpose,
) -> DeliveryContext<F, Purpose> {
    build_delivery_context::<F, Purpose>(
        purpose,
        DeliveryContextOverrides::<F> {
            suite_identifier: Some(context.suite_identifier.clone()),
            chain: Some(
                F::ChainIdentifier::from_field(
                    build_from_field_params(Default::default()),
                    context.chain.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            entitlement_contract: Some(
                F::Identity::from_field(
                    build_from_field_params(Default::default()),
                    context.entitlement_contract.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            asset_identity_hash: Some(
                AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                    bytes: *context.asset_root.as_bytes(),
                })
                .expect("a root's bytes admit an asset identity hash"),
            ),
            parameter_set_identifier: Some(context.parameter_set_identifier.clone()),
            source_entitlement: Some(
                F::Entitlement::from_field(
                    build_from_field_params(Default::default()),
                    context.source_entitlement.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            target_entitlement: Some(
                F::Entitlement::from_field(
                    build_from_field_params(Default::default()),
                    context.target_entitlement.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            old_interval: Some(
                F::Interval::from_field(
                    build_from_field_params(Default::default()),
                    context.old_interval.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            new_interval: Some(
                F::Interval::from_field(
                    build_from_field_params(Default::default()),
                    context.new_interval.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            seller: Some(
                F::Identity::from_field(
                    build_from_field_params(Default::default()),
                    context.seller.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            buyer: Some(
                F::Identity::from_field(
                    build_from_field_params(Default::default()),
                    context.buyer.clone(),
                )
                .expect("a canonical value of the declared kind is admitted")
                .value,
            ),
            expiry: Some(context.expiry),
            ..Default::default()
        },
    )
}

/// Contract: `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE`
///   → `Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion { version })`,
///   before any other step.
/// Arrange: params with `statement_version` `2`; the declaration with
///   `challenge_tag` `b""` and the random source selected by
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`,
///   so the tag and a draw are also refused and only an earlier version check
///   returns the version.
/// Act:    the case consumer's single call to `challenge_vectors`.
/// Assert: `assert_eq!` of the result against
///   `Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion { version: 2 })`.
#[test]
fn challenge_vectors_refuse_a_statement_version_no_description_serves() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ChallengeVectorsReturn;

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            )
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
            statement_version: Some(2),
            ..Default::default()
        }),
    };
    let case_params = build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::Mock(
                    MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                )),
            },
        )),
        ..Default::default()
    });
    let case_payload = build_challenge_vectors_case_payload(ChallengeVectorsCasePayloadOverrides {
        delivery_proof: Some(build_delivery_proof_declaration(
            DeliveryProofDeclarationOverrides {
                challenge_tag: Some(b""),
                ..Default::default()
            },
        )),
        ..Default::default()
    });

    // Act
    let output = run_challenge_vectors_case(
        &build_challenge_vectors_case_deps(consumer, Default::default()),
        case_params,
        case_payload,
    )
    .expect("every selection admits its concrete")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion { version: 2 })
    );
}
// challenge_vectors_refuse_a_statement_version_no_description_serves

/// Contract: a declared tag `DomainTag::try_new` refuses →
///   `Err(ChallengeVectorsErrorReturn::DomainTag(error))`, the constructor's
///   error unchanged, before any draw.
/// Arrange: the declaration with `challenge_tag` `b""`; the random source
///   selected by `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, so a draw is also refused and only an earlier tag
///   check returns the tag refusal.
/// Act:    the case consumer's single call to `challenge_vectors`.
/// Assert: `assert_eq!` of the result against
///   `Err(ChallengeVectorsErrorReturn::DomainTag(DomainTagTryNewErrorReturn::Empty))`.
#[test]
fn challenge_vectors_return_the_domain_tag_refusal_whole() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ChallengeVectorsReturn;

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            )
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(Default::default()),
    };
    let case_params = build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::Mock(
                    MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                )),
            },
        )),
        ..Default::default()
    });
    let case_payload = build_challenge_vectors_case_payload(ChallengeVectorsCasePayloadOverrides {
        delivery_proof: Some(build_delivery_proof_declaration(
            DeliveryProofDeclarationOverrides {
                challenge_tag: Some(b""),
                ..Default::default()
            },
        )),
        ..Default::default()
    });

    // Act
    let output = run_challenge_vectors_case(
        &build_challenge_vectors_case_deps(consumer, Default::default()),
        case_params,
        case_payload,
    )
    .expect("every selection admits its concrete")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(ChallengeVectorsErrorReturn::DomainTag(
            DomainTagTryNewErrorReturn::Empty
        ))
    );
}
// challenge_vectors_return_the_domain_tag_refusal_whole

/// Contract: the random source refuses a draw →
///   `Err(ChallengeVectorsErrorReturn::FillBytes(error))`, the family's error
///   unchanged.
/// Arrange: the random source selected by `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`; every other
///   collaborator at its defaults.
/// Act:    the case consumer's single call to `challenge_vectors`.
/// Assert: `assert_eq!` of the result against `Err(ChallengeVectorsErrorReturn::
///   FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter))`.
#[test]
fn challenge_vectors_return_the_draw_refusal_whole() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ChallengeVectorsReturn;

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            )
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(Default::default()),
    };
    let case_params = build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
        random_source: Some(build_create_random_source_params(
            CreateRandomSourceParamsOverrides {
                kind: Some(RandomSourceKind::Mock(
                    MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                )),
            },
        )),
        ..Default::default()
    });
    let case_payload = build_challenge_vectors_case_payload(Default::default());

    // Act
    let output = run_challenge_vectors_case(
        &build_challenge_vectors_case_deps(consumer, Default::default()),
        case_params,
        case_payload,
    )
    .expect("every selection admits its concrete")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(ChallengeVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
}
// challenge_vectors_return_the_draw_refusal_whole

/// Contract: the pairing's sampling bound refuses the drawn uniform →
///   `Err(ChallengeVectorsErrorReturn::SampleScalar(error))`, the pairing's
///   error unchanged.
/// Arrange: the pairing selected by `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`; every other
///   collaborator at its defaults.
/// Act:    the case consumer's single call to `challenge_vectors`.
/// Assert: `assert_eq!` of the result against `Err(ChallengeVectorsErrorReturn::
///   SampleScalar(SampleUniformScalarErrorReturn::WrongLength { expected: 16,
///   actual: 16 }))`, the values the pairing family's mock concrete states.
#[test]
fn challenge_vectors_return_the_sampling_refusal_whole() {
    struct Consumer {
        params: ChallengeVectorsParams,
    }

    impl IChallengeVectorsCaseConsumer for Consumer {
        type Output = ChallengeVectorsReturn;

        fn consume_challenge_vectors_case<P, E, F>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            random: &dyn IRandomSourceAdapter,
            _forms: &F,
            payload: &ChallengeVectorsCasePayload,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter + IDecoderAdapter,
            F: IChainForms,
        {
            let mint = typed_context::<F, MintPurpose>(&payload.mint, payload.mint_purpose);
            let transfer =
                typed_context::<F, TransferPurpose>(&payload.transfer, payload.transfer_purpose);
            challenge_vectors(
                &build_challenge_vectors_deps(
                    pairing,
                    encoder,
                    hash_to_scalar,
                    random,
                    ChallengeVectorsDepsOverrides,
                ),
                self.params,
                build_challenge_vectors_payload(
                    &payload.delivery_proof,
                    &mint,
                    &transfer,
                    ChallengeVectorsPayloadOverrides,
                ),
            )
        }
    }

    // Arrange
    let consumer = Consumer {
        params: build_challenge_vectors_params(Default::default()),
    };
    let case_params = build_challenge_vectors_case_params(ChallengeVectorsCaseParamsOverrides {
        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::SampleScalarWrongLength,
            )),
            ..Default::default()
        })),
        ..Default::default()
    });
    let case_payload = build_challenge_vectors_case_payload(Default::default());

    // Act
    let output = run_challenge_vectors_case(
        &build_challenge_vectors_case_deps(consumer, Default::default()),
        case_params,
        case_payload,
    )
    .expect("every selection admits its concrete")
    .output;

    // Assert
    assert_eq!(
        output,
        Err(ChallengeVectorsErrorReturn::SampleScalar(
            SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 16,
            }
        ))
    );
}
// challenge_vectors_return_the_sampling_refusal_whole
