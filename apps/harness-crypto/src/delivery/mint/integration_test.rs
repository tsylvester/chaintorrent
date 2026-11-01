#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{Secret, SecretConstructorParams};
use encoding::{CreateEncodingParams, DECLARED_ENCODING_SELECTIONS};
use envelope::{
    CreateKeyAgreementParamsOverrides, DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeComponentsParams,
    EnvelopeComponentsPayload, GenerateKeysErrorReturn, GenerateKeysParams, GenerateKeysPayload,
    IKeyAgreementAdapter, KeyAgreementConcrete, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    PublicKeysComponentsParams, PublicKeysComponentsPayload, UnwrapParams, UnwrapPayload,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
    DomainTagConstructorParamsOverrides, HashToScalarConcrete, HashToScalarErrorReturn,
    HashToScalarIdentifier, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params, build_domain_tag, expected_uniform_bytes,
};
use hashing::{Root, RootConstructorParamsOverrides, build_root};
use kem::{
    CredentialComponentsParams, CredentialComponentsPayload, CredentialFromComponentsPayload,
    DECLARED_KEM_SELECTIONS, DecapsulateParams, DecapsulatePayload, DeriveIdentityErrorReturn,
    EncapsulateParams, EncapsulatePayload, ICredentialKemAdapter, IdentityElementComponentsParams,
    IdentityElementComponentsPayload, IdentityScope, IsWellFormedParams, IsWellFormedPayload,
    KemIdentity, ParameterSetComponentsParams, ParameterSetComponentsPayload, SetupErrorReturn,
    SetupScope, build_credential_from_components_params, build_is_valid_params,
    build_is_valid_payload,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, CreatePairingParamsOverrides,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES, SampleUniformScalarErrorReturn,
    SampleUniformScalarParams, SampleUniformScalarPayload, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, VerifyParams, VerifyPayload, build_create_delivery_proof_params,
    build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayload, IRandomSourceAdapter,
    RandomSourceKind, build_create_random_source_params,
};

use super::interface::{AdmittedKemIdentity, IMintCaseConsumer, MintErrorReturn};
use super::mint;
use super::mock::{
    AdmittedKemIdentityOverrides, MintCaseParamsOverrides, MintPayloadOverrides,
    build_admitted_kem_identity, build_mint_case_deps, build_mint_case_params,
    build_mint_case_payload, build_mint_deps, build_mint_params, build_mint_payload, run_mint_case,
};

/// Contract: `random/factory` entry
///   `a_source_from_the_factory_draws_random_bytes_through_the_family_trait`,
///   absorbed: `mint` over the operating-system source returns draws of six
///   uniforms, each `UNIFORM_BYTES_LENGTH` bytes long and each differing from
///   every other.
/// Arrange: every case of the private surface — every member of
///   `PAIRING_CONCRETES` crossed with every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS`, `DECLARED_ENCODING_SELECTIONS`,
///   `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS` with each scope,
///   `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES`.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  `delivery.draws` is six slices of `UNIFORM_BYTES_LENGTH` bytes
///   and every slice differs from every other.
/// Boundary: `mint` and the chain of real functions the private surface
///   names — `create_random_source` for the operating-system kind,
///   `create_pairing`, `create_hash_to_scalar`, `create_encoding`,
///   `create_chain_forms`, `create_kem`, `create_key_agreement`,
///   `create_delivery_proof`, and every adapter method the block calls, all
///   running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_source_from_the_factory_draws_random_bytes_through_the_family_trait() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(delivery.draws.len(), 6 * length);
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

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // a_source_from_the_factory_draws_random_bytes_through_the_family_trait
}

/// Contract: `pairing/factory` entry `a_sampled_scalar_encodes_through_secret`,
///   absorbed: the witnesses' issuance randomness, read through
///   `Secret::expose`, encodes through `encode_scalar` to the encoding of the
///   scalar the received scalar type samples from the delivery's fourth
///   uniform.
/// Arrange: every case of the private surface; the randomness mock concrete's
///   successive draws differ, so a randomness relayed without sampling reports
///   otherwise.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  `encode_scalar` over `delivery.witnesses.credential_randomness`'s
///   exposed scalar equals `encode_scalar` over the scalar
///   `sample_from_uniform_bytes` returns for the delivery's fourth
///   `UNIFORM_BYTES_LENGTH` slice.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_sampled_scalar_encodes_through_secret() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                value: delivery.draws[3 * length..4 * length].to_vec(),
            });
            let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("a draw of the declared length samples");
            let Ok(expected) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            let Ok(actual) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: delivery.witnesses.credential_randomness.expose().clone(),
                },
            );
            assert_eq!(
                actual.bytes.expose().as_ref(),
                expected.bytes.expose().as_ref()
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // a_sampled_scalar_encodes_through_secret
}

