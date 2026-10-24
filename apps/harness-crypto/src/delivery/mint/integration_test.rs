#![allow(clippy::expect_used)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainFormsConcrete, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use domain::{
    ASSET_IDENTITY_HASH_LENGTH, AssetIdentityHash, AssetIdentityHashConstructorParamsOverrides,
    Secret, SecretConstructorParams, build_asset_identity_hash,
};
use encoding::{
    CreateEncodingParams, CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS,
    build_create_encoding_params,
};
use envelope::{
    CreateKeyAgreementParams, CreateKeyAgreementParamsOverrides,
    DECLARED_KEY_AGREEMENT_SELECTIONS, EnvelopeComponentsParams, EnvelopeComponentsPayload,
    GenerateKeysErrorReturn, GenerateKeysParams, GenerateKeysPayload, IKeyAgreementAdapter,
    KeyAgreementConcrete, KeyPairComponentsParams, KeyPairComponentsPayload,
    KeyPairPublicKeysParams, KeyPairPublicKeysPayload, MockIKeyAgreementAdapterFailureMode,
    MockIKeyAgreementAdapterGenerateKeysErrorReturn, MockIKeyAgreementAdapterWrapToErrorReturn,
    PublicKeysComponentsParams, PublicKeysComponentsPayload, UnwrapParams, UnwrapPayload,
    WrapToErrorReturn, build_create_key_agreement_params,
};
use hash_to_scalar::{
    CreateHashToScalarParams, CreateHashToScalarParamsOverrides,
    DECLARED_HASH_TO_SCALAR_SELECTIONS, DomainTagConstructorParamsOverrides, HashToScalarConcrete,
    HashToScalarErrorReturn, HashToScalarIdentifier, MockIHashToScalarAdapterFailureMode,
    build_create_hash_to_scalar_params, build_domain_tag, expected_uniform_bytes,
};
use kem::{
    CreateKemParams, CredentialComponentsParams, CredentialComponentsPayload,
    CredentialFromComponentsParams, CredentialFromComponentsPayload, DECLARED_KEM_SELECTIONS,
    DecapsulateParams, DecapsulatePayload, DeriveIdentityErrorReturn, DeriveIdentityParams,
    DeriveIdentityPayload, EncapsulateParams, EncapsulatePayload, ICredentialKemAdapter,
    IdentityElementComponentsParams, IdentityElementComponentsPayload, IdentityScope, IsValidParams,
    IsValidPayload, IsWellFormedParams, IsWellFormedPayload, IssueParams, IssuePayload, KemIdentity,
    ParameterSetComponentsParams, ParameterSetComponentsPayload, SetupErrorReturn, SetupParams,
    SetupPayload, SetupScope,
};
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, CreatePairingParamsOverrides,
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams,
    G2GeneratorPayload, IPairingArithmetic, IPairingReference, ISampleUniformScalar, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, PAIRING_CONCRETES, PairingConcrete,
    SampleUniformScalarErrorReturn, SampleUniformScalarParams, SampleUniformScalarPayload,
    VerifierGroupArithmetic, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DeliveryProofConcrete, IDeliveryProofAdapter, MintPurpose,
    MockIDeliveryProofAdapterFailureMode, MockIDeliveryProofAdapterProveMintErrorReturn,
    ProveMintErrorReturn, ProveMintParams, ProveMintPayload, VerifyParams, VerifyPayload,
    build_create_delivery_proof_params, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, FillBytesParams, FillBytesPayload, IRandomSourceAdapter,
    RandomSourceKind, build_create_random_source_params,
};

use super::interface::{AdmittedKemIdentity, MintErrorReturn};
use super::mint;
use super::mock::{
    AdmittedKemIdentityOverrides, IMintCaseConsumer, MintCaseParams, MintPayloadOverrides,
    build_admitted_kem_identity, build_mint_deps, build_mint_params, build_mint_payload,
    run_mint_case,
};

