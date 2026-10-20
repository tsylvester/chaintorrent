#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;
use std::collections::HashSet;

use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
};
use domain::{
    AssetIdentityConstructorParamsOverrides, build_asset_coordinate, build_asset_identity_hash,
};
use encoding::{
    ConsumeEncodingParams, ConsumeEncodingPayload, CreateEncodingParamsOverrides,
    DECLARED_ENCODING_SELECTIONS, IDecoderAdapter, IEncoderAdapter, IEncodingConsumer,
    build_create_encoding_deps, build_create_encoding_params, build_create_encoding_payload,
    create_encoding,
};
use envelope::{
    ConsumeKeyAgreementParams, ConsumeKeyAgreementPayload, DECLARED_KEY_AGREEMENT_SELECTIONS,
    EnvelopeAlgebra, IKeyAgreementAdapter, IKeyAgreementConsumer, build_create_key_agreement_deps,
    build_create_key_agreement_payload, create_key_agreement,
};
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    IHashToScalarAdapter, build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use kem::{
    ConsumeKemParams, ConsumeKemPayload, DECLARED_KEM_SELECTIONS, ICredentialKemAdapter,
    IKemConsumer, IdentityScope, build_create_kem_deps, build_create_kem_payload, create_kem,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides,
    CreatePairingPayload, IPairingArithmetic, IPairingConsumer, IPairingReference,
    PAIRING_CONCRETES, VerifierGroupArithmetic, build_create_pairing_deps,
    build_create_pairing_params, create_pairing,
};
use proof::{
    ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload, CreateDeliveryProofParamsOverrides,
    DELIVERY_PROOF_CONCRETES, DELIVERY_STATEMENT_VERSION_ONE, IDeliveryProofAdapter,
    IDeliveryProofConsumer, build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    IRandomSourceAdapter, RandomSourceKind, build_create_random_source_params,
    create_random_source,
};

use super::super::evm::provides::{CanonicalDeliveryContext, build_canonical_delivery_context};
use super::create_generate;
use super::interface::{
    ConsumeGenerateParams, ConsumeGeneratePayload, CreateGenerateReturn, EmitErrorReturn,
    EmitReturn, GENERATE_CONCRETES, GenerateConcrete, IGenerateAdapter, IGenerateConsumer,
    MockIGenerateAdapterEmitErrorReturn, MockIGenerateAdapterFailureMode,
};
use super::mock::{
    CreateGenerateParamsOverrides, GenerateConcreteOverrides, GenerateKindConfigurationsOverrides,
    build_create_generate_deps, build_create_generate_params, build_create_generate_payload,
    build_emit_params, build_emit_payload, build_generate_concrete,
    build_generate_kind_configurations,
};