/// Contract: `pairing/factory` entry and `kem/factory` carried entry
///   `the_consumer_receives_the_concrete_the_params_name`, absorbed: the
///   statement's `hpub` encodes, through the pairing the case's params name,
///   equal to `mul_g2` of that pairing's `g2_generator` by the master-scalar
///   witness, so every adapter `mint` composes computes over the received
///   pairing.
/// Arrange: every case of the private surface, each case member naming its
///   own pairing concrete.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  `encode_g2` of `delivery.statement.hpub` equals `encode_g2` of
///   `mul_g2` of the handed pairing's `g2_generator` by
///   `delivery.witnesses.master_scalar.value`'s exposed scalar.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(witnessed) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.master_scalar.value.expose().clone(),
                },
            );
            let Ok(expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: witnessed.product,
                },
            );
            let Ok(actual) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: delivery.statement.hpub,
                },
            );
            assert_eq!(actual.bytes.as_ref(), expected.bytes.as_ref());
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // the_consumer_receives_the_concrete_the_params_name
}

/// Contract: `kem/factory` entry
///   `every_concrete_issues_a_credential_valid_for_its_identity`, absorbed:
///   `is_valid` over `delivery.credential` under `delivery.parameter_set` and
///   `delivery.identity_element` returns `true`, under each scope.
/// Arrange: every case of the private surface, each scope of
///   `DECLARED_KEM_SELECTIONS` running.
/// Act:     `mint`, then `is_valid` on the handed KEM, in the case consumer's
///   method.
/// Assert:  `is_valid` returns `is_valid` `true`.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_issues_a_credential_valid_for_its_identity() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let verdict = kem
                .is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &delivery.parameter_set,
                        &delivery.identity_element,
                        &delivery.credential,
                        Default::default(),
                    ),
                )
                .expect("a validity verdict is infallible");
            assert!(verdict.is_valid);
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_concrete_issues_a_credential_valid_for_its_identity
}

/// Contract: `kem/factory` entry
///   `every_concrete_binds_the_public_value_to_the_master_scalar`, absorbed:
///   `parameter_set_components` over the delivery's parameter set holds an
///   `hpub` that encodes, through `encode_g2`, equal to `mul_g2` of `g2` by
///   the master-scalar witness, under each scope.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `parameter_set_components` and `mul_g2` on the
///   handed adapters, in the case consumer's method.
/// Assert:  `encode_g2` of the parameter set's `hpub` equals `encode_g2` of
///   `mul_g2` of its `g2` by `delivery.witnesses.master_scalar.value`'s
///   exposed scalar.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_binds_the_public_value_to_the_master_scalar() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(witnessed) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: parameter_set.components.g2,
                    scalar: delivery.witnesses.master_scalar.value.expose().clone(),
                },
            );
            let Ok(expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: witnessed.product,
                },
            );
            let Ok(actual) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: parameter_set.components.hpub,
                },
            );
            assert_eq!(actual.bytes.as_ref(), expected.bytes.as_ref());
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_concrete_binds_the_public_value_to_the_master_scalar
}

/// Contract: `kem/factory` entry
///   `every_concrete_returns_the_randomness_its_credential_was_formed_with`,
///   absorbed: `credential_components` over the delivery's credential holds a
///   `b` that encodes, through `encode_g2`, equal to `mul_g2` of `g2` by the
///   issuance-randomness witness, under each scope.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `credential_components`, `parameter_set_components`,
///   and `mul_g2` on the handed adapters, in the case consumer's method.
/// Assert:  `encode_g2` of the credential's `b` equals `encode_g2` of
///   `mul_g2` of the parameter set's `g2` by
///   `delivery.witnesses.credential_randomness`'s exposed scalar.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_returns_the_randomness_its_credential_was_formed_with() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let Ok(credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(witnessed) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: parameter_set.components.g2,
                    scalar: delivery.witnesses.credential_randomness.expose().clone(),
                },
            );
            let Ok(expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: witnessed.product,
                },
            );
            let Ok(actual) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: credential.components.b,
                },
            );
            assert_eq!(actual.bytes.as_ref(), expected.bytes.as_ref());
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_concrete_returns_the_randomness_its_credential_was_formed_with
}

