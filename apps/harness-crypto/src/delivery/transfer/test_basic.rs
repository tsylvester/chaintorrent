#![allow(clippy::expect_used)]

use chain::IChainForms;
use domain::{Secret, SecretConstructorParams};
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
    IdentityElementComponentsPayload, RerandomizeParams, RerandomizePayload,
};
use pairing::{
    AddG2Params, AddG2Payload, EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload,
    EncodeScalarParams, EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingReference,
    ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
};
use proof::{IDeliveryProofAdapter, MintPurpose, TransferPurpose, build_delivery_context};
use random::IRandomSourceAdapter;

use super::interface::ITransferCaseConsumer;
use super::mock::{
    build_transfer_case_deps, build_transfer_case_params, build_transfer_case_payload,
    build_transfer_deps, build_transfer_params, build_transfer_payload, run_transfer_case,
};
use super::transfer;
use crate::delivery::mint::provides::{
    AdmittedKemIdentity, MintPayloadOverrides, build_mint_deps, build_mint_params,
    build_mint_payload, mock_mint,
};

/// Contract: Computed → `Ok(TransferSuccessReturn { delivery })`, the
///   statement's context the payload's context, its `seller_keys` the
///   holder's public-key components, its `old_envelope` the holder
///   envelope's components, its `buyer_keys` the buyer's public-key
///   components, its `new_envelope` the buyer envelope's components, and
///   its `identity_element` the identity element's.
/// Arrange: a transfer context from `build_delivery_context` with distinct
///   values at every field.
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
///   holds `delivery.statement.identity_element`; every element compared
///   through `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn transfer_returns_a_statement_over_the_payloads_context_the_holders_objects_and_the_buyers_keys()
{
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
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
            assert_eq!(
                delivery.statement.context.asset_identity_hash,
                context.asset_identity_hash
            );
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
            let Ok(seller_keys) = key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &mint_delivery.recipient.public_keys,
                },
            );
            let Ok(seller_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: seller_keys.components.pk1,
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
                    point: seller_keys.components.pk2,
                },
            );
            let Ok(statement_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.seller_keys.pk2.clone(),
                },
            );
            assert_eq!(seller_pk2.bytes.as_ref(), statement_pk2.bytes.as_ref());
            let Ok(old_envelope) = key_agreement.envelope_components(
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
            let Ok(statement_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.old_envelope.c1.clone(),
                },
            );
            assert_eq!(old_c1.bytes.as_ref(), statement_c1.bytes.as_ref());
            let Ok(old_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: old_envelope.components.c2,
                },
            );
            let Ok(statement_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.old_envelope.c2.clone(),
                },
            );
            assert_eq!(old_c2.bytes.as_ref(), statement_c2.bytes.as_ref());
            let Ok(old_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: old_envelope.components.d1,
                },
            );
            let Ok(statement_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.old_envelope.d1.clone(),
                },
            );
            assert_eq!(old_d1.bytes.as_ref(), statement_d1.bytes.as_ref());
            let Ok(old_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: old_envelope.components.d2,
                },
            );
            let Ok(statement_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.old_envelope.d2.clone(),
                },
            );
            assert_eq!(old_d2.bytes.as_ref(), statement_d2.bytes.as_ref());
            let Ok(buyer_statement_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.buyer_keys.pk1.clone(),
                },
            );
            let Ok(buyer_statement_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.buyer_keys.pk2.clone(),
                },
            );
            let Ok(buyer_key_keys) = key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &delivery.buyer.key_pair,
                },
            );
            let Ok(buyer_keys_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: buyer_key_keys.components.pk1,
                },
            );
            assert_eq!(
                buyer_keys_pk1.bytes.as_ref(),
                buyer_statement_pk1.bytes.as_ref()
            );
            let Ok(buyer_keys_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: buyer_key_keys.components.pk2,
                },
            );
            assert_eq!(
                buyer_keys_pk2.bytes.as_ref(),
                buyer_statement_pk2.bytes.as_ref()
            );
            let Ok(buyer_admitted) = key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &delivery.buyer.public_keys,
                },
            );
            let Ok(buyer_admitted_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: buyer_admitted.components.pk1,
                },
            );
            assert_eq!(
                buyer_admitted_pk1.bytes.as_ref(),
                buyer_statement_pk1.bytes.as_ref()
            );
            let Ok(buyer_admitted_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: buyer_admitted.components.pk2,
                },
            );
            assert_eq!(
                buyer_admitted_pk2.bytes.as_ref(),
                buyer_statement_pk2.bytes.as_ref()
            );
            let Ok(new_envelope) = key_agreement.envelope_components(
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
            let Ok(identity_element) = kem.identity_element_components(
                IdentityElementComponentsParams,
                IdentityElementComponentsPayload {
                    identity_element: &mint_delivery.identity_element,
                },
            );
            let Ok(element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: identity_element.components.element,
                },
            );
            let Ok(statement_element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.identity_element.clone(),
                },
            );
            assert_eq!(element.bytes.as_ref(), statement_element.bytes.as_ref());
        }
    }

    let returned = run_transfer_case(
        &build_transfer_case_deps(Consumer, Default::default()),
        build_transfer_case_params(Default::default()),
        build_transfer_case_payload(
            b"pkg@1.0.0#1",
            &build_root(Default::default()),
            Default::default(),
        ),
    );
    assert!(returned.is_ok());
}

