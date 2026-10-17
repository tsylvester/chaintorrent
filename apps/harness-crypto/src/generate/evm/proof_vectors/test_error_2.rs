#![allow(clippy::expect_used)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, CreateChainFormsReturn, IChainForms,
    IChainFormsConsumer, build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::AssetIdentityHashTryNewErrorReturn;
use encoding::{
    CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingReturn,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, build_create_encoding_deps,
    build_create_encoding_params, build_create_encoding_payload, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, CreateKeyAgreementParamsOverrides,
    CreateKeyAgreementReturn, GenerateKeysErrorReturn, IKeyAgreementAdapter, IKeyAgreementConsumer,
    KeyAgreementConcrete, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    WrapToErrorReturn, build_create_key_agreement_deps, build_create_key_agreement_params,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, HashToScalarConcrete, IHashToScalarAdapter,
    MockIHashToScalarAdapterFailureMode, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload,
    build_hash_to_scalar_params, build_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, CreateKemParamsOverrides, CreateKemReturn,
    DeriveIdentityErrorReturn, ICredentialKemAdapter, IKemConsumer, IdentityScope,
    SetupErrorReturn, build_create_kem_deps, build_create_kem_params, build_create_kem_payload,
    create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, MockIPairingAdapterFailureMode, PairingConcrete,
    SampleUniformScalarParams, SampleUniformScalarPayload, VerifierGroupArithmetic,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParamsOverrides,
    CreateDeliveryProofReturn, DeliveryProofConcrete, IDeliveryProofAdapter,
    IDeliveryProofConsumer, MockIDeliveryProofAdapterFailureMode,
    MockIDeliveryProofAdapterProveMintErrorReturn, ProveMintErrorReturn,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayloadOverrides, IRandomSourceAdapter,
    MockIRandomSourceAdapterFailureMode, RandomSourceKind, build_create_random_source_params,
    build_fill_bytes_payload, create_random_source,
};

use super::super::challenge_vectors::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, build_canonical_delivery_context,
};
use super::interface::ProofVectorsErrorReturn;
use super::mock::{
    ProofVectorsParamsOverrides, ProofVectorsPayloadOverrides, build_proof_vectors_deps,
    build_proof_vectors_params, build_proof_vectors_payload,
};
use super::proof_vectors;

/// Contract: Identity under `Asset`,
///   `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity)`
///   fails → `Err(ProofVectorsErrorReturn::AssetIdentityLength { actual })`,
///   `actual` the identity's length.
/// Arrange:  `scope` `IdentityScope::Asset` and the identity
///   `vec![0x44; 31]`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::AssetIdentityLength { actual: 31 })`
///   whole.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_refuse_an_asset_identity_shorter_than_32_bytes() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 31];
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
    let kem_scope = Some(IdentityScope::Asset);
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
                                                            ProofVectorsErrorReturn::AssetIdentityLength {
                                                                actual: 31,
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
        identity: &identity,
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
    // proof_vectors_refuse_an_asset_identity_shorter_than_32_bytes
}

/// Contract: Identity under `Asset`,
///   `<[u8; ASSET_IDENTITY_HASH_LENGTH]>::try_from(payload.identity)`
///   fails → `Err(ProofVectorsErrorReturn::AssetIdentityLength { actual })`,
///   `actual` the identity's length.
/// Arrange:  `scope` `IdentityScope::Asset` and the identity
///   `vec![0x44; 33]`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::AssetIdentityLength { actual: 33 })`
///   whole.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_refuse_an_asset_identity_longer_than_32_bytes() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 33];
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
    let kem_scope = Some(IdentityScope::Asset);
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
                                                            ProofVectorsErrorReturn::AssetIdentityLength {
                                                                actual: 33,
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
        identity: &identity,
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
    // proof_vectors_refuse_an_asset_identity_longer_than_32_bytes
}