/// Contract: `kem/factory` entry
///   `every_concrete_maps_an_identity_under_its_declared_tag`, absorbed:
///   under the entitlement scope, `identity_element_components` over the
///   delivery's identity element holds a `scalar` that encodes to the
///   encoding of the scalar the received scalar type samples from
///   `expected_uniform_bytes` over the tag built from
///   `K::DECLARATION.identity_tag`, the identity's canonical bytes, and
///   `UNIFORM_BYTES_LENGTH`, and an `element` that encodes, through
///   `encode_g1`, equal to `add_g1` of the parameter set's `u0` and `mul_g1`
///   of its `u1` by that scalar.
/// Arrange: every case of the private surface whose hash-to-scalar selection
///   declares `HashToScalarIdentifier::Keccak256V1` and whose KEM selection is
///   under the entitlement scope.
/// Act:     `mint`, then `identity_element_components` and
///   `parameter_set_components` on the handed KEM, in the case consumer's
///   method.
/// Assert:  `encode_scalar` of the identity element's `scalar` equals
///   `encode_scalar` of the sampled expected scalar; `encode_g1` of its
///   `element` equals `encode_g1` of `add_g1` of the parameter set's `u0` and
///   `mul_g1` of its `u1` by that scalar.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_maps_an_identity_under_its_declared_tag() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            assert!(matches!(identity.identity, KemIdentity::Entitlement { .. }));
            let KemIdentity::Entitlement { canonical } = identity.identity else {
                return;
            };
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                value: expected_uniform_bytes(
                    &tag,
                    canonical,
                    <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                ),
            });
            let expected_scalar = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("the expected uniform bytes are of the declared length")
            .scalar;
            let Ok(identity_element) = kem.identity_element_components(
                IdentityElementComponentsParams,
                IdentityElementComponentsPayload {
                    identity_element: &delivery.identity_element,
                },
            );
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
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(scaled_u1) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: parameter_set.components.u1,
                    scalar: expected_scalar.expose().clone(),
                },
            );
            let Ok(expected_element) = pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: parameter_set.components.u0,
                    right: scaled_u1.product,
                },
            );
            let Ok(expected_encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: expected_element.sum,
                },
            );
            let Ok(actual_encoded) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: identity_element.components.element,
                },
            );
            assert_eq!(
                actual_encoded.bytes.as_ref(),
                expected_encoded.bytes.as_ref()
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
            .iter()
            .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
        {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS
                        .iter()
                        .filter(|selection| selection.scope == IdentityScope::Entitlement)
                    {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_concrete_maps_an_identity_under_its_declared_tag
}

/// Contract: `kem/factory` entry
///   `every_concrete_derives_under_the_asset_scope_only_the_identity_the_set_fixes`,
///   absorbed: under the asset scope, `mint` over a setup scope and an
///   identity of one hash returns a delivery whose statement and proof
///   `verify` accepts, and over a setup scope of one hash and an identity of
///   another returns `Err(MintErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::OutsideAssetScope))`.
/// Arrange: every case of the private surface under the asset scope; the
///   block's `identity_hash` and a second, distinct root the consumer builds
///   the mismatched `AdmittedKemIdentity` over.
/// Act:     `mint` twice, once over the runner's admitted identity and once
///   over the mismatched one, then `verify` over the delivery's statement
///   and proof, in the case consumer's method.
/// Assert:  the first call's delivery verifies `is_valid` `true`; the second
///   call returns the error whole.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_derives_under_the_asset_scope_only_the_identity_the_set_fixes() {
    struct Consumer<'a> {
        other_root: &'a Root,
    }

    impl IMintCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            let mismatched = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                            setup_scope: Some(identity.setup_scope),
                            identity: Some(KemIdentity::Asset {
                                root: self.other_root,
                            }),
                        })),
                    },
                ),
            );
            // Assert
            let verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Mint(&delivery.statement),
                        proof: &delivery.proof,
                    },
                )
                .expect("a verification verdict is returned");
            assert!(verdict.is_valid);
            assert_eq!(
                mismatched.err(),
                Some(MintErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::OutsideAssetScope
                ))
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    let other_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x45; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS
                        .iter()
                        .filter(|selection| selection.scope == IdentityScope::Asset)
                    {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(
                                        Consumer {
                                            other_root: &other_root,
                                        },
                                        Default::default(),
                                    ),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_concrete_derives_under_the_asset_scope_only_the_identity_the_set_fixes
}

/// Contract: `kem/factory` entry
///   `every_concrete_refuses_an_identity_of_the_other_scope`, absorbed:
///   `mint` over an admitted identity whose setup scope and identity are of
///   different scopes, in each direction, returns
///   `Err(MintErrorReturn::DeriveIdentity(
///   DeriveIdentityErrorReturn::WrongIdentityScope))`, the function admitting
///   nothing itself.
/// Arrange: every case of the private surface, the family declaring both
///   scopes; the consumer builds each mismatched `AdmittedKemIdentity` inside
///   `consume_mint_case` with `build_admitted_kem_identity`.
/// Act:     `mint` twice, once per direction of the mismatch, in the case
///   consumer's method.
/// Assert:  each call returns the error whole.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_refuses_an_identity_of_the_other_scope() {
    struct Consumer<'a> {
        canonical: &'a [u8],
        root: &'a Root,
    }

    impl IMintCaseConsumer for Consumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            _identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            let call = |admitted: AdmittedKemIdentity<'_>| {
                mint(
                    &build_mint_deps(
                        kem,
                        key_agreement,
                        delivery_proof,
                        random,
                        Default::default(),
                    ),
                    build_mint_params(Default::default()),
                    build_mint_payload(
                        &context,
                        MintPayloadOverrides {
                            identity: Some(admitted),
                        },
                    ),
                )
            };
            // Act
            let entitlement_setup =
                call(build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                    setup_scope: Some(SetupScope::Entitlement),
                    identity: Some(KemIdentity::Asset { root: self.root }),
                }));
            let asset_setup = call(build_admitted_kem_identity(AdmittedKemIdentityOverrides {
                setup_scope: Some(SetupScope::Asset { root: self.root }),
                identity: Some(KemIdentity::Entitlement {
                    canonical: self.canonical,
                }),
            }));
            // Assert
            assert_eq!(
                entitlement_setup.err(),
                Some(MintErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::WrongIdentityScope
                ))
            );
            assert_eq!(
                asset_setup.err(),
                Some(MintErrorReturn::DeriveIdentity(
                    DeriveIdentityErrorReturn::WrongIdentityScope
                ))
            );
        }
    }

    // Arrange
    let canonical: &[u8] = b"pkg@1.0.0#1";
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(
                                        Consumer {
                                            canonical,
                                            root: &root,
                                        },
                                        Default::default(),
                                    ),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(canonical, &root, Default::default()),
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
    // every_concrete_refuses_an_identity_of_the_other_scope
}

