#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{Secret, SecretConstructorParams};
use encoding::{CreateEncodingParams, DECLARED_ENCODING_SELECTIONS};
use envelope::{
    CreateKeyAgreementParams, CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS,
    EnvelopeComponentsParams, EnvelopeComponentsPayload, EnvelopeFromComponentsParams,
    EnvelopeFromComponentsPayload, GenerateKeysErrorReturn, GenerateKeysParams,
    GenerateKeysPayload, IKeyAgreementAdapter, KeyAgreementConcrete, KeyPairComponentsParams,
    KeyPairComponentsPayload, KeyPairFromComponentsParams, KeyPairFromComponentsPayload,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    PossessionComponentsParams, PossessionComponentsPayload, PossessionFromComponentsParams,
    PossessionFromComponentsPayload, PublicKeysComponentsParams, PublicKeysComponentsPayload,
    PublicKeysFromComponentsParams, PublicKeysFromComponentsPayload, UnwrapParams, UnwrapPayload,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{CreateHashToScalarParams, DECLARED_HASH_TO_SCALAR_SELECTIONS};
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::{
    CreateKemParams, CredentialComponentsParams, CredentialComponentsPayload,
    CredentialFromComponentsParams, CredentialFromComponentsPayload, DECLARED_KEM_SELECTIONS,
    ICredentialKemAdapter, IsValidParams, IsValidPayload,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, CreatePairingParamsOverrides,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES, PairingConcrete,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveTransferErrorReturn,
    ProveTransferErrorReturn, TransferPurpose, VerifyParams, VerifyPayload,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParams, CreateRandomSourceParamsOverrides, IRandomSourceAdapter,
    RandomSourceKind, build_create_random_source_params,
};

use super::interface::{ITransferCaseConsumer, TransferErrorReturn};
use super::mock::{
    TransferCaseParamsOverrides, build_transfer_case_deps, build_transfer_case_params,
    build_transfer_case_payload, build_transfer_deps, build_transfer_params,
    build_transfer_payload, run_transfer_case,
};
use super::transfer;
use crate::delivery::mint::provides::{
    AdmittedKemIdentity, MintCaseParamsOverrides, MintPayloadOverrides, build_delivery_holder,
    build_mint_case_params, build_mint_deps, build_mint_params, build_mint_payload, mint,
};

/// Contract: `delivery/mint` entry
///   `the_recipient_opens_its_envelope_to_a_credential_valid_for_the_identity`,
///   absorbed: `transfer` over a mint delivery's recipient returns witnesses
///   whose `seller_secrets` open the recipient's envelope, through `unwrap`
///   under the key pair `key_pair_from_components` rebuilds from them, to
///   components that encode equal to those of the mint delivery's credential,
///   and the credential `transfer` delivers is valid under the mint
///   delivery's parameter set and identity element.
#[test]
fn the_recipient_opens_its_envelope_to_a_credential_valid_for_the_identity() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let rebuilt_key_pair = transfer_key_agreement
                .key_pair_from_components(
                    KeyPairFromComponentsParams,
                    KeyPairFromComponentsPayload {
                        components: delivery.witnesses.seller_secrets,
                    },
                )
                .expect("witness secrets rebuild a key pair")
                .key_pair;
            let Ok(opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &rebuilt_key_pair,
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(mint_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &mint_delivery.credential,
                },
            );
            let Ok(opened_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: opened.credential.a,
                },
            );
            let Ok(mint_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: mint_credential.components.a,
                },
            );
            assert_eq!(opened_a.bytes.as_ref(), mint_a.bytes.as_ref());
            let Ok(opened_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: opened.credential.b,
                },
            );
            let Ok(mint_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: mint_credential.components.b,
                },
            );
            assert_eq!(opened_b.bytes.as_ref(), mint_b.bytes.as_ref());
            let Ok(mint_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &mint_delivery.credential,
                },
            );
            let Ok(reconstructed) = kem.credential_from_components(
                CredentialFromComponentsParams,
                CredentialFromComponentsPayload {
                    components: mint_credential.components,
                },
            );
            let Ok(delivered_valid) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &mint_delivery.parameter_set,
                    identity_element: &mint_delivery.identity_element,
                    credential: &delivery.credential,
                },
            );
            assert!(delivered_valid.is_valid);
            let Ok(reconstructed_valid) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &mint_delivery.parameter_set,
                    identity_element: &mint_delivery.identity_element,
                    credential: &reconstructed.credential,
                },
            );
            assert!(reconstructed_valid.is_valid);
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
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
                                            random_source: Some(CreateRandomSourceParams {
                                                kind: RandomSourceKind::OperatingSystem,
                                            }),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // the_recipient_opens_its_envelope_to_a_credential_valid_for_the_identity
}

