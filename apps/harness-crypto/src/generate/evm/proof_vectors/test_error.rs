#![allow(clippy::expect_used)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsReturn, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingReturn, FromFieldErrorReturn, IDecoderAdapter, IEncoderAdapter,
    IEncodingConsumer, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParamsOverrides,
    CreateKeyAgreementReturn, IKeyAgreementAdapter, IKeyAgreementConsumer, KeyAgreementConcrete,
    build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides, CreateKemReturn,
    ICredentialKemAdapter, IKemConsumer, IdentityScope, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParamsOverrides,
    CreateDeliveryProofReturn, DeliveryProofConcrete, DeliveryRelation, IDeliveryProofAdapter,
    IDeliveryProofConsumer, build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode, RandomSourceKind,
    build_create_random_source_params, create_random_source,
};

use super::super::challenge_vectors::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, DeliveryContextField,
    build_canonical_delivery_context,
};
use super::interface::ProofVectorsErrorReturn;
use super::mock::{
    ProofVectorsParamsOverrides, ProofVectorsPayloadOverrides, build_proof_vectors_deps,
    build_proof_vectors_params, build_proof_vectors_payload,
};
use super::proof_vectors;

/// Contract: Context, the form
///   `<F::ChainIdentifier as ICanonicalField>::from_field` over a clone
///   of `chain` refuses →
///   `Err(ProofVectorsErrorReturn::ChainIdentifier { relation, error })`,
///   `error` carried unchanged, `relation` `DeliveryRelation::Mint` for
///   the mint context.
/// Arrange:  the mint context overriding `chain` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`, a variant no form
///   declares; the transfer context at its defaults; the expected
///   `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::ChainIdentifier { relation: DeliveryRelation::Mint, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_chain_identifier_with_its_relation() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::Unsigned256,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(
                                                            ProofVectorsErrorReturn::ChainIdentifier {
                                                                relation: DeliveryRelation::Mint,
                                                                error,
                                                            }
                                                        )
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_chain_identifier_with_its_relation
}

/// Contract: Context, the form
///   `<F::Identity as ICanonicalField>::from_field` over a clone of
///   `entitlement_contract` refuses →
///   `Err(ProofVectorsErrorReturn::Identity { relation, field: DeliveryContextField::EntitlementContract, error })`.
/// Arrange:  the mint context overriding `entitlement_contract` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`; the transfer context at
///   its defaults; the expected `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes20 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Identity { relation: DeliveryRelation::Mint, field: DeliveryContextField::EntitlementContract, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_entitlement_contract_with_its_relation_and_field() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        entitlement_contract: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::FixedBytes20,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(
                                                            ProofVectorsErrorReturn::Identity {
                                                                relation: DeliveryRelation::Mint,
                                                                field:
                                                                    DeliveryContextField::EntitlementContract,
                                                                error,
                                                            }
                                                        )
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_entitlement_contract_with_its_relation_and_field
}

/// Contract: Context, the form
///   `<F::Entitlement as ICanonicalField>::from_field` over a clone of
///   `source_entitlement` refuses →
///   `Err(ProofVectorsErrorReturn::Entitlement { relation, field: DeliveryContextField::SourceEntitlement, error })`.
/// Arrange:  the mint context overriding `source_entitlement` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`; the transfer context at
///   its defaults; the expected `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Entitlement { relation: DeliveryRelation::Mint, field: DeliveryContextField::SourceEntitlement, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_source_entitlement_with_its_relation_and_field() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        source_entitlement: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::Unsigned256,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(
                                                            ProofVectorsErrorReturn::Entitlement {
                                                                relation: DeliveryRelation::Mint,
                                                                field:
                                                                    DeliveryContextField::SourceEntitlement,
                                                                error,
                                                            }
                                                        )
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_source_entitlement_with_its_relation_and_field
}

