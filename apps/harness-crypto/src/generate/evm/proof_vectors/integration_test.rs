#![allow(clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;

use super::interface::*;
use super::mock::{
    ProofVectorContextOverrides, ProofVectorsParamsOverrides, ProofVectorsPayloadOverrides,
    build_proof_vector_context, build_proof_vectors_deps, build_proof_vectors_params,
    build_proof_vectors_payload,
};
use super::proof_vectors;

use super::super::challenge_vectors::provides::{
    CanonicalDeliveryContext, CanonicalDeliveryContextOverrides, build_canonical_delivery_context,
};
use super::super::group_vectors::provides::{
    VectorCount, VectorCountConstructorParamsOverrides, build_vector_count,
};
use super::super::render::provides::{
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, build_solidity_member_name,
    build_solidity_record_member, build_solidity_record_members,
};
use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    AssetIdentityHash, AssetIdentityHashConstructorParams,
    AssetIdentityHashConstructorParamsOverrides, ParameterSetIdentifier,
    ParameterSetIdentifierConstructorParams, ParameterSetIdentifierConstructorParamsOverrides,
    SuiteIdentifier, SuiteIdentifierConstructorParams, SuiteIdentifierConstructorParamsOverrides,
    build_asset_identity_hash, build_parameter_set_identifier, build_secret,
    build_suite_identifier,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, FromFieldParams, ICanonicalField,
    IDecoderAdapter, IEncoderAdapter, IEncodingConsumer, IEncodingContract, ToFieldParams,
    ToFieldsParams, build_create_encoding_deps, build_create_encoding_params,
    build_create_encoding_payload, build_decode_params, build_encode_params, create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, DECLARED_KEY_AGREEMENT_SELECTIONS,
    EnvelopeAlgebra, EnvelopeComponents, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, IKeyAgreementConsumer, KeyPairPublicKeysParams, KeyPairPublicKeysPayload,
    PublicKeysComponents, build_create_key_agreement_deps, build_create_key_agreement_payload,
    create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, IHashToScalarAdapter,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, DeriveIdentityParams,
    DeriveIdentityPayload, ICredentialKemAdapter, IKemConsumer, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, IdentityScope, KemIdentity, ParameterSetComponentsParams,
    ParameterSetComponentsPayload, SetupParams, SetupPayload, SetupScope, build_create_kem_deps,
    build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, DecodeG1Params, DecodeG2Params, DecodeScalarParams, EncodeG1Params,
    EncodeG1Payload, EncodeG2Params, EncodeG2Payload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    ISampleUniformScalar, PAIRING_CONCRETES, VerifierGroupArithmetic, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParamsOverrides,
    DELIVERY_PROOF_CONCRETES, DELIVERY_PURPOSE_GRANT, DELIVERY_PURPOSE_MINT,
    DELIVERY_PURPOSE_REPLACEMENT, DELIVERY_PURPOSE_TRANSFER, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryContext, DeliveryProofComponents, IDeliveryProofAdapter, IDeliveryProofConsumer,
    MintPurpose, MintResponses, MintSecondGroupFirstMessages, ProofFromComponentsParams,
    ProofFromComponentsPayload, TransferPurpose, TransferResponses,
    TransferSecondGroupFirstMessages, VerifyParams, VerifyPayload,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

/// Contract: `proof_vectors` over a case with a mint count above one, every
///   adapter real and the operating-system source real, yields mint records
///   each of which rebuilds, from its bytes alone, into a statement and a
///   proof `verify` accepts.
/// Arrange: `mint_count` of `2`, `transfer_count` of `1`, the entitlement
///   scope, and the contexts, purposes, and identity at their defaults;
///   every case, each member of `PAIRING_CONCRETES` crossed with the
///   selections the other families declare, so both curves and both verifier
///   forms run.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  `mint` holds `2` records and each record's rebuild yields
///   `is_valid` true.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_mint_record_is_a_statement_and_proof_the_delivery_proof_verifies() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(success.mint.len(), 2);
            for record in &success.mint {
                let suite_identifier = SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                    identifier: record.context.suite_identifier,
                    version: record.context.suite_version,
                })
                .expect("the record's suite identifier is admitted");
                let asset_identity_hash =
                    AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                        bytes: record.context.asset_identity_hash,
                    })
                    .expect("the record's asset identity hash is admitted");
                let parameter_set_identifier =
                    ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                        bytes: record.context.parameter_set,
                    })
                    .expect("the record's parameter-set identifier is admitted");
                let chain =
                    F::ChainIdentifier::from_field(FromFieldParams, record.context.chain.clone())
                        .map(|success| success.value)
                        .expect("the record's chain is a value its form admits");
                let entitlement_contract = F::Identity::from_field(
                    FromFieldParams,
                    record.context.entitlement_contract.clone(),
                )
                .map(|success| success.value)
                .expect("the record's entitlement contract is a value its form admits");
                let source_entitlement = F::Entitlement::from_field(
                    FromFieldParams,
                    record.context.source_entitlement.clone(),
                )
                .map(|success| success.value)
                .expect("the record's source entitlement is a value its form admits");
                let target_entitlement = F::Entitlement::from_field(
                    FromFieldParams,
                    record.context.target_entitlement.clone(),
                )
                .map(|success| success.value)
                .expect("the record's target entitlement is a value its form admits");
                let old_interval =
                    F::Interval::from_field(FromFieldParams, record.context.old_interval.clone())
                        .map(|success| success.value)
                        .expect("the record's old interval is a value its form admits");
                let new_interval =
                    F::Interval::from_field(FromFieldParams, record.context.new_interval.clone())
                        .map(|success| success.value)
                        .expect("the record's new interval is a value its form admits");
                let seller =
                    F::Identity::from_field(FromFieldParams, record.context.seller.clone())
                        .map(|success| success.value)
                        .expect("the record's seller is a value its form admits");
                let buyer = F::Identity::from_field(FromFieldParams, record.context.buyer.clone())
                    .map(|success| success.value)
                    .expect("the record's buyer is a value its form admits");
                let purpose = MintPurpose::try_from(record.context.purpose)
                    .expect("the record's purpose code is declared");
                let statement = AlgebraicMintStatement {
                    context: DeliveryContext {
                        suite_identifier,
                        chain,
                        entitlement_contract,
                        asset_identity_hash,
                        parameter_set_identifier,
                        source_entitlement,
                        target_entitlement,
                        old_interval,
                        new_interval,
                        purpose,
                        seller,
                        buyer,
                        expiry: record.context.expiry,
                    },
                    hpub: self
                        .pairing
                        .decode_g2(DecodeG2Params, &record.hpub)
                        .expect("the record's hpub decodes")
                        .point,
                    identity_element: self
                        .pairing
                        .decode_g1(DecodeG1Params, &record.identity_element)
                        .expect("the record's identity element decodes")
                        .point,
                    buyer_keys: PublicKeysComponents {
                        pk1: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.buyer_pk1)
                            .expect("the record's buyer pk1 decodes")
                            .point,
                        pk2: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.buyer_pk2)
                            .expect("the record's buyer pk2 decodes")
                            .point,
                    },
                    envelope: EnvelopeComponents {
                        c1: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.envelope_c1)
                            .expect("the record's envelope c1 decodes")
                            .point,
                        c2: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.envelope_c2)
                            .expect("the record's envelope c2 decodes")
                            .point,
                        d1: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.envelope_d1)
                            .expect("the record's envelope d1 decodes")
                            .point,
                        d2: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.envelope_d2)
                            .expect("the record's envelope d2 decodes")
                            .point,
                    },
                };
                let challenge = self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &record.challenge)
                    .expect("the record's challenge decodes")
                    .scalar;
                let responses = MintResponses {
                    alpha: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.alpha)
                        .expect("the record's alpha decodes")
                        .scalar,
                    r: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.r)
                        .expect("the record's r decodes")
                        .scalar,
                    rho: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.rho)
                        .expect("the record's rho decodes")
                        .scalar,
                    sigma: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.sigma)
                        .expect("the record's sigma decodes")
                        .scalar,
                };
                let components = if record.second_group_first_messages.is_empty() {
                    DeliveryProofComponents::MintBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let length = record.second_group_first_messages.len() / 3;
                    DeliveryProofComponents::MintFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: MintSecondGroupFirstMessages {
                            hpub: self
                                .pairing
                                .decode_g2(
                                    DecodeG2Params,
                                    &record.second_group_first_messages[..length],
                                )
                                .expect("the record's first message hpub decodes")
                                .point,
                            d1: self
                                .pairing
                                .decode_g2(
                                    DecodeG2Params,
                                    &record.second_group_first_messages[length..length * 2],
                                )
                                .expect("the record's first message d1 decodes")
                                .point,
                            d2: self
                                .pairing
                                .decode_g2(
                                    DecodeG2Params,
                                    &record.second_group_first_messages[length * 2..],
                                )
                                .expect("the record's first message d2 decodes")
                                .point,
                        },
                    }
                };
                let proof = payload
                    .adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the record's components form a proof")
                    .proof;
                let verdict = payload
                    .adapter
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Mint(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the record's proof verifies without refusal");
                assert!(verdict.is_valid);
            }
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: Some(IdentityScope::Entitlement),
        mint: &mint,
        transfer: &transfer,
        identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: `proof_vectors` over a case with a transfer count above one,