/// Contract: `random/factory` entry
///   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait`,
///   absorbed: `transfer` over the operating-system source returns draws of
///   three uniforms, each `UNIFORM_BYTES_LENGTH` bytes long and each differing
///   from every other.
#[test]
fn a_source_from_the_factory_draws_random_bytes_through_the_family_trait() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let uniforms: Vec<&[u8]> = delivery.draws.chunks(length).collect();
            for (index, uniform) in uniforms.iter().enumerate() {
                for (other_index, other) in uniforms.iter().enumerate() {
                    if index != other_index {
                        assert_ne!(uniform, other);
                    }
                }
            }
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
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
                                            random_source: Some(CreateRandomSourceParams {
                                                kind: RandomSourceKind::OperatingSystem,
                                            }),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // a_source_from_the_factory_draws_random_bytes_through_the_family_trait
}

/// Contract: `pairing/factory` entry `a_sampled_scalar_encodes_through_secret`,
///   absorbed: the witnesses' offset, read through `Secret::expose`, encodes
///   through `encode_scalar` to the encoding of the scalar the received scalar
///   type samples from the delivery's third uniform.
#[test]
fn a_sampled_scalar_encodes_through_secret() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let third = delivery
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH)
                .nth(2)
                .expect("three draws");
            let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                value: third.to_vec(),
            });
            let sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("a uniform of the declared length samples");
            let Ok(sampled_encoded) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            let Ok(offset_encoded) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: delivery.witnesses.offset.expose().clone(),
                },
            );
            assert_eq!(
                sampled_encoded.bytes.expose().as_ref(),
                offset_encoded.bytes.expose().as_ref()
            );
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
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
                                            random_source: Some(CreateRandomSourceParams {
                                                kind: RandomSourceKind::OperatingSystem,
                                            }),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // a_sampled_scalar_encodes_through_secret
}

/// Contract: `pairing/factory` entry and `kem/factory` carried entry
///   `the_consumer_receives_the_concrete_the_params_name`, absorbed: the
///   delivered credential's `b` encodes, through the pairing the case's params
///   name, equal to `add_g2` of the holder's opened credential's `b` and
///   `mul_g2` of that pairing's `g2_generator` by the offset witness, so every
///   adapter `transfer` composes computes over the received pairing.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
                    key_pair: &mint_delivery.recipient.key_pair,
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.offset.expose().clone(),
                },
            );
            let Ok(expected_b) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: opened.credential.b,
                    right: product.product,
                },
            );
            let Ok(expected_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: expected_b.sum,
                },
            );
            let Ok(delivered) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(delivered_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered.components.b,
                },
            );
            assert_eq!(
                expected_encoded.bytes.as_ref(),
                delivered_encoded.bytes.as_ref()
            );
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
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
                                            random_source: Some(CreateRandomSourceParams {
                                                kind: RandomSourceKind::OperatingSystem,
                                            }),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // the_consumer_receives_the_concrete_the_params_name
}

/// Contract: `kem/factory` entry
///   `every_concrete_rerandomizes_a_credential_to_one_valid_for_its_identity`,
///   absorbed: `is_valid` over `delivery.credential` and over the credential
///   opened from the holder, under the payload's parameter set and identity
///   element, each returns `true`, under each scope.
#[test]
fn every_concrete_rerandomizes_a_credential_to_one_valid_for_its_identity() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let Ok(delivered_valid) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &mint_delivery.parameter_set,
                    identity_element: &mint_delivery.identity_element,
                    credential: &delivery.credential,
                },
            );
            assert!(delivered_valid.is_valid);
            let Ok(opened_valid) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &mint_delivery.parameter_set,
                    identity_element: &mint_delivery.identity_element,
                    credential: &opened_credential.credential,
                },
            );
            assert!(opened_valid.is_valid);
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
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
                                            random_source: Some(CreateRandomSourceParams {
                                                kind: RandomSourceKind::OperatingSystem,
                                            }),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // every_concrete_rerandomizes_a_credential_to_one_valid_for_its_identity
}

