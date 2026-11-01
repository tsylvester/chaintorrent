#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{Secret, SecretConstructorParams};
use envelope::{
    CreateKeyAgreementParamsOverrides, EnvelopeComponentsParams, EnvelopeComponentsPayload,
    EnvelopeFromComponentsParams, EnvelopeFromComponentsPayload, GenerateKeysErrorReturn,
    GenerateKeysParams, GenerateKeysPayload, IKeyAgreementAdapter, KeyAgreementConcrete,
    KeyPairComponentsParams, KeyPairComponentsPayload, KeyPairFromComponentsParams,
    KeyPairFromComponentsPayload, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    PossessionComponentsParams, PossessionComponentsPayload, PossessionFromComponentsParams,
    PossessionFromComponentsPayload, PublicKeysComponentsParams, PublicKeysComponentsPayload,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, WrapToErrorReturn,
    build_create_key_agreement_params,
};
use hashing::build_root;
use kem::ICredentialKemAdapter;
use pairing::{IPairingArithmetic, IPairingReference, ISampleUniformScalar};
use proof::{
    CreateDeliveryProofParamsOverrides, DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveTransferErrorReturn,
    ProveTransferErrorReturn, TransferPurpose, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use super::interface::{ITransferCaseConsumer, TransferErrorReturn};
use super::mock::{
    TransferCaseParamsOverrides, build_transfer_case_deps, build_transfer_case_params,
    build_transfer_case_payload, build_transfer_deps, build_transfer_params,
    build_transfer_payload, run_transfer_case,
};
use super::transfer;
use crate::delivery::mint::provides::{
    AdmittedKemIdentity, MintPayloadOverrides, build_delivery_holder, build_mint_deps,
    build_mint_params, build_mint_payload, mock_mint,
};

/// Contract: Draw, `fill_bytes` refuses → `Err(TransferErrorReturn::
///   FillBytes(error))`.
/// Arrange: `random_source` selected `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)` through
///   `TransferCaseParamsOverrides`, the mint's source at its defaults.
/// Act:     `transfer`.
/// Assert:  the result equals `Err(TransferErrorReturn::FillBytes(
///   FillBytesErrorReturn::MockIRandomSourceAdapter))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory.
#[test]
fn transfer_returns_a_refused_draw_whole() {
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A1,
            _transfer_key_agreement: &A2,
            delivery_proof: &D1,
            transfer_delivery_proof: &D2,
            mint_source: &dyn IRandomSourceAdapter,
            transfer_source: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A1: IKeyAgreementAdapter<Pairing = P>,
            A2: IKeyAgreementAdapter<Pairing = P>,
            D1: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            D2: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mock_mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    mint_source,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &mint_context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let mint_delivery = minted.expect("the default case mints").delivery;
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
            // Act
            let returned = transfer(
                &build_transfer_deps(
                    kem,
                    key_agreement,
                    transfer_delivery_proof,
                    transfer_source,
                    Default::default(),
                ),
                build_transfer_params(Default::default()),
                build_transfer_payload(
                    &context,
                    &mint_delivery.parameter_set,
                    &mint_delivery.identity_element,
                    &mint_delivery.recipient,
                    Default::default(),
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(TransferErrorReturn::FillBytes(
                    FillBytesErrorReturn::MockIRandomSourceAdapter
                ))
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(TransferCaseParamsOverrides {
                random_source: Some(build_create_random_source_params(
                    CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::Mock(
                            MockIRandomSourceAdapterFailureMode::FillBytesRefused
                        )),
                    }
                )),
                ..Default::default()
            }),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
}

/// Contract: Buyer, `generate_keys` refuses → `Err(TransferErrorReturn::
///   GenerateKeys(error))`.
/// Arrange: `key_agreement` selected `KeyAgreementConcrete::Mock(
///   MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)` through
///   `TransferCaseParamsOverrides`; the holder formed under the case's own
///   key agreement and rebuilt on the failing one.
/// Act:     `transfer`.
/// Assert:  the result equals `Err(TransferErrorReturn::GenerateKeys(
///   GenerateKeysErrorReturn::Mock(
///   MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused)))`
///   whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory.
#[test]
fn transfer_returns_a_refused_key_generation_whole() {
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A1,
            transfer_key_agreement: &A2,
            delivery_proof: &D1,
            _transfer_delivery_proof: &D2,
            mint_source: &dyn IRandomSourceAdapter,
            transfer_source: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A1: IKeyAgreementAdapter<Pairing = P>,
            A2: IKeyAgreementAdapter<Pairing = P>,
            D1: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            D2: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mock_mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    mint_source,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &mint_context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let mint_delivery = minted.expect("the default case mints").delivery;
            let mut slices = mint_delivery
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH);
            let fifth = slices.nth(4).expect("six draws");
            let sixth = slices.next().expect("six draws");
            let Ok(x_uniform) = Secret::try_new(SecretConstructorParams {
                value: fifth.to_vec(),
            });
            let Ok(y_uniform) = Secret::try_new(SecretConstructorParams {
                value: sixth.to_vec(),
            });
            let generated = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform,
                        y_uniform,
                    },
                )
                .expect("replaying the admitted keys' generation succeeds");
            let Ok(key_pair_parts) = key_agreement.key_pair_components(
                KeyPairComponentsParams,
                KeyPairComponentsPayload {
                    key_pair: &mint_delivery.recipient.key_pair,
                },
            );
            let rebuilt_key_pair = transfer_key_agreement
                .key_pair_from_components(
                    KeyPairFromComponentsParams,
                    KeyPairFromComponentsPayload {
                        components: key_pair_parts.components,
                    },
                )
                .expect("components a key agreement reads rebuild")
                .key_pair;
            let Ok(possession_parts) = key_agreement.possession_components(
                PossessionComponentsParams,
                PossessionComponentsPayload {
                    possession: &generated.possession,
                },
            );
            let Ok(rebuilt_possession) = transfer_key_agreement.possession_from_components(
                PossessionFromComponentsParams,
                PossessionFromComponentsPayload {
                    components: possession_parts.components,
                },
            );
            let Ok(public_keys_parts) = key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &mint_delivery.recipient.public_keys,
                },
            );
            let rebuilt_public_keys = transfer_key_agreement
                .public_keys_from_components(
                    PublicKeysFromComponentsParams,
                    PublicKeysFromComponentsPayload {
                        components: public_keys_parts.components,
                        possession: &rebuilt_possession.possession,
                    },
                )
                .expect("components a key agreement reads rebuild")
                .public_keys;
            let Ok(envelope_parts) = key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(rebuilt_envelope) = transfer_key_agreement.envelope_from_components(
                EnvelopeFromComponentsParams,
                EnvelopeFromComponentsPayload {
                    components: envelope_parts.components,
                },
            );
            let holder = build_delivery_holder(
                rebuilt_key_pair,
                rebuilt_public_keys,
                rebuilt_envelope.envelope,
                Default::default(),
            );
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
            // Act
            let returned = transfer(
                &build_transfer_deps(
                    kem,
                    transfer_key_agreement,
                    delivery_proof,
                    transfer_source,
                    Default::default(),
                ),
                build_transfer_params(Default::default()),
                build_transfer_payload(
                    &context,
                    &mint_delivery.parameter_set,
                    &mint_delivery.identity_element,
                    &holder,
                    Default::default(),
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(TransferErrorReturn::GenerateKeys(
                    GenerateKeysErrorReturn::Mock(
                        MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
                    )
                ))
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(TransferCaseParamsOverrides {
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::NonceDrawRefused
                        )),
                        ..Default::default()
                    }
                )),
                ..Default::default()
            }),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
}