/// Contract: Context, the form
///   `<F::Entitlement as ICanonicalField>::from_field` over a clone of
///   `target_entitlement` refuses →
///   `Err(ProofVectorsErrorReturn::Entitlement { relation, field: DeliveryContextField::TargetEntitlement, error })`.
/// Arrange:  the mint context overriding `target_entitlement` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`; the transfer context at
///   its defaults; the expected `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Entitlement { relation: DeliveryRelation::Mint, field: DeliveryContextField::TargetEntitlement, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_target_entitlement_with_its_relation_and_field() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        target_entitlement: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::Unsigned256,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(
                                                            ProofVectorsErrorReturn::Entitlement {
                                                                relation: DeliveryRelation::Mint,
                                                                field:
                                                                    DeliveryContextField::TargetEntitlement,
                                                                error,
                                                            }
                                                        )
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_target_entitlement_with_its_relation_and_field
}

/// Contract: Context, the form
///   `<F::Interval as ICanonicalField>::from_field` over a clone of
///   `old_interval` refuses →
///   `Err(ProofVectorsErrorReturn::Interval { relation, field: DeliveryContextField::OldInterval, error })`.
/// Arrange:  the mint context overriding `old_interval` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`; the transfer context at
///   its defaults; the expected `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned64 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Interval { relation: DeliveryRelation::Mint, field: DeliveryContextField::OldInterval, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_old_interval_with_its_relation_and_field() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        old_interval: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::Unsigned64,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(ProofVectorsErrorReturn::Interval {
                                                            relation: DeliveryRelation::Mint,
                                                            field:
                                                                DeliveryContextField::OldInterval,
                                                            error,
                                                        })
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_old_interval_with_its_relation_and_field
}

/// Contract: Context, the form
///   `<F::Interval as ICanonicalField>::from_field` over a clone of
///   `new_interval` refuses →
///   `Err(ProofVectorsErrorReturn::Interval { relation, field: DeliveryContextField::NewInterval, error })`.
/// Arrange:  the mint context overriding `new_interval` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`; the transfer context at
///   its defaults; the expected `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned64 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Interval { relation: DeliveryRelation::Mint, field: DeliveryContextField::NewInterval, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_new_interval_with_its_relation_and_field() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        new_interval: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::Unsigned64,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(ProofVectorsErrorReturn::Interval {
                                                            relation: DeliveryRelation::Mint,
                                                            field:
                                                                DeliveryContextField::NewInterval,
                                                            error,
                                                        })
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_new_interval_with_its_relation_and_field
}

/// Contract: Context, the form
///   `<F::Identity as ICanonicalField>::from_field` over a clone of
///   `seller` refuses →
///   `Err(ProofVectorsErrorReturn::Identity { relation, field: DeliveryContextField::Seller, error })`.
/// Arrange:  the mint context overriding `seller` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`; the transfer context at
///   its defaults; the expected `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes20 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Identity { relation: DeliveryRelation::Mint, field: DeliveryContextField::Seller, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_seller_with_its_relation_and_field() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        seller: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::FixedBytes20,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(ProofVectorsErrorReturn::Identity {
                                                            relation: DeliveryRelation::Mint,
                                                            field: DeliveryContextField::Seller,
                                                            error,
                                                        })
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_seller_with_its_relation_and_field
}

/// Contract: Context, the form
///   `<F::Identity as ICanonicalField>::from_field` over a clone of
///   `buyer` refuses →
///   `Err(ProofVectorsErrorReturn::Identity { relation, field: DeliveryContextField::Buyer, error })`.
/// Arrange:  the mint context overriding `buyer` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`; the transfer context at
///   its defaults; the expected `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::FixedBytes20 }`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Identity { relation: DeliveryRelation::Mint, field: DeliveryContextField::Buyer, error })`
///   whole, `error` the literal arranged.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_buyer_with_its_relation_and_field() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        buyer: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::FixedBytes20,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(ProofVectorsErrorReturn::Identity {
                                                            relation: DeliveryRelation::Mint,
                                                            field: DeliveryContextField::Buyer,
                                                            error,
                                                        })
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_buyer_with_its_relation_and_field
}

/// Contract: Context, `relation` `DeliveryRelation::Transfer` for the
///   transfer context → a refusal carries `DeliveryRelation::Transfer`.
/// Arrange:  the mint context at its defaults and the transfer context
///   overriding `chain` with `CanonicalFieldValue::Bytes(vec![0x01])`,
///   so the mint records rebuild and the transfer context is the one
///   refused.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::ChainIdentifier { relation: DeliveryRelation::Transfer, error })`
///   whole, `error` the chain identifier form's refusal of that value.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_the_transfer_context_with_the_transfer_relation() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::Unsigned256,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(
                                                            ProofVectorsErrorReturn::ChainIdentifier {
                                                                relation:
                                                                    DeliveryRelation::Transfer,
                                                                error,
                                                            }
                                                        )
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_refusal_of_the_transfer_context_with_the_transfer_relation
}

