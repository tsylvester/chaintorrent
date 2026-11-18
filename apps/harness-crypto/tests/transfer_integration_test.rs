#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::build_secret;
use encoding::{CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, EncodingIdentifier};
use envelope::{
    DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeCoins, EnvelopeComponentsParams,
    EnvelopeComponentsPayload, EnvelopeFromComponentsParams, EnvelopeFromComponentsPayload,
    GenerateKeysParams, GenerateKeysPayload, IKeyAgreementAdapter, KeyPairComponents,
    KeyPairComponentsParams, KeyPairComponentsPayload, KeyPairFromComponentsParams,
    KeyPairFromComponentsPayload, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    PossessionComponentsParams, PossessionComponentsPayload, PossessionFromComponentsParams,
    PossessionFromComponentsPayload, PublicKeysComponentsParams, PublicKeysComponentsPayload,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, UnwrapParams, UnwrapPayload,
};
use hash_to_scalar::{
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides,
    HashToScalarIdentifier, build_domain_tag, expected_uniform_bytes,
};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IdentityElementComponentsParams, IdentityElementComponentsPayload, IdentityScope,
    IsValidParams, IsValidPayload, KemIdentity, RerandomizeParams, RerandomizePayload,
};
use pairing::{
    CreatePairingParamsOverrides, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, SampleUniformScalarParams, SampleUniformScalarPayload,
    build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    IDeliveryProofAdapter, MintPurpose, ProveTransferParams, ProveTransferPayload, TransferPurpose,
    VerifyParams, VerifyPayload, build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesErrorReturn, MockIRandomSourceAdapterFailureMode,
    RANDOM_SOURCE_KINDS, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    ConsumeTransferCaseParams, ConsumeTransferCasePayload, DeliveryHolder, ITransferCaseConsumer,
    MintCaseParamsOverrides, MintPayloadOverrides, TransferCaseParamsOverrides,
    TransferErrorReturn, build_mint_case_params, build_mint_deps, build_mint_params,
    build_mint_payload, build_transfer_case_deps, build_transfer_case_params,
    build_transfer_case_payload, build_transfer_deps, build_transfer_params,
    build_transfer_payload, mint, run_transfer_case, transfer,
};