fn case_params(
    pairing_concrete: PairingConcrete,
    hash_to_scalar: CreateHashToScalarParams,
    encoding: &CreateEncodingParams,
    chain_forms: ChainFormsConcrete,
    kem: CreateKemParams,
    key_agreement: CreateKeyAgreementParams,
    delivery_proof: DeliveryProofConcrete,
    random_source: RandomSourceKind,
) -> MintCaseParams {
    let verifier_form = match pairing_concrete {
        PairingConcrete::Bn254Arkworks | PairingConcrete::Bn254Halo2curves => {
            VerifierGroupArithmetic::FirstGroupOnly
        }
        PairingConcrete::Bls12381Arkworks | PairingConcrete::Bls12381Halo2curves => {
            VerifierGroupArithmetic::BothGroups
        }
        PairingConcrete::Mock(_) => VerifierGroupArithmetic::FirstGroupOnly,
    };
    MintCaseParams {
        pairing: build_create_pairing_params(CreatePairingParamsOverrides {
            concrete: Some(pairing_concrete),
            ..Default::default()
        }),
        hash_to_scalar,
        encoding: build_create_encoding_params(CreateEncodingParamsOverrides {
            concrete: Some(encoding.concrete),
            identifier: Some(encoding.identifier),
        }),
        chain_forms: build_create_chain_forms_params(CreateChainFormsParamsOverrides {
            concrete: Some(chain_forms),
            ..Default::default()
        }),
        kem,
        key_agreement,
        delivery_proof: build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
            concrete: Some(delivery_proof),
            verifier_form: Some(verifier_form),
            ..Default::default()
        }),
        random_source: build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(random_source),
        }),
    }
}

fn case_identity<'a>(
    scope: IdentityScope,
    hash: &'a AssetIdentityHash,
) -> AdmittedKemIdentity<'a> {
    match scope {
        IdentityScope::Entitlement => build_admitted_kem_identity(Default::default()),
        IdentityScope::Asset => build_admitted_kem_identity(AdmittedKemIdentityOverrides {
            setup_scope: Some(SetupScope::Asset {
                identity_hash: hash,
            }),
            identity: Some(KemIdentity::Asset {
                identity_hash: hash,
            }),
        }),
    }
}