/// Contract: `kem/factory` entry
///   `every_concrete_returns_a_hash_to_scalar_refusal_whole`, absorbed:
///   `mint` over a case whose hash-to-scalar adapter is the family's mock
///   concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`,
///   selected by configuration, returns
///   `Err(MintErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::
///   HashToScalar(error)))` under the entitlement scope and
///   `Err(MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)))`
///   under the asset scope, `error`
///   `HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::
///   WrongLength { expected: UNIFORM_BYTES_LENGTH, actual: 0 })`, through
///   the function's own variant, unconverted.
/// Arrange: the hash-to-scalar mock selection under `SamplingRefused`;
///   every member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS`.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  the result equals the scope's error arm whole, the carried
///   sampling refusal stated field by field.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, running for real up to the selected family's edge.
/// Mocked:  the hash-to-scalar family's mock concrete under
///   `SamplingRefused`, selected by configuration in this block alone.
#[test]
fn every_concrete_returns_a_hash_to_scalar_refusal_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            let refusal =
                HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength {
                    expected: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    actual: 0,
                });
            match identity.setup_scope {
                SetupScope::Entitlement => assert_eq!(
                    returned.err(),
                    Some(MintErrorReturn::DeriveIdentity(
                        DeriveIdentityErrorReturn::HashToScalar(refusal)
                    ))
                ),
                SetupScope::Asset { .. } => assert_eq!(
                    returned.err(),
                    Some(MintErrorReturn::Setup(SetupErrorReturn::HashToScalar(
                        refusal
                    )))
                ),
            }
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for kem_selection in DECLARED_KEM_SELECTIONS {
            // Act
            let returned = run_mint_case(
                &build_mint_case_deps(Consumer, Default::default()),
                build_mint_case_params(MintCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    hash_to_scalar: Some(build_create_hash_to_scalar_params(
                        CreateHashToScalarParamsOverrides {
                            concrete: Some(HashToScalarConcrete::Mock(
                                MockIHashToScalarAdapterFailureMode::SamplingRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    kem: Some(*kem_selection),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_mint_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // every_concrete_returns_a_hash_to_scalar_refusal_whole
}

/// Contract: `kem/factory` entry
///   `a_kem_from_the_factory_round_trips_from_operating_system_draws`,
///   absorbed: `mint` over the operating-system source, then `encapsulate`
///   under `delivery.parameter_set` over a uniform `fill_bytes` returns from
///   that source for `UNIFORM_BYTES_LENGTH`; `is_valid` over the delivery's
///   credential and `is_well_formed` over the capsule return `true`, and
///   `decapsulate` with the delivery's identity element and credential
///   returns `key_material()` bytes equal to the bytes `encapsulate`
///   returned, under each scope.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `is_valid`, `encapsulate`, `is_well_formed`, and
///   `decapsulate` on the handed KEM, in the case consumer's method.
/// Assert:  both verdicts `true`; the decapsulated `key_material().expose()`
///   bytes equal the encapsulated ones.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_kem_from_the_factory_round_trips_from_operating_system_draws() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            let uniform = random
                .fill_bytes(
                    FillBytesParams,
                    FillBytesPayload {
                        length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
                    },
                )
                .expect("the operating-system source draws")
                .bytes;
            let encapsulated = kem
                .encapsulate(
                    EncapsulateParams,
                    EncapsulatePayload {
                        parameter_set: &delivery.parameter_set,
                        uniform,
                    },
                )
                .expect("the declared selection encapsulates");
            // Assert
            let valid = kem
                .is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &delivery.parameter_set,
                        &delivery.identity_element,
                        &delivery.credential,
                        Default::default(),
                    ),
                )
                .expect("a validity verdict is infallible");
            assert!(valid.is_valid);
            let well_formed = kem
                .is_well_formed(
                    IsWellFormedParams,
                    IsWellFormedPayload {
                        parameter_set: &delivery.parameter_set,
                        capsule: &encapsulated.capsule,
                    },
                )
                .expect("a well-formedness verdict is infallible");
            assert!(well_formed.is_well_formed);
            let decapsulated = kem
                .decapsulate(
                    DecapsulateParams,
                    DecapsulatePayload {
                        identity_element: &delivery.identity_element,
                        credential: &delivery.credential,
                        capsule: &encapsulated.capsule,
                    },
                )
                .expect("a decapsulation is infallible");
            assert_eq!(
                decapsulated.encapsulated.key_material().expose(),
                encapsulated.encapsulated.key_material().expose()
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // a_kem_from_the_factory_round_trips_from_operating_system_draws
}

/// Contract: `envelope/factory` entry
///   `every_key_agreement_forms_the_public_keys_its_secrets_form`, absorbed:
///   `key_pair_components` over the recipient's key pair holds secrets that
///   encode, through `encode_scalar`, to the encodings of the scalars the
///   received scalar type samples from the delivery's fifth and sixth
///   uniforms, and `key_pair_public_keys` holds a `pk1` and a `pk2` that
///   encode equal to `mul_g1` of `g1_generator` and `mul_g2` of
///   `g2_generator` by those scalars.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `key_pair_components`, `key_pair_public_keys`, and
///   the pairing's methods, in the case consumer's method.
/// Assert:  `encode_scalar` of each secret equals `encode_scalar` of the
///   scalar sampled from its uniform; `encode_g1` of `pk1` and `encode_g2`
///   of `pk2` equal the encodings of the generator multiples by those
///   scalars.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_key_agreement_forms_the_public_keys_its_secrets_form() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let sampled = |index: usize| -> Secret<P::Scalar> {
                let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                    value: delivery.draws[index * length..(index + 1) * length].to_vec(),
                });
                <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                    SampleUniformScalarParams,
                    SampleUniformScalarPayload { uniform },
                )
                .expect("a draw of the declared length samples")
                .scalar
            };
            let encoded_scalar = |scalar: &Secret<P::Scalar>| -> Vec<u8> {
                pairing
                    .encode_scalar(
                        EncodeScalarParams,
                        EncodeScalarPayload {
                            scalar: scalar.expose().clone(),
                        },
                    )
                    .expect("a scalar encoding is infallible")
                    .bytes
                    .expose()
                    .as_ref()
                    .to_vec()
            };
            let key_pair = key_agreement
                .key_pair_components(
                    KeyPairComponentsParams,
                    KeyPairComponentsPayload {
                        key_pair: &delivery.recipient.key_pair,
                    },
                )
                .expect("a key pair's components are infallible");
            let x = sampled(4);
            let y = sampled(5);
            assert_eq!(encoded_scalar(&key_pair.components.x), encoded_scalar(&x));
            assert_eq!(encoded_scalar(&key_pair.components.y), encoded_scalar(&y));
            let public = key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &delivery.recipient.key_pair,
                    },
                )
                .expect("a key pair's public keys are infallible");
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(expected_pk1) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: g1.point,
                    scalar: x.expose().clone(),
                },
            );
            let Ok(expected_pk2) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: y.expose().clone(),
                },
            );
            let Ok(expected_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: expected_pk1.product,
                },
            );
            let Ok(expected_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: expected_pk2.product,
                },
            );
            let Ok(actual_pk1) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: public.components.pk1,
                },
            );
            let Ok(actual_pk2) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: public.components.pk2,
                },
            );
            assert_eq!(actual_pk1.bytes.as_ref(), expected_pk1.bytes.as_ref());
            assert_eq!(actual_pk2.bytes.as_ref(), expected_pk2.bytes.as_ref());
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_key_agreement_forms_the_public_keys_its_secrets_form
}