/// Contract: Wrap, `wrap_to` refuses → `Err(TransferErrorReturn::WrapTo(
///   error))`.
/// Arrange: as `transfer_returns_a_refused_key_generation_whole`,
///   `key_agreement` selected `KeyAgreementConcrete::Mock(
///   MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)` through
///   `TransferCaseParamsOverrides`.
/// Act:     `transfer`.
/// Assert:  the result equals `Err(TransferErrorReturn::WrapTo(
///   WrapToErrorReturn::Mock(
///   MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused)))` whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory.
#[test]
fn transfer_returns_a_refused_wrap_whole() {
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A1,
            transfer_key_agreement: &A2,
            delivery_proof: &D1,
            _transfer_delivery_proof: &D2,
            mint_source: &dyn IRandomSourceAdapter,
            transfer_source: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A1: IKeyAgreementAdapter<Pairing = P>,
            A2: IKeyAgreementAdapter<Pairing = P>,
            D1: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            D2: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mock_mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    mint_source,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &mint_context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let mint_delivery = minted.expect("the default case mints").delivery;
            let mut slices = mint_delivery
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH);
            let fifth = slices.nth(4).expect("six draws");
            let sixth = slices.next().expect("six draws");
            let Ok(x_uniform) = Secret::try_new(SecretConstructorParams {
                value: fifth.to_vec(),
            });
            let Ok(y_uniform) = Secret::try_new(SecretConstructorParams {
                value: sixth.to_vec(),
            });
            let generated = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform,
                        y_uniform,
                    },
                )
                .expect("replaying the admitted keys' generation succeeds");
            let Ok(key_pair_parts) = key_agreement.key_pair_components(
                KeyPairComponentsParams,
                KeyPairComponentsPayload {
                    key_pair: &mint_delivery.recipient.key_pair,
                },
            );
            let rebuilt_key_pair = transfer_key_agreement
                .key_pair_from_components(
                    KeyPairFromComponentsParams,
                    KeyPairFromComponentsPayload {
                        components: key_pair_parts.components,
                    },
                )
                .expect("components a key agreement reads rebuild")
                .key_pair;
            let Ok(possession_parts) = key_agreement.possession_components(
                PossessionComponentsParams,
                PossessionComponentsPayload {
                    possession: &generated.possession,
                },
            );
            let Ok(rebuilt_possession) = transfer_key_agreement.possession_from_components(
                PossessionFromComponentsParams,
                PossessionFromComponentsPayload {
                    components: possession_parts.components,
                },
            );
            let Ok(public_keys_parts) = key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &mint_delivery.recipient.public_keys,
                },
            );
            let rebuilt_public_keys = transfer_key_agreement
                .public_keys_from_components(
                    PublicKeysFromComponentsParams,
                    PublicKeysFromComponentsPayload {
                        components: public_keys_parts.components,
                        possession: &rebuilt_possession.possession,
                    },
                )
                .expect("components a key agreement reads rebuild")
                .public_keys;
            let Ok(envelope_parts) = key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(rebuilt_envelope) = transfer_key_agreement.envelope_from_components(
                EnvelopeFromComponentsParams,
                EnvelopeFromComponentsPayload {
                    components: envelope_parts.components,
                },
            );
            let holder = build_delivery_holder(
                rebuilt_key_pair,
                rebuilt_public_keys,
                rebuilt_envelope.envelope,
                Default::default(),
            );
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
            // Act
            let returned = transfer(
                &build_transfer_deps(
                    kem,
                    transfer_key_agreement,
                    delivery_proof,
                    transfer_source,
                    Default::default(),
                ),
                build_transfer_params(Default::default()),
                build_transfer_payload(
                    &context,
                    &mint_delivery.parameter_set,
                    &mint_delivery.identity_element,
                    &holder,
                    Default::default(),
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(TransferErrorReturn::WrapTo(WrapToErrorReturn::Mock(
                    MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused
                )))
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(TransferCaseParamsOverrides {
                key_agreement: Some(build_create_key_agreement_params(
                    CreateKeyAgreementParamsOverrides {
                        concrete: Some(KeyAgreementConcrete::Mock(
                            MockIKeyAgreementAdapterFailureMode::CoinDrawRefused
                        )),
                        ..Default::default()
                    }
                )),
                ..Default::default()
            }),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
}

/// Contract: Proof, `prove_transfer` refuses → `Err(TransferErrorReturn::
///   ProveTransfer(error))`.
/// Arrange: `delivery_proof` selected `DeliveryProofConcrete::Mock(
///   MockIDeliveryProofAdapterFailureMode::NonceDrawRefused)` through
///   `TransferCaseParamsOverrides`; the holder `mock_mint` over the case's
///   own adapters forms.
/// Act:     `transfer`.
/// Assert:  the result equals `Err(TransferErrorReturn::ProveTransfer(
///   ProveTransferErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused)))`
///   whole.
/// Collaborators: every other family's mock concrete in its default mode
///   through its factory.
#[test]
fn transfer_returns_a_refused_proof_whole() {
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A1,
            _transfer_key_agreement: &A2,
            delivery_proof: &D1,
            transfer_delivery_proof: &D2,
            mint_source: &dyn IRandomSourceAdapter,
            transfer_source: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A1: IKeyAgreementAdapter<Pairing = P>,
            A2: IKeyAgreementAdapter<Pairing = P>,
            D1: IDeliveryProofAdapter<Pairing = P, Forms = F>,
            D2: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mock_mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    mint_source,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &mint_context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let mint_delivery = minted.expect("the default case mints").delivery;
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
            // Act
            let returned = transfer(
                &build_transfer_deps(
                    kem,
                    key_agreement,
                    transfer_delivery_proof,
                    transfer_source,
                    Default::default(),
                ),
                build_transfer_params(Default::default()),
                build_transfer_payload(
                    &context,
                    &mint_delivery.parameter_set,
                    &mint_delivery.identity_element,
                    &mint_delivery.recipient,
                    Default::default(),
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(TransferErrorReturn::ProveTransfer(
                    ProveTransferErrorReturn::Mock(
                        MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused
                    )
                ))
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(TransferCaseParamsOverrides {
                delivery_proof: Some(build_create_delivery_proof_params(
                    CreateDeliveryProofParamsOverrides {
                        concrete: Some(DeliveryProofConcrete::Mock(
                            MockIDeliveryProofAdapterFailureMode::NonceDrawRefused
                        )),
                        ..Default::default()
                    }
                )),
                ..Default::default()
            }),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
}