///   every adapter real and the operating-system source real, yields transfer
///   records each of which rebuilds, from its bytes alone, into a statement
///   and a proof `verify` accepts.
/// Arrange: `mint_count` of `1`, `transfer_count` of `2`, the entitlement
///   scope, and the contexts, purposes, and identity at their defaults;
///   every case.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  `transfer` holds `2` records and each record's rebuild yields
///   `is_valid` true, the old envelope, the new envelope, the seller's keys,
///   and the buyer's keys each taken from the record.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_transfer_record_is_a_statement_and_proof_the_delivery_proof_verifies() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = b"pkg@1.0.0#1".as_slice();

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            assert_eq!(success.transfer.len(), 2);
            for record in &success.transfer {
                let suite_identifier = SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                    identifier: record.context.suite_identifier,
                    version: record.context.suite_version,
                })
                .expect("the record's suite identifier is admitted");
                let asset_identity_hash =
                    AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                        bytes: record.context.asset_identity_hash,
                    })
                    .expect("the record's asset identity hash is admitted");
                let parameter_set_identifier =
                    ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                        bytes: record.context.parameter_set,
                    })
                    .expect("the record's parameter-set identifier is admitted");
                let chain =
                    F::ChainIdentifier::from_field(FromFieldParams, record.context.chain.clone())
                        .map(|success| success.value)
                        .expect("the record's chain is a value its form admits");
                let entitlement_contract = F::Identity::from_field(
                    FromFieldParams,
                    record.context.entitlement_contract.clone(),
                )
                .map(|success| success.value)
                .expect("the record's entitlement contract is a value its form admits");
                let source_entitlement = F::Entitlement::from_field(
                    FromFieldParams,
                    record.context.source_entitlement.clone(),
                )
                .map(|success| success.value)
                .expect("the record's source entitlement is a value its form admits");
                let target_entitlement = F::Entitlement::from_field(
                    FromFieldParams,
                    record.context.target_entitlement.clone(),
                )
                .map(|success| success.value)
                .expect("the record's target entitlement is a value its form admits");
                let old_interval =
                    F::Interval::from_field(FromFieldParams, record.context.old_interval.clone())
                        .map(|success| success.value)
                        .expect("the record's old interval is a value its form admits");
                let new_interval =
                    F::Interval::from_field(FromFieldParams, record.context.new_interval.clone())
                        .map(|success| success.value)
                        .expect("the record's new interval is a value its form admits");
                let seller =
                    F::Identity::from_field(FromFieldParams, record.context.seller.clone())
                        .map(|success| success.value)
                        .expect("the record's seller is a value its form admits");
                let buyer = F::Identity::from_field(FromFieldParams, record.context.buyer.clone())
                    .map(|success| success.value)
                    .expect("the record's buyer is a value its form admits");
                let purpose = TransferPurpose::try_from(record.context.purpose)
                    .expect("the record's purpose code is declared");
                let statement = AlgebraicTransferStatement {
                    context: DeliveryContext {
                        suite_identifier,
                        chain,
                        entitlement_contract,
                        asset_identity_hash,
                        parameter_set_identifier,
                        source_entitlement,
                        target_entitlement,
                        old_interval,
                        new_interval,
                        purpose,
                        seller,
                        buyer,
                        expiry: record.context.expiry,
                    },
                    identity_element: self
                        .pairing
                        .decode_g1(DecodeG1Params, &record.identity_element)
                        .expect("the record's identity element decodes")
                        .point,
                    seller_keys: PublicKeysComponents {
                        pk1: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.seller_pk1)
                            .expect("the record's seller pk1 decodes")
                            .point,
                        pk2: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.seller_pk2)
                            .expect("the record's seller pk2 decodes")
                            .point,
                    },
                    buyer_keys: PublicKeysComponents {
                        pk1: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.buyer_pk1)
                            .expect("the record's buyer pk1 decodes")
                            .point,
                        pk2: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.buyer_pk2)
                            .expect("the record's buyer pk2 decodes")
                            .point,
                    },
                    old_envelope: EnvelopeComponents {
                        c1: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.old_envelope_c1)
                            .expect("the record's old envelope c1 decodes")
                            .point,
                        c2: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.old_envelope_c2)
                            .expect("the record's old envelope c2 decodes")
                            .point,
                        d1: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.old_envelope_d1)
                            .expect("the record's old envelope d1 decodes")
                            .point,
                        d2: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.old_envelope_d2)
                            .expect("the record's old envelope d2 decodes")
                            .point,
                    },
                    new_envelope: EnvelopeComponents {
                        c1: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.new_envelope_c1)
                            .expect("the record's new envelope c1 decodes")
                            .point,
                        c2: self
                            .pairing
                            .decode_g1(DecodeG1Params, &record.new_envelope_c2)
                            .expect("the record's new envelope c2 decodes")
                            .point,
                        d1: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.new_envelope_d1)
                            .expect("the record's new envelope d1 decodes")
                            .point,
                        d2: self
                            .pairing
                            .decode_g2(DecodeG2Params, &record.new_envelope_d2)
                            .expect("the record's new envelope d2 decodes")
                            .point,
                    },
                };
                let challenge = self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &record.challenge)
                    .expect("the record's challenge decodes")
                    .scalar;
                let responses = TransferResponses {
                    x: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.x)
                        .expect("the record's x decodes")
                        .scalar,
                    y: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.y)
                        .expect("the record's y decodes")
                        .scalar,
                    s: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.s)
                        .expect("the record's s decodes")
                        .scalar,
                    rho: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.rho)
                        .expect("the record's rho decodes")
                        .scalar,
                    sigma: self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &record.sigma)
                        .expect("the record's sigma decodes")
                        .scalar,
                };
                let components = if record.second_group_first_messages.is_empty() {
                    DeliveryProofComponents::TransferBothGroups {
                        challenge,
                        responses,
                    }
                } else {
                    let length = record.second_group_first_messages.len() / 3;
                    DeliveryProofComponents::TransferFirstGroupOnly {
                        challenge,
                        responses,
                        first_messages: TransferSecondGroupFirstMessages {
                            pk2: self
                                .pairing
                                .decode_g2(
                                    DecodeG2Params,
                                    &record.second_group_first_messages[..length],
                                )
                                .expect("the record's first message pk2 decodes")
                                .point,
                            d1: self
                                .pairing
                                .decode_g2(
                                    DecodeG2Params,
                                    &record.second_group_first_messages[length..length * 2],
                                )
                                .expect("the record's first message d1 decodes")
                                .point,
                            d2: self
                                .pairing
                                .decode_g2(
                                    DecodeG2Params,
                                    &record.second_group_first_messages[length * 2..],
                                )
                                .expect("the record's first message d2 decodes")
                                .point,
                        },
                    }
                };
                let proof = payload
                    .adapter
                    .proof_from_components(
                        ProofFromComponentsParams,
                        ProofFromComponentsPayload { components },
                    )
                    .expect("the record's components form a proof")
                    .proof;
                let verdict = payload
                    .adapter
                    .verify(
                        VerifyParams,
                        VerifyPayload {
                            statement: AlgebraicStatement::Transfer(&statement),
                            proof: &proof,
                        },
                    )
                    .expect("the record's proof verifies without refusal");
                assert!(verdict.is_valid);
            }
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: Some(IdentityScope::Entitlement),
        mint: &mint,
        transfer: &transfer,
        identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(2),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: `proof_vectors` under the asset scope over a 32-byte identity