/// Contract: `envelope/factory` entry
///   `public_keys_are_admitted_with_their_own_proof_of_possession`, absorbed:
///   `public_keys_components` over the recipient's admitted public keys
///   encode equal to `key_pair_public_keys` over its key pair.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `public_keys_components` and `key_pair_public_keys`
///   on the handed key agreement, in the case consumer's method.
/// Assert:  `encode_g1` of the admitted `pk1` and `encode_g2` of the
///   admitted `pk2` equal those of the key pair's.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn public_keys_are_admitted_with_their_own_proof_of_possession() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let admitted = key_agreement
                .public_keys_components(
                    PublicKeysComponentsParams,
                    PublicKeysComponentsPayload {
                        public_keys: &delivery.recipient.public_keys,
                    },
                )
                .expect("admitted public keys' components are infallible");
            let held = key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &delivery.recipient.key_pair,
                    },
                )
                .expect("a key pair's public keys are infallible");
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
                encode_g1(&admitted.components.pk1),
                encode_g1(&held.components.pk1)
            );
            assert_eq!(
                encode_g2(&admitted.components.pk2),
                encode_g2(&held.components.pk2)
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // public_keys_are_admitted_with_their_own_proof_of_possession
}

/// Contract: `envelope/factory` entry
///   `every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped`,
///   absorbed: `unwrap` of the recipient's envelope under its key pair
///   returns components that encode equal to `credential_components` over
///   the delivery's credential.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `unwrap` and `credential_components`, in the case
///   consumer's method.
/// Assert:  `encode_g1` of the opened `a` and `encode_g2` of the opened `b`
///   equal those of the delivered credential's components.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let opened = key_agreement
                .unwrap(
                    UnwrapParams,
                    UnwrapPayload {
                        key_pair: &delivery.recipient.key_pair,
                        envelope: &delivery.recipient.envelope,
                    },
                )
                .expect("an unwrap is infallible");
            let wrapped = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &delivery.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let Ok(opened_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: opened.credential.a,
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
                    point: opened.credential.b,
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
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_envelope_opens_under_its_key_pair_to_the_components_it_wrapped
}

