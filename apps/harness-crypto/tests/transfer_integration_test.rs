#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{CreateEncodingParams, DECLARED_ENCODING_SELECTIONS};
use envelope::{
    DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeComponentsParams, EnvelopeComponentsPayload,
    IKeyAgreementAdapter, PublicKeysComponentsParams, PublicKeysComponentsPayload, UnwrapParams,
    UnwrapPayload,
};
use hash_to_scalar::DECLARED_HASH_TO_SCALAR_SELECTIONS;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CredentialFromComponentsParams, CredentialFromComponentsPayload, DECLARED_KEM_SELECTIONS,
    ICredentialKemAdapter, IsValidParams, IsValidPayload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    IPairingArithmetic, IPairingReference, PAIRING_CONCRETES, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    IDeliveryProofAdapter, MintPurpose, TransferPurpose, VerifyParams, VerifyPayload,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, ITransferCaseConsumer, MintCaseParamsOverrides, MintPayloadOverrides,
    TransferCaseParamsOverrides, TransferErrorReturn, build_mint_case_params, build_mint_deps,
    build_mint_params, build_mint_payload, build_transfer_case_deps, build_transfer_case_params,
    build_transfer_case_payload, build_transfer_deps, build_transfer_params,
    build_transfer_payload, mint, run_transfer_case, transfer,
};