/// Contract: entry
///   `emit_returns_a_distinct_path_for_every_kind_the_declaration_does_not_gate`;
///   `emit` on the adapter `create_generate` hands the consumer for a member
///   of `GENERATE_CONCRETES`, and for the mock selection under `NoFailure`,
///   with the payload `build_emit_payload` builds over two canonical
///   delivery contexts, two distinct identities, and one of them as the
///   proof identity → `Ok` whose `files` names no path twice and holds at
///   least as many paths as the declaration the consumer reads from the
///   adapter has kinds whose `requires_second_group_arithmetic` is `false`.
/// Arrange: `build_canonical_delivery_context` at its default for the mint
///   and the transfer, two distinct canonical identities no
///   `AssetIdentityHash` admits — `first` the `as_ref()` bytes of
///   `build_asset_coordinate` at its default and `second` those at version
///   `2.0.0` — as `identities`, `proof_identity` the first; the random
///   source `RandomSourceKind::OperatingSystem`; every case the private
///   surface names: each member of `GENERATE_CONCRETES` through
///   `build_generate_concrete` with `kinds` from
///   `build_generate_kind_configurations`, and the mock selection under
///   `NoFailure` with the params builder's default `kinds`, crossed with
///   every case of the other families; the Evm selection's settings root
///   removed before the act.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  the result is `Ok` whose `files` holds at least the ungated
///   count of paths and inserts every path into a set without a repeat.
/// Boundary: `create_generate`, the selected concrete's `try_new`, `emit`,
///   and the functions `emit` runs, each real.
/// Mocked:  nothing on the chain; the filesystem is a directory beneath the
///   operating system's temporary directory and the operating system's
///   generator runs real; the mock selection runs no filesystem.
#[test]
fn emit_returns_a_distinct_path_for_every_kind_the_declaration_does_not_gate() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let first = build_asset_coordinate(Default::default()).as_ref().to_vec();
    let second = build_asset_coordinate(AssetIdentityConstructorParamsOverrides {
        version: Some("2.0.0".to_string()),
        ..Default::default()
    })
    .as_ref()
    .to_vec();
    let identities = vec![first.clone(), second];
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct GenerateConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identities: &'a [Vec<u8>],
        proof_identity: &'a [u8],
    }

    impl<'a> IGenerateConsumer for &'a GenerateConsumer<'a> {
        type Output = ();

        fn consume_generate<G: IGenerateAdapter>(
            &self,
            _params: ConsumeGenerateParams,
            payload: ConsumeGeneratePayload<G>,
        ) -> Self::Output {
            // Act
            let result = payload.adapter.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    self.mint,
                    self.transfer,
                    self.identities,
                    self.proof_identity,
                    Default::default(),
                ),
            );

            // Assert
            let Ok(success) = result else {
                panic!("emit admits the case")
            };
            let ungated = G::DECLARATION
                .kinds
                .iter()
                .filter(|declaration| !declaration.requires_second_group_arithmetic)
                .count();
            assert!(
                success.files.len() >= ungated,
                "the files hold at least the ungated kinds' paths"
            );
            let mut paths = HashSet::new();
            for file in &success.files {
                assert!(paths.insert(&file.path), "the emit returns no path twice");
            }
        }
    }

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
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
            for member in GENERATE_CONCRETES {
                let concrete = build_generate_concrete(*member, GenerateConcreteOverrides);
                let GenerateConcrete::Evm { settings } = &concrete else {
                    panic!("the declared selection carries the Evm settings")
                };
                let _ = std::fs::remove_dir_all(&settings.foundry_root);
                let created = create_generate(
                    &build_create_generate_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        self.kem,
                        self.key_agreement,
                        &payload.adapter,
                        self.generate,
                        Default::default(),
                    ),
                    build_create_generate_params(CreateGenerateParamsOverrides {
                        concrete: Some(concrete),
                        kinds: Some(build_generate_kind_configurations(
                            *member,
                            GenerateKindConfigurationsOverrides,
                        )),
                        ..Default::default()
                    }),
                    build_create_generate_payload(Default::default()),
                );
                assert!(created.is_ok(), "create_generate admits the selection");
            }
            let created = create_generate(
                &build_create_generate_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.generate,
                    Default::default(),
                ),
                build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::NoFailure,
                    )),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            );
            assert!(created.is_ok(), "create_generate admits the selection");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: &payload.adapter,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS {
                let created = create_kem(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
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
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
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
                            pairing: &payload.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            generate: self.generate,
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

    let generate = GenerateConsumer {
        mint: &mint,
        transfer: &transfer,
        identities: &identities,
        proof_identity: &first,
    };
    let consumer = PairingConsumer {
        random: &*source,
        generate: &generate,
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
    // emit_returns_a_distinct_path_for_every_kind_the_declaration_does_not_gate
}

/// Contract: entry `a_refused_write_returns_the_mock_concretes_own_error`;
///   `emit` on the adapter `create_generate` hands the consumer for
///   `GenerateConcrete::Mock` carrying
///   `MockIGenerateAdapterFailureMode::WriteRefused` →
///   `Err(EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::
///   WriteRefused))`, the one error arm no input reaches, selected only
///   through configuration.
/// Arrange: the mock selection with `MockIGenerateAdapterFailureMode::
///   WriteRefused` and the params builder's default `kinds`;
///   `build_canonical_delivery_context` at its default for the mint and the
///   transfer, two distinct canonical identities no `AssetIdentityHash`
///   admits — `first` the `as_ref()` bytes of `build_asset_coordinate` at
///   its default and `second` those at version `2.0.0` — as `identities`,
///   `proof_identity` the first; the random source
///   `RandomSourceKind::OperatingSystem`; every case of the other families
///   and no case of `GENERATE_CONCRETES`.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer.
/// Assert:  `matches!(result, Err(EmitErrorReturn::Mock(
///   MockIGenerateAdapterEmitErrorReturn::WriteRefused)))`, the pattern
///   naming every field, asserted in place.
/// Boundary: `create_generate`, the mock concrete's `try_new`, and `emit`.
/// Mocked:  the family's mock concrete under its failure mode, the declared
///   set unable to reach it.
#[test]
fn a_refused_write_returns_the_mock_concretes_own_error() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let first = build_asset_coordinate(Default::default()).as_ref().to_vec();
    let second = build_asset_coordinate(AssetIdentityConstructorParamsOverrides {
        version: Some("2.0.0".to_string()),
        ..Default::default()
    })
    .as_ref()
    .to_vec();
    let identities = vec![first.clone(), second];
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct GenerateConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identities: &'a [Vec<u8>],
        proof_identity: &'a [u8],
    }

    impl<'a> IGenerateConsumer for &'a GenerateConsumer<'a> {
        type Output = ();

        fn consume_generate<G: IGenerateAdapter>(
            &self,
            _params: ConsumeGenerateParams,
            payload: ConsumeGeneratePayload<G>,
        ) -> Self::Output {
            // Act
            let result = payload.adapter.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    self.mint,
                    self.transfer,
                    self.identities,
                    self.proof_identity,
                    Default::default(),
                ),
            );

            // Assert
            assert!(
                matches!(
                    result,
                    Err(EmitErrorReturn::Mock(
                        MockIGenerateAdapterEmitErrorReturn::WriteRefused
                    ))
                ),
                "the refused write returns the mock concrete's own error"
            );
        }
    }

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
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
            let created = create_generate(
                &build_create_generate_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.generate,
                    Default::default(),
                ),
                build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::WriteRefused,
                    )),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            );
            assert!(created.is_ok(), "create_generate admits the selection");
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = ();

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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
                    "the declared delivery-proof concrete is created"
                );
            }
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = ();

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: &payload.adapter,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = ();

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            for selection in DECLARED_KEM_SELECTIONS {
                let created = create_kem(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = ();

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
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
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = ();

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
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
                            pairing: &payload.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            generate: self.generate,
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

    let generate = GenerateConsumer {
        mint: &mint,
        transfer: &transfer,
        identities: &identities,
        proof_identity: &first,
    };
    let consumer = PairingConsumer {
        random: &*source,
        generate: &generate,
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
    // a_refused_write_returns_the_mock_concretes_own_error
}

/// Contract: entry
///   `every_concrete_refuses_under_the_asset_scope_an_identity_no_asset_identity_hash_admits`;
///   `emit` on the adapter `create_generate` hands the consumer for a
///   member of `GENERATE_CONCRETES`, and for the mock selection under
///   `NoFailure`, with `params.scope` `IdentityScope::Asset` and the
///   payload `build_emit_payload` builds over
///   `build_canonical_delivery_context`, the identities an asset identity
///   hash followed by a canonical identity, and the hash as the proof
///   identity → `Err(EmitErrorReturn::AssetIdentityLength { index: 1,
///   actual })`, `actual` the canonical identity's length, the refusal the
///   family's, decided before the concrete's vendor runs and naming no
///   concrete.
/// Arrange: `hash` the `as_bytes()` of `build_asset_identity_hash` at its
///   default, `first` the `as_ref()` bytes of `build_asset_coordinate` at
///   its default; `identities` `vec![hash.clone(), first.clone()]`,
///   `proof_identity` the hash; `create_generate`'s params carry `scope`
///   `Some(IdentityScope::Asset)`; `build_canonical_delivery_context` at
///   its default for the mint and the transfer; the random source
///   `RandomSourceKind::OperatingSystem`; each member of
///   `GENERATE_CONCRETES` through `build_generate_concrete` with `kinds`
///   from `build_generate_kind_configurations`, and the mock selection
///   under `NoFailure` with the params builder's default `kinds`, crossed
///   with every case of the other families.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer, its `EmitReturn` relayed whole to the block.
/// Assert:  per case, `matches!(generated.output, Err(EmitErrorReturn::
///   AssetIdentityLength { index: 1, actual }) if actual == first.len())`,
///   the pattern naming every field.
/// Boundary: `create_generate`, the selected concrete's `try_new`, and
///   `emit`, each real.
/// Mocked:  nothing on the chain; the filesystem is a directory beneath
///   the operating system's temporary directory and the operating
///   system's generator runs real; the mock selection runs no filesystem.
#[test]
fn every_concrete_refuses_under_the_asset_scope_an_identity_no_asset_identity_hash_admits() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let hash = build_asset_identity_hash(Default::default())
        .as_bytes()
        .to_vec();
    let first = build_asset_coordinate(Default::default()).as_ref().to_vec();
    let identities = vec![hash.clone(), first.clone()];
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct GenerateConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identities: &'a [Vec<u8>],
        proof_identity: &'a [u8],
    }

    impl<'a> IGenerateConsumer for &'a GenerateConsumer<'a> {
        type Output = EmitReturn;

        fn consume_generate<G: IGenerateAdapter>(
            &self,
            _params: ConsumeGenerateParams,
            payload: ConsumeGeneratePayload<G>,
        ) -> Self::Output {
            // Act
            payload.adapter.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    self.mint,
                    self.transfer,
                    self.identities,
                    self.proof_identity,
                    Default::default(),
                ),
            )
        }
    }

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for member in GENERATE_CONCRETES {
                let created = create_generate(
                    &build_create_generate_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        self.kem,
                        self.key_agreement,
                        &payload.adapter,
                        self.generate,
                        Default::default(),
                    ),
                    build_create_generate_params(CreateGenerateParamsOverrides {
                        concrete: Some(build_generate_concrete(*member, GenerateConcreteOverrides)),
                        kinds: Some(build_generate_kind_configurations(
                            *member,
                            GenerateKindConfigurationsOverrides,
                        )),
                        scope: Some(IdentityScope::Asset),
                        ..Default::default()
                    }),
                    build_create_generate_payload(Default::default()),
                );
                outputs.push(created);
            }
            outputs.push(create_generate(
                &build_create_generate_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.generate,
                    Default::default(),
                ),
                build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::NoFailure,
                    )),
                    scope: Some(IdentityScope::Asset),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            ));
            outputs
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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
                let Ok(success) = created else {
                    panic!("the declared delivery-proof concrete is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: &payload.adapter,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared key-agreement selection is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                let created = create_kem(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared kem selection is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared chain-forms concrete is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
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
                            pairing: &payload.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            generate: self.generate,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    let Ok(success) = created else {
                        panic!("the declared encoding selection is created")
                    };
                    outputs.extend(success.output);
                }
            }
            outputs
        }
    }

    let generate = GenerateConsumer {
        mint: &mint,
        transfer: &transfer,
        identities: &identities,
        proof_identity: &hash,
    };
    let consumer = PairingConsumer {
        random: &*source,
        generate: &generate,
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

        // Assert
        let Ok(success) = created else {
            panic!("the pairing factory ran the consumer")
        };
        for generated in &success.output {
            let Ok(generated) = generated else {
                panic!("create_generate admits the selection")
            };
            assert!(
                matches!(
                    generated.output,
                    Err(EmitErrorReturn::AssetIdentityLength { index: 1, actual })
                        if actual == first.len()
                ),
                "the asset scope refuses the canonical identity with its index and length"
            );
        }
    }
    // every_concrete_refuses_under_the_asset_scope_an_identity_no_asset_identity_hash_admits
}