/// Contract: own entry
///   `every_transfer_delivery_is_a_statement_and_proof_the_delivery_proof_verifies`;
///   `mint` over a case, then `transfer` over that delivery's parameter set,
///   identity element, and recipient, every adapter real and each member of
///   `RANDOM_SOURCE_KINDS` real, returns a delivery `verify` accepts over
///   `AlgebraicStatement::Transfer(&delivery.statement)` and
///   `&delivery.proof`; the statement is built from the holder's and the
///   buyer's own objects, so a statement over another envelope or other keys
///   reports otherwise — its `seller_keys`, `buyer_keys`, `old_envelope`,
///   and `new_envelope` are the named objects' components.
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
///   functions the public surface names — `create_random_source` for each
///   member of `RANDOM_SOURCE_KINDS`, `create_pairing`,
///   `create_hash_to_scalar`, `create_encoding`, `create_chain_forms`,
///   `create_kem`, `create_key_agreement`, `create_delivery_proof`, and every
///   adapter method the block calls, all running for real.
/// Mocked:  nothing; each declared member's generator runs real.
#[test]
fn every_transfer_delivery_is_a_statement_and_proof_the_delivery_proof_verifies() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _params: ConsumeTransferCaseParams,
            payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let transfer_key_agreement = payload.transfer_key_agreement;
            let delivery_proof = payload.delivery_proof;
            let transfer_delivery_proof = payload.transfer_delivery_proof;
            let mint_source = payload.mint_source;
            let transfer_source = payload.transfer_source;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mint(
                &build_mint_deps(
                    kem,
                    transfer_key_agreement,
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
            assert_eq!(
                delivery.statement.context.suite_identifier,
                context.suite_identifier
            );
            assert_eq!(delivery.statement.context.chain, context.chain);
            assert_eq!(
                delivery.statement.context.entitlement_contract,
                context.entitlement_contract
            );
            assert_eq!(delivery.statement.context.asset_root, context.asset_root);
            assert_eq!(
                delivery.statement.context.parameter_set_identifier,
                context.parameter_set_identifier
            );
            assert_eq!(
                delivery.statement.context.source_entitlement,
                context.source_entitlement
            );
            assert_eq!(
                delivery.statement.context.target_entitlement,
                context.target_entitlement
            );
            assert_eq!(
                delivery.statement.context.old_interval,
                context.old_interval
            );
            assert_eq!(
                delivery.statement.context.new_interval,
                context.new_interval
            );
            assert_eq!(delivery.statement.context.purpose, context.purpose);
            assert_eq!(delivery.statement.context.seller, context.seller);
            assert_eq!(delivery.statement.context.buyer, context.buyer);
            assert_eq!(delivery.statement.context.expiry, context.expiry);
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
                    point: delivery.statement.seller_keys.pk1.clone(),
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
                    point: delivery.statement.seller_keys.pk2.clone(),
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
                    point: delivery.statement.buyer_keys.pk1.clone(),
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
                    point: delivery.statement.buyer_keys.pk2.clone(),
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
                    point: delivery.statement.old_envelope.c1.clone(),
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
                    point: delivery.statement.old_envelope.c2.clone(),
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
                    point: delivery.statement.old_envelope.d1.clone(),
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
                    point: delivery.statement.old_envelope.d2.clone(),
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
                    point: delivery.statement.new_envelope.c1.clone(),
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
                    point: delivery.statement.new_envelope.c2.clone(),
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
                    point: delivery.statement.new_envelope.d1.clone(),
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
                    point: delivery.statement.new_envelope.d2.clone(),
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
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    for random_kind in RANDOM_SOURCE_KINDS {
                                        // Act
                                        let returned = run_transfer_case(
                                            &build_transfer_case_deps(
                                                Consumer { purpose },
                                                Default::default(),
                                            ),
                                            build_transfer_case_params(
                                                TransferCaseParamsOverrides {
                                                    mint: Some(build_mint_case_params(
                                                        MintCaseParamsOverrides {
                                                            pairing: Some(
                                                                build_create_pairing_params(
                                                                    CreatePairingParamsOverrides {
                                                                        concrete: Some(
                                                                            *pairing_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            hash_to_scalar: Some(*h2s_selection),
                                                            encoding: Some(CreateEncodingParams {
                                                                concrete: encoding_selection
                                                                    .concrete,
                                                                identifier: encoding_selection
                                                                    .identifier,
                                                            }),
                                                            chain_forms: Some(
                                                                build_create_chain_forms_params(
                                                                    CreateChainFormsParamsOverrides {
                                                                        concrete: Some(
                                                                            *forms_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            kem: Some(*kem_selection),
                                                            key_agreement: Some(
                                                                *key_agreement_selection,
                                                            ),
                                                            delivery_proof: Some(
                                                                build_create_delivery_proof_params(
                                                                    CreateDeliveryProofParamsOverrides {
                                                                        concrete: Some(
                                                                            *proof_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            random_source: Some(
                                                                build_create_random_source_params(
                                                                    CreateRandomSourceParamsOverrides {
                                                                        kind: Some(*random_kind),
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
                                                                kind: Some(*random_kind),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
    }
    // every_transfer_delivery_is_a_statement_and_proof_the_delivery_proof_verifies
}

/// Contract: own entry
///   `the_buyer_opens_its_envelope_to_a_credential_valid_for_the_identity`;
///   `unwrap` of `delivery.buyer.envelope` under `delivery.buyer.key_pair`,
///   `credential_from_components` over the result, and `is_valid` under the
///   payload's parameter set and identity element returns `is_valid` `true`,
///   and the opened credential's components equal those of
///   `delivery.credential`; the envelope is the one the statement names as
///   its new envelope.
/// Arrange: every case of the public surface; each holder from `mint` over
///   `build_mint_deps`, `build_mint_params`, and `build_mint_payload`.
/// Act:     `mint`, then `transfer`, then `unwrap`,
///   `credential_from_components`, `is_valid`, and `credential_components`,
///   once per case, in the case consumer's method.
/// Assert:  `is_valid` returns `true`; the opened `a` and `b` encode equal
///   to the delivered credential's components; each case's
///   `run_transfer_case` result `is_ok()`, proving the consumers ran.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; each declared member's generator runs real.
#[test]
fn the_buyer_opens_its_envelope_to_a_credential_valid_for_the_identity() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _params: ConsumeTransferCaseParams,
            payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let transfer_key_agreement = payload.transfer_key_agreement;
            let delivery_proof = payload.delivery_proof;
            let transfer_delivery_proof = payload.transfer_delivery_proof;
            let mint_source = payload.mint_source;
            let transfer_source = payload.transfer_source;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mint(
                &build_mint_deps(
                    kem,
                    transfer_key_agreement,
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
            let wrapped = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &delivery.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let rebuilt_components = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &opened_credential.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let Ok(opened_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: rebuilt_components.components.a,
                },
            );
            let Ok(wrapped_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: wrapped.components.a,
                },
            );
            assert_eq!(opened_a.bytes.as_ref(), wrapped_a.bytes.as_ref());
            let Ok(opened_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: rebuilt_components.components.b,
                },
            );
            let Ok(wrapped_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: wrapped.components.b,
                },
            );
            assert_eq!(opened_b.bytes.as_ref(), wrapped_b.bytes.as_ref());
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
        for pairing_concrete in PAIRING_CONCRETES {
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    for random_kind in RANDOM_SOURCE_KINDS {
                                        // Act
                                        let returned = run_transfer_case(
                                            &build_transfer_case_deps(
                                                Consumer { purpose },
                                                Default::default(),
                                            ),
                                            build_transfer_case_params(
                                                TransferCaseParamsOverrides {
                                                    mint: Some(build_mint_case_params(
                                                        MintCaseParamsOverrides {
                                                            pairing: Some(
                                                                build_create_pairing_params(
                                                                    CreatePairingParamsOverrides {
                                                                        concrete: Some(
                                                                            *pairing_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            hash_to_scalar: Some(*h2s_selection),
                                                            encoding: Some(CreateEncodingParams {
                                                                concrete: encoding_selection
                                                                    .concrete,
                                                                identifier: encoding_selection
                                                                    .identifier,
                                                            }),
                                                            chain_forms: Some(
                                                                build_create_chain_forms_params(
                                                                    CreateChainFormsParamsOverrides {
                                                                        concrete: Some(
                                                                            *forms_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            kem: Some(*kem_selection),
                                                            key_agreement: Some(
                                                                *key_agreement_selection,
                                                            ),
                                                            delivery_proof: Some(
                                                                build_create_delivery_proof_params(
                                                                    CreateDeliveryProofParamsOverrides {
                                                                        concrete: Some(
                                                                            *proof_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            random_source: Some(
                                                                build_create_random_source_params(
                                                                    CreateRandomSourceParamsOverrides {
                                                                        kind: Some(*random_kind),
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
                                                                kind: Some(*random_kind),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
    }
    // the_buyer_opens_its_envelope_to_a_credential_valid_for_the_identity
}

/// Contract: own entry `a_refused_draw_reaches_the_caller_whole`; `transfer`
///   over a case whose source is the randomness family's mock concrete under
///   `MockIRandomSourceAdapterFailureMode::FillBytesRefused`, selected by
///   configuration through `create_random_source`, the holder from a `mint`
///   over each member of `RANDOM_SOURCE_KINDS`, returns
///   `Err(TransferErrorReturn::FillBytes(FillBytesErrorReturn::
///   MockIRandomSourceAdapter))` — the refusal of the first draw reaches the
///   caller through the function's own variant after the opening and before
///   any other adapter call.
/// Arrange: the randomness mock selection under `FillBytesRefused` on the
///   transfer side; every member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS` and each member of `RANDOM_SOURCE_KINDS` on
///   the mint side; each holder from `mint` over the case's own adapters,
///   rebuilt on the transfer-side key agreement from components so the
///   holder is of the key agreement `transfer` composes.
/// Act:     `mint`, the holder's rebuild, then `transfer`, once per case, in
///   the case consumer's method.
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
            _params: ConsumeTransferCaseParams,
            payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
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
            let kem = payload.kem;
            let key_agreement = payload.key_agreement;
            let transfer_key_agreement = payload.transfer_key_agreement;
            let delivery_proof = payload.delivery_proof;
            let transfer_delivery_proof = payload.transfer_delivery_proof;
            let mint_source = payload.mint_source;
            let transfer_source = payload.transfer_source;
            let identity = payload.identity;
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
            let mut slices = mint_delivery
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH);
            let fifth = slices.nth(4).expect("six draws");
            let sixth = slices.next().expect("six draws");
            let generated = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: build_secret(fifth.to_vec(), Default::default()),
                        y_uniform: build_secret(sixth.to_vec(), Default::default()),
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
                .expect("the rebuilt possession is admitted")
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
            let holder = DeliveryHolder {
                key_pair: rebuilt_key_pair,
                public_keys: rebuilt_public_keys,
                envelope: rebuilt_envelope.envelope,
            };
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
                    &holder,
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
            for random_kind in RANDOM_SOURCE_KINDS {
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
                            pairing: Some(build_create_pairing_params(
                                CreatePairingParamsOverrides {
                                    concrete: Some(*pairing_concrete),
                                    ..Default::default()
                                },
                            )),
                            kem: Some(*kem_selection),
                            random_source: Some(build_create_random_source_params(
                                CreateRandomSourceParamsOverrides {
                                    kind: Some(*random_kind),
                                },
                            )),
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
    }
    // a_refused_draw_reaches_the_caller_whole
}

/// Contract: own entry `the_witnesses_prove_the_statement_again`; `transfer`
///   over a case, then `prove_transfer` called on the case's delivery proof
///   over the delivery's witnesses and statement, returns a second proof
///   `verify` accepts against the statement.
/// Arrange: every case of the public surface; the witnesses are copies
///   independent of the values the delivery's own proof consumed.
/// Act:     `mint`, then `transfer`, then `prove_transfer` over
///   `delivery.witnesses` and `&delivery.statement`, then `verify` over the
///   second proof, once per case, in the case consumer's method.
/// Assert:  the second proof's verdict is `is_valid` `true`; each case's
///   `run_transfer_case` result `is_ok()`, proving the consumers ran.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; each declared member's generator runs real.
#[test]
fn the_witnesses_prove_the_statement_again() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _params: ConsumeTransferCaseParams,
            payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
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
            let kem = payload.kem;
            let transfer_key_agreement = payload.transfer_key_agreement;
            let delivery_proof = payload.delivery_proof;
            let transfer_delivery_proof = payload.transfer_delivery_proof;
            let mint_source = payload.mint_source;
            let transfer_source = payload.transfer_source;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mint(
                &build_mint_deps(
                    kem,
                    transfer_key_agreement,
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
            let proved = transfer_delivery_proof
                .prove_transfer(
                    ProveTransferParams,
                    ProveTransferPayload {
                        seller_secrets: KeyPairComponents {
                            x: build_secret(
                                delivery.witnesses.seller_secrets.x.expose().clone(),
                                Default::default(),
                            ),
                            y: build_secret(
                                delivery.witnesses.seller_secrets.y.expose().clone(),
                                Default::default(),
                            ),
                        },
                        offset: build_secret(
                            delivery.witnesses.offset.expose().clone(),
                            Default::default(),
                        ),
                        coins: EnvelopeCoins {
                            rho: build_secret(
                                delivery.witnesses.coins.rho.expose().clone(),
                                Default::default(),
                            ),
                            sigma: build_secret(
                                delivery.witnesses.coins.sigma.expose().clone(),
                                Default::default(),
                            ),
                        },
                        statement: &delivery.statement,
                    },
                )
                .expect("the declared selection proves the transfer statement");
            // Assert
            let verdict = transfer_delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&delivery.statement),
                        proof: &proved.proof,
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
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    for random_kind in RANDOM_SOURCE_KINDS {
                                        // Act
                                        let returned = run_transfer_case(
                                            &build_transfer_case_deps(
                                                Consumer { purpose },
                                                Default::default(),
                                            ),
                                            build_transfer_case_params(
                                                TransferCaseParamsOverrides {
                                                    mint: Some(build_mint_case_params(
                                                        MintCaseParamsOverrides {
                                                            pairing: Some(
                                                                build_create_pairing_params(
                                                                    CreatePairingParamsOverrides {
                                                                        concrete: Some(
                                                                            *pairing_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            hash_to_scalar: Some(*h2s_selection),
                                                            encoding: Some(CreateEncodingParams {
                                                                concrete: encoding_selection
                                                                    .concrete,
                                                                identifier: encoding_selection
                                                                    .identifier,
                                                            }),
                                                            chain_forms: Some(
                                                                build_create_chain_forms_params(
                                                                    CreateChainFormsParamsOverrides {
                                                                        concrete: Some(
                                                                            *forms_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            kem: Some(*kem_selection),
                                                            key_agreement: Some(
                                                                *key_agreement_selection,
                                                            ),
                                                            delivery_proof: Some(
                                                                build_create_delivery_proof_params(
                                                                    CreateDeliveryProofParamsOverrides {
                                                                        concrete: Some(
                                                                            *proof_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            random_source: Some(
                                                                build_create_random_source_params(
                                                                    CreateRandomSourceParamsOverrides {
                                                                        kind: Some(*random_kind),
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
                                                                kind: Some(*random_kind),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
    }
    // the_witnesses_prove_the_statement_again
}

/// Contract: own entry
///   `a_transfer_from_a_transfers_buyer_is_a_transfer_the_delivery_proof_verifies`;
///   `transfer` over a case, then `transfer` again over the same parameter
///   set and identity element with `&delivery.buyer` as the holder, returns
///   a second delivery `verify` accepts, whose statement's `seller_keys`
///   equal the first statement's `buyer_keys` and whose `old_envelope`
///   equals the first statement's `new_envelope`.
/// Arrange: every case of the public surface; the returned buyer is a
///   complete holder, so a holder missing its admitted keys or its envelope
///   cannot transfer.
/// Act:     `mint`, then `transfer`, then `transfer` again over
///   `&delivery.buyer`, then `verify`, once per case, in the case
///   consumer's method.
/// Assert:  the second delivery's verdict is `is_valid` `true`; the second
///   statement's `seller_keys` and `old_envelope` encode equal to the first
///   statement's `buyer_keys` and `new_envelope`; each case's
///   `run_transfer_case` result `is_ok()`.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; each declared member's generator runs real.
#[test]
fn a_transfer_from_a_transfers_buyer_is_a_transfer_the_delivery_proof_verifies() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _params: ConsumeTransferCaseParams,
            payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let transfer_key_agreement = payload.transfer_key_agreement;
            let delivery_proof = payload.delivery_proof;
            let transfer_delivery_proof = payload.transfer_delivery_proof;
            let mint_source = payload.mint_source;
            let transfer_source = payload.transfer_source;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mint(
                &build_mint_deps(
                    kem,
                    transfer_key_agreement,
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
            let second = transfer(
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
                    &delivery.buyer,
                    Default::default(),
                ),
            );
            let second_delivery = second.expect("the subject's calls succeed").delivery;
            let verdict = transfer_delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&second_delivery.statement),
                        proof: &second_delivery.proof,
                    },
                )
                .expect("a verification verdict is returned");
            // Assert
            assert!(verdict.is_valid);
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("a first-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: &P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("a second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(
                encode_g1(&second_delivery.statement.seller_keys.pk1),
                encode_g1(&delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&second_delivery.statement.seller_keys.pk2),
                encode_g2(&delivery.statement.buyer_keys.pk2)
            );
            assert_eq!(
                encode_g1(&second_delivery.statement.old_envelope.c1),
                encode_g1(&delivery.statement.new_envelope.c1)
            );
            assert_eq!(
                encode_g1(&second_delivery.statement.old_envelope.c2),
                encode_g1(&delivery.statement.new_envelope.c2)
            );
            assert_eq!(
                encode_g2(&second_delivery.statement.old_envelope.d1),
                encode_g2(&delivery.statement.new_envelope.d1)
            );
            assert_eq!(
                encode_g2(&second_delivery.statement.old_envelope.d2),
                encode_g2(&delivery.statement.new_envelope.d2)
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
        for pairing_concrete in PAIRING_CONCRETES {
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    for random_kind in RANDOM_SOURCE_KINDS {
                                        // Act
                                        let returned = run_transfer_case(
                                            &build_transfer_case_deps(
                                                Consumer { purpose },
                                                Default::default(),
                                            ),
                                            build_transfer_case_params(
                                                TransferCaseParamsOverrides {
                                                    mint: Some(build_mint_case_params(
                                                        MintCaseParamsOverrides {
                                                            pairing: Some(
                                                                build_create_pairing_params(
                                                                    CreatePairingParamsOverrides {
                                                                        concrete: Some(
                                                                            *pairing_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            hash_to_scalar: Some(*h2s_selection),
                                                            encoding: Some(CreateEncodingParams {
                                                                concrete: encoding_selection
                                                                    .concrete,
                                                                identifier: encoding_selection
                                                                    .identifier,
                                                            }),
                                                            chain_forms: Some(
                                                                build_create_chain_forms_params(
                                                                    CreateChainFormsParamsOverrides {
                                                                        concrete: Some(
                                                                            *forms_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            kem: Some(*kem_selection),
                                                            key_agreement: Some(
                                                                *key_agreement_selection,
                                                            ),
                                                            delivery_proof: Some(
                                                                build_create_delivery_proof_params(
                                                                    CreateDeliveryProofParamsOverrides {
                                                                        concrete: Some(
                                                                            *proof_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            random_source: Some(
                                                                build_create_random_source_params(
                                                                    CreateRandomSourceParamsOverrides {
                                                                        kind: Some(*random_kind),
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
                                                                kind: Some(*random_kind),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
    }
    // a_transfer_from_a_transfers_buyer_is_a_transfer_the_delivery_proof_verifies
}

/// Contract: own entry `every_draw_replays_to_the_objects_it_produced`;
///   `transfer` over a case, the draws split at `UNIFORM_BYTES_LENGTH` into
///   the uniforms of its calls in draw order, `generate_keys` run on the
///   case's key agreement over the first two, and `rerandomize` run on the
///   case's KEM over the third with the credential `unwrap` and
///   `credential_from_components` open from the holder, reproduce the
///   objects the delivery holds.
/// Arrange: every case of the public surface; draws in another order would
///   replay to other objects.
/// Act:     `mint`, then `transfer`, then `generate_keys`, `unwrap`,
///   `credential_from_components`, `rerandomize`, and the components
///   methods, once per case, in the case consumer's method.
/// Assert:  `delivery.draws` is `3 * UNIFORM_BYTES_LENGTH` long; the
///   replayed key pair's `pk1` and `pk2` encode equal to the delivery's
///   buyer keys and the replayed credential's `a` and `b` encode equal to
///   the delivered credential's components; each case's `run_transfer_case`
///   result `is_ok()`.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; each declared member's generator runs real.
#[test]
fn every_draw_replays_to_the_objects_it_produced() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _params: ConsumeTransferCaseParams,
            payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let transfer_key_agreement = payload.transfer_key_agreement;
            let delivery_proof = payload.delivery_proof;
            let transfer_delivery_proof = payload.transfer_delivery_proof;
            let mint_source = payload.mint_source;
            let transfer_source = payload.transfer_source;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let minted = mint(
                &build_mint_deps(
                    kem,
                    transfer_key_agreement,
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
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(delivery.draws.len(), 3 * length);
            let uniform = |index: usize| {
                build_secret(
                    delivery.draws[index * length..(index + 1) * length].to_vec(),
                    Default::default(),
                )
            };
            let keys = transfer_key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: uniform(0),
                        y_uniform: uniform(1),
                    },
                )
                .expect("the replay's key generation succeeds");
            let Ok(opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &mint_delivery.recipient.key_pair,
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(opened_credential) = kem.credential_from_components(
                CredentialFromComponentsParams,
                CredentialFromComponentsPayload {
                    components: opened.credential,
                },
            );
            let rerandomized = kem
                .rerandomize(
                    RerandomizeParams,
                    RerandomizePayload {
                        parameter_set: &mint_delivery.parameter_set,
                        identity_element: &mint_delivery.identity_element,
                        credential: &opened_credential.credential,
                        uniform: uniform(2),
                    },
                )
                .expect("the replay's rerandomization succeeds");
            let public = transfer_key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &keys.key_pair,
                    },
                )
                .expect("a key pair's public keys are infallible");
            let replayed_components = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &rerandomized.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let delivered_components = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &delivery.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("a first-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_g2 = |point: &P::G2| -> Vec<u8> {
                pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("a second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            // Assert
            assert_eq!(
                encode_g1(&public.components.pk1),
                encode_g1(&delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&public.components.pk2),
                encode_g2(&delivery.statement.buyer_keys.pk2)
            );
            assert_eq!(
                encode_g1(&replayed_components.components.a),
                encode_g1(&delivered_components.components.a)
            );
            assert_eq!(
                encode_g2(&replayed_components.components.b),
                encode_g2(&delivered_components.components.b)
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
        for pairing_concrete in PAIRING_CONCRETES {
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    for random_kind in RANDOM_SOURCE_KINDS {
                                        // Act
                                        let returned = run_transfer_case(
                                            &build_transfer_case_deps(
                                                Consumer { purpose },
                                                Default::default(),
                                            ),
                                            build_transfer_case_params(
                                                TransferCaseParamsOverrides {
                                                    mint: Some(build_mint_case_params(
                                                        MintCaseParamsOverrides {
                                                            pairing: Some(
                                                                build_create_pairing_params(
                                                                    CreatePairingParamsOverrides {
                                                                        concrete: Some(
                                                                            *pairing_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            hash_to_scalar: Some(*h2s_selection),
                                                            encoding: Some(CreateEncodingParams {
                                                                concrete: encoding_selection
                                                                    .concrete,
                                                                identifier: encoding_selection
                                                                    .identifier,
                                                            }),
                                                            chain_forms: Some(
                                                                build_create_chain_forms_params(
                                                                    CreateChainFormsParamsOverrides {
                                                                        concrete: Some(
                                                                            *forms_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            kem: Some(*kem_selection),
                                                            key_agreement: Some(
                                                                *key_agreement_selection,
                                                            ),
                                                            delivery_proof: Some(
                                                                build_create_delivery_proof_params(
                                                                    CreateDeliveryProofParamsOverrides {
                                                                        concrete: Some(
                                                                            *proof_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            random_source: Some(
                                                                build_create_random_source_params(
                                                                    CreateRandomSourceParamsOverrides {
                                                                        kind: Some(*random_kind),
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
                                                                kind: Some(*random_kind),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
    }
    // every_draw_replays_to_the_objects_it_produced
}

/// Contract: `delivery/mint` carried entry
///   `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`;
///   `transfer` hands the identity element to `rerandomize` and into the
///   statement unchanged, so the transfer statement's `identity_element` is
///   the `element` of the identity element `derive_identity` returned inside
///   `mint`, whose `scalar` under the entitlement scope encodes to the
///   encoding of the scalar the pairing's scalar type samples from
///   `expected_uniform_bytes` over the tag built from
///   `K::DECLARATION.identity_tag` and the identity's canonical bytes.
/// Arrange: every member of `PAIRING_CONCRETES` crossed with each selection
///   of `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, each selection of
///   `DECLARED_ENCODING_SELECTIONS` declaring `EncodingIdentifier::
///   EthereumAbiV1`, each selection of `CHAIN_FORMS_CONCRETES`,
///   `DECLARED_KEM_SELECTIONS` under the entitlement scope,
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES`,
///   each member of `RANDOM_SOURCE_KINDS`, and each of
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant`.
/// Act:     `mint`, then `transfer`, then `identity_element_components` on
///   the handed KEM, once per case, in the case consumer's method.
/// Assert:  `encode_g1` of the statement's `identity_element` equals
///   `encode_g1` of the identity element's `element`; `encode_scalar` of the
///   identity element's `scalar` equals `encode_scalar` of the scalar
///   `sample_from_uniform_bytes` returns for
///   `expected_uniform_bytes(&tag, canonical, UNIFORM_BYTES_LENGTH)`; each
///   case's `run_transfer_case` result `is_ok()`.
/// Boundary: `transfer` invoked through the crate root and the chain of real
///   functions the public surface names, all running for real.
/// Mocked:  nothing; each declared member's generator runs real.
#[test]
fn every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle()
 {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _params: ConsumeTransferCaseParams,
            payload: ConsumeTransferCasePayload<'_, P, F, K, A1, A2, D1, D2>,
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
            let pairing = payload.pairing;
            let kem = payload.kem;
            let transfer_key_agreement = payload.transfer_key_agreement;
            let delivery_proof = payload.delivery_proof;
            let transfer_delivery_proof = payload.transfer_delivery_proof;
            let mint_source = payload.mint_source;
            let transfer_source = payload.transfer_source;
            let identity = payload.identity;
            // Arrange
            let mint_context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            assert!(matches!(identity.identity, KemIdentity::Entitlement { .. }));
            let KemIdentity::Entitlement { canonical } = identity.identity else {
                return;
            };
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            // Act
            let minted = mint(
                &build_mint_deps(
                    kem,
                    transfer_key_agreement,
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
            let identity_element = kem
                .identity_element_components(
                    IdentityElementComponentsParams,
                    IdentityElementComponentsPayload {
                        identity_element: &mint_delivery.identity_element,
                    },
                )
                .expect("an identity element's components are infallible");
            // Assert
            let uniform = build_secret(
                expected_uniform_bytes(
                    &tag,
                    canonical,
                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                ),
                Default::default(),
            );
            let expected_scalar = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("the expected uniform bytes are of the declared length")
            .scalar;
            let Ok(statement_element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.identity_element.clone(),
                },
            );
            let Ok(element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: identity_element.components.element,
                },
            );
            assert_eq!(statement_element.bytes.as_ref(), element.bytes.as_ref());
            let Ok(expected_encoding) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: expected_scalar.expose().clone(),
                },
            );
            let Ok(actual_encoding) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: identity_element.components.scalar.clone(),
                },
            );
            assert_eq!(
                actual_encoding.bytes.expose().as_ref(),
                expected_encoding.bytes.expose().as_ref()
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for purpose in [TransferPurpose::Transfer, TransferPurpose::Grant] {
        for pairing_concrete in PAIRING_CONCRETES {
            for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                for encoding_selection in DECLARED_ENCODING_SELECTIONS
                    .iter()
                    .filter(|selection| selection.identifier == EncodingIdentifier::EthereumAbiV1)
                {
                    for forms_concrete in CHAIN_FORMS_CONCRETES {
                        for kem_selection in DECLARED_KEM_SELECTIONS
                            .iter()
                            .filter(|selection| selection.scope == IdentityScope::Entitlement)
                        {
                            for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                                for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                    for random_kind in RANDOM_SOURCE_KINDS {
                                        // Act
                                        let returned = run_transfer_case(
                                            &build_transfer_case_deps(
                                                Consumer { purpose },
                                                Default::default(),
                                            ),
                                            build_transfer_case_params(
                                                TransferCaseParamsOverrides {
                                                    mint: Some(build_mint_case_params(
                                                        MintCaseParamsOverrides {
                                                            pairing: Some(
                                                                build_create_pairing_params(
                                                                    CreatePairingParamsOverrides {
                                                                        concrete: Some(
                                                                            *pairing_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            hash_to_scalar: Some(*h2s_selection),
                                                            encoding: Some(CreateEncodingParams {
                                                                concrete: encoding_selection
                                                                    .concrete,
                                                                identifier: encoding_selection
                                                                    .identifier,
                                                            }),
                                                            chain_forms: Some(
                                                                build_create_chain_forms_params(
                                                                    CreateChainFormsParamsOverrides {
                                                                        concrete: Some(
                                                                            *forms_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            kem: Some(*kem_selection),
                                                            key_agreement: Some(
                                                                *key_agreement_selection,
                                                            ),
                                                            delivery_proof: Some(
                                                                build_create_delivery_proof_params(
                                                                    CreateDeliveryProofParamsOverrides {
                                                                        concrete: Some(
                                                                            *proof_concrete,
                                                                        ),
                                                                        ..Default::default()
                                                                    },
                                                                ),
                                                            ),
                                                            random_source: Some(
                                                                build_create_random_source_params(
                                                                    CreateRandomSourceParamsOverrides {
                                                                        kind: Some(*random_kind),
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
                                                                kind: Some(*random_kind),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
    }
    // every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle
}