///   yields `Ok`, every record rebuilt and verified.
/// Arrange: `scope` `IdentityScope::Asset`, the identity `vec![0x44; 32]`,
///   `mint_count` and `transfer_count` of `1`; every case that declares the
///   asset scope.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  the result is `Ok` and the mint record's and the transfer
///   record's rebuilds each yield `is_valid` true.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn an_asset_identity_is_proved_under_the_asset_scope() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let mint_record = &success.mint[0];
            let suite_identifier = SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                identifier: mint_record.context.suite_identifier,
                version: mint_record.context.suite_version,
            })
            .expect("the record's suite identifier is admitted");
            let asset_identity_hash =
                AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                    bytes: mint_record.context.asset_identity_hash,
                })
                .expect("the record's asset identity hash is admitted");
            let parameter_set_identifier =
                ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                    bytes: mint_record.context.parameter_set,
                })
                .expect("the record's parameter-set identifier is admitted");
            let chain =
                F::ChainIdentifier::from_field(FromFieldParams, mint_record.context.chain.clone())
                    .map(|success| success.value)
                    .expect("the record's chain is a value its form admits");
            let entitlement_contract = F::Identity::from_field(
                FromFieldParams,
                mint_record.context.entitlement_contract.clone(),
            )
            .map(|success| success.value)
            .expect("the record's entitlement contract is a value its form admits");
            let source_entitlement = F::Entitlement::from_field(
                FromFieldParams,
                mint_record.context.source_entitlement.clone(),
            )
            .map(|success| success.value)
            .expect("the record's source entitlement is a value its form admits");
            let target_entitlement = F::Entitlement::from_field(
                FromFieldParams,
                mint_record.context.target_entitlement.clone(),
            )
            .map(|success| success.value)
            .expect("the record's target entitlement is a value its form admits");
            let old_interval =
                F::Interval::from_field(FromFieldParams, mint_record.context.old_interval.clone())
                    .map(|success| success.value)
                    .expect("the record's old interval is a value its form admits");
            let new_interval =
                F::Interval::from_field(FromFieldParams, mint_record.context.new_interval.clone())
                    .map(|success| success.value)
                    .expect("the record's new interval is a value its form admits");
            let seller =
                F::Identity::from_field(FromFieldParams, mint_record.context.seller.clone())
                    .map(|success| success.value)
                    .expect("the record's seller is a value its form admits");
            let buyer = F::Identity::from_field(FromFieldParams, mint_record.context.buyer.clone())
                .map(|success| success.value)
                .expect("the record's buyer is a value its form admits");
            let mint_purpose = MintPurpose::try_from(mint_record.context.purpose)
                .expect("the record's purpose code is declared");
            let mint_statement = AlgebraicMintStatement {
                context: DeliveryContext {
                    suite_identifier,
                    chain,
                    entitlement_contract,
                    asset_identity_hash,
                    parameter_set_identifier,
                    source_entitlement,
                    target_entitlement,
                    old_interval,
                    new_interval,
                    purpose: mint_purpose,
                    seller,
                    buyer,
                    expiry: mint_record.context.expiry,
                },
                hpub: self
                    .pairing
                    .decode_g2(DecodeG2Params, &mint_record.hpub)
                    .expect("the record's hpub decodes")
                    .point,
                identity_element: self
                    .pairing
                    .decode_g1(DecodeG1Params, &mint_record.identity_element)
                    .expect("the record's identity element decodes")
                    .point,
                buyer_keys: PublicKeysComponents {
                    pk1: self
                        .pairing
                        .decode_g1(DecodeG1Params, &mint_record.buyer_pk1)
                        .expect("the record's buyer pk1 decodes")
                        .point,
                    pk2: self
                        .pairing
                        .decode_g2(DecodeG2Params, &mint_record.buyer_pk2)
                        .expect("the record's buyer pk2 decodes")
                        .point,
                },
                envelope: EnvelopeComponents {
                    c1: self
                        .pairing
                        .decode_g1(DecodeG1Params, &mint_record.envelope_c1)
                        .expect("the record's envelope c1 decodes")
                        .point,
                    c2: self
                        .pairing
                        .decode_g1(DecodeG1Params, &mint_record.envelope_c2)
                        .expect("the record's envelope c2 decodes")
                        .point,
                    d1: self
                        .pairing
                        .decode_g2(DecodeG2Params, &mint_record.envelope_d1)
                        .expect("the record's envelope d1 decodes")
                        .point,
                    d2: self
                        .pairing
                        .decode_g2(DecodeG2Params, &mint_record.envelope_d2)
                        .expect("the record's envelope d2 decodes")
                        .point,
                },
            };
            let mint_challenge = self
                .pairing
                .decode_scalar(DecodeScalarParams, &mint_record.challenge)
                .expect("the record's challenge decodes")
                .scalar;
            let mint_responses = MintResponses {
                alpha: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &mint_record.alpha)
                    .expect("the record's alpha decodes")
                    .scalar,
                r: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &mint_record.r)
                    .expect("the record's r decodes")
                    .scalar,
                rho: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &mint_record.rho)
                    .expect("the record's rho decodes")
                    .scalar,
                sigma: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &mint_record.sigma)
                    .expect("the record's sigma decodes")
                    .scalar,
            };
            let mint_components = if mint_record.second_group_first_messages.is_empty() {
                DeliveryProofComponents::MintBothGroups {
                    challenge: mint_challenge,
                    responses: mint_responses,
                }
            } else {
                let length = mint_record.second_group_first_messages.len() / 3;
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge: mint_challenge,
                    responses: mint_responses,
                    first_messages: MintSecondGroupFirstMessages {
                        hpub: self
                            .pairing
                            .decode_g2(
                                DecodeG2Params,
                                &mint_record.second_group_first_messages[..length],
                            )
                            .expect("the record's first message hpub decodes")
                            .point,
                        d1: self
                            .pairing
                            .decode_g2(
                                DecodeG2Params,
                                &mint_record.second_group_first_messages[length..length * 2],
                            )
                            .expect("the record's first message d1 decodes")
                            .point,
                        d2: self
                            .pairing
                            .decode_g2(
                                DecodeG2Params,
                                &mint_record.second_group_first_messages[length * 2..],
                            )
                            .expect("the record's first message d2 decodes")
                            .point,
                    },
                }
            };
            let mint_proof = payload
                .adapter
                .proof_from_components(
                    ProofFromComponentsParams,
                    ProofFromComponentsPayload {
                        components: mint_components,
                    },
                )
                .expect("the record's components form a proof")
                .proof;
            let mint_verdict = payload
                .adapter
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Mint(&mint_statement),
                        proof: &mint_proof,
                    },
                )
                .expect("the record's proof verifies without refusal");
            assert!(mint_verdict.is_valid);

            let transfer_record = &success.transfer[0];
            let suite_identifier = SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                identifier: transfer_record.context.suite_identifier,
                version: transfer_record.context.suite_version,
            })
            .expect("the record's suite identifier is admitted");
            let asset_identity_hash =
                AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                    bytes: transfer_record.context.asset_identity_hash,
                })
                .expect("the record's asset identity hash is admitted");
            let parameter_set_identifier =
                ParameterSetIdentifier::try_new(ParameterSetIdentifierConstructorParams {
                    bytes: transfer_record.context.parameter_set,
                })
                .expect("the record's parameter-set identifier is admitted");
            let chain = F::ChainIdentifier::from_field(
                FromFieldParams,
                transfer_record.context.chain.clone(),
            )
            .map(|success| success.value)
            .expect("the record's chain is a value its form admits");
            let entitlement_contract = F::Identity::from_field(
                FromFieldParams,
                transfer_record.context.entitlement_contract.clone(),
            )
            .map(|success| success.value)
            .expect("the record's entitlement contract is a value its form admits");
            let source_entitlement = F::Entitlement::from_field(
                FromFieldParams,
                transfer_record.context.source_entitlement.clone(),
            )
            .map(|success| success.value)
            .expect("the record's source entitlement is a value its form admits");
            let target_entitlement = F::Entitlement::from_field(
                FromFieldParams,
                transfer_record.context.target_entitlement.clone(),
            )
            .map(|success| success.value)
            .expect("the record's target entitlement is a value its form admits");
            let old_interval = F::Interval::from_field(
                FromFieldParams,
                transfer_record.context.old_interval.clone(),
            )
            .map(|success| success.value)
            .expect("the record's old interval is a value its form admits");
            let new_interval = F::Interval::from_field(
                FromFieldParams,
                transfer_record.context.new_interval.clone(),
            )
            .map(|success| success.value)
            .expect("the record's new interval is a value its form admits");
            let seller =
                F::Identity::from_field(FromFieldParams, transfer_record.context.seller.clone())
                    .map(|success| success.value)
                    .expect("the record's seller is a value its form admits");
            let buyer =
                F::Identity::from_field(FromFieldParams, transfer_record.context.buyer.clone())
                    .map(|success| success.value)
                    .expect("the record's buyer is a value its form admits");
            let transfer_purpose = TransferPurpose::try_from(transfer_record.context.purpose)
                .expect("the record's purpose code is declared");
            let transfer_statement = AlgebraicTransferStatement {
                context: DeliveryContext {
                    suite_identifier,
                    chain,
                    entitlement_contract,
                    asset_identity_hash,
                    parameter_set_identifier,
                    source_entitlement,
                    target_entitlement,
                    old_interval,
                    new_interval,
                    purpose: transfer_purpose,
                    seller,
                    buyer,
                    expiry: transfer_record.context.expiry,
                },
                identity_element: self
                    .pairing
                    .decode_g1(DecodeG1Params, &transfer_record.identity_element)
                    .expect("the record's identity element decodes")
                    .point,
                seller_keys: PublicKeysComponents {
                    pk1: self
                        .pairing
                        .decode_g1(DecodeG1Params, &transfer_record.seller_pk1)
                        .expect("the record's seller pk1 decodes")
                        .point,
                    pk2: self
                        .pairing
                        .decode_g2(DecodeG2Params, &transfer_record.seller_pk2)
                        .expect("the record's seller pk2 decodes")
                        .point,
                },
                buyer_keys: PublicKeysComponents {
                    pk1: self
                        .pairing
                        .decode_g1(DecodeG1Params, &transfer_record.buyer_pk1)
                        .expect("the record's buyer pk1 decodes")
                        .point,
                    pk2: self
                        .pairing
                        .decode_g2(DecodeG2Params, &transfer_record.buyer_pk2)
                        .expect("the record's buyer pk2 decodes")
                        .point,
                },
                old_envelope: EnvelopeComponents {
                    c1: self
                        .pairing
                        .decode_g1(DecodeG1Params, &transfer_record.old_envelope_c1)
                        .expect("the record's old envelope c1 decodes")
                        .point,
                    c2: self
                        .pairing
                        .decode_g1(DecodeG1Params, &transfer_record.old_envelope_c2)
                        .expect("the record's old envelope c2 decodes")
                        .point,
                    d1: self
                        .pairing
                        .decode_g2(DecodeG2Params, &transfer_record.old_envelope_d1)
                        .expect("the record's old envelope d1 decodes")
                        .point,
                    d2: self
                        .pairing
                        .decode_g2(DecodeG2Params, &transfer_record.old_envelope_d2)
                        .expect("the record's old envelope d2 decodes")
                        .point,
                },
                new_envelope: EnvelopeComponents {
                    c1: self
                        .pairing
                        .decode_g1(DecodeG1Params, &transfer_record.new_envelope_c1)
                        .expect("the record's new envelope c1 decodes")
                        .point,
                    c2: self
                        .pairing
                        .decode_g1(DecodeG1Params, &transfer_record.new_envelope_c2)
                        .expect("the record's new envelope c2 decodes")
                        .point,
                    d1: self
                        .pairing
                        .decode_g2(DecodeG2Params, &transfer_record.new_envelope_d1)
                        .expect("the record's new envelope d1 decodes")
                        .point,
                    d2: self
                        .pairing
                        .decode_g2(DecodeG2Params, &transfer_record.new_envelope_d2)
                        .expect("the record's new envelope d2 decodes")
                        .point,
                },
            };
            let transfer_challenge = self
                .pairing
                .decode_scalar(DecodeScalarParams, &transfer_record.challenge)
                .expect("the record's challenge decodes")
                .scalar;
            let transfer_responses = TransferResponses {
                x: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &transfer_record.x)
                    .expect("the record's x decodes")
                    .scalar,
                y: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &transfer_record.y)
                    .expect("the record's y decodes")
                    .scalar,
                s: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &transfer_record.s)
                    .expect("the record's s decodes")
                    .scalar,
                rho: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &transfer_record.rho)
                    .expect("the record's rho decodes")
                    .scalar,
                sigma: self
                    .pairing
                    .decode_scalar(DecodeScalarParams, &transfer_record.sigma)
                    .expect("the record's sigma decodes")
                    .scalar,
            };
            let transfer_components = if transfer_record.second_group_first_messages.is_empty() {
                DeliveryProofComponents::TransferBothGroups {
                    challenge: transfer_challenge,
                    responses: transfer_responses,
                }
            } else {
                let length = transfer_record.second_group_first_messages.len() / 3;
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge: transfer_challenge,
                    responses: transfer_responses,
                    first_messages: TransferSecondGroupFirstMessages {
                        pk2: self
                            .pairing
                            .decode_g2(
                                DecodeG2Params,
                                &transfer_record.second_group_first_messages[..length],
                            )
                            .expect("the record's first message pk2 decodes")
                            .point,
                        d1: self
                            .pairing
                            .decode_g2(
                                DecodeG2Params,
                                &transfer_record.second_group_first_messages[length..length * 2],
                            )
                            .expect("the record's first message d1 decodes")
                            .point,
                        d2: self
                            .pairing
                            .decode_g2(
                                DecodeG2Params,
                                &transfer_record.second_group_first_messages[length * 2..],
                            )
                            .expect("the record's first message d2 decodes")
                            .point,
                    },
                }
            };
            let transfer_proof = payload
                .adapter
                .proof_from_components(
                    ProofFromComponentsParams,
                    ProofFromComponentsPayload {
                        components: transfer_components,
                    },
                )
                .expect("the record's components form a proof")
                .proof;
            let transfer_verdict = payload
                .adapter
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Transfer(&transfer_statement),
                        proof: &transfer_proof,
                    },
                )
                .expect("the record's proof verifies without refusal");
            assert!(transfer_verdict.is_valid);
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: Some(IdentityScope::Asset),
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: each record's `draws`, split at `UNIFORM_BYTES_LENGTH` into the
///   uniforms of its calls in draw order and replayed through `setup`,
///   `derive_identity`, and `generate_keys` on the case's adapters, reproduce
///   the record's `hpub`, `identity_element`, and key fields.
/// Arrange: `mint_count` and `transfer_count` of `1`, the entitlement scope,
///   each case run under each of the identities `b"pkg@1.0.0#1"` and
///   `b"pkg@2.0.0#7"`, so a replay over an identity other than the configured
///   one reports otherwise.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  the replayed parameter set's `hpub`, the replayed identity
///   element's `element`, and each replayed key pair's `pk1` and `pk2`, each
///   encoded through the received pairing, equal the mint record's `hpub`,
///   `identity_element`, `buyer_pk1`, and `buyer_pk2` and the transfer
///   record's `identity_element`, `seller_pk1`, `seller_pk2`, `buyer_pk1`,
///   and `buyer_pk2`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_draw_replays_to_the_statement_elements_it_produced() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let mint_record = &success.mint[0];
            let asset_hash = match self.scope {
                IdentityScope::Asset => Some(
                    AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                        bytes: <[u8; 32]>::try_from(self.identity)
                            .expect("the configured identity is 32 bytes"),
                    })
                    .expect("the configured identity is an asset identity hash"),
                ),
                IdentityScope::Entitlement => None,
            };
            let setup_scope = match &asset_hash {
                Some(hash) => SetupScope::Asset {
                    identity_hash: hash,
                },
                None => SetupScope::Entitlement,
            };
            let kem_identity = match &asset_hash {
                Some(hash) => KemIdentity::Asset {
                    identity_hash: hash,
                },
                None => KemIdentity::Entitlement {
                    canonical: self.identity,
                },
            };
            let replayed_parameter_set = self
                .kem
                .setup(
                    SetupParams { scope: setup_scope },
                    SetupPayload {
                        master_uniform: build_secret(
                            mint_record.draws[..length].to_vec(),
                            Default::default(),
                        ),
                        u0_uniform: build_secret(
                            mint_record.draws[length..length * 2].to_vec(),
                            Default::default(),
                        ),
                        u1_uniform: build_secret(
                            mint_record.draws[length * 2..length * 3].to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the recorded uniforms set the parameter set")
                .parameter_set;
            let replayed_identity_element = self
                .kem
                .derive_identity(
                    DeriveIdentityParams,
                    DeriveIdentityPayload {
                        parameter_set: &replayed_parameter_set,
                        identity: kem_identity,
                    },
                )
                .expect("the recorded parameter set derives the identity")
                .identity_element;
            let replayed_buyer = self
                .key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: build_secret(
                            mint_record.draws[length * 4..length * 5].to_vec(),
                            Default::default(),
                        ),
                        y_uniform: build_secret(
                            mint_record.draws[length * 5..length * 6].to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the recorded uniforms generate the buyer keys")
                .key_pair;
            let replayed_buyer_keys = self
                .key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &replayed_buyer,
                    },
                )
                .expect("the replayed buyer key pair yields its public keys")
                .components;
            let replayed_parameter_set_components = self
                .kem
                .parameter_set_components(
                    ParameterSetComponentsParams,
                    ParameterSetComponentsPayload {
                        parameter_set: &replayed_parameter_set,
                    },
                )
                .expect("the replayed parameter set yields its components")
                .components;
            let replayed_element_components = self
                .kem
                .identity_element_components(
                    IdentityElementComponentsParams,
                    IdentityElementComponentsPayload {
                        identity_element: &replayed_identity_element,
                    },
                )
                .expect("the replayed identity element yields its components")
                .components;
            assert_eq!(
                self.pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: replayed_parameter_set_components.hpub,
                        },
                    )
                    .expect("the replayed hpub encodes")
                    .bytes
                    .as_ref(),
                mint_record.hpub.as_slice(),
            );
            assert_eq!(
                self.pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: replayed_element_components.element,
                        },
                    )
                    .expect("the replayed identity element encodes")
                    .bytes
                    .as_ref(),
                mint_record.identity_element.as_slice(),
            );
            assert_eq!(
                self.pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: replayed_buyer_keys.pk1,
                        },
                    )
                    .expect("the replayed buyer pk1 encodes")
                    .bytes
                    .as_ref(),
                mint_record.buyer_pk1.as_slice(),
            );
            assert_eq!(
                self.pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: replayed_buyer_keys.pk2,
                        },
                    )
                    .expect("the replayed buyer pk2 encodes")
                    .bytes
                    .as_ref(),
                mint_record.buyer_pk2.as_slice(),
            );

            let transfer_record = &success.transfer[0];
            let asset_hash = match self.scope {
                IdentityScope::Asset => Some(
                    AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                        bytes: <[u8; 32]>::try_from(self.identity)
                            .expect("the configured identity is 32 bytes"),
                    })
                    .expect("the configured identity is an asset identity hash"),
                ),
                IdentityScope::Entitlement => None,
            };
            let setup_scope = match &asset_hash {
                Some(hash) => SetupScope::Asset {
                    identity_hash: hash,
                },
                None => SetupScope::Entitlement,
            };
            let kem_identity = match &asset_hash {
                Some(hash) => KemIdentity::Asset {
                    identity_hash: hash,
                },
                None => KemIdentity::Entitlement {
                    canonical: self.identity,
                },
            };
            let replayed_parameter_set = self
                .kem
                .setup(
                    SetupParams { scope: setup_scope },
                    SetupPayload {
                        master_uniform: build_secret(
                            transfer_record.draws[..length].to_vec(),
                            Default::default(),
                        ),
                        u0_uniform: build_secret(
                            transfer_record.draws[length..length * 2].to_vec(),
                            Default::default(),
                        ),
                        u1_uniform: build_secret(
                            transfer_record.draws[length * 2..length * 3].to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the recorded uniforms set the parameter set")
                .parameter_set;
            let replayed_identity_element = self
                .kem
                .derive_identity(
                    DeriveIdentityParams,
                    DeriveIdentityPayload {
                        parameter_set: &replayed_parameter_set,
                        identity: kem_identity,
                    },
                )
                .expect("the recorded parameter set derives the identity")
                .identity_element;
            let replayed_seller = self
                .key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: build_secret(
                            transfer_record.draws[length * 4..length * 5].to_vec(),
                            Default::default(),
                        ),
                        y_uniform: build_secret(
                            transfer_record.draws[length * 5..length * 6].to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the recorded uniforms generate the seller keys")
                .key_pair;
            let replayed_seller_keys = self
                .key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &replayed_seller,
                    },
                )
                .expect("the replayed seller key pair yields its public keys")
                .components;
            let replayed_buyer = self
                .key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: build_secret(
                            transfer_record.draws[length * 6..length * 7].to_vec(),
                            Default::default(),
                        ),
                        y_uniform: build_secret(
                            transfer_record.draws[length * 7..length * 8].to_vec(),
                            Default::default(),
                        ),
                    },
                )
                .expect("the recorded uniforms generate the buyer keys")
                .key_pair;
            let replayed_buyer_keys = self
                .key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &replayed_buyer,
                    },
                )
                .expect("the replayed buyer key pair yields its public keys")
                .components;
            let replayed_element_components = self
                .kem
                .identity_element_components(
                    IdentityElementComponentsParams,
                    IdentityElementComponentsPayload {
                        identity_element: &replayed_identity_element,
                    },
                )
                .expect("the replayed identity element yields its components")
                .components;
            assert_eq!(
                self.pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: replayed_element_components.element,
                        },
                    )
                    .expect("the replayed identity element encodes")
                    .bytes
                    .as_ref(),
                transfer_record.identity_element.as_slice(),
            );
            assert_eq!(
                self.pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: replayed_seller_keys.pk1,
                        },
                    )
                    .expect("the replayed seller pk1 encodes")
                    .bytes
                    .as_ref(),
                transfer_record.seller_pk1.as_slice(),
            );
            assert_eq!(
                self.pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: replayed_seller_keys.pk2,
                        },
                    )
                    .expect("the replayed seller pk2 encodes")
                    .bytes
                    .as_ref(),
                transfer_record.seller_pk2.as_slice(),
            );
            assert_eq!(
                self.pairing
                    .encode_g1(
                        EncodeG1Params,
                        EncodeG1Payload {
                            point: replayed_buyer_keys.pk1,
                        },
                    )
                    .expect("the replayed buyer pk1 encodes")
                    .bytes
                    .as_ref(),
                transfer_record.buyer_pk1.as_slice(),
            );
            assert_eq!(
                self.pairing
                    .encode_g2(
                        EncodeG2Params,
                        EncodeG2Payload {
                            point: replayed_buyer_keys.pk2,
                        },
                    )
                    .expect("the replayed buyer pk2 encodes")
                    .bytes
                    .as_ref(),
                transfer_record.buyer_pk2.as_slice(),
            );
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    // Act & Assert per identity, each running every case
    for identity in [b"pkg@1.0.0#1".as_slice(), b"pkg@2.0.0#7".as_slice()] {
        let consumer = PairingConsumer {
            random: &*source,
            kem_scope: Some(IdentityScope::Entitlement),
            mint: &mint,
            transfer: &transfer,
            identity,
            mint_count: build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(1),
            }),
            transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
                count: Some(1),
            }),
            mint_purpose: DELIVERY_PURPOSE_MINT,
            transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
        };
        for member in PAIRING_CONCRETES {
            let created = create_pairing(
                &build_create_pairing_deps(&consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*member),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );
            assert!(created.is_ok(), "the pairing factory ran the consumer");
        }
    }
}