/// Contract: `kem/factory` entry
///   `every_concrete_returns_the_offset_it_applied_under_rerandomization`,
///   absorbed: `credential_components` over the delivery's credential holds a
///   `b` that encodes, through `encode_g2`, equal to `add_g2` of the opened
///   credential's `b` and `mul_g2` of `g2` by the offset witness, under each
///   scope.
#[test]
fn every_concrete_returns_the_offset_it_applied_under_rerandomization() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
                    key_pair: &mint_delivery.recipient.key_pair,
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.offset.expose().clone(),
                },
            );
            let Ok(expected_b) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: opened.credential.b,
                    right: product.product,
                },
            );
            let Ok(expected_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: expected_b.sum,
                },
            );
            let Ok(delivered) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(delivered_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered.components.b,
                },
            );
            assert_eq!(
                expected_encoded.bytes.as_ref(),
                delivered_encoded.bytes.as_ref()
            );
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
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
                                            random_source: Some(CreateRandomSourceParams {
                                                kind: RandomSourceKind::OperatingSystem,
                                            }),
                                        }),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // every_concrete_returns_the_offset_it_applied_under_rerandomization
}

fn success_overrides(
    pairing_concrete: PairingConcrete,
    h2s_selection: CreateHashToScalarParams,
    encoding_selection: &CreateEncodingParams,
    forms_concrete: ChainFormsConcrete,
    kem_selection: CreateKemParams,
    key_agreement_selection: CreateKeyAgreementParams,
    proof_concrete: DeliveryProofConcrete,
) -> TransferCaseParamsOverrides {
    TransferCaseParamsOverrides {
        mint: Some(build_mint_case_params(MintCaseParamsOverrides {
            pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(pairing_concrete),
                ..Default::default()
            })),
            hash_to_scalar: Some(h2s_selection),
            encoding: Some(CreateEncodingParams {
                concrete: encoding_selection.concrete,
                identifier: encoding_selection.identifier,
            }),
            chain_forms: Some(build_create_chain_forms_params(
                CreateChainFormsParamsOverrides {
                    concrete: Some(forms_concrete),
                    ..Default::default()
                },
            )),
            kem: Some(kem_selection),
            key_agreement: Some(key_agreement_selection),
            delivery_proof: Some(build_create_delivery_proof_params(
                CreateDeliveryProofParamsOverrides {
                    concrete: Some(proof_concrete),
                    ..Default::default()
                },
            )),
            random_source: Some(CreateRandomSourceParams {
                kind: RandomSourceKind::OperatingSystem,
            }),
        })),
        key_agreement: Some(key_agreement_selection),
        delivery_proof: Some(build_create_delivery_proof_params(
            CreateDeliveryProofParamsOverrides {
                concrete: Some(proof_concrete),
                ..Default::default()
            },
        )),
        random_source: Some(CreateRandomSourceParams {
            kind: RandomSourceKind::OperatingSystem,
        }),
    }
}

