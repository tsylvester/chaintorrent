#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::build_secret;
use envelope::{
    EnvelopeComponentsParams, EnvelopeComponentsPayload, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, PublicKeysComponentsParams,
    PublicKeysComponentsPayload, UnwrapParams, UnwrapPayload,
};
use hashing::build_root;
use kem::{
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsParams,
    CredentialFromComponentsPayload, ICredentialKemAdapter, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, RerandomizePayloadOverrides, build_rerandomize_params,
    build_rerandomize_payload,
};
use pairing::{
    AddG2Params, AddG2Payload, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
};
use proof::{IDeliveryProofAdapter, MintPurpose, TransferPurpose, build_delivery_context};
use super::interface::{
    ConsumeTransferCaseParams, ConsumeTransferCasePayload, ITransferCaseConsumer,
};
use super::mock::{
    build_transfer_case_deps, build_transfer_case_params, build_transfer_case_payload,
    build_transfer_deps, build_transfer_params, build_transfer_payload, run_transfer_case,
};
use super::transfer;
use crate::delivery::mint::provides::{
    MintPayloadOverrides, build_mint_deps, build_mint_params, build_mint_payload, mock_mint,
};

/// Contract: Computed → `Ok(TransferSuccessReturn { delivery })`, the
///   statement's context the payload's context, its `seller_keys` the
///   holder's public-key components, its `old_envelope` the holder
///   envelope's components, its `buyer_keys` the buyer's public-key
///   components, its `new_envelope` the buyer envelope's components, and
///   its `identity_element` the identity element's.
/// Arrange: a transfer context from `build_delivery_context` with distinct
///   values at every field, so a statement that echoes another context
///   reports otherwise.
/// Act:     `transfer`.
/// Assert:  the statement's context fields equal the payload context's,
///   field by field; `public_keys_components` over
///   `mint_delivery.recipient.public_keys` equals
///   `delivery.statement.seller_keys`; `envelope_components` over
///   `mint_delivery.recipient.envelope` equals
///   `delivery.statement.old_envelope`; `key_pair_public_keys` over
///   `delivery.buyer.key_pair` and `public_keys_components` over
///   `delivery.buyer.public_keys` each equal
///   `delivery.statement.buyer_keys`; `envelope_components` over
///   `delivery.buyer.envelope` equals `delivery.statement.new_envelope`;
///   `identity_element_components` over `mint_delivery.identity_element`
///   holds `delivery.statement.identity_element`, every element compared
///   through `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn transfer_returns_a_statement_over_the_payloads_context_the_holders_objects_and_the_buyers_keys()
{
    struct Consumer;

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
            let minted = mock_mint(
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
            let mint_delivery = minted.expect("the default case mints").delivery;
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
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
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
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
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
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
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let Ok(holder_public_keys) = transfer_key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &mint_delivery.recipient.public_keys,
                },
            );
            assert_eq!(
                encode_g1(&holder_public_keys.components.pk1),
                encode_g1(&delivery.statement.seller_keys.pk1)
            );
            assert_eq!(
                encode_g2(&holder_public_keys.components.pk2),
                encode_g2(&delivery.statement.seller_keys.pk2)
            );
            let Ok(holder_envelope) = transfer_key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            assert_eq!(
                encode_g1(&holder_envelope.components.c1),
                encode_g1(&delivery.statement.old_envelope.c1)
            );
            assert_eq!(
                encode_g1(&holder_envelope.components.c2),
                encode_g1(&delivery.statement.old_envelope.c2)
            );
            assert_eq!(
                encode_g2(&holder_envelope.components.d1),
                encode_g2(&delivery.statement.old_envelope.d1)
            );
            assert_eq!(
                encode_g2(&holder_envelope.components.d2),
                encode_g2(&delivery.statement.old_envelope.d2)
            );
            let Ok(buyer_public_keys) = transfer_key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &delivery.buyer.key_pair,
                },
            );
            assert_eq!(
                encode_g1(&buyer_public_keys.components.pk1),
                encode_g1(&delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&buyer_public_keys.components.pk2),
                encode_g2(&delivery.statement.buyer_keys.pk2)
            );
            let Ok(buyer_admitted) = transfer_key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &delivery.buyer.public_keys,
                },
            );
            assert_eq!(
                encode_g1(&buyer_admitted.components.pk1),
                encode_g1(&delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&buyer_admitted.components.pk2),
                encode_g2(&delivery.statement.buyer_keys.pk2)
            );
            let Ok(buyer_envelope) = transfer_key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &delivery.buyer.envelope,
                },
            );
            assert_eq!(
                encode_g1(&buyer_envelope.components.c1),
                encode_g1(&delivery.statement.new_envelope.c1)
            );
            assert_eq!(
                encode_g1(&buyer_envelope.components.c2),
                encode_g1(&delivery.statement.new_envelope.c2)
            );
            assert_eq!(
                encode_g2(&buyer_envelope.components.d1),
                encode_g2(&delivery.statement.new_envelope.d1)
            );
            assert_eq!(
                encode_g2(&buyer_envelope.components.d2),
                encode_g2(&delivery.statement.new_envelope.d2)
            );
            let Ok(identity_element) = kem.identity_element_components(
                IdentityElementComponentsParams,
                IdentityElementComponentsPayload {
                    identity_element: &mint_delivery.identity_element,
                },
            );
            assert_eq!(
                encode_g1(&identity_element.components.element),
                encode_g1(&delivery.statement.identity_element)
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(Default::default()),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // transfer_returns_a_statement_over_the_payloads_context_the_holders_objects_and_the_buyers_keys
}

