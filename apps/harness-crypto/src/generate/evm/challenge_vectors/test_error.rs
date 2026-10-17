#![allow(clippy::expect_used)]

use chain::{
    ConsumeChainFormsParams, ConsumeChainFormsPayload, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use encoding::{
    CanonicalFieldKind, CanonicalFieldValue, ConsumeEncodingParams, ConsumeEncodingPayload,
    FromFieldErrorReturn, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use hash_to_scalar::{
    DomainTagTryNewErrorReturn, IHashToScalarAdapter, build_create_hash_to_scalar_deps,
    build_create_hash_to_scalar_params, build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingAdapter, IPairingArithmetic, IPairingConsumer, IPairingReference,
    MockIPairingAdapterFailureMode, PairingConcrete, SampleUniformScalarErrorReturn,
    build_create_pairing_deps, build_create_pairing_params, create_pairing,
};
use proof::{
    DeliveryProofDeclaration, DeliveryProofDeclarationOverrides, DeliveryRelation,
    build_delivery_proof_declaration,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesErrorReturn, IRandomSourceAdapter, MockIRandomSourceAdapterFailureMode,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

use super::challenge_vectors;
use super::interface::*;
use super::mock::{
    CanonicalDeliveryContextOverrides, ChallengeVectorsParamsOverrides,
    build_canonical_delivery_context, build_challenge_vectors_deps, build_challenge_vectors_params,
    build_challenge_vectors_payload,
};

/// Contract: `params.statement_version` is not `DELIVERY_STATEMENT_VERSION_ONE`
///   → `Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion {
///   version })`, decided by equality before any other step.
/// Arrange:  the params with `statement_version` `2`; the declaration with
///   `challenge_tag` `b""` and the mint context with `chain`
///   `CanonicalFieldValue::Unsigned64(1)`, so the tag and a form are also
///   refused and only an earlier version check returns the version.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   UnsupportedStatementVersion { version: 2 })`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_refuse_a_statement_version_no_description_serves() {
    // Arrange
    let declaration = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b""),
        ..Default::default()
    });
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned64(1)),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(ChallengeVectorsParamsOverrides {
        statement_version: Some(2),
        ..Default::default()
    });

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::UnsupportedStatementVersion { version: 2 })
    );
    // challenge_vectors_refuse_a_statement_version_no_description_serves
}

/// Contract: a declared tag `DomainTag::try_new` refuses →
///   `Err(ChallengeVectorsErrorReturn::DomainTag(error))`, the
///   constructor's error unchanged, before any form is rebuilt.
/// Arrange:  the declaration with `challenge_tag` `b""`; the mint context
///   with `chain` `CanonicalFieldValue::Unsigned64(1)`, so a form is also
///   refused and only an earlier tag check returns the tag refusal.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::DomainTag(
///   DomainTagTryNewErrorReturn::Empty))`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_return_the_domain_tag_refusal_whole() {
    // Arrange
    let declaration = build_delivery_proof_declaration(DeliveryProofDeclarationOverrides {
        challenge_tag: Some(b""),
        ..Default::default()
    });
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned64(1)),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::DomainTag(
            DomainTagTryNewErrorReturn::Empty
        ))
    );
    // challenge_vectors_return_the_domain_tag_refusal_whole
}

/// Contract: the mint context's `chain` value is one the chain-identifier
///   form refuses → `Err(ChallengeVectorsErrorReturn::ChainIdentifier {
///   relation: DeliveryRelation::Mint, error })`, the form's refusal
///   unchanged.
/// Arrange:  the mint context with `chain`
///   `CanonicalFieldValue::Unsigned64(1)`, a kind the chain-identifier form
///   does not declare; the transfer context, the declaration, and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   ChainIdentifier { relation: DeliveryRelation::Mint, error })`,
///   `error` the literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_chain_identifier_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned64(1)),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::ChainIdentifier {
            relation: DeliveryRelation::Mint,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_carry_the_mint_chain_identifier_refusal
}