/// Contract: `envelope/factory` entry
///   `every_key_agreement_forms_the_public_keys_its_secrets_form`, absorbed:
///   buyer secrets encode to the scalars sampled from the first and second
///   uniforms; `pk1`, `pk2` equal `g1·x`, `g2·y`.
#[test]
fn every_key_agreement_forms_the_public_keys_its_secrets_form() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let mut slices = delivery
                .draws
                .chunks(<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH);
            let Ok(x_uniform) = Secret::try_new(SecretConstructorParams {
                value: slices.next().expect("three draws").to_vec(),
            });
            let Ok(y_uniform) = Secret::try_new(SecretConstructorParams {
                value: slices.next().expect("three draws").to_vec(),
            });
            let x_sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform: x_uniform },
            )
            .expect("a uniform of the declared length samples")
            .scalar;
            let y_sampled = P::Scalar::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform: y_uniform },
            )
            .expect("a uniform of the declared length samples")
            .scalar;
            let Ok(secrets) = transfer_key_agreement.key_pair_components(
                KeyPairComponentsParams,
                KeyPairComponentsPayload {
                    key_pair: &delivery.buyer.key_pair,
                },
            );
            let Ok(x_encoded) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: secrets.components.x.expose().clone(),
                },
            );
            let Ok(x_sampled_encoded) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: x_sampled.expose().clone(),
                },
            );
            assert_eq!(
                x_encoded.bytes.expose().as_ref(),
                x_sampled_encoded.bytes.expose().as_ref()
            );
            let Ok(y_encoded) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: secrets.components.y.expose().clone(),
                },
            );
            let Ok(y_sampled_encoded) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: y_sampled.expose().clone(),
                },
            );
            assert_eq!(
                y_encoded.bytes.expose().as_ref(),
                y_sampled_encoded.bytes.expose().as_ref()
            );
            let Ok(keys) = transfer_key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &delivery.buyer.key_pair,
                },
            );
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(pk1_product) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: secrets.components.x.expose().clone(),
                },
            );
            let Ok(pk1_expected) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: pk1_product.product,
                },
            );
            let Ok(pk1_actual) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: keys.components.pk1,
                },
            );
            assert_eq!(pk1_expected.bytes.as_ref(), pk1_actual.bytes.as_ref());
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(pk2_product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: secrets.components.y.expose().clone(),
                },
            );
            let Ok(pk2_expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: pk2_product.product,
                },
            );
            let Ok(pk2_actual) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: keys.components.pk2,
                },
            );
            assert_eq!(pk2_expected.bytes.as_ref(), pk2_actual.bytes.as_ref());
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // every_key_agreement_forms_the_public_keys_its_secrets_form
}

/// Contract: `envelope/factory` entry
///   `public_keys_are_admitted_with_their_own_proof_of_possession`, absorbed:
///   `public_keys_components` over the buyer's admitted public keys encode
///   equal to `key_pair_public_keys` over its key pair.
#[test]
fn public_keys_are_admitted_with_their_own_proof_of_possession() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let Ok(admitted) = transfer_key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &delivery.buyer.public_keys,
                },
            );
            let Ok(keys) = transfer_key_agreement.key_pair_public_keys(
                KeyPairPublicKeysParams,
                KeyPairPublicKeysPayload {
                    key_pair: &delivery.buyer.key_pair,
                },
            );
            let Ok(admitted_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: admitted.components.pk1,
                },
            );
            let Ok(keys_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: keys.components.pk1,
                },
            );
            assert_eq!(admitted_pk1.bytes.as_ref(), keys_pk1.bytes.as_ref());
            let Ok(admitted_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: admitted.components.pk2,
                },
            );
            let Ok(keys_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: keys.components.pk2,
                },
            );
            assert_eq!(admitted_pk2.bytes.as_ref(), keys_pk2.bytes.as_ref());
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // public_keys_are_admitted_with_their_own_proof_of_possession
}

/// Contract: `envelope/factory` entry
///   `every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped`,
///   absorbed: `unwrap` of the buyer's envelope under its key pair returns
///   components that encode equal to `credential_components` over the
///   delivery's credential, and `unwrap` of the holder's envelope under the
///   holder's key pair returns components that encode equal to those of the
///   mint delivery's credential.
#[test]
fn every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let Ok(buyer_opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &delivery.buyer.key_pair,
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(delivered) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(buyer_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: buyer_opened.credential.a,
                },
            );
            let Ok(delivered_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: delivered.components.a,
                },
            );
            assert_eq!(buyer_a.bytes.as_ref(), delivered_a.bytes.as_ref());
            let Ok(buyer_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: buyer_opened.credential.b,
                },
            );
            let Ok(delivered_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivered.components.b,
                },
            );
            assert_eq!(buyer_b.bytes.as_ref(), delivered_b.bytes.as_ref());
            let Ok(holder_opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &mint_delivery.recipient.key_pair,
                    envelope: &mint_delivery.recipient.envelope,
                },
            );
            let Ok(mint_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &mint_delivery.credential,
                },
            );
            let Ok(holder_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: holder_opened.credential.a,
                },
            );
            let Ok(mint_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: mint_credential.components.a,
                },
            );
            assert_eq!(holder_a.bytes.as_ref(), mint_a.bytes.as_ref());
            let Ok(holder_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: holder_opened.credential.b,
                },
            );
            let Ok(mint_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: mint_credential.components.b,
                },
            );
            assert_eq!(holder_b.bytes.as_ref(), mint_b.bytes.as_ref());
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped
}