/// Contract: Ordering → the draws are the three uniforms buyer `x`, buyer
///   `y`, and rerandomization, concatenated, each consumed by its call.
/// Arrange: the randomness family's mock concrete at its defaults, whose
///   successive draws differ, so a uniform handed to another call forms
///   another object.
/// Act:     `transfer`, then `unwrap`, `credential_from_components`,
///   `generate_keys`, and `rerandomize` replayed on the handed adapters
///   over the draw slices.
/// Assert:  `delivery.draws` is three slices of `UNIFORM_BYTES_LENGTH`
///   bytes; `generate_keys` on the handed key agreement over the first and
///   second slices as `x_uniform` and `y_uniform` returns a key pair whose
///   `key_pair_public_keys` equal `delivery.statement.buyer_keys`;
///   `rerandomize` on the handed KEM under `mint_delivery.parameter_set`
///   and `mint_delivery.identity_element`, over the credential
///   `credential_from_components` builds from `unwrap` of
///   `mint_delivery.recipient.envelope` under
///   `mint_delivery.recipient.key_pair` and over the third slice, returns
///   a credential whose `credential_components` equal those of
///   `delivery.credential`; every element compared through `encode_g1` or
///   `encode_g2`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn transfer_draws_three_uniforms_in_order() {
    struct Consumer;

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
            let minted = mock_mint(
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
            let mint_delivery = minted.expect("the default case mints").delivery;
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
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
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(delivery.draws.len(), 3 * length);
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
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
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
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
            let replayed_keys = transfer_key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: build_secret(
                            delivery.draws[..length].to_vec(),
                            Default::default(),
                        ),
                        y_uniform: build_secret(
                            delivery.draws[length..2 * length].to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the replay's key generation succeeds");
            let Ok(replayed_public_keys) = transfer_key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &replayed_keys.key_pair,
                },
            );
            assert_eq!(
                encode_g1(&replayed_public_keys.components.pk1),
                encode_g1(&delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&replayed_public_keys.components.pk2),
                encode_g2(&delivery.statement.buyer_keys.pk2)
            );
            let replayed = kem
                .rerandomize(
                    build_rerandomize_params(Default::default()),
                    build_rerandomize_payload::<P::Scalar, _, _, _>(
                        &mint_delivery.parameter_set,
                        &mint_delivery.identity_element,
                        &opened_credential.credential,
                        RerandomizePayloadOverrides {
                            uniform: Some(build_secret(
                                delivery.draws[2 * length..].to_vec(),
                                Default::default(),
                            )),
                        },
                    ),
                )
                .expect("the replay's rerandomization succeeds");
            let Ok(replayed_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &replayed.credential,
                },
            );
            let Ok(delivered_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            assert_eq!(
                encode_g1(&replayed_credential.components.a),
                encode_g1(&delivered_credential.components.a)
            );
            assert_eq!(
                encode_g2(&replayed_credential.components.b),
                encode_g2(&delivered_credential.components.b)
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(Default::default()),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // transfer_draws_three_uniforms_in_order
}

/// Contract: Witnesses → the witnesses are copies of the holder's key-pair
///   secrets, the rerandomization offset, and the wrap's coins the
///   delivery's credential and new envelope were formed with.
/// Arrange: every family's mock concrete at its defaults, each answering
///   its family's contract entries, so each relation below holds for the
///   values the delivery's objects were formed with and for no other.
/// Act:     `transfer`.
/// Assert:  through the handed pairing, every scalar encoded through
///   `encode_scalar` and every element through `encode_g1` or `encode_g2`:
///   `delivery.witnesses.seller_secrets.x` and `.y` equal the `x` and `y`
///   of `key_pair_components` over `mint_delivery.recipient.key_pair`; the
///   `b` of `credential_components` over `delivery.credential` equals
///   `add_g2` of the `b` of `mint_delivery.credential`'s components and
///   `mul_g2` of `g2_generator` by `delivery.witnesses.offset`; the `c1`
///   of `envelope_components` over `delivery.buyer.envelope` equals
///   `mul_g1` of `g1_generator` by `delivery.witnesses.coins.rho`, and its
///   `d1` equals `mul_g2` of `g2_generator` by
///   `delivery.witnesses.coins.sigma`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn transfer_returns_the_witnesses_the_objects_were_formed_with() {
    struct Consumer;

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
            let minted = mock_mint(
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
            let mint_delivery = minted.expect("the default case mints").delivery;
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
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
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
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
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            let encode_scalar = |scalar: &P::Scalar| -> Vec<u8> {
                pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: scalar.clone(),
                        },
                    )
                    .expect("the scalar encoding is infallible")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(recipient_secrets) = transfer_key_agreement.key_pair_components(
                KeyPairComponentsParams,
                KeyPairComponentsPayload {
                    key_pair: &mint_delivery.recipient.key_pair,
                },
            );
            assert_eq!(
                encode_scalar(delivery.witnesses.seller_secrets.x.expose()),
                encode_scalar(recipient_secrets.components.x.expose())
            );
            assert_eq!(
                encode_scalar(delivery.witnesses.seller_secrets.y.expose()),
                encode_scalar(recipient_secrets.components.y.expose())
            );
            let Ok(delivered_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(mint_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &mint_delivery.credential,
                },
            );
            let Ok(offset_times_g2) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: delivery.witnesses.offset.expose().clone(),
                },
            );
            let Ok(expected_b) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: mint_credential.components.b.clone(),
                    right: offset_times_g2.product,
                },
            );
            assert_eq!(
                encode_g2(&delivered_credential.components.b),
                encode_g2(&expected_b.sum)
            );
            let Ok(buyer_envelope) = transfer_key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(expected_c1) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: delivery.witnesses.coins.rho.expose().clone(),
                },
            );
            assert_eq!(
                encode_g1(&buyer_envelope.components.c1),
                encode_g1(&expected_c1.product)
            );
            let Ok(expected_d1) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.coins.sigma.expose().clone(),
                },
            );
            assert_eq!(
                encode_g2(&buyer_envelope.components.d1),
                encode_g2(&expected_d1.product)
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(Default::default()),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // transfer_returns_the_witnesses_the_objects_were_formed_with
}