/// Contract: Context, forms rebuilt in transcript order, the first
///   failure returns.
/// Arrange:  the mint context overriding `chain` and `buyer` each with
///   `CanonicalFieldValue::Bytes(vec![0x01])`, two refused forms, the
///   chain identifier the earlier in transcript order; the transfer
///   context at its defaults.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::ChainIdentifier { relation: DeliveryRelation::Mint, error })`
///   whole, `error` the chain identifier form's refusal of that value.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_first_refused_form_in_transcript_order() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        buyer: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: None,
        transfer_count: None,
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let pairing_concrete = None;
    let hashing_concrete = None;
    let random_failure_mode = None;
    let kem_scope = None;
    let key_agreement_concrete = None;
    let verifier_form = None;
    let delivery_proof_concrete = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        hashing_concrete: Option<HashToScalarConcrete>,
        random_failure_mode: Option<MockIRandomSourceAdapterFailureMode>,
        kem_scope: Option<IdentityScope>,
        key_agreement_concrete: Option<KeyAgreementConcrete>,
        verifier_form: Option<VerifierGroupArithmetic>,
        delivery_proof_concrete: Option<DeliveryProofConcrete>,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = CreateEncodingReturn<
            CreateChainFormsReturn<
                CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
            >,
        >;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the random source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                identity: &'a [u8],
                params_overrides: ProofVectorsParamsOverrides,
                payload_overrides: ProofVectorsPayloadOverrides,
                kem_scope: Option<IdentityScope>,
                key_agreement_concrete: Option<KeyAgreementConcrete>,
                verifier_form: Option<VerifierGroupArithmetic>,
                delivery_proof_concrete: Option<DeliveryProofConcrete>,
            }

            impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = CreateChainFormsReturn<
                    CreateKemReturn<CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>>,
                >;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter> {
                        pairing: &'a P,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        encoder: &'a E,
                        random: &'a dyn IRandomSourceAdapter,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        identity: &'a [u8],
                        params_overrides: ProofVectorsParamsOverrides,
                        payload_overrides: ProofVectorsPayloadOverrides,
                        kem_scope: Option<IdentityScope>,
                        key_agreement_concrete: Option<KeyAgreementConcrete>,
                        verifier_form: Option<VerifierGroupArithmetic>,
                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                    }

                    impl<P: IPairingArithmetic, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = CreateKemReturn<
                            CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>,
                        >;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            struct KemConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter, F> {
                                pairing: &'a P,
                                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                encoder: &'a E,
                                forms: &'a F,
                                random: &'a dyn IRandomSourceAdapter,
                                mint: &'a CanonicalDeliveryContext,
                                transfer: &'a CanonicalDeliveryContext,
                                identity: &'a [u8],
                                params_overrides: ProofVectorsParamsOverrides,
                                payload_overrides: ProofVectorsPayloadOverrides,
                                key_agreement_concrete: Option<KeyAgreementConcrete>,
                                verifier_form: Option<VerifierGroupArithmetic>,
                                delivery_proof_concrete: Option<DeliveryProofConcrete>,
                            }

                            impl<P: IPairingArithmetic, E: IEncoderAdapter, F: IChainForms>
                                IKemConsumer<P> for KemConsumer<'_, P, E, F>
                            {
                                type Output =
                                    CreateKeyAgreementReturn<CreateDeliveryProofReturn<()>>;

                                fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
                                    &self,
                                    _params: ConsumeKemParams,
                                    payload: ConsumeKemPayload<K>,
                                ) -> Self::Output {
                                    struct KeyAgreementConsumer<
                                        'a,
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > {
                                        pairing: &'a P,
                                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                                        encoder: &'a E,
                                        forms: &'a F,
                                        random: &'a dyn IRandomSourceAdapter,
                                        kem: &'a K,
                                        scope: IdentityScope,
                                        mint: &'a CanonicalDeliveryContext,
                                        transfer: &'a CanonicalDeliveryContext,
                                        identity: &'a [u8],
                                        params_overrides: ProofVectorsParamsOverrides,
                                        payload_overrides: ProofVectorsPayloadOverrides,
                                        verifier_form: Option<VerifierGroupArithmetic>,
                                        delivery_proof_concrete: Option<DeliveryProofConcrete>,
                                    }

                                    impl<
                                        P: IPairingArithmetic,
                                        E: IEncoderAdapter,
                                        F: IChainForms,
                                        K: ICredentialKemAdapter<Pairing = P>,
                                    > IKeyAgreementConsumer<P>
                                        for KeyAgreementConsumer<'_, P, E, F, K>
                                    {
                                        type Output = CreateDeliveryProofReturn<()>;

                                        fn consume_key_agreement<
                                            A: IKeyAgreementAdapter<Pairing = P>,
                                        >(
                                            &self,
                                            _params: ConsumeKeyAgreementParams,
                                            payload: ConsumeKeyAgreementPayload<A>,
                                        ) -> Self::Output {
                                            struct DeliveryProofConsumer<
                                                'a,
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            > {
                                                pairing: &'a P,
                                                forms: &'a F,
                                                random: &'a dyn IRandomSourceAdapter,
                                                kem: &'a K,
                                                key_agreement: &'a A,
                                                scope: IdentityScope,
                                                mint: &'a CanonicalDeliveryContext,
                                                transfer: &'a CanonicalDeliveryContext,
                                                identity: &'a [u8],
                                                params_overrides: ProofVectorsParamsOverrides,
                                                payload_overrides: ProofVectorsPayloadOverrides,
                                            }

                                            impl<
                                                P: IPairingArithmetic,
                                                F: IChainForms,
                                                K: ICredentialKemAdapter<Pairing = P>,
                                                A: IKeyAgreementAdapter<Pairing = P>,
                                            >
                                                IDeliveryProofConsumer<P, F>
                                                for DeliveryProofConsumer<'_, P, F, K, A>
                                            {
                                                type Output = ();

                                                fn consume_delivery_proof<
                                                    D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
                                                >(
                                                    &self,
                                                    _params: ConsumeDeliveryProofParams,
                                                    payload: ConsumeDeliveryProofPayload<D>,
                                                ) -> Self::Output
                                                {
                                                    _ = self.forms;

                                                    // Arrange
                                                    let error = FromFieldErrorReturn::WrongKind {
                                                        expected: CanonicalFieldKind::Unsigned256,
                                                    };

                                                    // Act
                                                    let result = proof_vectors::<P, K, A, D>(
                                                        &build_proof_vectors_deps(
                                                            self.pairing,
                                                            self.kem,
                                                            self.key_agreement,
                                                            &payload.adapter,
                                                            self.random,
                                                            Default::default(),
                                                        ),
                                                        build_proof_vectors_params(
                                                            ProofVectorsParamsOverrides {
                                                                mint_count: self
                                                                    .params_overrides
                                                                    .mint_count,
                                                                transfer_count: self
                                                                    .params_overrides
                                                                    .transfer_count,
                                                                scope: Some(self.scope),
                                                            },
                                                        ),
                                                        build_proof_vectors_payload(
                                                            self.mint,
                                                            self.transfer,
                                                            self.identity,
                                                            ProofVectorsPayloadOverrides {
                                                                mint_purpose: self
                                                                    .payload_overrides
                                                                    .mint_purpose,
                                                                transfer_purpose: self
                                                                    .payload_overrides
                                                                    .transfer_purpose,
                                                            },
                                                        ),
                                                    );

                                                    // Assert
                                                    assert_eq!(
                                                        result,
                                                        Err(
                                                            ProofVectorsErrorReturn::ChainIdentifier {
                                                                relation: DeliveryRelation::Mint,
                                                                error,
                                                            }
                                                        )
                                                    );
                                                }
                                            }

                                            // Act
                                            create_delivery_proof::<P, E, F, _>(
                                                &build_create_delivery_proof_deps(
                                                    self.pairing,
                                                    self.hash_to_scalar,
                                                    self.encoder,
                                                    self.random,
                                                    DeliveryProofConsumer {
                                                        pairing: self.pairing,
                                                        forms: self.forms,
                                                        random: self.random,
                                                        kem: self.kem,
                                                        key_agreement: &payload.adapter,
                                                        scope: self.scope,
                                                        mint: self.mint,
                                                        transfer: self.transfer,
                                                        identity: self.identity,
                                                        params_overrides: self.params_overrides,
                                                        payload_overrides: self.payload_overrides,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: self.delivery_proof_concrete,
                                                        verifier_form: self.verifier_form,
                                                        ..Default::default()
                                                    },
                                                ),
                                                build_create_delivery_proof_payload(
                                                    Default::default(),
                                                ),
                                            )
                                        }
                                    }

                                    // Act
                                    create_key_agreement(
                                        &build_create_key_agreement_deps(
                                            self.pairing,
                                            self.hash_to_scalar,
                                            self.encoder,
                                            self.random,
                                            KeyAgreementConsumer {
                                                pairing: self.pairing,
                                                hash_to_scalar: self.hash_to_scalar,
                                                encoder: self.encoder,
                                                forms: self.forms,
                                                random: self.random,
                                                kem: &payload.adapter,
                                                scope: payload.scope(),
                                                mint: self.mint,
                                                transfer: self.transfer,
                                                identity: self.identity,
                                                params_overrides: self.params_overrides,
                                                payload_overrides: self.payload_overrides,
                                                verifier_form: self.verifier_form,
                                                delivery_proof_concrete: self
                                                    .delivery_proof_concrete,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(
                                            CreateKeyAgreementParamsOverrides {
                                                concrete: self.key_agreement_concrete,
                                                ..Default::default()
                                            },
                                        ),
                                        build_create_key_agreement_payload(Default::default()),
                                    )
                                }
                            }

                            // Act
                            create_kem(
                                &build_create_kem_deps(
                                    self.pairing,
                                    self.hash_to_scalar,
                                    KemConsumer {
                                        pairing: self.pairing,
                                        hash_to_scalar: self.hash_to_scalar,
                                        encoder: self.encoder,
                                        forms: &payload.forms,
                                        random: self.random,
                                        mint: self.mint,
                                        transfer: self.transfer,
                                        identity: self.identity,
                                        params_overrides: self.params_overrides,
                                        payload_overrides: self.payload_overrides,
                                        key_agreement_concrete: self.key_agreement_concrete,
                                        verifier_form: self.verifier_form,
                                        delivery_proof_concrete: self.delivery_proof_concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_kem_params(CreateKemParamsOverrides {
                                    scope: self.kem_scope,
                                    ..Default::default()
                                }),
                                build_create_kem_payload(Default::default()),
                            )
                        }
                    }

                    // Act
                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                hash_to_scalar: self.hash_to_scalar,
                                encoder: &payload.adapter,
                                random: self.random,
                                mint: self.mint,
                                transfer: self.transfer,
                                identity: self.identity,
                                params_overrides: self.params_overrides,
                                payload_overrides: self.payload_overrides,
                                kem_scope: self.kem_scope,
                                key_agreement_concrete: self.key_agreement_concrete,
                                verifier_form: self.verifier_form,
                                delivery_proof_concrete: self.delivery_proof_concrete,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                }
            }

            // Act
            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hashing.adapter,
                    random: &*random.adapter,
                    mint: self.mint,
                    transfer: self.transfer,
                    identity: self.identity,
                    params_overrides: self.params_overrides,
                    payload_overrides: self.payload_overrides,
                    kem_scope: self.kem_scope,
                    key_agreement_concrete: self.key_agreement_concrete,
                    verifier_form: self.verifier_form,
                    delivery_proof_concrete: self.delivery_proof_concrete,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity,
        params_overrides,
        payload_overrides,
        hashing_concrete,
        random_failure_mode,
        kem_scope,
        key_agreement_concrete,
        verifier_form,
        delivery_proof_concrete,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: pairing_concrete,
            ..Default::default()
        }),
        CreatePairingPayload,
    );

    // Assert
    assert!(created.is_ok());
    created
        .expect("the default selection constructs the pairing")
        .output
        .expect("the default selection constructs the encoding")
        .output
        .expect("the default selection constructs the chain forms")
        .output
        .expect("the default selection constructs the KEM")
        .output
        .expect("the default selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_first_refused_form_in_transcript_order
}