/// Contract: `envelope/factory` entry
///   `every_wrap_returns_the_coins_its_envelope_was_formed_with`, absorbed:
///   `c1 == g1·rho`, `c2 == cred.a + pk1·rho`, `d1 == g2·sigma`,
///   `d2 == cred.b + pk2·sigma`.
#[test]
fn every_wrap_returns_the_coins_its_envelope_was_formed_with() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let Ok(envelope) = transfer_key_agreement.envelope_components(
                EnvelopeComponentsParams,
                EnvelopeComponentsPayload {
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(buyer_keys) = transfer_key_agreement.public_keys_components(
                PublicKeysComponentsParams,
                PublicKeysComponentsPayload {
                    public_keys: &delivery.buyer.public_keys,
                },
            );
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(c1_product) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: delivery.witnesses.coins.rho.expose().clone(),
                },
            );
            let Ok(c1_expected) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: c1_product.product,
                },
            );
            let Ok(c1_actual) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: envelope.components.c1,
                },
            );
            assert_eq!(c1_expected.bytes.as_ref(), c1_actual.bytes.as_ref());
            let Ok(c2_term) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: buyer_keys.components.pk1,
                    scalar: delivery.witnesses.coins.rho.expose().clone(),
                },
            );
            let Ok(c2_expected) = pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: credential.components.a,
                    right: c2_term.product,
                },
            );
            let Ok(c2_encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: c2_expected.sum,
                },
            );
            let Ok(c2_actual) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: envelope.components.c2,
                },
            );
            assert_eq!(c2_encoded.bytes.as_ref(), c2_actual.bytes.as_ref());
            let Ok(d1_product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.coins.sigma.expose().clone(),
                },
            );
            let Ok(d1_expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: d1_product.product,
                },
            );
            let Ok(d1_actual) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: envelope.components.d1,
                },
            );
            assert_eq!(d1_expected.bytes.as_ref(), d1_actual.bytes.as_ref());
            let Ok(d2_term) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: buyer_keys.components.pk2,
                    scalar: delivery.witnesses.coins.sigma.expose().clone(),
                },
            );
            let Ok(d2_expected) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: credential.components.b,
                    right: d2_term.product,
                },
            );
            let Ok(d2_encoded) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: d2_expected.sum,
                },
            );
            let Ok(d2_actual) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: envelope.components.d2,
                },
            );
            assert_eq!(d2_encoded.bytes.as_ref(), d2_actual.bytes.as_ref());
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // every_wrap_returns_the_coins_its_envelope_was_formed_with
}

/// Contract: `envelope/factory` entry
///   `a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair`,
///   absorbed: `unwrap` of the buyer's envelope under the buyer's key pair and
///   under the holder's key pair, `credential_from_components` over each opened
///   pair, and `is_valid` return `true` for the buyer's opening and `false` for
///   the holder's.
#[test]
fn a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            let Ok(buyer_opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &delivery.buyer.key_pair,
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(buyer_credential) = kem.credential_from_components(
                CredentialFromComponentsParams,
                CredentialFromComponentsPayload {
                    components: buyer_opened.credential,
                },
            );
            let Ok(buyer_valid) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &mint_delivery.parameter_set,
                    identity_element: &mint_delivery.identity_element,
                    credential: &buyer_credential.credential,
                },
            );
            assert!(buyer_valid.is_valid);
            let Ok(holder_opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &mint_delivery.recipient.key_pair,
                    envelope: &delivery.buyer.envelope,
                },
            );
            let Ok(holder_credential) = kem.credential_from_components(
                CredentialFromComponentsParams,
                CredentialFromComponentsPayload {
                    components: holder_opened.credential,
                },
            );
            let Ok(holder_valid) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &mint_delivery.parameter_set,
                    identity_element: &mint_delivery.identity_element,
                    credential: &holder_credential.credential,
                },
            );
            assert!(!holder_valid.is_valid);
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair
}

