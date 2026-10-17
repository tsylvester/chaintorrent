#![allow(clippy::expect_used)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsReturn, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifierConstructorParamsOverrides,
    SuiteIdentifierConstructorParamsOverrides, build_asset_identity_hash,
    build_parameter_set_identifier, build_suite_identifier,
};
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingReturn,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementReturn,
    IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_params, build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides, CreateKemReturn,
    DeriveIdentityPayloadOverrides, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    KemIdentity, SetupParamsOverrides, SetupPayloadOverrides, SetupScope, build_create_kem_deps,
    build_create_kem_params, build_create_kem_payload, build_derive_identity_params,
    build_derive_identity_payload, build_identity_element_components_params,
    build_identity_element_components_payload, build_setup_params, build_setup_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingPayload, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, IPairingConsumer, IPairingReference, ISampleUniformScalar,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParamsOverrides,
    CreateDeliveryProofReturn, DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_REPLACEMENT,
    IDeliveryProofAdapter, IDeliveryProofConsumer, build_create_delivery_proof_deps,
    build_create_delivery_proof_params, build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourcePayload, FillBytesParams, FillBytesPayloadOverrides,
    IRandomSourceAdapter, build_create_random_source_params, build_fill_bytes_payload,
    create_random_source,
};

use super::super::challenge_vectors::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, build_canonical_delivery_context,
};
use super::super::group_vectors::provides::{
    VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::mock::{
    ProofVectorContextOverrides, ProofVectorsParamsOverrides, ProofVectorsPayloadOverrides,
    build_proof_vector_context, build_proof_vectors_deps, build_proof_vectors_params,
    build_proof_vectors_payload,
};
use super::proof_vectors;

/// Contract: Computed → `Ok(ProofVectorsSuccessReturn { mint, transfer })`,
///   a record for each index from `0` below `params.mint_count.get()` and
///   below `params.transfer_count.get()`, each list in index order.
/// Arrange:  `mint_count` of `2` and `transfer_count` of `3`, counts that
///   differ from each other and from the builder's default; every other
///   input at its default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, `success.mint` holds `2` records and
///   `success.transfer` holds `3`.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_emit_the_configured_counts_of_mint_and_transfer_records() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(2),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(success.mint.len(), 2);
                                                    assert_eq!(success.transfer.len(), 3);
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
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
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
                                verifier_form: self.verifier_form,
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
                    verifier_form: self.verifier_form,
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
        kem_scope,
        verifier_form,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_emit_the_configured_counts_of_mint_and_transfer_records
}

/// Contract: Draw → every uniform has the length the pairing's scalar
///   sampling bound declares, and a record's `draws` are its uniforms
///   concatenated in draw order; a mint record draws the master, `u0`,
///   `u1`, issuance, seller `x`, and seller `y` uniforms, and a transfer
///   record those and the buyer `x`, buyer `y`, and rerandomization
///   uniforms.
/// Arrange:  `mint_count` and `transfer_count` of `1`; every other input
///   at its default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the mint record's `draws` hold `6` times
///   `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` bytes and
///   the transfer record's `draws` hold `9` times that length.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_draw_each_record_the_uniforms_of_its_calls_at_the_declared_length() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.mint[0].draws.len(),
                                                        6 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
                                                    );
                                                    assert_eq!(
                                                        success.transfer[0].draws.len(),
                                                        9 * <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH
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
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
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
                                verifier_form: self.verifier_form,
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
                    verifier_form: self.verifier_form,
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
        kem_scope,
        verifier_form,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_draw_each_record_the_uniforms_of_its_calls_at_the_declared_length
}

