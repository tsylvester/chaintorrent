#![cfg(feature = "mocks")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    
)]

use core::marker::PhantomData;

use chain::{
    CHAIN_FORMS_CONCRETES, ConsumeChainFormsParams, ConsumeChainFormsPayload,
    CreateChainFormsParamsOverrides, IChainForms, IChainFormsConsumer,
    build_create_chain_forms_deps, build_create_chain_forms_params,
    build_create_chain_forms_payload, create_chain_forms,
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
    IKemConsumer, build_create_kem_deps, build_create_kem_payload, create_kem,
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

use harness_crypto::{
    CanonicalDeliveryContext, ConsumeGenerateParams, ConsumeGeneratePayload,
    CreateGenerateParamsOverrides, CreateGenerateReturn, EmitReturn, GENERATE_CONCRETES,
    GenerateConcrete, GenerateConcreteKind, GenerateConcreteOverrides,
    GenerateKindConfigurationsOverrides, IGenerateAdapter, IGenerateConsumer, VectorCount,
    VectorCountConstructorParamsOverrides, VectorKind, VectorKindConfiguration,
    VectorKindConfigurationOverrides, VectorKindQuantitiesOverrides,
    build_canonical_delivery_context, build_create_generate_deps, build_create_generate_params,
    build_create_generate_payload, build_emit_params, build_emit_payload, build_generate_concrete,
    build_generate_kind_configurations, build_vector_count, build_vector_kind_configuration,
    build_vector_kind_quantities, create_generate,
};

/// Contract: entry
///   `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`;
///   `create_generate` over the selection `build_generate_concrete` returns
///   for a member of `GENERATE_CONCRETES` and `kinds` the configuration
///   `build_generate_kind_configurations` returns for it, reversed, the
///   possession entry replaced by one carrying `count` other than the
///   builder's default and other than every other entry's count → the
///   consumer's `emit` returns `Ok` whose possession `EmittedFile` holds a
///   path whose file holds exactly the configured count of lines, the file
///   found by its emitted kind tag, never a concrete's naming convention,
///   and no expected value read from the declaration or the pairing.
/// Arrange: `build_canonical_delivery_context` at its default for the mint
///   and the transfer, two distinct byte strings as `identities`,
///   `proof_identity` the first; `count` from `build_vector_count` with
///   `count: Some(7)`, other than the builder's default `2` and other than
///   every other entry's count, which the configuration builder leaves at
///   that default; the random source `RandomSourceKind::OperatingSystem`;
///   every member of `GENERATE_CONCRETES` crossed with every member of
///   `PAIRING_CONCRETES`; the Evm selection's settings root removed before
///   the act.
/// Act:     `create_generate` in the consumer `create_delivery_proof` hands
///   its adapter to, the consumer's `emit` inside it.
/// Assert:  in place, the consumer finds the `EmittedFile` whose `kind` is
///   `Some(VectorKind::Possession)`, reads its `path` by
///   `std::fs::read_to_string`, and its line count equals `count.get()`;
///   the block asserts the results the consumers return whole.
/// Boundary: `create_generate`, the selected concrete's `try_new` and
///   `emit`, and the functions `emit` runs, each real, with the pairing,
///   hash-to-scalar, encoding, chain-forms, KEM, key-agreement, and
///   delivery-proof factories, reached through the crate's public surface.
/// Mocked:  nothing; the filesystem is a directory beneath the operating
///   system's temporary directory under the default root the settings
///   builder names, and the operating system's generator runs real.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    let mint = build_canonical_delivery_context(Default::default());
    let transfer = build_canonical_delivery_context(Default::default());
    let identities = vec![b"identity-one".to_vec(), b"identity-two".to_vec()];
    let count = build_vector_count(VectorCountConstructorParamsOverrides { count: Some(7) });
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
        count: VectorCount,
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
            let Ok(success) = &result else {
                panic!("emit admits the case")
            };
            let file = success
                .files
                .iter()
                .find(|file| file.kind == Some(VectorKind::Possession))
                .expect("the emit returns the possession file");
            let text = std::fs::read_to_string(&file.path).expect("the emitted file reads");
            assert_eq!(
                text.lines().count(),
                self.count.get() as usize,
                "the file holds the configured count of records"
            );
            result
        }
    }

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, E, K, A, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        key_agreement: &'a A,
        kind: GenerateConcreteKind,
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
            let concrete = build_generate_concrete(self.kind, GenerateConcreteOverrides);
            if let GenerateConcrete::Evm { settings } = &concrete {
                let _ = std::fs::remove_dir_all(&settings.foundry_root);
            }
            let entries: Vec<VectorKindConfiguration> =
                build_generate_kind_configurations(self.kind, GenerateKindConfigurationsOverrides)
                    .into_iter()
                    .rev()
                    .map(|entry| {
                        if entry.kind == VectorKind::Possession {
                            build_vector_kind_configuration(VectorKindConfigurationOverrides {
                                kind: Some(VectorKind::Possession),
                                quantities: Some(build_vector_kind_quantities(
                                    VectorKindQuantitiesOverrides {
                                        count: Some(Some(self.generate.count)),
                                        ..Default::default()
                                    },
                                )),
                            })
                        } else {
                            entry
                        }
                    })
                    .collect();

            // Act
            vec![create_generate(
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
                    kinds: Some(entries),
                    ..Default::default()
                }),
                build_create_generate_payload(Default::default()),
            )]
        }
    }

    struct KeyAgreementConsumer<'a, P: IPairingArithmetic, E, K, F> {
        pairing: &'a P,
        hashing: &'a dyn IHashToScalarAdapter<P::Scalar>,
        encoder: &'a E,
        random: &'a dyn IRandomSourceAdapter,
        kem: &'a K,
        verifier_form: VerifierGroupArithmetic,
        kind: GenerateConcreteKind,
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
                            kind: self.kind,
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
        kind: GenerateConcreteKind,
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
                            kind: self.kind,
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
        kind: GenerateConcreteKind,
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
                            kind: self.kind,
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
        kind: GenerateConcreteKind,
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
                            kind: self.kind,
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
        kind: GenerateConcreteKind,
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
                            kind: self.kind,
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
        count,
        mint: &mint,
        transfer: &transfer,
        identities: &identities,
        proof_identity: &identities[0],
    };
    for member in GENERATE_CONCRETES {
        let consumer = PairingConsumer {
            random: &*source,
            kind: *member,
            generate: &generate,
        };
        for pairing_member in PAIRING_CONCRETES {
            let result = create_pairing(
                &build_create_pairing_deps(&consumer, Default::default()),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_member),
                    ..Default::default()
                }),
                CreatePairingPayload,
            );

            // Assert
            let Ok(success) = result else {
                panic!("the pairing factory ran the consumer")
            };
            for generated in &success.output {
                let Ok(generated) = generated else {
                    panic!("create_generate admits the selection")
                };
                assert!(generated.output.is_ok(), "emit admits the case");
            }
        }
    }
    // a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
}