/// Contract: the mint context's `entitlement_contract` value is one the
///   identity form refuses → `Err(ChallengeVectorsErrorReturn::Identity {
///   relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::EntitlementContract, error })`, the form's
///   refusal unchanged.
/// Arrange:  the mint context with `entitlement_contract`
///   `CanonicalFieldValue::Unsigned256([0x31; 32])`, a kind the identity
///   form does not declare; the transfer context, the declaration, and
///   the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Identity
///   { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::EntitlementContract, error })`, `error` the
///   literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_entitlement_contract_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        entitlement_contract: Some(CanonicalFieldValue::Unsigned256([0x31; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Identity {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::EntitlementContract,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
    // challenge_vectors_carry_the_mint_entitlement_contract_refusal
}

/// Contract: the mint context's `source_entitlement` value is one the
///   entitlement form refuses → `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::SourceEntitlement, error })`, the form's
///   refusal unchanged.
/// Arrange:  the mint context with `source_entitlement`
///   `CanonicalFieldValue::FixedBytes32([0x43; 32])`, a kind the
///   entitlement form does not declare; the transfer context, the
///   declaration, and the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::SourceEntitlement, error })`, `error` the
///   literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_source_entitlement_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        source_entitlement: Some(CanonicalFieldValue::FixedBytes32([0x43; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Entitlement {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::SourceEntitlement,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_carry_the_mint_source_entitlement_refusal
}

/// Contract: the mint context's `target_entitlement` value is one the
///   entitlement form refuses → `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::TargetEntitlement, error })`, the form's
///   refusal unchanged.
/// Arrange:  the mint context with `target_entitlement`
///   `CanonicalFieldValue::FixedBytes32([0x44; 32])`, a kind the
///   entitlement form does not declare; the transfer context, the
///   declaration, and the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::TargetEntitlement, error })`, `error` the
///   literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_target_entitlement_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        target_entitlement: Some(CanonicalFieldValue::FixedBytes32([0x44; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Entitlement {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::TargetEntitlement,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_carry_the_mint_target_entitlement_refusal
}

/// Contract: the mint context's `old_interval` value is one the interval
///   form refuses → `Err(ChallengeVectorsErrorReturn::Interval {
///   relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::OldInterval, error })`, the form's refusal
///   unchanged.
/// Arrange:  the mint context with `old_interval`
///   `CanonicalFieldValue::Unsigned256([0x07; 32])`, a kind the interval
///   form does not declare; the transfer context, the declaration, and
///   the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Interval
///   { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::OldInterval, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned64 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_old_interval_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        old_interval: Some(CanonicalFieldValue::Unsigned256([0x07; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Interval {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::OldInterval,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            },
        })
    );
    // challenge_vectors_carry_the_mint_old_interval_refusal
}

/// Contract: the mint context's `new_interval` value is one the interval
///   form refuses → `Err(ChallengeVectorsErrorReturn::Interval {
///   relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::NewInterval, error })`, the form's refusal
///   unchanged.
/// Arrange:  the mint context with `new_interval`
///   `CanonicalFieldValue::Unsigned256([0x08; 32])`, a kind the interval
///   form does not declare; the transfer context, the declaration, and
///   the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Interval
///   { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::NewInterval, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned64 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_new_interval_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        new_interval: Some(CanonicalFieldValue::Unsigned256([0x08; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Interval {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::NewInterval,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            },
        })
    );
    // challenge_vectors_carry_the_mint_new_interval_refusal
}

/// Contract: the mint context's `seller` value is one the identity form
///   refuses → `Err(ChallengeVectorsErrorReturn::Identity { relation:
///   DeliveryRelation::Mint, field: DeliveryContextField::Seller, error
///   })`, the form's refusal unchanged.
/// Arrange:  the mint context with `seller`
///   `CanonicalFieldValue::Unsigned256([0x61; 32])`, a kind the identity
///   form does not declare; the transfer context, the declaration, and
///   the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Identity
///   { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::Seller, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_seller_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        seller: Some(CanonicalFieldValue::Unsigned256([0x61; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Identity {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::Seller,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
    // challenge_vectors_carry_the_mint_seller_refusal
}