/// Contract: Ordering → the mint records, then the transfer records;
///   within a record its draws in the order master, `u0`, `u1`, issuance,
///   seller `x` and `y`, and for a transfer buyer `x` and `y` and
///   rerandomization, each draw immediately before its consuming call.
/// Arrange:  `mint_count` of `2` and `transfer_count` of `2`; the
///   expected draws from the independent source, consecutive calls
///   grouped for each mint record in turn and then each transfer record
///   in turn, so a transposed record or draw reports otherwise; every
///   other input at its default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, each record's `draws` equal its expected draws.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`, plus a second
///   random-source instance as the independent source of expected draws.
#[test]
fn proof_vectors_record_the_draws_in_the_sequence_the_source_yields() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(2),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(2),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = None;
    let expected = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    )
    .expect("the default selection constructs the random source");

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
        expected_random: &'a dyn IRandomSourceAdapter,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
                expected_random: &'a dyn IRandomSourceAdapter,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                                    let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                                                    let mut expected_mint: Vec<Vec<u8>> =
                                                        Vec::new();
                                                    let mut expected_transfer: Vec<Vec<u8>> =
                                                        Vec::new();
                                                    for _ in 0..2 {
                                                        let mut draws = Vec::new();
                                                        for _ in 0..6 {
                                                            draws.extend_from_slice(
                                                                &self
                                                                    .expected_random
                                                                    .fill_bytes(
                                                                        FillBytesParams,
                                                                        build_fill_bytes_payload(
                                                                            FillBytesPayloadOverrides {
                                                                                length: Some(
                                                                                    uniform,
                                                                                ),
                                                                            },
                                                                        ),
                                                                    )
                                                                    .expect(
                                                                        "the independent source yields the uniform",
                                                                    )
                                                                    .bytes
                                                                    .expose()
                                                                    .clone(),
                                                            );
                                                        }
                                                        expected_mint.push(draws);
                                                    }
                                                    for _ in 0..2 {
                                                        let mut draws = Vec::new();
                                                        for _ in 0..9 {
                                                            draws.extend_from_slice(
                                                                &self
                                                                    .expected_random
                                                                    .fill_bytes(
                                                                        FillBytesParams,
                                                                        build_fill_bytes_payload(
                                                                            FillBytesPayloadOverrides {
                                                                                length: Some(
                                                                                    uniform,
                                                                                ),
                                                                            },
                                                                        ),
                                                                    )
                                                                    .expect(
                                                                        "the independent source yields the uniform",
                                                                    )
                                                                    .bytes
                                                                    .expose()
                                                                    .clone(),
                                                            );
                                                        }
                                                        expected_transfer.push(draws);
                                                    }

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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    for (index, expected) in
                                                        expected_mint.iter().enumerate()
                                                    {
                                                        assert_eq!(
                                                            success.mint[index].draws,
                                                            *expected
                                                        );
                                                    }
                                                    for (index, expected) in
                                                        expected_transfer.iter().enumerate()
                                                    {
                                                        assert_eq!(
                                                            success.transfer[index].draws,
                                                            *expected
                                                        );
                                                    }
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
                                                        expected_random: self.expected_random,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
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
                                                expected_random: self.expected_random,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
                                        expected_random: self.expected_random,
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
                                verifier_form: self.verifier_form,
                                expected_random: self.expected_random,
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
                    verifier_form: self.verifier_form,
                    expected_random: self.expected_random,
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
        kem_scope,
        verifier_form,
        expected_random: &*expected.adapter,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_record_the_draws_in_the_sequence_the_source_yields
}