/// Contract: Ordering → the draws are the three uniforms buyer `x`, buyer
///   `y`, and rerandomization, concatenated, each consumed by its call.
/// Arrange: the randomness family's mock concrete at its defaults, whose
///   successive draws differ, so a uniform handed to another call forms
///   another object.
/// Act:     `transfer`.
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
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn transfer_draws_three_uniforms_in_order() {
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
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
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let mut slices = delivery
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH);
            let x_bytes = slices.next().expect("three draws");
            let y_bytes = slices.next().expect("three draws");
            let r_bytes = slices.next().expect("three draws");
            assert!(slices.next().is_none());
            let Ok(x_uniform) = Secret::try_new(SecretConstructorParams {
                value: x_bytes.to_vec(),
            });
            let Ok(y_uniform) = Secret::try_new(SecretConstructorParams {
                value: y_bytes.to_vec(),
            });
            let Ok(r_uniform) = Secret::try_new(SecretConstructorParams {
                value: r_bytes.to_vec(),
            });
            let generated = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform,
                        y_uniform,
                    },
                )
                .expect("the replayed generation succeeds");
            let Ok(replayed_keys) = key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &generated.key_pair,
                },
            );
            let Ok(replayed_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: replayed_keys.components.pk1,
                },
            );
            let Ok(statement_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivery.statement.buyer_keys.pk1.clone(),
                },
            );
            assert_eq!(replayed_pk1.bytes.as_ref(), statement_pk1.bytes.as_ref());
            let Ok(replayed_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: replayed_keys.components.pk2,
                },
            );
            let Ok(statement_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.buyer_keys.pk2.clone(),
                },
            );
            assert_eq!(replayed_pk2.bytes.as_ref(), statement_pk2.bytes.as_ref());
            let Ok(opened) = key_agreement.unwrap(
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
                        uniform: r_uniform,
                    },
                )
                .expect("the replayed rerandomization succeeds");
            let Ok(replayed_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &rerandomized.credential,
                },
            );
            let Ok(delivered_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(replayed_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: replayed_credential.components.a,
                },
            );
            let Ok(delivered_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivered_credential.components.a,
                },
            );
            assert_eq!(replayed_a.bytes.as_ref(), delivered_a.bytes.as_ref());
            let Ok(replayed_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: replayed_credential.components.b,
                },
            );
            let Ok(delivered_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered_credential.components.b,
                },
            );
            assert_eq!(replayed_b.bytes.as_ref(), delivered_b.bytes.as_ref());
        }
    }

    let returned = run_transfer_case(
        &build_transfer_case_deps(Consumer, Default::default()),
        build_transfer_case_params(Default::default()),
        build_transfer_case_payload(
            b"pkg@1.0.0#1",
            &build_root(Default::default()),
            Default::default(),
        ),
    );
    assert!(returned.is_ok());
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
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn transfer_returns_the_witnesses_the_objects_were_formed_with() {
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
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
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let Ok(holder_secrets) = key_agreement.key_pair_components(
                KeyPairComponentsParams,
                KeyPairComponentsPayload {
                    key_pair: &mint_delivery.recipient.key_pair,
                },
            );
            let Ok(holder_x) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: holder_secrets.components.x.expose().clone(),
                },
            );
            let Ok(witness_x) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: delivery.witnesses.seller_secrets.x.expose().clone(),
                },
            );
            assert_eq!(
                holder_x.bytes.expose().as_ref(),
                witness_x.bytes.expose().as_ref()
            );
            let Ok(holder_y) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: holder_secrets.components.y.expose().clone(),
                },
            );
            let Ok(witness_y) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: delivery.witnesses.seller_secrets.y.expose().clone(),
                },
            );
            assert_eq!(
                holder_y.bytes.expose().as_ref(),
                witness_y.bytes.expose().as_ref()
            );
            let Ok(mint_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &mint_delivery.credential,
                },
            );
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(offset_product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point.clone(),
                    scalar: delivery.witnesses.offset.expose().clone(),
                },
            );
            let Ok(expected_b) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: mint_credential.components.b,
                    right: offset_product.product,
                },
            );
            let Ok(delivered_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(expected_b_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: expected_b.sum,
                },
            );
            let Ok(delivered_b_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered_credential.components.b,
                },
            );
            assert_eq!(
                expected_b_encoded.bytes.as_ref(),
                delivered_b_encoded.bytes.as_ref()
            );
            let Ok(buyer_envelope) = key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(expected_c1) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: delivery.witnesses.coins.rho.expose().clone(),
                },
            );
            let Ok(expected_c1_encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: expected_c1.product,
                },
            );
            let Ok(delivered_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: buyer_envelope.components.c1,
                },
            );
            assert_eq!(
                expected_c1_encoded.bytes.as_ref(),
                delivered_c1.bytes.as_ref()
            );
            let Ok(expected_d1) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.coins.sigma.expose().clone(),
                },
            );
            let Ok(expected_d1_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: expected_d1.product,
                },
            );
            let Ok(delivered_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: buyer_envelope.components.d1,
                },
            );
            assert_eq!(
                expected_d1_encoded.bytes.as_ref(),
                delivered_d1.bytes.as_ref()
            );
        }
    }

    let returned = run_transfer_case(
        &build_transfer_case_deps(Consumer, Default::default()),
        build_transfer_case_params(Default::default()),
        build_transfer_case_payload(
            b"pkg@1.0.0#1",
            &build_root(Default::default()),
            Default::default(),
        ),
    );
    assert!(returned.is_ok());
}