/// Contract: `envelope/factory` entry `a_refused_nonce_draw_returns_the_mock_error_whole`,
///   absorbed: `transfer` over a case whose key agreement is the family's mock
///   concrete under `NonceDrawRefused`, a holder rebuilt on it from a mint
///   delivery formed under the case's own key agreement, returns
///   `Err(TransferErrorReturn::GenerateKeys(GenerateKeysErrorReturn::Mock(
///   MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused)))`;
///   within it `a_key_pair_restored_from_its_components_opens_the_envelope`,
///   `an_envelope_rebuilt_from_its_components_opens`, and
///   `a_possession_rebuilt_from_its_components_is_admitted` are proven.
#[test]
fn a_refused_nonce_draw_returns_the_mock_error_whole() {
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
            let Ok(opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &rebuilt_key_pair,
                    envelope: &rebuilt_envelope.envelope,
                },
            );
            let Ok(mint_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &mint_delivery.credential,
                },
            );
            let Ok(opened_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: opened.credential.a,
                },
            );
            let Ok(mint_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: mint_credential.components.a,
                },
            );
            assert_eq!(opened_a.bytes.as_ref(), mint_a.bytes.as_ref());
            let Ok(opened_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: opened.credential.b,
                },
            );
            let Ok(mint_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: mint_credential.components.b,
                },
            );
            assert_eq!(opened_b.bytes.as_ref(), mint_b.bytes.as_ref());
            let holder = build_delivery_holder(
                rebuilt_key_pair,
                rebuilt_public_keys,
                rebuilt_envelope.envelope,
                Default::default(),
            );
            let context =
                build_delivery_context::<F, TransferPurpose>(self.purpose, Default::default());
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

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
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
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }),
                build_transfer_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            assert!(returned.is_ok());
        }
    }
    // a_refused_nonce_draw_returns_the_mock_error_whole
}

/// Contract: `envelope/factory` entry `a_refused_coin_draw_returns_the_mock_error_whole`,
///   absorbed: as the nonce block under `CoinDrawRefused`, returns
///   `Err(TransferErrorReturn::WrapTo(WrapToErrorReturn::Mock(
///   MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused)))`; the same
///   rebuild entries are proven within it.
#[test]
fn a_refused_coin_draw_returns_the_mock_error_whole() {
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
            let Ok(opened) = transfer_key_agreement.unwrap(
                UnwrapParams,
                UnwrapPayload {
                    key_pair: &rebuilt_key_pair,
                    envelope: &rebuilt_envelope.envelope,
                },
            );
            let Ok(mint_credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &mint_delivery.credential,
                },
            );
            let Ok(opened_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: opened.credential.a,
                },
            );
            let Ok(mint_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: mint_credential.components.a,
                },
            );
            assert_eq!(opened_a.bytes.as_ref(), mint_a.bytes.as_ref());
            let Ok(opened_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: opened.credential.b,
                },
            );
            let Ok(mint_b) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: mint_credential.components.b,
                },
            );
            assert_eq!(opened_b.bytes.as_ref(), mint_b.bytes.as_ref());
            let holder = build_delivery_holder(
                rebuilt_key_pair,
                rebuilt_public_keys,
                rebuilt_envelope.envelope,
                Default::default(),
            );
            let context =
                build_delivery_context::<F, TransferPurpose>(self.purpose, Default::default());
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
            assert_eq!(
                returned.err(),
                Some(TransferErrorReturn::WrapTo(WrapToErrorReturn::Mock(
                    MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused
                )))
            );
        }
    }

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
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
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }),
                build_transfer_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            assert!(returned.is_ok());
        }
    }
    // a_refused_coin_draw_returns_the_mock_error_whole
}