/// Contract: Record context → a `ProofVectorContext` read from the
///   `DeliveryContext`: `suite_identifier` a copy of `identifier()`,
///   `suite_version` its `version()`, each form-typed value the field its
///   form's `to_field` returns, `asset_identity_hash` and `parameter_set`
///   copies of their `as_bytes()`, `purpose` the purpose's `code()`, and
///   `expiry` the context's.
/// Arrange:  `mint_count` and `transfer_count` of `1`, the purposes
///   `DELIVERY_PURPOSE_REPLACEMENT` and `DELIVERY_PURPOSE_GRANT`, the
///   mint and transfer contexts overriding every value with distinct
///   contents, every other input at its default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, each record's `context` equals
///   `build_proof_vector_context` over the context's values.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_place_each_contexts_configured_values_in_its_records() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x72; 32]),
                version: Some(2),
            },
        )),
        chain: Some(CanonicalFieldValue::Unsigned256([0x23; 32])),
        entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x33; 20])),
        asset_identity_hash: Some(build_asset_identity_hash(
            AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0x74; 32]),
            },
        )),
        parameter_set_identifier: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x76; 32]),
            },
        )),
        source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x43; 32])),
        target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x44; 32])),
        old_interval: Some(CanonicalFieldValue::Unsigned64(5)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(6)),
        seller: Some(CanonicalFieldValue::FixedBytes20([0x63; 20])),
        buyer: Some(CanonicalFieldValue::FixedBytes20([0x64; 20])),
        expiry: Some(1_800_000_001),
    });
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0x73; 32]),
                version: Some(3),
            },
        )),
        chain: Some(CanonicalFieldValue::Unsigned256([0x24; 32])),
        entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x32; 20])),
        asset_identity_hash: Some(build_asset_identity_hash(
            AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0x75; 32]),
            },
        )),
        parameter_set_identifier: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0x77; 32]),
            },
        )),
        source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x45; 32])),
        target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x46; 32])),
        old_interval: Some(CanonicalFieldValue::Unsigned64(6)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(7)),
        seller: Some(CanonicalFieldValue::FixedBytes20([0x65; 20])),
        buyer: Some(CanonicalFieldValue::FixedBytes20([0x66; 20])),
        expiry: Some(1_900_000_000),
    });
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: Some(DELIVERY_PURPOSE_REPLACEMENT),
        transfer_purpose: Some(DELIVERY_PURPOSE_GRANT),
    };
    let kem_scope = None;
    let verifier_form = None;

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
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
                                                    let expected_mint = build_proof_vector_context(
                                                        ProofVectorContextOverrides {
                                                            suite_identifier: Some([0x72; 32]),
                                                            suite_version: Some(2),
                                                            chain: Some(
                                                                CanonicalFieldValue::Unsigned256(
                                                                    [0x23; 32],
                                                                ),
                                                            ),
                                                            entitlement_contract: Some(
                                                                CanonicalFieldValue::FixedBytes20(
                                                                    [0x33; 20],
                                                                ),
                                                            ),
                                                            asset_identity_hash: Some([0x74; 32]),
                                                            parameter_set: Some([0x76; 32]),
                                                            source_entitlement: Some(
                                                                CanonicalFieldValue::Unsigned256(
                                                                    [0x43; 32],
                                                                ),
                                                            ),
                                                            target_entitlement: Some(
                                                                CanonicalFieldValue::Unsigned256(
                                                                    [0x44; 32],
                                                                ),
                                                            ),
                                                            old_interval: Some(
                                                                CanonicalFieldValue::Unsigned64(5),
                                                            ),
                                                            new_interval: Some(
                                                                CanonicalFieldValue::Unsigned64(6),
                                                            ),
                                                            purpose: Some(4),
                                                            seller: Some(
                                                                CanonicalFieldValue::FixedBytes20(
                                                                    [0x63; 20],
                                                                ),
                                                            ),
                                                            buyer: Some(
                                                                CanonicalFieldValue::FixedBytes20(
                                                                    [0x64; 20],
                                                                ),
                                                            ),
                                                            expiry: Some(1_800_000_001),
                                                        },
                                                    );
                                                    let expected_transfer =
                                                        build_proof_vector_context(
                                                            ProofVectorContextOverrides {
                                                                suite_identifier: Some(
                                                                    [0x73; 32],
                                                                ),
                                                                suite_version: Some(3),
                                                                chain: Some(
                                                                    CanonicalFieldValue::Unsigned256(
                                                                        [0x24; 32],
                                                                    ),
                                                                ),
                                                                entitlement_contract: Some(
                                                                    CanonicalFieldValue::FixedBytes20(
                                                                        [0x32; 20],
                                                                    ),
                                                                ),
                                                                asset_identity_hash: Some(
                                                                    [0x75; 32],
                                                                ),
                                                                parameter_set: Some([0x77; 32]),
                                                                source_entitlement: Some(
                                                                    CanonicalFieldValue::Unsigned256(
                                                                        [0x45; 32],
                                                                    ),
                                                                ),
                                                                target_entitlement: Some(
                                                                    CanonicalFieldValue::Unsigned256(
                                                                        [0x46; 32],
                                                                    ),
                                                                ),
                                                                old_interval: Some(
                                                                    CanonicalFieldValue::Unsigned64(
                                                                        6,
                                                                    ),
                                                                ),
                                                                new_interval: Some(
                                                                    CanonicalFieldValue::Unsigned64(
                                                                        7,
                                                                    ),
                                                                ),
                                                                purpose: Some(3),
                                                                seller: Some(
                                                                    CanonicalFieldValue::FixedBytes20(
                                                                        [0x65; 20],
                                                                    ),
                                                                ),
                                                                buyer: Some(
                                                                    CanonicalFieldValue::FixedBytes20(
                                                                        [0x66; 20],
                                                                    ),
                                                                ),
                                                                expiry: Some(1_900_000_000),
                                                            },
                                                        );

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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.mint[0].context,
                                                        expected_mint
                                                    );
                                                    assert_eq!(
                                                        success.transfer[0].context,
                                                        expected_transfer
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
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
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
                                verifier_form: self.verifier_form,
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
                    verifier_form: self.verifier_form,
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
        kem_scope,
        verifier_form,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_place_each_contexts_configured_values_in_its_records
}