/// Contract: the mint context's `buyer` value is one the identity form
///   refuses → `Err(ChallengeVectorsErrorReturn::Identity { relation:
///   DeliveryRelation::Mint, field: DeliveryContextField::Buyer, error
///   })`, the form's refusal unchanged.
/// Arrange:  the mint context with `buyer`
///   `CanonicalFieldValue::Unsigned256([0x62; 32])`, a kind the identity
///   form does not declare; the transfer context, the declaration, and
///   the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Identity
///   { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::Buyer, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_mint_buyer_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        buyer: Some(CanonicalFieldValue::Unsigned256([0x62; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Identity {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::Buyer,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
    // challenge_vectors_carry_the_mint_buyer_refusal
}

/// Contract: the transfer context's `chain` value is one the
///   chain-identifier form refuses → `Err(ChallengeVectorsErrorReturn::
///   ChainIdentifier { relation: DeliveryRelation::Transfer, error })`,
///   the form's refusal unchanged.
/// Arrange:  the transfer context with `chain`
///   `CanonicalFieldValue::Unsigned64(1)`, a kind the chain-identifier
///   form does not declare; the mint context, the declaration, and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   ChainIdentifier { relation: DeliveryRelation::Transfer, error })`,
///   `error` the literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_chain_identifier_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned64(1)),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::ChainIdentifier {
            relation: DeliveryRelation::Transfer,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_chain_identifier_refusal
}

/// Contract: the transfer context's `entitlement_contract` value is one
///   the identity form refuses → `Err(ChallengeVectorsErrorReturn::
///   Identity { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::EntitlementContract, error })`, the form's
///   refusal unchanged.
/// Arrange:  the transfer context with `entitlement_contract`
///   `CanonicalFieldValue::Unsigned256([0x31; 32])`, a kind the identity
///   form does not declare; the mint context, the declaration, and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Identity
///   { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::EntitlementContract, error })`, `error` the
///   literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_entitlement_contract_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        entitlement_contract: Some(CanonicalFieldValue::Unsigned256([0x31; 32])),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Identity {
            relation: DeliveryRelation::Transfer,
            field: DeliveryContextField::EntitlementContract,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_entitlement_contract_refusal
}

/// Contract: the transfer context's `source_entitlement` value is one the
///   entitlement form refuses → `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::SourceEntitlement, error })`, the form's
///   refusal unchanged.
/// Arrange:  the transfer context with `source_entitlement`
///   `CanonicalFieldValue::FixedBytes32([0x43; 32])`, a kind the
///   entitlement form does not declare; the mint context, the
///   declaration, and the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::SourceEntitlement, error })`, `error` the
///   literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_source_entitlement_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        source_entitlement: Some(CanonicalFieldValue::FixedBytes32([0x43; 32])),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Entitlement {
            relation: DeliveryRelation::Transfer,
            field: DeliveryContextField::SourceEntitlement,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_source_entitlement_refusal
}

/// Contract: the transfer context's `target_entitlement` value is one the
///   entitlement form refuses → `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::TargetEntitlement, error })`, the form's
///   refusal unchanged.
/// Arrange:  the transfer context with `target_entitlement`
///   `CanonicalFieldValue::FixedBytes32([0x44; 32])`, a kind the
///   entitlement form does not declare; the mint context, the
///   declaration, and the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   Entitlement { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::TargetEntitlement, error })`, `error` the
///   literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_target_entitlement_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        target_entitlement: Some(CanonicalFieldValue::FixedBytes32([0x44; 32])),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Entitlement {
            relation: DeliveryRelation::Transfer,
            field: DeliveryContextField::TargetEntitlement,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_target_entitlement_refusal
}

/// Contract: the transfer context's `old_interval` value is one the
///   interval form refuses → `Err(ChallengeVectorsErrorReturn::Interval {
///   relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::OldInterval, error })`, the form's refusal
///   unchanged.
/// Arrange:  the transfer context with `old_interval`
///   `CanonicalFieldValue::Unsigned256([0x07; 32])`, a kind the interval
///   form does not declare; the mint context, the declaration, and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Interval
///   { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::OldInterval, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned64 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_old_interval_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        old_interval: Some(CanonicalFieldValue::Unsigned256([0x07; 32])),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Interval {
            relation: DeliveryRelation::Transfer,
            field: DeliveryContextField::OldInterval,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_old_interval_refusal
}