/// Contract: `envelope/factory` entry
///   `every_wrap_returns_the_coins_its_envelope_was_formed_with`, absorbed:
///   `envelope_components` over the recipient's envelope holds `c1` equal to
///   `mul_g1` of `g1_generator` by the `rho` coin, `c2` equal to `add_g1` of
///   the credential's `a` and `mul_g1` of `pk1` by `rho`, `d1` equal to
///   `mul_g2` of `g2_generator` by the `sigma` coin, and `d2` equal to
///   `add_g2` of the credential's `b` and `mul_g2` of `pk2` by `sigma`, the
///   coins being `delivery.witnesses.envelope_coins`.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `envelope_components`, `credential_components`,
///   `key_pair_public_keys`, and the pairing's methods, in the case
///   consumer's method.
/// Assert:  each encoded component equals the encoding of the pairing
///   arithmetic the witness coins state.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_wrap_returns_the_coins_its_envelope_was_formed_with() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let Ok(g1) = pairing.g1_generator(G1GeneratorParams, G1GeneratorPayload);
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let credential = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &delivery.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let public = key_agreement
                .key_pair_public_keys(
                    KeyPairPublicKeysParams,
                    KeyPairPublicKeysPayload {
                        key_pair: &delivery.recipient.key_pair,
                    },
                )
                .expect("a key pair's public keys are infallible");
            let envelope = key_agreement
                .envelope_components(
                    EnvelopeComponentsParams,
                    EnvelopeComponentsPayload {
                        envelope: &delivery.recipient.envelope,
                    },
                )
                .expect("an envelope's components are infallible");
            let rho = delivery.witnesses.coins.rho.expose().clone();
            let sigma = delivery.witnesses.coins.sigma.expose().clone();
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
            let mul_g1 = |point: &P::G1, scalar: &P::Scalar| -> P::G1 {
                pairing
                    .mul_g1(
                        MulG1Params,
                        MulG1Payload {
                            point: point.clone(),
                            scalar: scalar.clone(),
                        },
                    )
                    .expect("a first-group multiplication is infallible")
                    .product
            };
            let mul_g2 = |point: &P::G2, scalar: &P::Scalar| -> P::G2 {
                pairing
                    .mul_g2(
                        MulG2Params,
                        MulG2Payload {
                            point: point.clone(),
                            scalar: scalar.clone(),
                        },
                    )
                    .expect("a second-group multiplication is infallible")
                    .product
            };
            let Ok(expected_c2) = pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: credential.components.a,
                    right: mul_g1(&public.components.pk1, &rho),
                },
            );
            let Ok(expected_d2) = pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: credential.components.b,
                    right: mul_g2(&public.components.pk2, &sigma),
                },
            );
            assert_eq!(
                encode_g1(&envelope.components.c1),
                encode_g1(&mul_g1(&g1.point, &rho))
            );
            assert_eq!(
                encode_g1(&envelope.components.c2),
                encode_g1(&expected_c2.sum)
            );
            assert_eq!(
                encode_g2(&envelope.components.d1),
                encode_g2(&mul_g2(&g2.point, &sigma))
            );
            assert_eq!(
                encode_g2(&envelope.components.d2),
                encode_g2(&expected_d2.sum)
            );
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // every_wrap_returns_the_coins_its_envelope_was_formed_with
}

/// Contract: `envelope/factory` entry
///   `a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair`,
///   absorbed: the recipient's envelope `unwrap`s under its key pair to
///   components `credential_from_components` rebuilds a credential `is_valid`
///   accepts under the delivery's parameter set and identity element, and
///   the same unwrap under a key pair `generate_keys` returns over draws
///   from the operating-system source builds a credential `is_valid`
///   refuses.
/// Arrange: every case of the private surface; the other key pair's draws
///   come from the case's operating-system source, distinct from the
///   recipient's.
/// Act:     `mint`, then `unwrap`, `credential_from_components`,
///   `generate_keys`, and `is_valid`, in the case consumer's method.
/// Assert:  the recipient's rebuilt credential is `is_valid` `true`; the
///   other key pair's is `false`.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let draw = |random: &dyn IRandomSourceAdapter| {
                random
                    .fill_bytes(FillBytesParams, FillBytesPayload { length })
                    .expect("the operating-system source draws")
                    .bytes
            };
            let other = key_agreement
                .generate_keys(
                    GenerateKeysParams,
                    GenerateKeysPayload {
                        x_uniform: draw(random),
                        y_uniform: draw(random),
                    },
                )
                .expect("the declared selection generates keys");
            // Assert
            let verdict_of = |key_pair: &A::KeyPair| {
                let opened = key_agreement
                    .unwrap(
                        UnwrapParams,
                        UnwrapPayload {
                            key_pair,
                            envelope: &delivery.recipient.envelope,
                        },
                    )
                    .expect("an unwrap is infallible");
                let rebuilt = kem
                    .credential_from_components(
                        build_credential_from_components_params(Default::default()),
                        CredentialFromComponentsPayload {
                            components: opened.credential,
                        },
                    )
                    .expect("a credential's reassembly is infallible");
                kem.is_valid(
                    build_is_valid_params(Default::default()),
                    build_is_valid_payload(
                        &delivery.parameter_set,
                        &delivery.identity_element,
                        &rebuilt.credential,
                        Default::default(),
                    ),
                )
                .expect("a validity verdict is infallible")
                .is_valid
            };
            assert!(verdict_of(&delivery.recipient.key_pair));
            assert!(!verdict_of(&other.key_pair));
        }
    }

    // Arrange
    let root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x44; 32]),
    });
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // a_delivered_credential_opens_valid_for_its_recipient_and_invalid_for_another_key_pair
}