/// Contract: under a pairing whose declaration names the first-group-only
///   verifier form each record's `second_group_first_messages` holds three
///   second-group point encodings and the record with one byte of them
///   changed is refused; under both groups it is empty.
/// Arrange: `mint_count` and `transfer_count` of `1`; every member of
///   `PAIRING_CONCRETES`, which holds both forms, with
///   `build_create_delivery_proof_params` overriding `verifier_form` with the
///   form the pairing's declaration names.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  under the first-group-only form each record's
///   `second_group_first_messages` is as long as `3` times the `encode_g2` of
///   the second-group generator and its rebuild with one byte changed is
///   refused; under both groups it equals `Vec::new()`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_carries_second_group_first_messages_exactly_in_the_first_group_only_form() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let Ok(g2_generator) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(reference_g2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: g2_generator.point,
                },
            );
            let g2_width = reference_g2.bytes.as_ref().len();
            let mint_record = &success.mint[0];
            let transfer_record = &success.transfer[0];
            match P::DECLARATION.verifier_group_arithmetic {
                VerifierGroupArithmetic::BothGroups => {
                    assert_eq!(mint_record.second_group_first_messages, Vec::<u8>::new());
                    assert_eq!(
                        transfer_record.second_group_first_messages,
                        Vec::<u8>::new()
                    );
                }
                VerifierGroupArithmetic::FirstGroupOnly => {
                    assert_eq!(mint_record.second_group_first_messages.len(), 3 * g2_width);
                    assert_eq!(
                        transfer_record.second_group_first_messages.len(),
                        3 * g2_width
                    );

                    let length = mint_record.second_group_first_messages.len() / 3;
                    let mint_challenge = self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &mint_record.challenge)
                        .expect("the record's challenge decodes")
                        .scalar;
                    let mint_responses = MintResponses {
                        alpha: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &mint_record.alpha)
                            .expect("the record's alpha decodes")
                            .scalar,
                        r: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &mint_record.r)
                            .expect("the record's r decodes")
                            .scalar,
                        rho: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &mint_record.rho)
                            .expect("the record's rho decodes")
                            .scalar,
                        sigma: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &mint_record.sigma)
                            .expect("the record's sigma decodes")
                            .scalar,
                    };
                    let mut corrupted = mint_record.second_group_first_messages.clone();
                    corrupted[0] ^= 0x01;
                    let mint_refused = match (
                        self.pairing.decode_g2(DecodeG2Params, &corrupted[..length]),
                        self.pairing
                            .decode_g2(DecodeG2Params, &corrupted[length..length * 2]),
                        self.pairing
                            .decode_g2(DecodeG2Params, &corrupted[length * 2..]),
                    ) {
                        (Ok(hpub), Ok(d1), Ok(d2)) => {
                            let corrupted_proof = payload
                                .adapter
                                .proof_from_components(
                                    ProofFromComponentsParams,
                                    ProofFromComponentsPayload {
                                        components: DeliveryProofComponents::MintFirstGroupOnly {
                                            challenge: mint_challenge,
                                            responses: mint_responses,
                                            first_messages: MintSecondGroupFirstMessages {
                                                hpub: hpub.point,
                                                d1: d1.point,
                                                d2: d2.point,
                                            },
                                        },
                                    },
                                )
                                .expect("the changed record's components form a proof")
                                .proof;
                            let suite_identifier =
                                SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                                    identifier: mint_record.context.suite_identifier,
                                    version: mint_record.context.suite_version,
                                })
                                .expect("the record's suite identifier is admitted");
                            let asset_identity_hash =
                                AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                                    bytes: mint_record.context.asset_identity_hash,
                                })
                                .expect("the record's asset identity hash is admitted");
                            let parameter_set_identifier = ParameterSetIdentifier::try_new(
                                ParameterSetIdentifierConstructorParams {
                                    bytes: mint_record.context.parameter_set,
                                },
                            )
                            .expect("the record's parameter-set identifier is admitted");
                            let statement = AlgebraicMintStatement {
                                context: DeliveryContext {
                                    suite_identifier,
                                    chain: F::ChainIdentifier::from_field(
                                        FromFieldParams,
                                        mint_record.context.chain.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect("the record's chain is a value its form admits"),
                                    entitlement_contract: F::Identity::from_field(
                                        FromFieldParams,
                                        mint_record.context.entitlement_contract.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's entitlement contract is a value its form admits",
                                    ),
                                    asset_identity_hash,
                                    parameter_set_identifier,
                                    source_entitlement: F::Entitlement::from_field(
                                        FromFieldParams,
                                        mint_record.context.source_entitlement.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's source entitlement is a value its form admits",
                                    ),
                                    target_entitlement: F::Entitlement::from_field(
                                        FromFieldParams,
                                        mint_record.context.target_entitlement.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's target entitlement is a value its form admits",
                                    ),
                                    old_interval: F::Interval::from_field(
                                        FromFieldParams,
                                        mint_record.context.old_interval.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's old interval is a value its form admits",
                                    ),
                                    new_interval: F::Interval::from_field(
                                        FromFieldParams,
                                        mint_record.context.new_interval.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's new interval is a value its form admits",
                                    ),
                                    purpose: MintPurpose::try_from(mint_record.context.purpose)
                                        .expect("the record's purpose code is declared"),
                                    seller: F::Identity::from_field(
                                        FromFieldParams,
                                        mint_record.context.seller.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect("the record's seller is a value its form admits"),
                                    buyer: F::Identity::from_field(
                                        FromFieldParams,
                                        mint_record.context.buyer.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect("the record's buyer is a value its form admits"),
                                    expiry: mint_record.context.expiry,
                                },
                                hpub: self
                                    .pairing
                                    .decode_g2(DecodeG2Params, &mint_record.hpub)
                                    .expect("the record's hpub decodes")
                                    .point,
                                identity_element: self
                                    .pairing
                                    .decode_g1(
                                        DecodeG1Params,
                                        &mint_record.identity_element,
                                    )
                                    .expect("the record's identity element decodes")
                                    .point,
                                buyer_keys: PublicKeysComponents {
                                    pk1: self
                                        .pairing
                                        .decode_g1(DecodeG1Params, &mint_record.buyer_pk1)
                                        .expect("the record's buyer pk1 decodes")
                                        .point,
                                    pk2: self
                                        .pairing
                                        .decode_g2(DecodeG2Params, &mint_record.buyer_pk2)
                                        .expect("the record's buyer pk2 decodes")
                                        .point,
                                },
                                envelope: EnvelopeComponents {
                                    c1: self
                                        .pairing
                                        .decode_g1(DecodeG1Params, &mint_record.envelope_c1)
                                        .expect("the record's envelope c1 decodes")
                                        .point,
                                    c2: self
                                        .pairing
                                        .decode_g1(DecodeG1Params, &mint_record.envelope_c2)
                                        .expect("the record's envelope c2 decodes")
                                        .point,
                                    d1: self
                                        .pairing
                                        .decode_g2(DecodeG2Params, &mint_record.envelope_d1)
                                        .expect("the record's envelope d1 decodes")
                                        .point,
                                    d2: self
                                        .pairing
                                        .decode_g2(DecodeG2Params, &mint_record.envelope_d2)
                                        .expect("the record's envelope d2 decodes")
                                        .point,
                                },
                            };
                            !payload
                                .adapter
                                .verify(
                                    VerifyParams,
                                    VerifyPayload {
                                        statement: AlgebraicStatement::Mint(&statement),
                                        proof: &corrupted_proof,
                                    },
                                )
                                .expect("the changed record's proof reaches verify")
                                .is_valid
                        }
                        _ => true,
                    };
                    assert!(mint_refused, "the record with one byte changed is refused");

                    let length = transfer_record.second_group_first_messages.len() / 3;
                    let transfer_challenge = self
                        .pairing
                        .decode_scalar(DecodeScalarParams, &transfer_record.challenge)
                        .expect("the record's challenge decodes")
                        .scalar;
                    let transfer_responses = TransferResponses {
                        x: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &transfer_record.x)
                            .expect("the record's x decodes")
                            .scalar,
                        y: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &transfer_record.y)
                            .expect("the record's y decodes")
                            .scalar,
                        s: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &transfer_record.s)
                            .expect("the record's s decodes")
                            .scalar,
                        rho: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &transfer_record.rho)
                            .expect("the record's rho decodes")
                            .scalar,
                        sigma: self
                            .pairing
                            .decode_scalar(DecodeScalarParams, &transfer_record.sigma)
                            .expect("the record's sigma decodes")
                            .scalar,
                    };
                    let mut corrupted = transfer_record.second_group_first_messages.clone();
                    corrupted[0] ^= 0x01;
                    let transfer_refused = match (
                        self.pairing.decode_g2(DecodeG2Params, &corrupted[..length]),
                        self.pairing
                            .decode_g2(DecodeG2Params, &corrupted[length..length * 2]),
                        self.pairing
                            .decode_g2(DecodeG2Params, &corrupted[length * 2..]),
                    ) {
                        (Ok(pk2), Ok(d1), Ok(d2)) => {
                            let corrupted_proof = payload
                                .adapter
                                .proof_from_components(
                                    ProofFromComponentsParams,
                                    ProofFromComponentsPayload {
                                        components:
                                            DeliveryProofComponents::TransferFirstGroupOnly {
                                                challenge: transfer_challenge,
                                                responses: transfer_responses,
                                                first_messages: TransferSecondGroupFirstMessages {
                                                    pk2: pk2.point,
                                                    d1: d1.point,
                                                    d2: d2.point,
                                                },
                                            },
                                    },
                                )
                                .expect("the changed record's components form a proof")
                                .proof;
                            let suite_identifier =
                                SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                                    identifier: transfer_record.context.suite_identifier,
                                    version: transfer_record.context.suite_version,
                                })
                                .expect("the record's suite identifier is admitted");
                            let asset_identity_hash =
                                AssetIdentityHash::try_new(AssetIdentityHashConstructorParams {
                                    bytes: transfer_record.context.asset_identity_hash,
                                })
                                .expect("the record's asset identity hash is admitted");
                            let parameter_set_identifier = ParameterSetIdentifier::try_new(
                                ParameterSetIdentifierConstructorParams {
                                    bytes: transfer_record.context.parameter_set,
                                },
                            )
                            .expect("the record's parameter-set identifier is admitted");
                            let statement = AlgebraicTransferStatement {
                                context: DeliveryContext {
                                    suite_identifier,
                                    chain: F::ChainIdentifier::from_field(
                                        FromFieldParams,
                                        transfer_record.context.chain.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect("the record's chain is a value its form admits"),
                                    entitlement_contract: F::Identity::from_field(
                                        FromFieldParams,
                                        transfer_record.context.entitlement_contract.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's entitlement contract is a value its form admits",
                                    ),
                                    asset_identity_hash,
                                    parameter_set_identifier,
                                    source_entitlement: F::Entitlement::from_field(
                                        FromFieldParams,
                                        transfer_record.context.source_entitlement.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's source entitlement is a value its form admits",
                                    ),
                                    target_entitlement: F::Entitlement::from_field(
                                        FromFieldParams,
                                        transfer_record.context.target_entitlement.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's target entitlement is a value its form admits",
                                    ),
                                    old_interval: F::Interval::from_field(
                                        FromFieldParams,
                                        transfer_record.context.old_interval.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's old interval is a value its form admits",
                                    ),
                                    new_interval: F::Interval::from_field(
                                        FromFieldParams,
                                        transfer_record.context.new_interval.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect(
                                        "the record's new interval is a value its form admits",
                                    ),
                                    purpose: TransferPurpose::try_from(
                                        transfer_record.context.purpose,
                                    )
                                    .expect("the record's purpose code is declared"),
                                    seller: F::Identity::from_field(
                                        FromFieldParams,
                                        transfer_record.context.seller.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect("the record's seller is a value its form admits"),
                                    buyer: F::Identity::from_field(
                                        FromFieldParams,
                                        transfer_record.context.buyer.clone(),
                                    )
                                    .map(|success| success.value)
                                    .expect("the record's buyer is a value its form admits"),
                                    expiry: transfer_record.context.expiry,
                                },
                                identity_element: self
                                    .pairing
                                    .decode_g1(
                                        DecodeG1Params,
                                        &transfer_record.identity_element,
                                    )
                                    .expect("the record's identity element decodes")
                                    .point,
                                seller_keys: PublicKeysComponents {
                                    pk1: self
                                        .pairing
                                        .decode_g1(DecodeG1Params, &transfer_record.seller_pk1)
                                        .expect("the record's seller pk1 decodes")
                                        .point,
                                    pk2: self
                                        .pairing
                                        .decode_g2(DecodeG2Params, &transfer_record.seller_pk2)
                                        .expect("the record's seller pk2 decodes")
                                        .point,
                                },
                                buyer_keys: PublicKeysComponents {
                                    pk1: self
                                        .pairing
                                        .decode_g1(DecodeG1Params, &transfer_record.buyer_pk1)
                                        .expect("the record's buyer pk1 decodes")
                                        .point,
                                    pk2: self
                                        .pairing
                                        .decode_g2(DecodeG2Params, &transfer_record.buyer_pk2)
                                        .expect("the record's buyer pk2 decodes")
                                        .point,
                                },
                                old_envelope: EnvelopeComponents {
                                    c1: self
                                        .pairing
                                        .decode_g1(
                                            DecodeG1Params,
                                            &transfer_record.old_envelope_c1,
                                        )
                                        .expect("the record's old envelope c1 decodes")
                                        .point,
                                    c2: self
                                        .pairing
                                        .decode_g1(
                                            DecodeG1Params,
                                            &transfer_record.old_envelope_c2,
                                        )
                                        .expect("the record's old envelope c2 decodes")
                                        .point,
                                    d1: self
                                        .pairing
                                        .decode_g2(
                                            DecodeG2Params,
                                            &transfer_record.old_envelope_d1,
                                        )
                                        .expect("the record's old envelope d1 decodes")
                                        .point,
                                    d2: self
                                        .pairing
                                        .decode_g2(
                                            DecodeG2Params,
                                            &transfer_record.old_envelope_d2,
                                        )
                                        .expect("the record's old envelope d2 decodes")
                                        .point,
                                },
                                new_envelope: EnvelopeComponents {
                                    c1: self
                                        .pairing
                                        .decode_g1(
                                            DecodeG1Params,
                                            &transfer_record.new_envelope_c1,
                                        )
                                        .expect("the record's new envelope c1 decodes")
                                        .point,
                                    c2: self
                                        .pairing
                                        .decode_g1(
                                            DecodeG1Params,
                                            &transfer_record.new_envelope_c2,
                                        )
                                        .expect("the record's new envelope c2 decodes")
                                        .point,
                                    d1: self
                                        .pairing
                                        .decode_g2(
                                            DecodeG2Params,
                                            &transfer_record.new_envelope_d1,
                                        )
                                        .expect("the record's new envelope d1 decodes")
                                        .point,
                                    d2: self
                                        .pairing
                                        .decode_g2(
                                            DecodeG2Params,
                                            &transfer_record.new_envelope_d2,
                                        )
                                        .expect("the record's new envelope d2 decodes")
                                        .point,
                                },
                            };
                            !payload
                                .adapter
                                .verify(
                                    VerifyParams,
                                    VerifyPayload {
                                        statement: AlgebraicStatement::Transfer(&statement),
                                        proof: &corrupted_proof,
                                    },
                                )
                                .expect("the changed record's proof reaches verify")
                                .is_valid
                        }
                        _ => true,
                    };
                    assert!(
                        transfer_refused,
                        "the record with one byte changed is refused"
                    );
                }
            }
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: None,
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: every first-group point of a record is as long as the
///   `encode_g1` of the first-group generator, every second-group point as
///   long as the `encode_g2` of the second-group generator, and every scalar
///   field 32 bytes.
/// Arrange: `mint_count` and `transfer_count` of `1`; every member of
///   `PAIRING_CONCRETES`, whose encoded lengths differ between the curves.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  the mint record's `identity_element`, `buyer_pk1`,
///   `envelope_c1`, and `envelope_c2` and the transfer record's
///   `identity_element`, `seller_pk1`, `buyer_pk1`, and both envelopes' `c1`
///   and `c2` each hold the length of the first-group generator's encoding;
///   the mint record's `hpub`, `buyer_pk2`, `envelope_d1`, and `envelope_d2`
///   and the transfer record's `seller_pk2`, `buyer_pk2`, and both
///   envelopes' `d1` and `d2` each hold the length of the second-group
///   generator's encoding; the mint record's `challenge`, `alpha`, `r`,
///   `rho`, and `sigma` and the transfer record's `challenge`, `x`, `y`,
///   `s`, `rho`, and `sigma` each hold `32` bytes; the generators' encodings
///   are read through the pairing's generator operations and `encode_g1` and
///   `encode_g2`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_encodes_its_elements_in_the_selected_pairings_encodings() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let Ok(g1_generator) = self
                .pairing
                .g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2_generator) = self
                .pairing
                .g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(reference_g1) = self.pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: g1_generator.point,
                },
            );
            let Ok(reference_g2) = self.pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: g2_generator.point,
                },
            );
            let g1_width = reference_g1.bytes.as_ref().len();
            let g2_width = reference_g2.bytes.as_ref().len();
            let mint_record = &success.mint[0];
            assert_eq!(mint_record.identity_element.len(), g1_width);
            assert_eq!(mint_record.buyer_pk1.len(), g1_width);
            assert_eq!(mint_record.envelope_c1.len(), g1_width);
            assert_eq!(mint_record.envelope_c2.len(), g1_width);
            assert_eq!(mint_record.hpub.len(), g2_width);
            assert_eq!(mint_record.buyer_pk2.len(), g2_width);
            assert_eq!(mint_record.envelope_d1.len(), g2_width);
            assert_eq!(mint_record.envelope_d2.len(), g2_width);
            assert_eq!(mint_record.challenge.len(), 32);
            assert_eq!(mint_record.alpha.len(), 32);
            assert_eq!(mint_record.r.len(), 32);
            assert_eq!(mint_record.rho.len(), 32);
            assert_eq!(mint_record.sigma.len(), 32);
            let transfer_record = &success.transfer[0];
            assert_eq!(transfer_record.identity_element.len(), g1_width);
            assert_eq!(transfer_record.seller_pk1.len(), g1_width);
            assert_eq!(transfer_record.buyer_pk1.len(), g1_width);
            assert_eq!(transfer_record.old_envelope_c1.len(), g1_width);
            assert_eq!(transfer_record.old_envelope_c2.len(), g1_width);
            assert_eq!(transfer_record.new_envelope_c1.len(), g1_width);
            assert_eq!(transfer_record.new_envelope_c2.len(), g1_width);
            assert_eq!(transfer_record.seller_pk2.len(), g2_width);
            assert_eq!(transfer_record.buyer_pk2.len(), g2_width);
            assert_eq!(transfer_record.old_envelope_d1.len(), g2_width);
            assert_eq!(transfer_record.old_envelope_d2.len(), g2_width);
            assert_eq!(transfer_record.new_envelope_d1.len(), g2_width);
            assert_eq!(transfer_record.new_envelope_d2.len(), g2_width);
            assert_eq!(transfer_record.challenge.len(), 32);
            assert_eq!(transfer_record.x.len(), 32);
            assert_eq!(transfer_record.y.len(), 32);
            assert_eq!(transfer_record.s.len(), 32);
            assert_eq!(transfer_record.rho.len(), 32);
            assert_eq!(transfer_record.sigma.len(), 32);
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: None,
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: each record's context holds the canonical context's values,
///   each form-typed field read back through its form's `to_field` after
///   `from_field`.
/// Arrange: `mint_count` and `transfer_count` of `1`, the mint context and
///   the transfer context each overriding every form-typed field with a
///   value distinct from every other field's in both contexts, of the kind
///   the case's forms declare, `suite_identifier`, `asset_identity_hash`,
///   and `parameter_set_identifier` with distinct values from
///   `build_suite_identifier`, `build_asset_identity_hash`, and
///   `build_parameter_set_identifier`, and distinct purposes; every case
///   over `CHAIN_FORMS_CONCRETES`.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  each record's `context` equals `build_proof_vector_context`
///   overriding the form-typed fields and `expiry` with the context's
///   values, `purpose` with the purpose's code, and `suite_identifier`,
///   `suite_version`, `asset_identity_hash`, and `parameter_set` with the
///   literal values the arrangement built the domain values from.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_record_context_is_the_canonical_context_its_forms_rebuilt() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0xA1; 32]),
                version: Some(7),
            },
        )),
        chain: Some(CanonicalFieldValue::Unsigned256([0x61; 32])),
        entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x62; 20])),
        asset_identity_hash: Some(build_asset_identity_hash(
            AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xA2; 32]),
            },
        )),
        parameter_set_identifier: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0xA3; 32]),
            },
        )),
        source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x63; 32])),
        target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x64; 32])),
        old_interval: Some(CanonicalFieldValue::Unsigned64(0x0102_0304_0506_0708)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(0x1112_1314_1516_1718)),
        seller: Some(CanonicalFieldValue::FixedBytes20([0x67; 20])),
        buyer: Some(CanonicalFieldValue::FixedBytes20([0x68; 20])),
        expiry: Some(1_700_000_001),
    });
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        suite_identifier: Some(build_suite_identifier(
            SuiteIdentifierConstructorParamsOverrides {
                identifier: Some([0xB1; 32]),
                version: Some(9),
            },
        )),
        chain: Some(CanonicalFieldValue::Unsigned256([0x71; 32])),
        entitlement_contract: Some(CanonicalFieldValue::FixedBytes20([0x72; 20])),
        asset_identity_hash: Some(build_asset_identity_hash(
            AssetIdentityHashConstructorParamsOverrides {
                bytes: Some([0xB2; 32]),
            },
        )),
        parameter_set_identifier: Some(build_parameter_set_identifier(
            ParameterSetIdentifierConstructorParamsOverrides {
                bytes: Some([0xB3; 32]),
            },
        )),
        source_entitlement: Some(CanonicalFieldValue::Unsigned256([0x73; 32])),
        target_entitlement: Some(CanonicalFieldValue::Unsigned256([0x74; 32])),
        old_interval: Some(CanonicalFieldValue::Unsigned64(0x2122_2324_2526_2728)),
        new_interval: Some(CanonicalFieldValue::Unsigned64(0x3132_3334_3536_3738)),
        seller: Some(CanonicalFieldValue::FixedBytes20([0x77; 20])),
        buyer: Some(CanonicalFieldValue::FixedBytes20([0x78; 20])),
        expiry: Some(1_800_000_002),
    });
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let mint_record = &success.mint[0];
            assert_eq!(
                mint_record.context,
                build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some(*self.mint.suite_identifier.identifier()),
                    suite_version: Some(self.mint.suite_identifier.version()),
                    chain: Some(self.mint.chain.clone()),
                    entitlement_contract: Some(self.mint.entitlement_contract.clone()),
                    asset_identity_hash: Some(*self.mint.asset_identity_hash.as_bytes()),
                    parameter_set: Some(*self.mint.parameter_set_identifier.as_bytes()),
                    source_entitlement: Some(self.mint.source_entitlement.clone()),
                    target_entitlement: Some(self.mint.target_entitlement.clone()),
                    old_interval: Some(self.mint.old_interval.clone()),
                    new_interval: Some(self.mint.new_interval.clone()),
                    purpose: Some(self.mint_purpose.code()),
                    seller: Some(self.mint.seller.clone()),
                    buyer: Some(self.mint.buyer.clone()),
                    expiry: Some(self.mint.expiry),
                }),
            );
            let transfer_record = &success.transfer[0];
            assert_eq!(
                transfer_record.context,
                build_proof_vector_context(ProofVectorContextOverrides {
                    suite_identifier: Some(*self.transfer.suite_identifier.identifier()),
                    suite_version: Some(self.transfer.suite_identifier.version()),
                    chain: Some(self.transfer.chain.clone()),
                    entitlement_contract: Some(self.transfer.entitlement_contract.clone()),
                    asset_identity_hash: Some(*self.transfer.asset_identity_hash.as_bytes()),
                    parameter_set: Some(*self.transfer.parameter_set_identifier.as_bytes()),
                    source_entitlement: Some(self.transfer.source_entitlement.clone()),
                    target_entitlement: Some(self.transfer.target_entitlement.clone()),
                    old_interval: Some(self.transfer.old_interval.clone()),
                    new_interval: Some(self.transfer.new_interval.clone()),
                    purpose: Some(self.transfer_purpose.code()),
                    seller: Some(self.transfer.seller.clone()),
                    buyer: Some(self.transfer.buyer.clone()),
                    expiry: Some(self.transfer.expiry),
                }),
            );
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: None,
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_REPLACEMENT,
        transfer_purpose: DELIVERY_PURPOSE_GRANT,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: the lists hold the configured counts, each mint record's