/// Contract: Identity under `Asset`, `AssetIdentityHash::try_new`
///   refuses → `Err(ProofVectorsErrorReturn::AssetIdentityHash(error))`,
///   `error` unchanged.
/// Arrange:  `scope` `IdentityScope::Asset` and the identity
///   `vec![0x00; 32]`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::AssetIdentityHash(AssetIdentityHashTryNewErrorReturn::AllZero))`
///   whole.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_refusal_of_an_all_zero_asset_identity() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x00; 32];
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
    let kem_scope = Some(IdentityScope::Asset);
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
                                                            ProofVectorsErrorReturn::AssetIdentityHash(
                                                                AssetIdentityHashTryNewErrorReturn::AllZero,
                                                            )
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
        identity: &identity,
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
    // proof_vectors_return_the_refusal_of_an_all_zero_asset_identity
}

/// Contract: Identity, an exhaustive `match` on `params.scope` before
///   any record.
/// Arrange:  `scope` `IdentityScope::Asset`, the identity
///   `vec![0x44; 31]`, and the mint context overriding `chain` with
///   `CanonicalFieldValue::Bytes(vec![0x01])`, a refused identity and a
///   refused form together.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::AssetIdentityLength { actual: 31 })`
///   whole.
/// Collaborators: the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, key agreement's, and delivery
///   proof's mock concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_refuse_the_asset_identity_before_any_context() {
    // Arrange
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Bytes(vec![0x01])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 31];
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
    let kem_scope = Some(IdentityScope::Asset);
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
                                                            ProofVectorsErrorReturn::AssetIdentityLength {
                                                                actual: 31,
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
        identity: &identity,
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
    // proof_vectors_refuse_the_asset_identity_before_any_context
}

/// Contract: Draw, `deps.random.fill_bytes` fails →
///   `Err(ProofVectorsErrorReturn::FillBytes(error))`, `error`
///   unchanged.
/// Arrange:  `build_create_random_source_params` overriding `kind`
///   with
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::FillBytesRefused)`;
///   the expected `error` the refusal of the independent source's
///   `fill_bytes`, built over the same kind.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::FillBytes(error))` whole.
/// Collaborators: the random source's mock concrete refusing fills and
///   the pairing's, hash-to-scalar's, encoding's, chain forms', KEM's,
///   key agreement's, and delivery proof's mock concretes in their
///   succeeding modes through `create_random_source`, `create_pairing`,
///   `create_hash_to_scalar`, `create_encoding`, `create_chain_forms`,
///   `create_kem`, `create_key_agreement`, and
///   `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_draw_refusal_whole() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
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
    let random_failure_mode = Some(MockIRandomSourceAdapterFailureMode::FillBytesRefused);
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
            .expect("the named selection constructs the random source");
            let expected_source = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the named selection constructs the independent source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                expected_random: &'a dyn IRandomSourceAdapter,
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
                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                                    let error = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(
                                                                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .err()
                                                        .expect(
                                                            "the fill refusal returns",
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
                                                    assert_eq!(
                                                        result,
                                                        Err(ProofVectorsErrorReturn::FillBytes(
                                                            error
                                                        ))
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
                                                        expected_random: self.expected_random,
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
                                                expected_random: self.expected_random,
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
                                        expected_random: self.expected_random,
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
                                expected_random: self.expected_random,
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
                    expected_random: &*expected_source.adapter,
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
    // proof_vectors_return_the_draw_refusal_whole
}

/// Contract: Issued credential, `deps.kem.derive_identity` fails →
///   `Err(ProofVectorsErrorReturn::DeriveIdentity(error))`, `error`
///   unchanged.
/// Arrange:  `scope` `IdentityScope::Entitlement` and
///   `build_create_hash_to_scalar_params` overriding `concrete` with
///   `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`;
///   the expected sampling refusal the `Err` of `hash_to_scalar` over
///   `build_hash_to_scalar_params` and `build_hash_to_scalar_payload`
///   at their defaults on an adapter built under the same mode.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))`
///   whole, `error` the expected sampling refusal.
/// Collaborators: the hash-to-scalar's mock concrete refusing sampling
///   and the random source's, pairing's, encoding's, chain forms',
///   KEM's, key agreement's, and delivery proof's mock concretes in
///   their succeeding modes through `create_random_source`,
///   `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_chain_forms`, `create_kem`, `create_key_agreement`, and
///   `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_hash_to_scalar_refusal_of_identity_derivation_whole() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
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
    let hashing_concrete = Some(HashToScalarConcrete::Mock(
        MockIHashToScalarAdapterFailureMode::SamplingRefused,
    ));
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
            .expect("the named selection constructs the hash-to-scalar adapter");
            let expected_hashing = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                    concrete: self.hashing_concrete,
                    ..Default::default()
                }),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the named selection constructs the independent hash-to-scalar adapter");
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
                expected_hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
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
                        expected_hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
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
                                expected_hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
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
                                        expected_hash_to_scalar:
                                            &'a dyn IHashToScalarAdapter<P::Scalar>,
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
                                                expected_hash_to_scalar:
                                                    &'a dyn IHashToScalarAdapter<P::Scalar>,
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
                                                    let error = self
                                                        .expected_hash_to_scalar
                                                        .hash_to_scalar(
                                                            build_hash_to_scalar_params(
                                                                Default::default(),
                                                            ),
                                                            build_hash_to_scalar_payload(
                                                                Default::default(),
                                                            ),
                                                        )
                                                        .err()
                                                        .expect("the sampling refusal returns");

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
                                                            ProofVectorsErrorReturn::DeriveIdentity(
                                                                DeriveIdentityErrorReturn::HashToScalar(
                                                                    error,
                                                                ),
                                                            )
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
                                                        expected_hash_to_scalar: self
                                                            .expected_hash_to_scalar,
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
                                                expected_hash_to_scalar: self
                                                    .expected_hash_to_scalar,
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
                                        expected_hash_to_scalar: self.expected_hash_to_scalar,
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
                                expected_hash_to_scalar: self.expected_hash_to_scalar,
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
                    expected_hash_to_scalar: &*expected_hashing.adapter,
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
    // proof_vectors_return_the_hash_to_scalar_refusal_of_identity_derivation_whole
}