/// Contract: entry
///   `every_concrete_refuses_under_the_asset_scope_a_proof_identity_no_asset_identity_hash_admits`;
///   `emit` on the adapter `create_generate` hands the consumer for a
///   member of `GENERATE_CONCRETES`, and for the mock selection under
///   `NoFailure`, with `params.scope` `IdentityScope::Asset` and the
///   payload `build_emit_payload` builds over
///   `build_canonical_delivery_context`, the identities an asset identity
///   hash, and a canonical identity as the proof identity →
///   `Err(EmitErrorReturn::AssetProofIdentityLength { actual })`,
///   `actual` the canonical identity's length, the refusal the family's,
///   decided before the concrete's vendor runs and naming no concrete.
/// Arrange: `hash` the `as_bytes()` of `build_asset_identity_hash` at its
///   default, `first` the `as_ref()` bytes of `build_asset_coordinate` at
///   its default; `identities` `vec![hash.clone()]`, `proof_identity`
///   `first`, the configured identities admitted so only the proof
///   identity's admission refuses; `create_generate`'s params carry
///   `scope` `Some(IdentityScope::Asset)`;
///   `build_canonical_delivery_context` at its default for the mint and
///   the transfer; the random source `RandomSourceKind::OperatingSystem`;
///   each member of `GENERATE_CONCRETES` through `build_generate_concrete`
///   with `kinds` from `build_generate_kind_configurations`, and the mock
///   selection under `NoFailure` with the params builder's default
///   `kinds`, crossed with every case of the other families.
/// Act:     `emit(build_emit_params(Default::default()), payload)` in the
///   innermost consumer, its `EmitReturn` relayed whole to the block.
/// Assert:  per case, `matches!(generated.output, Err(EmitErrorReturn::
///   AssetProofIdentityLength { actual }) if actual == first.len())`, the
///   pattern naming every field.
/// Boundary: `create_generate`, the selected concrete's `try_new`, and
///   `emit`, each real.
/// Mocked:  nothing on the chain; the filesystem is a directory beneath
///   the operating system's temporary directory and the operating
///   system's generator runs real; the mock selection runs no filesystem.
#[test]
fn every_concrete_refuses_under_the_asset_scope_a_proof_identity_no_asset_identity_hash_admits() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let hash = build_asset_identity_hash(Default::default())
        .as_bytes()
        .to_vec();
    let first = build_asset_coordinate(Default::default()).as_ref().to_vec();
    let identities = vec![hash.clone()];
    let source = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    )
    .expect("the random-source factory admits the params")
    .adapter;

    struct GenerateConsumer<'a> {
        mint: &'a CanonicalDeliveryContext,
        transfer: &'a CanonicalDeliveryContext,
        identities: &'a [Vec<u8>],
        proof_identity: &'a [u8],
    }

    impl<'a> IGenerateConsumer for &'a GenerateConsumer<'a> {
        type Output = EmitReturn;

        fn consume_generate<G: IGenerateAdapter>(
            &self,
            _params: ConsumeGenerateParams,
            payload: ConsumeGeneratePayload<G>,
        ) -> Self::Output {
            // Act
            payload.adapter.emit(
                build_emit_params(Default::default()),
                build_emit_payload(
                    self.mint,
                    self.transfer,
                    self.identities,
                    self.proof_identity,
                    Default::default(),
                ),
            )
        }
    }

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, A, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, E, K, A, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        A: IKeyAgreementAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for member in GENERATE_CONCRETES {
                let created = create_generate(
                    &build_create_generate_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        self.kem,
                        self.key_agreement,
                        &payload.adapter,
                        self.generate,
                        Default::default(),
                    ),
                    build_create_generate_params(CreateGenerateParamsOverrides {
                        concrete: Some(build_generate_concrete(*member, GenerateConcreteOverrides)),
                        kinds: Some(build_generate_kind_configurations(
                            *member,
                            GenerateKindConfigurationsOverrides,
                        )),
                        scope: Some(IdentityScope::Asset),
                        ..Default::default()
                    }),
                    build_create_generate_payload(Default::default()),
                );
                outputs.push(created);
            }
            outputs.push(create_generate(
                &build_create_generate_deps(
                    self.pairing,
                    self.hashing,
                    self.encoder,
                    self.random,
                    self.kem,
                    self.key_agreement,
                    &payload.adapter,
                    self.generate,
                    Default::default(),
                ),
                build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(GenerateConcrete::Mock(
                        MockIGenerateAdapterFailureMode::NoFailure,
                    )),
                    scope: Some(IdentityScope::Asset),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            ));
            outputs
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, K, F> IKeyAgreementConsumer<P> for KeyAgreementConsumer<'a, P, E, K, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        K: ICredentialKemAdapter<Pairing = P>,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_key_agreement<A: IKeyAgreementAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKeyAgreementParams,
            payload: ConsumeKeyAgreementPayload<A>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for concrete in DELIVERY_PROOF_CONCRETES {
                let created = create_delivery_proof(
                    &build_create_delivery_proof_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        DeliveryProofConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: self.kem,
                            key_agreement: &payload.adapter,
                            generate: self.generate,
                            forms: PhantomData::<F>,
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
                let Ok(success) = created else {
                    panic!("the declared delivery-proof concrete is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct KemConsumer<'a, P: IPairingArithmetic, E, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
        forms: PhantomData<F>,
    }

    impl<'a, P, E, F> IKemConsumer<P> for KemConsumer<'a, P, E, F>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
        F: IChainForms,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_kem<K: ICredentialKemAdapter<Pairing = P>>(
            &self,
            _params: ConsumeKemParams,
            payload: ConsumeKemPayload<K>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for selection in DECLARED_KEY_AGREEMENT_SELECTIONS {
                let created = create_key_agreement(
                    &build_create_key_agreement_deps(
                        self.pairing,
                        self.hashing,
                        self.encoder,
                        self.random,
                        KeyAgreementConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            kem: &payload.adapter,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_key_agreement_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared kem selection is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for selection in DECLARED_KEM_SELECTIONS {
                let created = create_kem(
                    &build_create_kem_deps(
                        self.pairing,
                        self.hashing,
                        KemConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: self.encoder,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                            forms: PhantomData::<F>,
                        },
                        Default::default(),
                    ),
                    *selection,
                    build_create_kem_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared kem selection is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        random: &'a dyn IRandomSourceAdapter,
        verifier_form: VerifierGroupArithmetic,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            hashing: self.hashing,
                            encoder: &payload.adapter,
                            random: self.random,
                            verifier_form: self.verifier_form,
                            generate: self.generate,
                        },
                        Default::default(),
                    ),
                    build_create_chain_forms_params(CreateChainFormsParamsOverrides {
                        concrete: Some(*concrete),
                        ..Default::default()
                    }),
                    build_create_chain_forms_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the declared chain-forms concrete is created")
                };
                outputs.extend(success.output);
            }
            outputs
        }
    }

    struct PairingConsumer<'a> {
        random: &'a dyn IRandomSourceAdapter,
        generate: &'a GenerateConsumer<'a>,
    }

    impl<'a> IPairingConsumer for &PairingConsumer<'a> {
        type Output = Vec<CreateGenerateReturn<EmitReturn>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut outputs = Vec::new();
            let verifier_form = P::DECLARATION.verifier_group_arithmetic;
            for hashing_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
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
                            pairing: &payload.adapter,
                            hashing: &*hashing.adapter,
                            random: self.random,
                            verifier_form,
                            generate: self.generate,
                        }),
                        build_create_encoding_params(CreateEncodingParamsOverrides {
                            concrete: Some(encoding_selection.concrete),
                            identifier: Some(encoding_selection.identifier),
                        }),
                        build_create_encoding_payload(Default::default()),
                    );
                    let Ok(success) = created else {
                        panic!("the declared encoding selection is created")
                    };
                    outputs.extend(success.output);
                }
            }
            outputs
        }
    }

    let generate = GenerateConsumer {
        mint: &mint,
        transfer: &transfer,
        identities: &identities,
        proof_identity: &first,
    };
    let consumer = PairingConsumer {
        random: &*source,
        generate: &generate,
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

        // Assert
        let Ok(success) = created else {
            panic!("the pairing factory ran the consumer")
        };
        for generated in &success.output {
            let Ok(generated) = generated else {
                panic!("create_generate admits the selection")
            };
            assert!(
                matches!(
                    generated.output,
                    Err(EmitErrorReturn::AssetProofIdentityLength { actual })
                        if actual == first.len()
                ),
                "the asset scope refuses the canonical proof identity with its length"
            );
        }
    }
    // every_concrete_refuses_under_the_asset_scope_a_proof_identity_no_asset_identity_hash_admits
}