/// Contract: the transfer context's `new_interval` value is one the
///   interval form refuses → `Err(ChallengeVectorsErrorReturn::Interval {
///   relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::NewInterval, error })`, the form's refusal
///   unchanged.
/// Arrange:  the transfer context with `new_interval`
///   `CanonicalFieldValue::Unsigned256([0x08; 32])`, a kind the interval
///   form does not declare; the mint context, the declaration, and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Interval
///   { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::NewInterval, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned64 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_new_interval_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        new_interval: Some(CanonicalFieldValue::Unsigned256([0x08; 32])),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Interval {
            relation: DeliveryRelation::Transfer,
            field: DeliveryContextField::NewInterval,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned64,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_new_interval_refusal
}

/// Contract: the transfer context's `seller` value is one the identity
///   form refuses → `Err(ChallengeVectorsErrorReturn::Identity {
///   relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::Seller, error })`, the form's refusal
///   unchanged.
/// Arrange:  the transfer context with `seller`
///   `CanonicalFieldValue::Unsigned256([0x61; 32])`, a kind the identity
///   form does not declare; the mint context, the declaration, and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Identity
///   { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::Seller, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_seller_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        seller: Some(CanonicalFieldValue::Unsigned256([0x61; 32])),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Identity {
            relation: DeliveryRelation::Transfer,
            field: DeliveryContextField::Seller,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_seller_refusal
}

/// Contract: the transfer context's `buyer` value is one the identity
///   form refuses → `Err(ChallengeVectorsErrorReturn::Identity {
///   relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::Buyer, error })`, the form's refusal
///   unchanged.
/// Arrange:  the transfer context with `buyer`
///   `CanonicalFieldValue::Unsigned256([0x62; 32])`, a kind the identity
///   form does not declare; the mint context, the declaration, and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Identity
///   { relation: DeliveryRelation::Transfer, field:
///   DeliveryContextField::Buyer, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_carry_the_transfer_buyer_refusal() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        buyer: Some(CanonicalFieldValue::Unsigned256([0x62; 32])),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Identity {
            relation: DeliveryRelation::Transfer,
            field: DeliveryContextField::Buyer,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
    // challenge_vectors_carry_the_transfer_buyer_refusal
}

/// Contract: a context holding several refused values → the refusal for
///   the first in transcript order, the chain identifier before the
///   entitlement contract, the entitlements, the intervals, the seller,
///   and the buyer.
/// Arrange:  the mint context with `chain`
///   `CanonicalFieldValue::Unsigned64(1)` and `buyer`
///   `CanonicalFieldValue::Unsigned256([0x62; 32])`, so the chain
///   identifier and the buyer are both refused; the transfer context, the
///   declaration, and the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   ChainIdentifier { relation: DeliveryRelation::Mint, error })`,
///   `error` the literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_return_the_first_refused_form_in_transcript_order() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned64(1)),
        buyer: Some(CanonicalFieldValue::Unsigned256([0x62; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::ChainIdentifier {
            relation: DeliveryRelation::Mint,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_return_the_first_refused_form_in_transcript_order
}

/// Contract: both contexts hold a refused value → the refusal carries
///   `DeliveryRelation::Mint`, the mint context's forms rebuilt before
///   the transfer context's.
/// Arrange:  the mint context with `seller`
///   `CanonicalFieldValue::Unsigned256([0x61; 32])` and the transfer
///   context with `chain` `CanonicalFieldValue::Unsigned64(1)`, so the
///   transfer context's refusal is the earlier field and only a
///   mint-first order returns the mint refusal; the declaration and the
///   params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::Identity
///   { relation: DeliveryRelation::Mint, field:
///   DeliveryContextField::Seller, error })`, `error` the literal
///   `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::FixedBytes20 }`.
/// Collaborators: every family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_rebuild_the_mint_contexts_forms_before_the_transfer_contexts() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        seller: Some(CanonicalFieldValue::Unsigned256([0x61; 32])),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned64(1)),
        ..Default::default()
    });
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::Identity {
            relation: DeliveryRelation::Mint,
            field: DeliveryContextField::Seller,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::FixedBytes20,
            },
        })
    );
    // challenge_vectors_rebuild_the_mint_contexts_forms_before_the_transfer_contexts
}