/// Contract: Issued credential, `deps.kem.setup` fails →
///   `Err(ProofVectorsErrorReturn::Setup(error))`, `error` unchanged.
/// Arrange:  `build_create_pairing_params` overriding `concrete` with
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`;
///   the expected `error` the `Err` of
///   `<P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes`
///   over the independent source's first draw.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::Setup(SetupErrorReturn::MasterScalarSampling(error)))`
///   whole.
/// Collaborators: the pairing's mock concrete sampling at the wrong
///   length and the random source's, hash-to-scalar's, encoding's,
///   chain forms', KEM's, key agreement's, and delivery proof's mock
///   concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_sampling_refusal_of_setup_whole() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
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
    let pairing_concrete = Some(PairingConcrete::Mock(
        MockIPairingAdapterFailureMode::SampleScalarWrongLength,
    ));
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
            let expected_source = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: self.random_failure_mode.map(RandomSourceKind::Mock),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the default selection constructs the independent source");

            struct EncodingConsumer<'a, P: IPairingArithmetic> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                expected_random: &'a dyn IRandomSourceAdapter,
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
                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                        expected_random: &'a dyn IRandomSourceAdapter,
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
                                                expected_random: &'a dyn IRandomSourceAdapter,
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
                                                    let uniform = self
                                                        .expected_random
                                                        .fill_bytes(
                                                            FillBytesParams,
                                                            build_fill_bytes_payload(
                                                                FillBytesPayloadOverrides {
                                                                    length: Some(
                                                                        <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                                                                    ),
                                                                },
                                                            ),
                                                        )
                                                        .expect(
                                                            "the independent source yields the uniform",
                                                        )
                                                        .bytes;
                                                    let error = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                                                        SampleUniformScalarParams,
                                                        SampleUniformScalarPayload { uniform },
                                                    )
                                                    .err()
                                                    .expect(
                                                        "the scalar sampling is refused",
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
                                                    assert_eq!(
                                                        result,
                                                        Err(ProofVectorsErrorReturn::Setup(
                                                            SetupErrorReturn::MasterScalarSampling(
                                                                error,
                                                            ),
                                                        ))
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
                                                        expected_random: self.expected_random,
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
                                                expected_random: self.expected_random,
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
                                        expected_random: self.expected_random,
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
                                expected_random: self.expected_random,
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
                    expected_random: &*expected_source.adapter,
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
        .expect("the named selection constructs the pairing")
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
    // proof_vectors_return_the_sampling_refusal_of_setup_whole
}

/// Contract: Issued credential, `deps.key_agreement.generate_keys`
///   fails → `Err(ProofVectorsErrorReturn::GenerateKeys(error))`,
///   `error` unchanged.
/// Arrange:  `build_create_key_agreement_params` overriding `concrete`
///   with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::NonceDrawRefused)`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused)))`
///   whole.
/// Collaborators: the key agreement's mock concrete refusing nonce
///   draws and the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, and delivery proof's mock
///   concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_nonce_draw_refusal_of_key_generation_whole() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
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
    let key_agreement_concrete = Some(KeyAgreementConcrete::Mock(
        MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
    ));
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
                                                            ProofVectorsErrorReturn::GenerateKeys(
                                                                GenerateKeysErrorReturn::Mock(
                                                                    MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused,
                                                                ),
                                                            )
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
        .expect("the named selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_nonce_draw_refusal_of_key_generation_whole
}

/// Contract: Issued credential, `deps.key_agreement.wrap_to` fails →
///   `Err(ProofVectorsErrorReturn::WrapTo(error))`, `error` unchanged.
/// Arrange:  `build_create_key_agreement_params` overriding `concrete`
///   with
///   `KeyAgreementConcrete::Mock(MockIKeyAgreementAdapterFailureMode::CoinDrawRefused)`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::WrapTo(WrapToErrorReturn::Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused)))`
///   whole.
/// Collaborators: the key agreement's mock concrete refusing coin
///   draws and the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, and delivery proof's mock
///   concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_coin_draw_refusal_of_wrapping_whole() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
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
    let key_agreement_concrete = Some(KeyAgreementConcrete::Mock(
        MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
    ));
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
                                                        Err(ProofVectorsErrorReturn::WrapTo(
                                                            WrapToErrorReturn::Mock(
                                                                MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused,
                                                            ),
                                                        ))
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
        .expect("the named selection constructs the key agreement")
        .output
        .expect("the default selection constructs the delivery proof");
    // proof_vectors_return_the_coin_draw_refusal_of_wrapping_whole
}