/// Contract: Identity under `Entitlement` → the identity is
///   `KemIdentity::Entitlement { canonical: payload.identity }` and the
///   setup scope `SetupScope::Entitlement`; the mint record's
///   `identity_element` is the `encode_g1` bytes of the derived identity
///   element's `element`.
/// Arrange:  `mint_count` and `transfer_count` of `1`, the identity
///   `b"pkg@1.0.0#1"`; the expected element from the received KEM over
///   the independent source's first three draws as `master_uniform`,
///   `u0_uniform`, and `u1_uniform`: `setup` under
///   `SetupScope::Entitlement`, `derive_identity` over
///   `KemIdentity::Entitlement { canonical: &identity }` and the
///   parameter set `setup` returned, `identity_element_components` over
///   the result, and the received pairing's `encode_g1` over `element`,
///   each call through the family's builders; every other input at its
///   default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the mint record's `identity_element` equals the
///   expected bytes.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`, plus a second
///   random-source instance as the independent source of expected draws.
#[test]
fn proof_vectors_carry_the_identity_element_of_an_entitlement_identity() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = None;
    let expected = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    )
    .expect("the default selection constructs the random source");

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
        expected_random: &'a dyn IRandomSourceAdapter,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
                expected_random: &'a dyn IRandomSourceAdapter,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                                    let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                                                    let master_uniform = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the independent source yields the uniform",
                                                        )
                                                        .bytes;
                                                    let u0_uniform = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the independent source yields the uniform",
                                                        )
                                                        .bytes;
                                                    let u1_uniform = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the independent source yields the uniform",
                                                        )
                                                        .bytes;
                                                    let setup = self
                                                        .kem
                                                        .setup(
                                                            build_setup_params(
                                                                SetupParamsOverrides {
                                                                    scope: Some(
                                                                        SetupScope::Entitlement,
                                                                    ),
                                                                },
                                                            ),
                                                            build_setup_payload::<P::Scalar>(
                                                                SetupPayloadOverrides {
                                                                    master_uniform: Some(
                                                                        master_uniform,
                                                                    ),
                                                                    u0_uniform: Some(u0_uniform),
                                                                    u1_uniform: Some(u1_uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the received KEM sets up the parameter set",
                                                        );
                                                    let derived = self
                                                        .kem
                                                        .derive_identity(
                                                            build_derive_identity_params(
                                                                Default::default(),
                                                            ),
                                                            build_derive_identity_payload(
                                                                &setup.parameter_set,
                                                                DeriveIdentityPayloadOverrides {
                                                                    identity: Some(
                                                                        KemIdentity::Entitlement {
                                                                            canonical: self
                                                                                .identity,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the received KEM derives the identity element",
                                                        );
                                                    let components = self
                                                        .kem
                                                        .identity_element_components(
                                                            build_identity_element_components_params(
                                                                Default::default(),
                                                            ),
                                                            build_identity_element_components_payload(
                                                                &derived.identity_element,
                                                                Default::default(),
                                                            ),
                                                        )
                                                        .expect(
                                                            "the received KEM decomposes the identity element",
                                                        );
                                                    let expected = self
                                                        .pairing
                                                        .encode_g1(
                                                            EncodeG1Params,
                                                            EncodeG1Payload {
                                                                point: components
                                                                    .components
                                                                    .element,
                                                            },
                                                        )
                                                        .expect(
                                                            "the pairing encodes the first-group point",
                                                        )
                                                        .bytes;

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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.mint[0].identity_element.as_slice(),
                                                        expected.as_ref()
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
                                                        expected_random: self.expected_random,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
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
                                                expected_random: self.expected_random,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
                                        expected_random: self.expected_random,
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
                                verifier_form: self.verifier_form,
                                expected_random: self.expected_random,
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
                    verifier_form: self.verifier_form,
                    expected_random: self.expected_random,
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
        kem_scope,
        verifier_form,
        expected_random: &*expected.adapter,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_carry_the_identity_element_of_an_entitlement_identity
}

/// Contract: Identity under `Asset` → `AssetIdentityHash::try_new`
///   admits the 32 bytes, the identity is
///   `KemIdentity::Asset { identity_hash: &hash }` and the setup scope
///   `SetupScope::Asset { identity_hash: &hash }`; the mint record's
///   `identity_element` is the `encode_g1` bytes of the derived identity
///   element's `element`.
/// Arrange:  `scope` `IdentityScope::Asset`, the identity
///   `vec![0x44; 32]`, `mint_count` and `transfer_count` of `1`; the
///   expected element as the entitlement block builds it, over the hash
///   `build_asset_identity_hash` builds from the bytes `[0x44; 32]`,
///   `SetupScope::Asset`, and `KemIdentity::Asset`; every other input at
///   its default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the mint record's `identity_element` equals the
///   expected bytes.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`, plus a second
///   random-source instance as the independent source of expected draws.
#[test]
fn proof_vectors_carry_the_identity_element_of_an_asset_identity() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = Some(IdentityScope::Asset);
    let verifier_form = None;
    let expected = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(Default::default()),
        CreateRandomSourcePayload,
    )
    .expect("the default selection constructs the random source");

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
        expected_random: &'a dyn IRandomSourceAdapter,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
                expected_random: &'a dyn IRandomSourceAdapter,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                                    let identity_hash = build_asset_identity_hash(
                                                        AssetIdentityHashConstructorParamsOverrides {
                                                            bytes: Some([0x44; 32]),
                                                        },
                                                    );
                                                    let uniform = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
                                                    let master_uniform = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the independent source yields the uniform",
                                                        )
                                                        .bytes;
                                                    let u0_uniform = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the independent source yields the uniform",
                                                        )
                                                        .bytes;
                                                    let u1_uniform = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the independent source yields the uniform",
                                                        )
                                                        .bytes;
                                                    let setup = self
                                                        .kem
                                                        .setup(
                                                            build_setup_params(
                                                                SetupParamsOverrides {
                                                                    scope: Some(SetupScope::Asset {
                                                                        identity_hash:
                                                                            &identity_hash,
                                                                    }),
                                                                },
                                                            ),
                                                            build_setup_payload::<P::Scalar>(
                                                                SetupPayloadOverrides {
                                                                    master_uniform: Some(
                                                                        master_uniform,
                                                                    ),
                                                                    u0_uniform: Some(u0_uniform),
                                                                    u1_uniform: Some(u1_uniform),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the received KEM sets up the parameter set",
                                                        );
                                                    let derived = self
                                                        .kem
                                                        .derive_identity(
                                                            build_derive_identity_params(
                                                                Default::default(),
                                                            ),
                                                            build_derive_identity_payload(
                                                                &setup.parameter_set,
                                                                DeriveIdentityPayloadOverrides {
                                                                    identity: Some(
                                                                        KemIdentity::Asset {
                                                                            identity_hash:
                                                                                &identity_hash,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the received KEM derives the identity element",
                                                        );
                                                    let components = self
                                                        .kem
                                                        .identity_element_components(
                                                            build_identity_element_components_params(
                                                                Default::default(),
                                                            ),
                                                            build_identity_element_components_payload(
                                                                &derived.identity_element,
                                                                Default::default(),
                                                            ),
                                                        )
                                                        .expect(
                                                            "the received KEM decomposes the identity element",
                                                        );
                                                    let expected = self
                                                        .pairing
                                                        .encode_g1(
                                                            EncodeG1Params,
                                                            EncodeG1Payload {
                                                                point: components
                                                                    .components
                                                                    .element,
                                                            },
                                                        )
                                                        .expect(
                                                            "the pairing encodes the first-group point",
                                                        )
                                                        .bytes;

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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.mint[0].identity_element.as_slice(),
                                                        expected.as_ref()
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
                                                        expected_random: self.expected_random,
                                                    },
                                                    Default::default(),
                                                ),
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
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
                                                expected_random: self.expected_random,
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
                                        expected_random: self.expected_random,
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
                                verifier_form: self.verifier_form,
                                expected_random: self.expected_random,
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
                    verifier_form: self.verifier_form,
                    expected_random: self.expected_random,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
        }
    }

    let consumer = PairingConsumer {
        mint: &mint,
        transfer: &transfer,
        identity: identity.as_slice(),
        params_overrides,
        payload_overrides,
        kem_scope,
        verifier_form,
        expected_random: &*expected.adapter,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_carry_the_identity_element_of_an_asset_identity
}

/// Contract: Mint proof components,
///   `MintFirstGroupOnly { challenge, responses, first_messages }` →
///   `second_group_first_messages` holds one `encode_g2` encoding for
///   each of `first_messages.hpub`, `d1`, and `d2`.
/// Arrange:  `build_create_delivery_proof_params` overriding
///   `verifier_form` with `VerifierGroupArithmetic::FirstGroupOnly`;
///   `mint_count` and `transfer_count` of `1`; the expected length from
///   the received pairing: the length of `encode_g2` over its
///   second-group generator, read through the pairing's generator
///   operation, times `3`; every other input at its default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the mint record's `second_group_first_messages`
///   holds the expected length in bytes.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_mint() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = Some(VerifierGroupArithmetic::FirstGroupOnly);

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
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
                                                    let generator = self
                                                        .pairing
                                                        .g2_generator(
                                                            G2GeneratorParams,
                                                            G2GeneratorPayload,
                                                        )
                                                        .expect(
                                                            "the pairing yields the second-group generator",
                                                        )
                                                        .point;
                                                    let expected = 3
                                                        * self
                                                            .pairing
                                                            .encode_g2(
                                                                EncodeG2Params,
                                                                EncodeG2Payload {
                                                                    point: generator,
                                                                },
                                                            )
                                                            .expect(
                                                                "the pairing encodes the second-group point",
                                                            )
                                                            .bytes
                                                            .as_ref()
                                                            .len();

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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.mint[0]
                                                            .second_group_first_messages
                                                            .len(),
                                                        expected
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
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
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
                                verifier_form: self.verifier_form,
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
                    verifier_form: self.verifier_form,
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
        kem_scope,
        verifier_form,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_mint
}

/// Contract: Transfer proof components,
///   `TransferFirstGroupOnly { challenge, responses, first_messages }` →
///   `second_group_first_messages` holds one `encode_g2` encoding for
///   each of `first_messages.pk2`, `d1`, and `d2`.
/// Arrange:  `build_create_delivery_proof_params` overriding
///   `verifier_form` with `VerifierGroupArithmetic::FirstGroupOnly`;
///   `mint_count` and `transfer_count` of `1`; the expected length from
///   the received pairing: the length of `encode_g2` over its
///   second-group generator, read through the pairing's generator
///   operation, times `3`; every other input at its default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the transfer record's `second_group_first_messages`
///   holds the expected length in bytes.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_transfer() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = Some(VerifierGroupArithmetic::FirstGroupOnly);

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
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
                                                    let generator = self
                                                        .pairing
                                                        .g2_generator(
                                                            G2GeneratorParams,
                                                            G2GeneratorPayload,
                                                        )
                                                        .expect(
                                                            "the pairing yields the second-group generator",
                                                        )
                                                        .point;
                                                    let expected = 3
                                                        * self
                                                            .pairing
                                                            .encode_g2(
                                                                EncodeG2Params,
                                                                EncodeG2Payload {
                                                                    point: generator,
                                                                },
                                                            )
                                                            .expect(
                                                                "the pairing encodes the second-group point",
                                                            )
                                                            .bytes
                                                            .as_ref()
                                                            .len();

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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.transfer[0]
                                                            .second_group_first_messages
                                                            .len(),
                                                        expected
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
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
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
                                verifier_form: self.verifier_form,
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
                    verifier_form: self.verifier_form,
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
        kem_scope,
        verifier_form,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_carry_second_group_first_messages_in_the_first_group_only_form_for_a_transfer
}

/// Contract: Mint proof components,
///   `MintBothGroups { challenge, responses }` →
///   `second_group_first_messages` empty.
/// Arrange:  `build_create_delivery_proof_params` at its default
///   `verifier_form`, `VerifierGroupArithmetic::BothGroups`;
///   `mint_count` and `transfer_count` of `1`; every other input at its
///   default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the mint record's `second_group_first_messages`
///   equals `Vec::new()`.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_mint() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = Some(VerifierGroupArithmetic::BothGroups);

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.mint[0].second_group_first_messages,
                                                        Vec::<u8>::new()
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
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
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
                                verifier_form: self.verifier_form,
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
                    verifier_form: self.verifier_form,
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
        kem_scope,
        verifier_form,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_mint
}