/// Contract: a refused form → the form's refusal, before the random
///   source is called.
/// Arrange:  the mint context with `chain`
///   `CanonicalFieldValue::Unsigned64(1)`; the random source selected by
///   `RandomSourceKind::Mock(MockIRandomSourceAdapterFailureMode::
///   FillBytesRefused)`, so a draw would be refused and only a
///   forms-first order returns the form refusal; the transfer context,
///   the declaration, and the params at their defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   ChainIdentifier { relation: DeliveryRelation::Mint, error })`,
///   `error` the literal `FromFieldErrorReturn::WrongKind { expected:
///   CanonicalFieldKind::Unsigned256 }`.
/// Collaborators: the random family's mock concrete in its
///   `FillBytesRefused` mode through `create_random_source`; every other
///   family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_rebuild_every_form_before_any_draw() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(CanonicalDeliveryContextOverrides {
        chain: Some(CanonicalFieldValue::Unsigned64(1)),
        ..Default::default()
    });
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::ChainIdentifier {
            relation: DeliveryRelation::Mint,
            error: FromFieldErrorReturn::WrongKind {
                expected: CanonicalFieldKind::Unsigned256,
            },
        })
    );
    // challenge_vectors_rebuild_every_form_before_any_draw
}

/// Contract: the random source refuses a draw →
///   `Err(ChallengeVectorsErrorReturn::FillBytes(error))`, the family's
///   error unchanged.
/// Arrange:  the random source selected by `RandomSourceKind::Mock(
///   MockIRandomSourceAdapterFailureMode::FillBytesRefused)`; every other
///   collaborator at its defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   FillBytes(FillBytesErrorReturn::MockIRandomSourceAdapter))`.
/// Collaborators: the random family's mock concrete in its
///   `FillBytesRefused` mode through `create_random_source`; every other
///   family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_return_the_draw_refusal_whole() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(CreateRandomSourceParamsOverrides {
                    kind: Some(RandomSourceKind::Mock(
                        MockIRandomSourceAdapterFailureMode::FillBytesRefused,
                    )),
                }),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(Default::default()),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::FillBytes(
            FillBytesErrorReturn::MockIRandomSourceAdapter
        ))
    );
    // challenge_vectors_return_the_draw_refusal_whole
}