/// Contract: `proof/factory` entry
///   `an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version`,
///   absorbed: inside the consumer `transfer` runs in, `D::DECLARATION.algebras`
///   holds `A::DECLARATION.algebra`, `D::DECLARATION.verifier_forms` holds the
///   verifier form the case's pairing declares, and
///   `D::DECLARATION.statement_versions` holds the version the case's selection
///   names, and the delivery's statement and proof `verify` accepts.
#[test]
fn an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version() {
    struct Consumer {
        purpose: TransferPurpose,
        statement_version: u16,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            assert!(D2::DECLARATION.algebras.contains(&A2::DECLARATION.algebra));
            assert!(
                D2::DECLARATION
                    .verifier_forms
                    .contains(&P::DECLARATION.verifier_group_arithmetic)
            );
            assert!(
                D2::DECLARATION
                    .statement_versions
                    .contains(&self.statement_version)
            );
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
                                    let statement_version = build_create_delivery_proof_params(
                                        CreateDeliveryProofParamsOverrides {
                                            concrete: Some(*proof_concrete),
                                            ..Default::default()
                                        },
                                    )
                                    .statement_version;
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer {
                                                purpose,
                                                statement_version,
                                            },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version
}

/// Contract: `proof/factory` entry `a_transfer_proof_the_adapter_returns_verifies`,
///   absorbed: `verify` over `AlgebraicStatement::Transfer(&delivery.statement)`
///   and `&delivery.proof` returns `is_valid` `true`, for each of
///   `TransferPurpose::Transfer` and `TransferPurpose::Grant`.
#[test]
fn a_transfer_proof_the_adapter_returns_verifies() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // a_transfer_proof_the_adapter_returns_verifies
}

/// Contract: `proof/factory` entry
///   `a_credential_rerandomized_and_wrapped_to_the_buyer_is_proved_by_a_transfer_proof_that_verifies_and_opens`,
///   absorbed: the delivery's proof verifies against its statement, and the new
///   envelope the statement names opens under the buyer's key pair to a
///   credential `is_valid` accepts under the payload's parameter set and
///   identity element.
#[test]
fn a_credential_rerandomized_and_wrapped_to_the_buyer_is_proved_by_a_transfer_proof_that_verifies_and_opens()
 {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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
            assert!(valid.is_valid);
        }
    }

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
                                    let returned = run_transfer_case(
                                        &build_transfer_case_deps(
                                            Consumer { purpose },
                                            Default::default(),
                                        ),
                                        build_transfer_case_params(success_overrides(
                                            *pairing_concrete,
                                            *h2s_selection,
                                            encoding_selection,
                                            *forms_concrete,
                                            *kem_selection,
                                            *key_agreement_selection,
                                            *proof_concrete,
                                        )),
                                        build_transfer_case_payload(
                                            b"pkg@1.0.0#1",
                                            &root,
                                            Default::default(),
                                        ),
                                    );
                                    assert!(returned.is_ok());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // a_credential_rerandomized_and_wrapped_to_the_buyer_is_proved_by_a_transfer_proof_that_verifies_and_opens
}

/// Contract: `proof/factory` entry `a_refused_nonce_draw_is_returned_by_each_prover`,
///   absorbed: `transfer` over a case whose delivery proof is the family's mock
///   concrete under `NonceDrawRefused`, a holder from a mint over the case's own
///   delivery proof, returns `Err(TransferErrorReturn::ProveTransfer(
///   ProveTransferErrorReturn::Mock(
///   MockIDeliveryProofAdapterProveTransferErrorReturn::NonceDrawRefused)))`.
#[test]
fn a_refused_nonce_draw_is_returned_by_each_prover() {
    struct Consumer {
        purpose: TransferPurpose,
    }

    impl ITransferCaseConsumer for Consumer {
        type Output = ();

        fn consume_transfer_case<P, F, K, A1, A2, D1, D2>(
            &self,
            _pairing: &P,
            kem: &K,
            _key_agreement: &A1,
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

    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
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
                    delivery_proof: Some(build_create_delivery_proof_params(
                        CreateDeliveryProofParamsOverrides {
                            concrete: Some(DeliveryProofConcrete::Mock(
                                MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }),
                build_transfer_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            assert!(returned.is_ok());
        }
    }
    // a_refused_nonce_draw_is_returned_by_each_prover
}