/// Contract: Computed → the returned buyer is a holder a further
///   `transfer` takes.
/// Arrange: the first delivery from `transfer` as above.
/// Act:     `transfer` over the same parameter set, identity element, and
///   context with `&delivery.buyer` as the holder.
/// Assert:  the second result is `Ok`; its statement's `seller_keys` equal
///   the first statement's `buyer_keys`, and its `old_envelope` equals the
///   first statement's `new_envelope`, every element compared through
///   `encode_g1` or `encode_g2`.
/// Collaborators: every family's mock concrete in its default mode through
///   its factory; the subject's randomness source, the randomness family's
///   mock concrete at the default selection.
#[test]
fn transfer_from_the_returned_buyer_transfers_again() {
    struct Consumer;

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
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
            let first = transfer(
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
            let first_delivery = first.expect("the subject's calls succeed").delivery;
            let second = transfer(
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
                    &first_delivery.buyer,
                    Default::default(),
                ),
            );
            // Assert
            let second_delivery = second.expect("the second subject's calls succeed").delivery;
            let Ok(first_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: first_delivery.statement.buyer_keys.pk1.clone(),
                },
            );
            let Ok(second_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: second_delivery.statement.seller_keys.pk1.clone(),
                },
            );
            assert_eq!(first_pk1.bytes.as_ref(), second_pk1.bytes.as_ref());
            let Ok(first_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_delivery.statement.buyer_keys.pk2.clone(),
                },
            );
            let Ok(second_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: second_delivery.statement.seller_keys.pk2.clone(),
                },
            );
            assert_eq!(first_pk2.bytes.as_ref(), second_pk2.bytes.as_ref());
            let Ok(first_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: first_delivery.statement.new_envelope.c1.clone(),
                },
            );
            let Ok(second_c1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: second_delivery.statement.old_envelope.c1.clone(),
                },
            );
            assert_eq!(first_c1.bytes.as_ref(), second_c1.bytes.as_ref());
            let Ok(first_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: first_delivery.statement.new_envelope.c2.clone(),
                },
            );
            let Ok(second_c2) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: second_delivery.statement.old_envelope.c2.clone(),
                },
            );
            assert_eq!(first_c2.bytes.as_ref(), second_c2.bytes.as_ref());
            let Ok(first_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_delivery.statement.new_envelope.d1.clone(),
                },
            );
            let Ok(second_d1) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: second_delivery.statement.old_envelope.d1.clone(),
                },
            );
            assert_eq!(first_d1.bytes.as_ref(), second_d1.bytes.as_ref());
            let Ok(first_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: first_delivery.statement.new_envelope.d2.clone(),
                },
            );
            let Ok(second_d2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: second_delivery.statement.old_envelope.d2.clone(),
                },
            );
            assert_eq!(first_d2.bytes.as_ref(), second_d2.bytes.as_ref());
        }
    }

    let returned = run_transfer_case(
        &build_transfer_case_deps(Consumer, Default::default()),
        build_transfer_case_params(Default::default()),
        build_transfer_case_payload(
            b"pkg@1.0.0#1",
            &build_root(Default::default()),
            Default::default(),
        ),
    );
    assert!(returned.is_ok());
}