/// Contract: the pairing's sampling bound refuses the drawn uniform →
///   `Err(ChallengeVectorsErrorReturn::SampleScalar(error))`, the
///   pairing's error unchanged.
/// Arrange:  the pairing selected by `PairingConcrete::Mock(
///   MockIPairingAdapterFailureMode::SampleScalarWrongLength)`; every
///   other collaborator at its defaults.
/// Act:      the forms consumer's single call to `challenge_vectors`.
/// Assert:   the result equals `Err(ChallengeVectorsErrorReturn::
///   SampleScalar(SampleUniformScalarErrorReturn::WrongLength {
///   expected: 16, actual: 16 }))`, the values the pairing family's mock
///   concrete states.
/// Collaborators: the pairing family's mock concrete in its
///   `SampleScalarWrongLength` mode through `create_pairing`; every other
///   family's mock concrete in its succeeding mode.
#[test]
fn challenge_vectors_return_the_sampling_refusal_whole() {
    // Arrange
    let declaration = build_delivery_proof_declaration(Default::default());
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let challenge_params = build_challenge_vectors_params(Default::default());

    struct PairingConsumer {
        declaration: DeliveryProofDeclaration,
        mint: CanonicalDeliveryContext,
        transfer: CanonicalDeliveryContext,
        challenge_params: ChallengeVectorsParams,
    }

    impl IPairingConsumer for PairingConsumer {
        type Output = ChallengeVectorsReturn;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let hash_to_scalar = create_hash_to_scalar::<P::Scalar>(
                &build_create_hash_to_scalar_deps(Default::default()),
                build_create_hash_to_scalar_params(Default::default()),
                build_create_hash_to_scalar_payload(Default::default()),
            )
            .expect("the hash-to-scalar factory admits the selection");
            let random = create_random_source(
                &CreateRandomSourceDeps,
                build_create_random_source_params(Default::default()),
                CreateRandomSourcePayload,
            )
            .expect("the random-source factory admits the selection");

            struct EncodingConsumer<'a, P: IPairingAdapter> {
                pairing: &'a P,
                hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                random: &'a dyn IRandomSourceAdapter,
                declaration: &'a DeliveryProofDeclaration,
                mint: &'a CanonicalDeliveryContext,
                transfer: &'a CanonicalDeliveryContext,
                challenge_params: ChallengeVectorsParams,
            }

            impl<P: IPairingAdapter> IEncodingConsumer for EncodingConsumer<'_, P> {
                type Output = ChallengeVectorsReturn;

                fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
                    &self,
                    _params: ConsumeEncodingParams,
                    payload: ConsumeEncodingPayload<E>,
                ) -> Self::Output {
                    struct FormsConsumer<'a, P: IPairingAdapter, E: IEncoderAdapter> {
                        pairing: &'a P,
                        encoder: &'a E,
                        hash_to_scalar: &'a dyn IHashToScalarAdapter<P::Scalar>,
                        random: &'a dyn IRandomSourceAdapter,
                        declaration: &'a DeliveryProofDeclaration,
                        mint: &'a CanonicalDeliveryContext,
                        transfer: &'a CanonicalDeliveryContext,
                        challenge_params: ChallengeVectorsParams,
                    }

                    impl<P: IPairingAdapter, E: IEncoderAdapter> IChainFormsConsumer for FormsConsumer<'_, P, E> {
                        type Output = ChallengeVectorsReturn;

                        fn consume_chain_forms<F: IChainForms>(
                            &self,
                            _params: ConsumeChainFormsParams,
                            _payload: ConsumeChainFormsPayload<F>,
                        ) -> Self::Output {
                            // Act
                            challenge_vectors::<P, E, F>(
                                &build_challenge_vectors_deps(
                                    self.pairing,
                                    self.encoder,
                                    self.hash_to_scalar,
                                    self.random,
                                    Default::default(),
                                ),
                                self.challenge_params,
                                build_challenge_vectors_payload(
                                    self.declaration,
                                    self.mint,
                                    self.transfer,
                                    Default::default(),
                                ),
                            )
                        }
                    }

                    create_chain_forms(
                        &build_create_chain_forms_deps(
                            FormsConsumer {
                                pairing: self.pairing,
                                encoder: &payload.adapter,
                                hash_to_scalar: self.hash_to_scalar,
                                random: self.random,
                                declaration: self.declaration,
                                mint: self.mint,
                                transfer: self.transfer,
                                challenge_params: self.challenge_params,
                            },
                            Default::default(),
                        ),
                        build_create_chain_forms_params(Default::default()),
                        build_create_chain_forms_payload(Default::default()),
                    )
                    .expect("the chain-forms factory ran the consumer")
                    .output
                }
            }

            create_encoding(
                &build_create_encoding_deps(EncodingConsumer {
                    pairing: &payload.adapter,
                    hash_to_scalar: &*hash_to_scalar.adapter,
                    random: &*random.adapter,
                    declaration: &self.declaration,
                    mint: &self.mint,
                    transfer: &self.transfer,
                    challenge_params: self.challenge_params,
                }),
                build_create_encoding_params(Default::default()),
                build_create_encoding_payload(Default::default()),
            )
            .expect("the encoding factory ran the consumer")
            .output
        }
    }

    let result = create_pairing(
        &build_create_pairing_deps(
            PairingConsumer {
                declaration,
                mint,
                transfer,
                challenge_params,
            },
            Default::default(),
        ),
        build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::SampleScalarWrongLength,
            )),
            ..Default::default()
        }),
        CreatePairingPayload,
    )
    .expect("the pairing factory ran the consumer")
    .output;

    // Assert
    assert_eq!(
        result,
        Err(ChallengeVectorsErrorReturn::SampleScalar(
            SampleUniformScalarErrorReturn::WrongLength {
                expected: 16,
                actual: 16,
            }
        ))
    );
    // challenge_vectors_return_the_sampling_refusal_whole
}