/// Contract: Mint record, `deps.delivery_proof.prove_mint` fails →
///   `Err(ProofVectorsErrorReturn::ProveMint(error))`, `error`
///   unchanged.
/// Arrange:  `build_create_delivery_proof_params` overriding `concrete`
///   with
///   `DeliveryProofConcrete::Mock(MockIDeliveryProofAdapterFailureMode::NonceDrawRefused)`.
/// Act:      `proof_vectors`, once, in the innermost consumer's method.
/// Assert:   in place, the result equals
///   `Err(ProofVectorsErrorReturn::ProveMint(ProveMintErrorReturn::Mock(MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused)))`
///   whole.
/// Collaborators: the delivery proof's mock concrete refusing nonce
///   draws and the random source's, pairing's, hash-to-scalar's,
///   encoding's, chain forms', KEM's, and key agreement's mock
///   concretes in their succeeding modes through
///   `create_random_source`, `create_pairing`, `create_hash_to_scalar`,
///   `create_encoding`, `create_chain_forms`, `create_kem`,
///   `create_key_agreement`, and `create_delivery_proof`.
#[test]
fn proof_vectors_return_the_nonce_draw_refusal_of_the_mint_prover_whole() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
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
    let delivery_proof_concrete = Some(DeliveryProofConcrete::Mock(
        MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
    ));

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
                                                        Err(ProofVectorsErrorReturn::ProveMint(
                                                            ProveMintErrorReturn::Mock(
                                                                MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused,
                                                            ),
                                                        ))
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
        .expect("the named selection constructs the delivery proof");
    // proof_vectors_return_the_nonce_draw_refusal_of_the_mint_prover_whole
}