/// Contract: Computed → the returned buyer is a holder a further
///   `transfer` takes.
/// Arrange: the first delivery from `transfer`.
/// Act:     `transfer` over the same parameter set, identity element, and
///   context with `&delivery.buyer` as the holder.
/// Assert:  the second result is `Ok`; its statement's `seller_keys`
///   equal the first statement's `buyer_keys`, and its `old_envelope`
///   equals the first statement's `new_envelope`, every element compared
///   through `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete, reached through its factory
///   at the defaults of its params builder.
#[test]
fn transfer_from_the_returned_buyer_transfers_again() {
    struct Consumer;

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
            let minted = mock_mint(
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
            let mint_delivery = minted.expect("the default case mints").delivery;
            let context = build_delivery_context::<F, TransferPurpose>(
                TransferPurpose::Transfer,
                Default::default(),
            );
            let first = transfer(
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
            let first_delivery = first.expect("the first transfer succeeds").delivery;
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
                    &first_delivery.buyer,
                    Default::default(),
                ),
            );
            let second_delivery = returned.expect("the second transfer succeeds").delivery;
            // Assert
            let encode_g1 = |point: &P::G1| -> Vec<u8> {
                pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: point.clone(),
                        },
                    )
                    .expect("the first-group encoding is infallible")
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
                    .expect("the second-group encoding is infallible")
                    .bytes
                    .as_ref()
                    .to_vec()
            };
            assert_eq!(
                encode_g1(&second_delivery.statement.seller_keys.pk1),
                encode_g1(&first_delivery.statement.buyer_keys.pk1)
            );
            assert_eq!(
                encode_g2(&second_delivery.statement.seller_keys.pk2),
                encode_g2(&first_delivery.statement.buyer_keys.pk2)
            );
            assert_eq!(
                encode_g1(&second_delivery.statement.old_envelope.c1),
                encode_g1(&first_delivery.statement.new_envelope.c1)
            );
            assert_eq!(
                encode_g1(&second_delivery.statement.old_envelope.c2),
                encode_g1(&first_delivery.statement.new_envelope.c2)
            );
            assert_eq!(
                encode_g2(&second_delivery.statement.old_envelope.d1),
                encode_g2(&first_delivery.statement.new_envelope.d1)
            );
            assert_eq!(
                encode_g2(&second_delivery.statement.old_envelope.d2),
                encode_g2(&first_delivery.statement.new_envelope.d2)
            );
        }
    }

    assert!(
        run_transfer_case(
            &build_transfer_case_deps(Consumer, Default::default()),
            build_transfer_case_params(Default::default()),
            build_transfer_case_payload(
                b"pkg@1.0.0#1",
                &build_root(Default::default()),
                Default::default(),
            ),
        )
        .is_ok()
    );
    // transfer_from_the_returned_buyer_transfers_again
}