/// Contract: entry `a_source_from_the_factory_draws_random_bytes_through_the_family_trait`,
///   absorbed — `mint` over the operating-system source returns draws of six
///   uniforms, each `UNIFORM_BYTES_LENGTH` bytes long and each differing
///   from every other.
/// Arrange: every case of the private surface.
/// Act:     `mint` over `RandomSourceKind::OperatingSystem`, once per case.
/// Assert:  `delivery.draws` is `6 * UNIFORM_BYTES_LENGTH` long, and no two
///   `UNIFORM_BYTES_LENGTH` slices are equal.
/// Boundary: `create_random_source` for the operating-system kind, the real
///   factory chain, and `mint`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_source_from_the_factory_draws_random_bytes_through_the_family_trait() {
    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
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
                        identity: Some(self.identity),
                    },
                ),
            );
            let delivery = returned.expect("the subject's calls succeed").delivery;
            // Assert
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            assert_eq!(delivery.draws.len(), 6 * length);
            for first in 0..6 {
                for second in (first + 1)..6 {
                    assert_ne!(
                        &delivery.draws[first * length..(first + 1) * length],
                        &delivery.draws[second * length..(second + 1) * length]
                    );
                }
            }
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let hash = build_asset_identity_hash(
                                    AssetIdentityHashConstructorParamsOverrides {
                                        bytes: Some([0x44; ASSET_IDENTITY_HASH_LENGTH]),
                                    },
                                );
                                run_mint_case(
                                    case_params(
                                        *pairing_concrete,
                                        *h2s_selection,
                                        encoding_selection,
                                        *forms_concrete,
                                        *kem_selection,
                                        *key_agreement_selection,
                                        *proof_concrete,
                                        RandomSourceKind::OperatingSystem,
                                    ),
                                    CaseConsumer {
                                        identity: case_identity(kem_selection.scope, &hash),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // a_source_from_the_factory_draws_random_bytes_through_the_family_trait
}

/// Contract: entry `a_sampled_scalar_encodes_through_secret`, absorbed — the
///   witnesses' issuance randomness, read through `Secret::expose`, encodes
///   through `encode_scalar` to the encoding of the scalar the received
///   scalar type samples from the delivery's fourth uniform.
/// Arrange: every case of the private surface.
/// Act:     `mint` over the operating-system source, once per case, then
///   `sample_from_uniform_bytes` over the fourth draw slice and
///   `encode_scalar` over both scalars.
/// Assert:  the encodings' bytes are equal.
/// Boundary: the real factory chain, `mint`, `sample_from_uniform_bytes`,
///   `encode_scalar`, and `Secret::expose`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn a_sampled_scalar_encodes_through_secret() {
    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
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
                        identity: Some(self.identity),
                    },
                ),
            );
            let delivery = returned.expect("the subject's calls succeed").delivery;
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                value: delivery.draws[3 * length..4 * length].to_vec(),
            });
            let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("a uniform of the declared length samples");
            // Assert
            let Ok(witnessed) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: delivery.witnesses.credential_randomness.expose().clone(),
                },
            );
            let Ok(expected) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            assert_eq!(
                witnessed.bytes.expose().as_ref(),
                expected.bytes.expose().as_ref()
            );
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let hash = build_asset_identity_hash(
                                    AssetIdentityHashConstructorParamsOverrides {
                                        bytes: Some([0x44; ASSET_IDENTITY_HASH_LENGTH]),
                                    },
                                );
                                run_mint_case(
                                    case_params(
                                        *pairing_concrete,
                                        *h2s_selection,
                                        encoding_selection,
                                        *forms_concrete,
                                        *kem_selection,
                                        *key_agreement_selection,
                                        *proof_concrete,
                                        RandomSourceKind::OperatingSystem,
                                    ),
                                    CaseConsumer {
                                        identity: case_identity(kem_selection.scope, &hash),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // a_sampled_scalar_encodes_through_secret
}

/// Contract: entry `the_consumer_receives_the_concrete_the_params_name`,
///   absorbed — the statement's `hpub` encodes, through the pairing the
///   case's params name, equal to `mul_g2` of that pairing's `g2_generator`
///   by the master-scalar witness.
/// Arrange: every case of the private surface.
/// Act:     `mint` over the operating-system source, once per case.
/// Assert:  `encode_g2` of `parameter_set_components`' `hpub` equals
///   `encode_g2` of `mul_g2` of `g2_generator` by
///   `delivery.witnesses.master_scalar.value`.
/// Boundary: the real factory chain and `mint`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn the_consumer_receives_the_concrete_the_params_name() {
    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
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
                        identity: Some(self.identity),
                    },
                ),
            );
            let delivery = returned.expect("the subject's calls succeed").delivery;
            // Assert
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(delivered) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: parameter_set.components.hpub,
                },
            );
            let Ok(product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.master_scalar.value.expose().clone(),
                },
            );
            let Ok(expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: product.product,
                },
            );
            assert_eq!(delivered.bytes.as_ref(), expected.bytes.as_ref());
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let hash = build_asset_identity_hash(
                                    AssetIdentityHashConstructorParamsOverrides {
                                        bytes: Some([0x44; ASSET_IDENTITY_HASH_LENGTH]),
                                    },
                                );
                                run_mint_case(
                                    case_params(
                                        *pairing_concrete,
                                        *h2s_selection,
                                        encoding_selection,
                                        *forms_concrete,
                                        *kem_selection,
                                        *key_agreement_selection,
                                        *proof_concrete,
                                        RandomSourceKind::OperatingSystem,
                                    ),
                                    CaseConsumer {
                                        identity: case_identity(kem_selection.scope, &hash),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // the_consumer_receives_the_concrete_the_params_name
}

/// Contract: entry `every_concrete_issues_a_credential_valid_for_its_identity`,
///   absorbed — `is_valid` over `delivery.credential` under
///   `delivery.parameter_set` and `delivery.identity_element` returns `true`,
///   under each scope.
/// Arrange: every case of the private surface.
/// Act:     `mint` over the operating-system source, then `is_valid`, once
///   per case.
/// Assert:  `is_valid` is `true`.
/// Boundary: the real factory chain, `mint`, and `is_valid`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_issues_a_credential_valid_for_its_identity() {
    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            _pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
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
                        identity: Some(self.identity),
                    },
                ),
            );
            let delivery = returned.expect("the subject's calls succeed").delivery;
            // Assert
            let Ok(verdict) = kem.is_valid(
                IsValidParams,
                IsValidPayload {
                    parameter_set: &delivery.parameter_set,
                    identity_element: &delivery.identity_element,
                    credential: &delivery.credential,
                },
            );
            assert!(verdict.is_valid);
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let hash = build_asset_identity_hash(
                                    AssetIdentityHashConstructorParamsOverrides {
                                        bytes: Some([0x44; ASSET_IDENTITY_HASH_LENGTH]),
                                    },
                                );
                                run_mint_case(
                                    case_params(
                                        *pairing_concrete,
                                        *h2s_selection,
                                        encoding_selection,
                                        *forms_concrete,
                                        *kem_selection,
                                        *key_agreement_selection,
                                        *proof_concrete,
                                        RandomSourceKind::OperatingSystem,
                                    ),
                                    CaseConsumer {
                                        identity: case_identity(kem_selection.scope, &hash),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // every_concrete_issues_a_credential_valid_for_its_identity
}

/// Contract: entry `every_concrete_binds_the_public_value_to_the_master_scalar`,
///   absorbed — `parameter_set_components` over the delivery's parameter set
///   holds an `hpub` that encodes, through `encode_g2`, equal to `mul_g2` of
///   `g2` by the master-scalar witness, under each scope.
/// Arrange: every case of the private surface.
/// Act:     `mint` over the operating-system source, once per case.
/// Assert:  `encode_g2` of the parameter set's `hpub` equals `encode_g2` of
///   `mul_g2` of `g2_generator` by `delivery.witnesses.master_scalar.value`.
/// Boundary: the real factory chain and `mint`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_binds_the_public_value_to_the_master_scalar() {
    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
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
                        identity: Some(self.identity),
                    },
                ),
            );
            let delivery = returned.expect("the subject's calls succeed").delivery;
            // Assert
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(delivered) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: parameter_set.components.hpub,
                },
            );
            let Ok(product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.master_scalar.value.expose().clone(),
                },
            );
            let Ok(expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: product.product,
                },
            );
            assert_eq!(delivered.bytes.as_ref(), expected.bytes.as_ref());
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let hash = build_asset_identity_hash(
                                    AssetIdentityHashConstructorParamsOverrides {
                                        bytes: Some([0x44; ASSET_IDENTITY_HASH_LENGTH]),
                                    },
                                );
                                run_mint_case(
                                    case_params(
                                        *pairing_concrete,
                                        *h2s_selection,
                                        encoding_selection,
                                        *forms_concrete,
                                        *kem_selection,
                                        *key_agreement_selection,
                                        *proof_concrete,
                                        RandomSourceKind::OperatingSystem,
                                    ),
                                    CaseConsumer {
                                        identity: case_identity(kem_selection.scope, &hash),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // every_concrete_binds_the_public_value_to_the_master_scalar
}

/// Contract: entry `every_concrete_returns_the_randomness_its_credential_was_formed_with`,
///   absorbed — `credential_components` over the delivery's credential holds
///   a `b` that encodes, through `encode_g2`, equal to `mul_g2` of `g2` by
///   the issuance-randomness witness, under each scope.
/// Arrange: every case of the private surface.
/// Act:     `mint` over the operating-system source, once per case.
/// Assert:  `encode_g2` of the credential's `b` equals `encode_g2` of
///   `mul_g2` of `g2_generator` by
///   `delivery.witnesses.credential_randomness`.
/// Boundary: the real factory chain and `mint`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_returns_the_randomness_its_credential_was_formed_with() {
    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
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
                        identity: Some(self.identity),
                    },
                ),
            );
            let delivery = returned.expect("the subject's calls succeed").delivery;
            // Assert
            let Ok(g2) = pairing.g2_generator(G2GeneratorParams, G2GeneratorPayload);
            let Ok(credential) = kem.credential_components(
                CredentialComponentsParams,
                CredentialComponentsPayload {
                    credential: &delivery.credential,
                },
            );
            let Ok(delivered) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: credential.components.b,
                },
            );
            let Ok(product) = pairing.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: g2.point,
                    scalar: delivery.witnesses.credential_randomness.expose().clone(),
                },
            );
            let Ok(expected) = pairing.encode_g2(
                EncodeG2Params,
                EncodeG2Payload {
                    point: product.product,
                },
            );
            assert_eq!(delivered.bytes.as_ref(), expected.bytes.as_ref());
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                let hash = build_asset_identity_hash(
                                    AssetIdentityHashConstructorParamsOverrides {
                                        bytes: Some([0x44; ASSET_IDENTITY_HASH_LENGTH]),
                                    },
                                );
                                run_mint_case(
                                    case_params(
                                        *pairing_concrete,
                                        *h2s_selection,
                                        encoding_selection,
                                        *forms_concrete,
                                        *kem_selection,
                                        *key_agreement_selection,
                                        *proof_concrete,
                                        RandomSourceKind::OperatingSystem,
                                    ),
                                    CaseConsumer {
                                        identity: case_identity(kem_selection.scope, &hash),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // every_concrete_returns_the_randomness_its_credential_was_formed_with
}

/// Contract: entry `every_concrete_maps_an_identity_under_its_declared_tag`,
///   absorbed — under the entitlement scope, `identity_element_components`
///   over the delivery's identity element holds a `scalar` that encodes to
///   the encoding of the scalar the received scalar type samples from
///   `expected_uniform_bytes` over the tag built from
///   `K::DECLARATION.identity_tag`, the identity's canonical bytes, and
///   `UNIFORM_BYTES_LENGTH`, and an `element` that encodes, through
///   `encode_g1`, equal to `add_g1` of the parameter set's `u0` and `mul_g1`
///   of its `u1` by that scalar; over each case of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declares
///   `HashToScalarIdentifier::Keccak256V1`.
/// Arrange: every case whose hash-to-scalar selection declares
///   `Keccak256V1`, under the entitlement scope.
/// Act:     `mint` over the operating-system source, once per case.
/// Assert:  the identity element's `scalar` encodes to the expected
///   sampling's encoding and its `element` encodes to `add_g1` of `u0` and
///   `mul_g1` of `u1` by it.
/// Boundary: the real factory chain, `mint`, `expected_uniform_bytes`,
///   `sample_from_uniform_bytes`, and `DomainTag::try_new`.
/// Mocked:  nothing; the operating-system generator runs real.
#[test]
fn every_concrete_maps_an_identity_under_its_declared_tag() {
    struct CaseConsumer<'a> {
        identity: AdmittedKemIdentity<'a>,
        canonical: &'a [u8],
    }

    impl IMintCaseConsumer for CaseConsumer<'_> {
        type Output = ();

        fn consume_mint_case<P, F, K, A, D>(
            &self,
            pairing: &P,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
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
                        identity: Some(self.identity),
                    },
                ),
            );
            let delivery = returned.expect("the subject's calls succeed").delivery;
            let tag = build_domain_tag(DomainTagConstructorParamsOverrides {
                bytes: Some(K::DECLARATION.identity_tag.to_vec()),
            });
            let length = <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH;
            let Ok(uniform) = Secret::try_new(SecretConstructorParams {
                value: expected_uniform_bytes(&tag, self.canonical, length),
            });
            let sampled = <P::Scalar as ISampleUniformScalar>::sample_from_uniform_bytes(
                SampleUniformScalarParams,
                SampleUniformScalarPayload { uniform },
            )
            .expect("a uniform of the declared length samples");
            // Assert
            let Ok(components) = kem.identity_element_components(
                IdentityElementComponentsParams,
                IdentityElementComponentsPayload {
                    identity_element: &delivery.identity_element,
                },
            );
            let Ok(derived_scalar) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: components.components.scalar.clone(),
                },
            );
            let Ok(expected_scalar) = pairing.encode_scalar(
                EncodeScalarParams,
                EncodeScalarPayload {
                    scalar: sampled.scalar.expose().clone(),
                },
            );
            assert_eq!(
                derived_scalar.bytes.expose().as_ref(),
                expected_scalar.bytes.expose().as_ref()
            );
            let Ok(parameter_set) = kem.parameter_set_components(
                ParameterSetComponentsParams,
                ParameterSetComponentsPayload {
                    parameter_set: &delivery.parameter_set,
                },
            );
            let Ok(product) = pairing.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: parameter_set.components.u1,
                    scalar: components.components.scalar,
                },
            );
            let Ok(sum) = pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: parameter_set.components.u0,
                    right: product.product,
                },
            );
            let Ok(expected_element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload { point: sum.sum },
            );
            let Ok(derived_element) = pairing.encode_g1(
                EncodeG1Params,
                EncodeG1Payload {
                    point: components.components.element,
                },
            );
            assert_eq!(
                derived_element.bytes.as_ref(),
                expected_element.bytes.as_ref()
            );
        }
    }

    for pairing_concrete in PAIRING_CONCRETES {
        for h2s_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
            if h2s_selection.identifier != HashToScalarIdentifier::Keccak256V1 {
                continue;
            }
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for forms_concrete in CHAIN_FORMS_CONCRETES {
                    for kem_selection in DECLARED_KEM_SELECTIONS {
                        if kem_selection.scope != IdentityScope::Entitlement {
                            continue;
                        }
                        for key_agreement_selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                            for proof_concrete in DELIVERY_PROOF_CONCRETES {
                                run_mint_case(
                                    case_params(
                                        *pairing_concrete,
                                        *h2s_selection,
                                        encoding_selection,
                                        *forms_concrete,
                                        *kem_selection,
                                        *key_agreement_selection,
                                        *proof_concrete,
                                        RandomSourceKind::OperatingSystem,
                                    ),
                                    CaseConsumer {
                                        identity: case_identity(
                                            kem_selection.scope,
                                            &build_asset_identity_hash(
                                                AssetIdentityHashConstructorParamsOverrides {
                                                    bytes: Some(
                                                        [0x44; ASSET_IDENTITY_HASH_LENGTH],
                                                    ),
                                                },
                                            ),
                                        ),
                                        canonical: b"pkg@1.0.0#1",
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // every_concrete_maps_an_identity_under_its_declared_tag
}