///   `draws` hold the uniforms of its calls and each transfer record's
///   `draws` those of its calls, each `UNIFORM_BYTES_LENGTH` long, and no
///   two records of either list share `draws`.
/// Arrange: `mint_count` of `2` and `transfer_count` of `3`; every member
///   of `PAIRING_CONCRETES`, whose sampling bounds may differ.
/// Act:     `proof_vectors`, once per case, in the innermost consumer's
///   method.
/// Assert:  `mint` holds `2` records whose `draws` each hold `6` times
///   `UNIFORM_BYTES_LENGTH` bytes, `transfer` holds `3` records whose
///   `draws` each hold `9` times that length, and no two records of either
///   list share `draws`.
/// Boundary: `proof_vectors` and the chain of real functions the private
///   surface names.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_run_emits_the_configured_count_of_records_with_distinct_draws() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(success.mint.len(), 2);
            assert_eq!(success.transfer.len(), 3);
            for (index, record) in success.mint.iter().enumerate() {
                assert_eq!(record.draws.len(), 6 * length);
                for (other_index, other) in success.mint.iter().enumerate() {
                    if index != other_index {
                        assert_ne!(record.draws, other.draws);
                    }
                }
            }
            for (index, record) in success.transfer.iter().enumerate() {
                assert_eq!(record.draws.len(), 9 * length);
                for (other_index, other) in success.transfer.iter().enumerate() {
                    if index != other_index {
                        assert_ne!(record.draws, other.draws);
                    }
                }
            }
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: None,
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(2) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(3),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: a mint record from `proof_vectors`, encoded under
///   `MintProofVectorDescription` by each concrete of
///   `DECLARED_ENCODING_SELECTIONS` and decoded under the same description,
///   equals itself.
/// Arrange: `mint_count` and `transfer_count` of `1`; every case, each
///   encoding selection crossed with each member of `PAIRING_CONCRETES`,
///   the description built over the case's forms.
/// Act:     `encode` of the encoder the consumer received, over the record,
///   with `build_encode_params`.
/// Assert:  `decode` of the returned bytes, with `build_decode_params`,
///   returns a described record equal to the mint record, its form-typed
///   context fields holding the values their forms rebuilt.
/// Boundary: the description's `to_fields` and `fields_to_value` through
///   the real encoder, after `proof_vectors`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_mint_record_survives_its_description_round_trip() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, E, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A, E>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let record = &success.mint[0];
            let Ok(description) = MintProofVectorDescription::<F>::try_new(
                MintProofVectorDescriptionConstructorParams,
            );
            let encoded = self
                .encoder
                .encode(build_encode_params(&description), record)
                .expect("the mint record encodes under its description")
                .bytes;
            let decoded = self
                .encoder
                .decode(build_decode_params(&description), &encoded)
                .expect("the encoded mint record decodes under its description")
                .described;
            assert_eq!(decoded, *record);
            let chain =
                F::ChainIdentifier::from_field(FromFieldParams, record.context.chain.clone())
                    .map(|success| success.value)
                    .expect("the record's chain is a value its form admits");
            let Ok(yielded) = F::ChainIdentifier::to_field(ToFieldParams, &chain);
            assert_eq!(decoded.context.chain, yielded.field);
            let entitlement_contract = F::Identity::from_field(
                FromFieldParams,
                record.context.entitlement_contract.clone(),
            )
            .map(|success| success.value)
            .expect("the record's entitlement contract is a value its form admits");
            let Ok(yielded) = F::Identity::to_field(ToFieldParams, &entitlement_contract);
            assert_eq!(decoded.context.entitlement_contract, yielded.field);
            let source_entitlement = F::Entitlement::from_field(
                FromFieldParams,
                record.context.source_entitlement.clone(),
            )
            .map(|success| success.value)
            .expect("the record's source entitlement is a value its form admits");
            let Ok(yielded) = F::Entitlement::to_field(ToFieldParams, &source_entitlement);
            assert_eq!(decoded.context.source_entitlement, yielded.field);
            let target_entitlement = F::Entitlement::from_field(
                FromFieldParams,
                record.context.target_entitlement.clone(),
            )
            .map(|success| success.value)
            .expect("the record's target entitlement is a value its form admits");
            let Ok(yielded) = F::Entitlement::to_field(ToFieldParams, &target_entitlement);
            assert_eq!(decoded.context.target_entitlement, yielded.field);
            let old_interval =
                F::Interval::from_field(FromFieldParams, record.context.old_interval.clone())
                    .map(|success| success.value)
                    .expect("the record's old interval is a value its form admits");
            let Ok(yielded) = F::Interval::to_field(ToFieldParams, &old_interval);
            assert_eq!(decoded.context.old_interval, yielded.field);
            let new_interval =
                F::Interval::from_field(FromFieldParams, record.context.new_interval.clone())
                    .map(|success| success.value)
                    .expect("the record's new interval is a value its form admits");
            let Ok(yielded) = F::Interval::to_field(ToFieldParams, &new_interval);
            assert_eq!(decoded.context.new_interval, yielded.field);
            let seller = F::Identity::from_field(FromFieldParams, record.context.seller.clone())
                .map(|success| success.value)
                .expect("the record's seller is a value its form admits");
            let Ok(yielded) = F::Identity::to_field(ToFieldParams, &seller);
            assert_eq!(decoded.context.seller, yielded.field);
            let buyer = F::Identity::from_field(FromFieldParams, record.context.buyer.clone())
                .map(|success| success.value)
                .expect("the record's buyer is a value its form admits");
            let Ok(yielded) = F::Identity::to_field(ToFieldParams, &buyer);
            assert_eq!(decoded.context.buyer, yielded.field);
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A, E>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            encoder: self.encoder,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: None,
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: each member `members()` declares has the kind of the value
///   `to_fields` emits at the same index, for each description over the
///   forms of each member of `CHAIN_FORMS_CONCRETES`.
/// Arrange: `mint_count` and `transfer_count` of `1`; every case, the mint
///   description and the transfer description each in turn.
/// Act:     `members()` on the description, in the innermost consumer's
///   method after `proof_vectors`.
/// Assert:  each member's kind equals the kind of the value `to_fields`
///   over the record from `proof_vectors` emits at its index, the
///   form-typed members taking the kinds the forms declare.
/// Boundary: the description's `members` and `to_fields`, with the
///   `render` constructors `members` calls, after `proof_vectors`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn each_record_kind_declares_the_kinds_its_fields_emit() {
    /// The canonical kind a `CanonicalFieldValue` carries.
    fn field_kind(value: &CanonicalFieldValue) -> CanonicalFieldKind {
        match value {
            CanonicalFieldValue::FixedBytes32(_) => CanonicalFieldKind::FixedBytes32,
            CanonicalFieldValue::Unsigned16(_) => CanonicalFieldKind::Unsigned16,
            CanonicalFieldValue::Unsigned32(_) => CanonicalFieldKind::Unsigned32,
            CanonicalFieldValue::Unsigned64(_) => CanonicalFieldKind::Unsigned64,
            CanonicalFieldValue::Text(_) => CanonicalFieldKind::Text,
            CanonicalFieldValue::Bytes(_) => CanonicalFieldKind::Bytes,
            CanonicalFieldValue::FixedBytes20(_) => CanonicalFieldKind::FixedBytes20,
            CanonicalFieldValue::Unsigned256(_) => CanonicalFieldKind::Unsigned256,
        }
    }

    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let success = proof_vectors::<P, K, A, D>(
                &build_proof_vectors_deps(
                    self.pairing,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.random,
                    Default::default(),
                ),
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            )
            .expect("the declared selections admit the call");

            // Act
            let Ok(mint_description) = MintProofVectorDescription::<F>::try_new(
                MintProofVectorDescriptionConstructorParams,
            );
            let Ok(transfer_description) = TransferProofVectorDescription::<F>::try_new(
                TransferProofVectorDescriptionConstructorParams,
            );
            let Ok(mint_fields) = mint_description.to_fields(ToFieldsParams, &success.mint[0]);
            let mint_members = mint_description.members();
            let Ok(transfer_fields) =
                transfer_description.to_fields(ToFieldsParams, &success.transfer[0]);
            let transfer_members = transfer_description.members();

            // Assert
            let expected_mint =
                build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
                    members: Some(
                        MINT_PROOF_VECTOR_MEMBER_NAMES
                            .iter()
                            .zip(mint_fields.fields.values.iter())
                            .map(|(name, value)| {
                                build_solidity_record_member(SolidityRecordMemberOverrides {
                                    name: Some(build_solidity_member_name(
                                        SolidityMemberNameConstructorParamsOverrides {
                                            text: Some((*name).to_string()),
                                        },
                                    )),
                                    kind: Some(field_kind(value)),
                                })
                            })
                            .collect(),
                    ),
                });
            assert_eq!(mint_members, Ok(expected_mint));
            let expected_transfer =
                build_solidity_record_members(SolidityRecordMembersConstructorParamsOverrides {
                    members: Some(
                        TRANSFER_PROOF_VECTOR_MEMBER_NAMES
                            .iter()
                            .zip(transfer_fields.fields.values.iter())
                            .map(|(name, value)| {
                                build_solidity_record_member(SolidityRecordMemberOverrides {
                                    name: Some(build_solidity_member_name(
                                        SolidityMemberNameConstructorParamsOverrides {
                                            text: Some((*name).to_string()),
                                        },
                                    )),
                                    kind: Some(field_kind(value)),
                                })
                            })
                            .collect(),
                    ),
                });
            assert_eq!(transfer_members, Ok(expected_transfer));
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: None,
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}