/// Contract: own entry `a_transfer_returns_a_statement_and_proof_binding_context_and_parties`;
///   `transfer` over a case, every adapter real and the operating-system
///   source real, returns a statement `verify` accepts over
///   `AlgebraicStatement::Transfer(&delivery.statement)` and
///   `&delivery.proof`, whose context is the payload's, whose `seller_keys`
///   are the holder's public-key components, whose `old_envelope` is the
///   holder envelope's components, whose `buyer_keys` are the buyer's
///   public-key components, and whose `new_envelope` is the buyer envelope's
///   components.
/// Arrange: every case of the public surface — every member of
///   `PAIRING_CONCRETES` crossed with the selections
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, `DECLARED_ENCODING_SELECTIONS`,
///   `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS` with each scope,
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES`, and
///   each of `TransferPurpose::Transfer` and `TransferPurpose::Grant`; the
///   identity `b"pkg@1.0.0#1"` under the entitlement scope and `vec![0x44;
///   32]` admitted as a `Root` under the asset scope; each holder from `mint`
///   over `build_mint_deps`, `build_mint_params`, and `build_mint_payload`.
/// Act:     `mint`, then `transfer`, then `verify` on the handed transfer
///   delivery proof, once per case, in the case consumer's method.
/// Assert:  the verdict is `is_valid` `true`; the statement's fields encode
///   equal to the named objects' components; each case's `run_transfer_case`
///   result `is_ok()`, proving the consumers ran.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names — `create_random_source` for the
///   operating-system kind, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, `create_delivery_proof`, and every adapter
///   method the block calls, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_transfer_returns_a_statement_and_proof_binding_context_and_parties() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A1,
            transfer_key_agreement: &A2,
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
            let minted = mint(
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
            let mint_delivery = minted.expect("the case's mint succeeds").delivery;
            let context =
                build_delivery_context::<F, TransferPurpose>(self.purpose, Default::default());
            // Act
            let returned = transfer(
                &build_transfer_deps(
                    kem,
                    transfer_key_agreement,
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
            let delivery = returned.expect("the subject's calls succeed").delivery;
            // Assert
            assert_eq!(delivery.statement.context, context);
            let Ok(seller_components) = transfer_key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &mint_delivery.recipient.public_keys,
                },
            );
            let Ok(seller_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: seller_components.components.pk1,
                },
            );
            let Ok(statement_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.seller_keys.pk1,
                },
            );
            assert_eq!(seller_pk1.bytes.as_ref(), statement_pk1.bytes.as_ref());
            let Ok(seller_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: seller_components.components.pk2,
                },
            );
            let Ok(statement_seller_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.seller_keys.pk2,
                },
            );
            assert_eq!(
                seller_pk2.bytes.as_ref(),
                statement_seller_pk2.bytes.as_ref()
            );
            let Ok(buyer_components) = transfer_key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &delivery.buyer.public_keys,
                },
            );
            let Ok(buyer_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: buyer_components.components.pk1,
                },
            );
            let Ok(statement_buyer_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.buyer_keys.pk1,
                },
            );
            assert_eq!(buyer_pk1.bytes.as_ref(), statement_buyer_pk1.bytes.as_ref());
            let Ok(buyer_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: buyer_components.components.pk2,
                },
            );
            let Ok(statement_buyer_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.buyer_keys.pk2,
                },
            );
            assert_eq!(buyer_pk2.bytes.as_ref(), statement_buyer_pk2.bytes.as_ref());
            let Ok(old_envelope) = transfer_key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(old_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: old_envelope.components.c1,
                },
            );
            let Ok(statement_old_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.old_envelope.c1,
                },
            );
            assert_eq!(old_c1.bytes.as_ref(), statement_old_c1.bytes.as_ref());
            let Ok(old_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: old_envelope.components.c2,
                },
            );
            let Ok(statement_old_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.old_envelope.c2,
                },
            );
            assert_eq!(old_c2.bytes.as_ref(), statement_old_c2.bytes.as_ref());
            let Ok(old_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: old_envelope.components.d1,
                },
            );
            let Ok(statement_old_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.old_envelope.d1,
                },
            );
            assert_eq!(old_d1.bytes.as_ref(), statement_old_d1.bytes.as_ref());
            let Ok(old_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: old_envelope.components.d2,
                },
            );
            let Ok(statement_old_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.old_envelope.d2,
                },
            );
            assert_eq!(old_d2.bytes.as_ref(), statement_old_d2.bytes.as_ref());
            let Ok(new_envelope) = transfer_key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(new_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: new_envelope.components.c1,
                },
            );
            let Ok(statement_new_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.new_envelope.c1,
                },
            );
            assert_eq!(new_c1.bytes.as_ref(), statement_new_c1.bytes.as_ref());
            let Ok(new_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: new_envelope.components.c2,
                },
            );
            let Ok(statement_new_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.new_envelope.c2,
                },
            );
            assert_eq!(new_c2.bytes.as_ref(), statement_new_c2.bytes.as_ref());
            let Ok(new_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: new_envelope.components.d1,
                },
            );
            let Ok(statement_new_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.new_envelope.d1,
                },
            );
            assert_eq!(new_d1.bytes.as_ref(), statement_new_d1.bytes.as_ref());
            let Ok(new_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: new_envelope.components.d2,
                },
            );
            let Ok(statement_new_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.new_envelope.d2,
                },
            );
            assert_eq!(new_d2.bytes.as_ref(), statement_new_d2.bytes.as_ref());
            let verdict = transfer_delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&delivery.statement),
                        proof: &delivery.proof,
                    },
                )
                .expect("a verification verdict is returned");
            assert!(verdict.is_valid);
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
        for pairing_concrete in PAIRING_CONCRETES {
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    // Act
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose: *purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(TransferCaseParamsOverrides {
                                            mint: Some(build_mint_case_params(
                                                MintCaseParamsOverrides {
                                                    pairing: Some(build_create_pairing_params(
                                                        CreatePairingParamsOverrides {
                                                            concrete: Some(*pairing_concrete),
                                                            ..Default::default()
                                                        },
                                                    )),
                                                    hash_to_scalar: Some(*h2s_selection),
                                                    encoding: Some(CreateEncodingParams {
                                                        concrete: encoding_selection.concrete,
                                                        identifier: encoding_selection.identifier,
                                                    }),
                                                    chain_forms: Some(
                                                        build_create_chain_forms_params(
                                                            CreateChainFormsParamsOverrides {
                                                                concrete: Some(*forms_concrete),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    kem: Some(*kem_selection),
                                                    key_agreement: Some(*key_agreement_selection),
                                                    delivery_proof: Some(
                                                        build_create_delivery_proof_params(
                                                            CreateDeliveryProofParamsOverrides {
                                                                concrete: Some(*proof_concrete),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    random_source: Some(
                                                        build_create_random_source_params(
                                                            CreateRandomSourceParamsOverrides {
                                                                kind: Some(
                                                                    RandomSourceKind::OperatingSystem,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            )),
                                            key_agreement: Some(*key_agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(
                                                build_create_random_source_params(
                                                    CreateRandomSourceParamsOverrides {
                                                        kind: Some(
                                                            RandomSourceKind::OperatingSystem,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    // Assert
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // a_transfer_returns_a_statement_and_proof_binding_context_and_parties
}

/// Contract: own entry `a_transfer_opens_a_credential_the_buyer_holds`;
///   `transfer` over a case returns a buyer whose envelope `unwrap` opens
///   under the buyer's key pair to components `credential_from_components`
///   rebuilds to a credential `is_valid` accepts under the payload's
///   parameter set and identity element.
/// Arrange: every case of the public surface; each holder from `mint` over
///   `build_mint_deps`, `build_mint_params`, and `build_mint_payload`.
/// Act:     `mint`, then `transfer`, then `unwrap`,
///   `credential_from_components`, and `is_valid`, once per case, in the case
///   consumer's method.
/// Assert:  the verdict is `is_valid` `true`; each case's
///   `run_transfer_case` result `is_ok()`, proving the consumers ran.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_transfer_opens_a_credential_the_buyer_holds() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A1,
            transfer_key_agreement: &A2,
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
            let minted = mint(
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
            let mint_delivery = minted.expect("the case's mint succeeds").delivery;
            let context =
                build_delivery_context::<F, TransferPurpose>(self.purpose, Default::default());
            // Act
            let returned = transfer(
                &build_transfer_deps(
                    kem,
                    transfer_key_agreement,
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
            let delivery = returned.expect("the subject's calls succeed").delivery;
            let Ok(opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &delivery.buyer.key_pair,
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(opened_credential) = kem.credential_from_components(
                CredentialFromComponentsParams,
                CredentialFromComponentsPayload {
                    components: opened.credential,
                },
            );
            let Ok(valid) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &mint_delivery.parameter_set,
                    identity_element: &mint_delivery.identity_element,
                    credential: &opened_credential.credential,
                },
            );
            // Assert
            assert!(valid.is_valid);
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
        for pairing_concrete in PAIRING_CONCRETES {
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    // Act
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose: *purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(TransferCaseParamsOverrides {
                                            mint: Some(build_mint_case_params(
                                                MintCaseParamsOverrides {
                                                    pairing: Some(build_create_pairing_params(
                                                        CreatePairingParamsOverrides {
                                                            concrete: Some(*pairing_concrete),
                                                            ..Default::default()
                                                        },
                                                    )),
                                                    hash_to_scalar: Some(*h2s_selection),
                                                    encoding: Some(CreateEncodingParams {
                                                        concrete: encoding_selection.concrete,
                                                        identifier: encoding_selection.identifier,
                                                    }),
                                                    chain_forms: Some(
                                                        build_create_chain_forms_params(
                                                            CreateChainFormsParamsOverrides {
                                                                concrete: Some(*forms_concrete),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    kem: Some(*kem_selection),
                                                    key_agreement: Some(*key_agreement_selection),
                                                    delivery_proof: Some(
                                                        build_create_delivery_proof_params(
                                                            CreateDeliveryProofParamsOverrides {
                                                                concrete: Some(*proof_concrete),
                                                                ..Default::default()
                                                            },
                                                        ),
                                                    ),
                                                    random_source: Some(
                                                        build_create_random_source_params(
                                                            CreateRandomSourceParamsOverrides {
                                                                kind: Some(
                                                                    RandomSourceKind::OperatingSystem,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            )),
                                            key_agreement: Some(*key_agreement_selection),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*proof_concrete),
                                                        ..Default::default()
                                                    },
                                                ),
                                            ),
                                            random_source: Some(
                                                build_create_random_source_params(
                                                    CreateRandomSourceParamsOverrides {
                                                        kind: Some(
                                                            RandomSourceKind::OperatingSystem,
                                                        ),
                                                    },
                                                ),
                                            ),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    // Assert
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // a_transfer_opens_a_credential_the_buyer_holds
}

/// Contract: own entry `a_refused_draw_reaches_the_caller_whole`; `transfer`
///   over a case whose transfer-side source is the randomness family's mock
///   concrete under `MockIRandomSourceAdapterFailureMode::FillBytesRefused`,
///   selected by the public configuration through `create_random_source`,
///   returns `Err(TransferErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` — the refusal of the first draw reaches the
///   caller through the function's own variant.
/// Arrange: the randomness mock selection under `FillBytesRefused` on the
///   transfer side; every member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS`; each holder from `mint` over the case's own
///   adapters.
/// Act:     `mint`, then `transfer`, once per case, in the case consumer's
///   method.
/// Assert:  the result equals the error arm whole.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names, running for real up to the
///   outer-edge collaborator.
/// Mocked:  the randomness family's mock concrete under `FillBytesRefused`,
///   selected by the public configuration in this block alone.
#[test]
fn a_refused_draw_reaches_the_caller_whole() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A1,
            transfer_key_agreement: &A2,
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
            let minted = mint(
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
            let mint_delivery = minted.expect("the case's mint succeeds").delivery;
            let context =
                build_delivery_context::<F, TransferPurpose>(self.purpose, Default::default());
            // Act
            let returned = transfer(
                &build_transfer_deps(
                    kem,
                    transfer_key_agreement,
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

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_transfer_case(
                &build_transfer_case_deps(
                    Consumer {
                        purpose: TransferPurpose::Transfer,
                    },
                    Default::default(),
                ),
                build_transfer_case_params(TransferCaseParamsOverrides {
                    mint: Some(build_mint_case_params(MintCaseParamsOverrides {
                        pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                            concrete: Some(*pairing_concrete),
                            ..Default::default()
                        })),
                        kem: Some(*kem_selection),
                        ..Default::default()
                    })),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::Mock(
                                MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                            )),
                        },
                    )),
                    ..Default::default()
                }),
                build_transfer_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_refused_draw_reaches_the_caller_whole
}