/// Contract: `envelope/factory` entry
///   `a_refused_nonce_draw_returns_the_mock_error_whole`, absorbed: `mint`
///   over a case whose key-agreement adapter is the family's mock concrete
///   under `MockIKeyAgreementAdapterFailureMode::NonceDrawRefused`, selected
///   by configuration, returns
///   `Err(MintErrorReturn::GenerateKeys(GenerateKeysErrorReturn::
///   Mock(MockIKeyAgreementAdapterGenerateKeysErrorReturn::
///   NonceDrawRefused)))`, through the function's own variant, unconverted.
/// Arrange: the key-agreement mock selection under `NonceDrawRefused`;
///   every member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS`.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  the result equals the error arm whole.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, running for real up to the selected family's edge.
/// Mocked:  the key-agreement family's mock concrete under
///   `NonceDrawRefused`, selected by configuration in this block alone.
#[test]
fn a_refused_nonce_draw_returns_the_mock_error_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::GenerateKeys(
                    GenerateKeysErrorReturn::Mock(
                        MockIKeyAgreementAdapterGenerateKeysErrorReturn::NonceDrawRefused
                    )
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
            let returned = run_mint_case(
                &build_mint_case_deps(Consumer, Default::default()),
                build_mint_case_params(MintCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_mint_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_refused_nonce_draw_returns_the_mock_error_whole
}

/// Contract: `envelope/factory` entry
///   `a_refused_coin_draw_returns_the_mock_error_whole`, absorbed: `mint`
///   over a case whose key-agreement adapter is the family's mock concrete
///   under `MockIKeyAgreementAdapterFailureMode::CoinDrawRefused`, selected
///   by configuration, returns
///   `Err(MintErrorReturn::WrapTo(WrapToErrorReturn::
///   Mock(MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused)))`,
///   through the function's own variant, unconverted.
/// Arrange: the key-agreement mock selection under `CoinDrawRefused`; every
///   member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS`.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  the result equals the error arm whole.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, running for real up to the selected family's edge.
/// Mocked:  the key-agreement family's mock concrete under
///   `CoinDrawRefused`, selected by configuration in this block alone.
#[test]
fn a_refused_coin_draw_returns_the_mock_error_whole() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::WrapTo(WrapToErrorReturn::Mock(
                    MockIKeyAgreementAdapterWrapToErrorReturn::CoinDrawRefused
                )))
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
            let returned = run_mint_case(
                &build_mint_case_deps(Consumer, Default::default()),
                build_mint_case_params(MintCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    key_agreement: Some(build_create_key_agreement_params(
                        CreateKeyAgreementParamsOverrides {
                            concrete: Some(KeyAgreementConcrete::Mock(
                                MockIKeyAgreementAdapterFailureMode::CoinDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_mint_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_refused_coin_draw_returns_the_mock_error_whole
}

/// Contract: `proof/factory` entry
///   `an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version`,
///   absorbed: the `D::DECLARATION.algebras` of the delivery-proof adapter
///   the consumer is handed holds `A::DECLARATION.algebra`, its
///   `verifier_forms` holds the verifier form the case's pairing declares,
///   and its `statement_versions` holds the version the case's selection
///   names, and `mint` returns a delivery `verify` accepts.
/// Arrange: every case of the private surface, each case member's
///   statement version handed to the consumer with the case's delivery-proof
///   selection.
/// Act:     `mint`, then `verify` over the delivery's statement and proof,
///   in the case consumer's method.
/// Assert:  the three declaration contains each hold; the verdict is
///   `is_valid` `true`.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version() {
    struct Consumer {
        statement_version: u16,
    }

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            assert!(D::DECLARATION.algebras.contains(&A::DECLARATION.algebra));
            assert!(
                D::DECLARATION
                    .verifier_forms
                    .contains(&P::DECLARATION.verifier_group_arithmetic)
            );
            assert!(
                D::DECLARATION
                    .statement_versions
                    .contains(&self.statement_version)
            );
            let verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Mint(&delivery.statement),
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
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let delivery_proof = build_create_delivery_proof_params(
                                    CreateDeliveryProofParamsOverrides {
                                        concrete: Some(*proof_concrete),
                                        ..Default::default()
                                    },
                                );
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(
                                        Consumer {
                                            statement_version: delivery_proof.statement_version,
                                        },
                                        Default::default(),
                                    ),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // an_adapter_handed_to_the_consumer_declares_the_required_algebra_form_and_version
}

/// Contract: `proof/factory` entry
///   `a_mint_proof_the_adapter_returns_verifies`, absorbed: `verify` over
///   `AlgebraicStatement::Mint` of the delivery's statement and the
///   delivery's proof returns `is_valid` `true`.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `verify` on the handed delivery-proof adapter, in
///   the case consumer's method.
/// Assert:  the verdict is `is_valid` `true`.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_mint_proof_the_adapter_returns_verifies() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Mint(&delivery.statement),
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
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // a_mint_proof_the_adapter_returns_verifies
}

/// Contract: `proof/factory` entry
///   `a_credential_issued_and_wrapped_through_the_families_is_proved_by_a_mint_proof_that_verifies`,
///   absorbed: `mint` over an issued credential wrapped to a recipient
///   through the family adapters returns a delivery `verify` accepts, and
///   the envelope opens to the credential's own components.
/// Arrange: every case of the private surface.
/// Act:     `mint`, then `verify` and `unwrap`, in the case consumer's
///   method.
/// Assert:  the verdict is `is_valid` `true`; the opened components encode
///   equal to `credential_components` over the delivery's credential.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, all running for real.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_credential_issued_and_wrapped_through_the_families_is_proved_by_a_mint_proof_that_verifies() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            let success = returned.expect("the subject's calls succeed");
            let delivery = success.delivery;
            // Assert
            let verdict = delivery_proof
                .verify(
                    VerifyParams,
                    VerifyPayload {
                        statement: AlgebraicStatement::Mint(&delivery.statement),
                        proof: &delivery.proof,
                    },
                )
                .expect("a verification verdict is returned");
            assert!(verdict.is_valid);
            let opened = key_agreement
                .unwrap(
                    UnwrapParams,
                    UnwrapPayload {
                        key_pair: &delivery.recipient.key_pair,
                        envelope: &delivery.recipient.envelope,
                    },
                )
                .expect("an unwrap is infallible");
            let wrapped = kem
                .credential_components(
                    CredentialComponentsParams,
                    CredentialComponentsPayload {
                        credential: &delivery.credential,
                    },
                )
                .expect("a credential's components are infallible");
            let Ok(opened_a) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: opened.credential.a,
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
                    point: opened.credential.b,
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
    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                // Act
                                let returned = run_mint_case(
                                    &build_mint_case_deps(Consumer, Default::default()),
                                    build_mint_case_params(MintCaseParamsOverrides {
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
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        kem: Some(*kem_selection),
                                        key_agreement: Some(*key_agreement_selection),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*proof_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    }),
                                    build_mint_case_payload(
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
    // a_credential_issued_and_wrapped_through_the_families_is_proved_by_a_mint_proof_that_verifies
}

/// Contract: `proof/factory` entry
///   `a_refused_nonce_draw_is_returned_by_each_prover`, absorbed: `mint`
///   over a case whose delivery-proof adapter is the family's mock concrete
///   under `MockIDeliveryProofAdapterFailureMode::NonceDrawRefused`,
///   selected by configuration, returns
///   `Err(MintErrorReturn::ProveMint(ProveMintErrorReturn::
///   Mock(MockIDeliveryProofAdapterProveMintErrorReturn::
///   NonceDrawRefused)))`, through the function's own variant, unconverted.
/// Arrange: the delivery-proof mock selection under `NonceDrawRefused`;
///   every member of `PAIRING_CONCRETES` with each scope of
///   `DECLARED_KEM_SELECTIONS`.
/// Act:     `mint`, once per case, in the case consumer's method.
/// Assert:  the result equals the error arm whole.
/// Boundary: `mint` and the chain of real functions the private surface
///   names, running for real up to the selected family's edge.
/// Mocked:  the delivery-proof family's mock concrete under
///   `NonceDrawRefused`, selected by configuration in this block alone.
#[test]
fn a_refused_nonce_draw_is_returned_by_each_prover() {
    struct Consumer;

    impl IMintCaseConsumer for Consumer {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identity: AdmittedKemIdentity<'_>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let context =
                build_delivery_context::<F, MintPurpose>(MintPurpose::Mint, Default::default());
            // Act
            let returned = mint(
                &build_mint_deps(
                    kem,
                    key_agreement,
                    delivery_proof,
                    random,
                    Default::default(),
                ),
                build_mint_params(Default::default()),
                build_mint_payload(
                    &context,
                    MintPayloadOverrides {
                        identity: Some(identity),
                    },
                ),
            );
            // Assert
            assert_eq!(
                returned.err(),
                Some(MintErrorReturn::ProveMint(ProveMintErrorReturn::Mock(
                    MockIDeliveryProofAdapterProveMintErrorReturn::NonceDrawRefused
                )))
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
            let returned = run_mint_case(
                &build_mint_case_deps(Consumer, Default::default()),
                build_mint_case_params(MintCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing_concrete),
                        ..Default::default()
                    })),
                    kem: Some(*kem_selection),
                    delivery_proof: Some(build_create_delivery_proof_params(
                        CreateDeliveryProofParamsOverrides {
                            concrete: Some(DeliveryProofConcrete::Mock(
                                MockIDeliveryProofAdapterFailureMode::NonceDrawRefused,
                            )),
                            ..Default::default()
                        },
                    )),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                }),
                build_mint_case_payload(b"pkg@1.0.0#1", &root, Default::default()),
            );
            // Assert
            assert!(returned.is_ok());
        }
    }
    // a_refused_nonce_draw_is_returned_by_each_prover
}