/// Contract: Transfer proof components,
///   `TransferBothGroups { challenge, responses }` →
///   `second_group_first_messages` empty.
/// Arrange:  `build_create_delivery_proof_params` at its default
///   `verifier_form`, `VerifierGroupArithmetic::BothGroups`;
///   `mint_count` and `transfer_count` of `1`; every other input at its
///   default.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the transfer record's `second_group_first_messages`
///   equals `Vec::new()`.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_transfer() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();
    let params_overrides = ProofVectorsParamsOverrides {
        mint_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        transfer_count: Some(build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        })),
        scope: None,
    };
    let payload_overrides = ProofVectorsPayloadOverrides {
        mint_purpose: None,
        transfer_purpose: None,
    };
    let kem_scope = None;
    let verifier_form = Some(VerifierGroupArithmetic::BothGroups);

    struct PairingConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        params_overrides: ProofVectorsParamsOverrides,
        payload_overrides: ProofVectorsPayloadOverrides,
        kem_scope: Option<IdentityScope>,
        verifier_form: Option<VerifierGroupArithmetic>,
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
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the default selection constructs the hash-to-scalar adapter");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
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
                verifier_form: Option<VerifierGroupArithmetic>,
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
                        verifier_form: Option<VerifierGroupArithmetic>,
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
                                verifier_form: Option<VerifierGroupArithmetic>,
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
                                                    let success = result.expect(
                                                        "the configured counts are admitted",
                                                    );
                                                    assert_eq!(
                                                        success.transfer[0]
                                                            .second_group_first_messages,
                                                        Vec::<u8>::new()
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
                                            },
                                            Default::default(),
                                        ),
                                        build_create_key_agreement_params(Default::default()),
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
                                        verifier_form: self.verifier_form,
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
                                verifier_form: self.verifier_form,
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
                    verifier_form: self.verifier_form,
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
        kem_scope,
        verifier_form,
    };

    // Act
    let created = create_pairing(
        &build_create_pairing_deps(consumer, Default::default()),
        build_create_pairing_params(Default::default()),
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
    // proof_vectors_carry_no_second_group_first_messages_in_the_both_groups_form_for_a_transfer
}