/// Contract: a transfer record from `proof_vectors`, encoded under
///   `TransferProofVectorDescription` by each concrete of
///   `DECLARED_ENCODING_SELECTIONS` and decoded under the same description,
///   equals itself.
/// Arrange: as the mint block.
/// Act:     `encode` of the encoder the consumer received, over the record,
///   with `build_encode_params`.
/// Assert:  `decode` of the returned bytes returns a described record equal
///   to the transfer record.
/// Boundary: the description's `to_fields` and `fields_to_value` through
///   the real encoder, after `proof_vectors`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_transfer_record_survives_its_description_round_trip() {
    // Arrange
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identity = vec![0x44; 32];

    struct DeliveryProofConsumer<
        'a,
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        key_agreement: &'a A,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, K, A, E, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'_, P, K, A, E>
    where
        P: IPairingArithmetic,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
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
                build_proof_vectors_params(ProofVectorsParamsOverrides {
                    mint_count: Some(self.mint_count),
                    transfer_count: Some(self.transfer_count),
                    scope: Some(self.scope),
                }),
                build_proof_vectors_payload(
                    self.mint,
                    self.transfer,
                    self.identity,
                    ProofVectorsPayloadOverrides {
                        mint_purpose: Some(self.mint_purpose),
                        transfer_purpose: Some(self.transfer_purpose),
                    },
                ),
            );

            // Assert
            let success = result.expect("the declared selections admit the call");
            let record = &success.transfer[0];
            let Ok(description) = TransferProofVectorDescription::<F>::try_new(
                TransferProofVectorDescriptionConstructorParams,
            );
            let encoded = self
                .encoder
                .encode(build_encode_params(&description), record)
                .expect("the transfer record encodes under its description")
                .bytes;
            let decoded = self
                .encoder
                .decode(build_decode_params(&description), &encoded)
                .expect("the encoded transfer record decodes under its description")
                .described;
            assert_eq!(decoded, *record);
        }
    }

    struct KeyAgreementConsumer<
        'a,
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    > {
        pairing: &'a P,
        kem: &'a K,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        scope: IdentityScope,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, F, K, E> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'_, P, F, K, E>
    where
        P: IPairingArithmetic,
        F: IChainForms,
        K: ICredentialKemAdapter<Pairing = P>,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof::<P, E, F, DeliveryProofConsumer<'_, P, K, A, E>>(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            encoder: self.encoder,
                            random: self.random,
                            scope: self.scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                        concrete: Some(*concrete),
                        algebra: Some(EnvelopeAlgebra::PairingElGamal),
                        verifier_form: Some(self.verifier_form),
                        statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                    }),
                    build_create_delivery_proof_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared delivery-proof selection is created"
                );
            }
        }
    }

    struct KemConsumer<
        'a,
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    > {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        forms: PhantomData<F>,
        verifier_form: VerifierGroupArithmetic,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E, F> IKemConsumer<P> for KemConsumer<'_, P, E, F>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement::<P, E, KeyAgreementConsumer<'_, P, F, K, E>>(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            kem: &payload.adapter,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            scope: payload.scope(),
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared key-agreement selection is created"
                );
            }
        }
    }

    struct FormsConsumer<'a, P: IPairingArithmetic, E: IEncoderAdapter + IDecoderAdapter> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P, E> IChainFormsConsumer for FormsConsumer<'_, P, E>
    where
        P: IPairingArithmetic,
        E: IEncoderAdapter + IDecoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS
                .iter()
                .filter(|selection| self.kem_scope.is_none_or(|scope| selection.scope == scope))
            {
                let created = create_kem::<P, KemConsumer<'_, P, E, F>>(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            forms: PhantomData::<F>,
                            verifier_form: self.verifier_form,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                assert!(created.is_ok(), "the declared kem selection is created");
            }
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl<P: IPairingArithmetic> IEncodingConsumer for EncodingConsumer<'_, P> {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        FormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                assert!(
                    created.is_ok(),
                    "the declared chain-forms concrete is created"
                );
            }
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        kem_scope: Option<IdentityScope>,
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identity: &'a [u8],
        mint_count: VectorCount,
        transfer_count: VectorCount,
        mint_purpose: MintPurpose,
        transfer_purpose: TransferPurpose,
    }

    impl IPairingConsumer for &PairingConsumer<'_> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            consumed: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                let Ok(hashing) = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(hashing_selection.concrete),
                        identifier: Some(hashing_selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                ) else {
                    panic!("the declared hash-to-scalar selection is created")
                };
                for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                    let created = create_encoding(
                        &build_create_encoding_deps(EncodingConsumer {
                            pairing: &consumed.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            kem_scope: self.kem_scope,
                            mint: self.mint,
                            transfer: self.transfer,
                            identity: self.identity,
                            mint_count: self.mint_count,
                            transfer_count: self.transfer_count,
                            mint_purpose: self.mint_purpose,
                            transfer_purpose: self.transfer_purpose,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    assert!(
                        created.is_ok(),
                        "the declared encoding selection is created"
                    );
                }
            }
        }
    }

    let consumer = PairingConsumer {
        random: &*source,
        kem_scope: None,
        mint: &mint,
        transfer: &transfer,
        identity: &identity,
        mint_count: build_vector_count(VectorCountConstructorParamsOverrides { count: Some(1) }),
        transfer_count: build_vector_count(VectorCountConstructorParamsOverrides {
            count: Some(1),
        }),
        mint_purpose: DELIVERY_PURPOSE_MINT,
        transfer_purpose: DELIVERY_PURPOSE_TRANSFER,
    };

    // Act & Assert per pairing concrete
    for member in PAIRING_CONCRETES {
        let created = create_pairing(
            &build_create_pairing_deps(&consumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*member),
                ..Default::default()
            }),
            CreatePairingPayload,
        );
        assert!(created.is_ok(), "the pairing factory ran the consumer");
    }
}
