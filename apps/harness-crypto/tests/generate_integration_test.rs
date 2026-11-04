#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{CreateEncodingParams, DECLARED_ENCODING_SELECTIONS, IEncoderAdapter};
use envelope::IKeyAgreementAdapter;
use hash_to_scalar::IHashToScalarAdapter;
use hashing::{RootConstructorParamsOverrides, build_root};
use kem::ICredentialKemAdapter;
use pairing::{
    CreatePairingParamsOverrides, IPairingArithmetic, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_params,
};
use proof::{
    DELIVERY_PURPOSE_MINT, DELIVERY_PURPOSE_TRANSFER, DeliveryContext, IDeliveryProofAdapter,
    MintPurpose, TransferPurpose, build_delivery_context,
};
use random::{
    CreateRandomSourceParamsOverrides, IRandomSourceAdapter, RandomSourceKind,
    build_create_random_source_params,
};

use harness_crypto::{
    AdmittedKemIdentity, ConsumeGenerateParams, ConsumeGeneratePayload,
    CreateGenerateCaseParamsOverrides, CreateGenerateParams, CreateGenerateParamsOverrides,
    CreateGenerateReturn, EmitPayloadOverrides, EmitReturn, GENERATE_CONCRETES,
    GenerateConcreteOverrides, GenerateKindConfigurationsOverrides, ICreateGenerateCaseConsumer,
    IGenerateAdapter, IGenerateConsumer, VectorCount, VectorCountConstructorParamsOverrides,
    VectorKind, VectorKindConfigurationOverrides, VectorKindQuantitiesOverrides,
    build_create_generate_case_deps, build_create_generate_case_params,
    build_create_generate_case_payload, build_create_generate_deps, build_create_generate_params,
    build_create_generate_payload, build_emit_params, build_emit_payload, build_generate_concrete,
    build_generate_kind_configurations, build_vector_count, build_vector_kind_configuration,
    build_vector_kind_quantities, create_generate, run_create_generate_case,
};

/// Contract: entry
///   `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`;
///   `create_generate` over the selection `build_generate_concrete` returns
///   for a member of `GENERATE_CONCRETES` and `kinds` the configuration
///   `build_generate_kind_configurations` returns for it, reversed, the
///   possession entry's count overridden to a value other than the builder's
///   default → the consumer's `emit` returns `Ok` whose possession
///   `EmittedFile` holds a path whose file holds exactly the configured
///   count of lines, the file found by its emitted kind tag and read from
///   the real filesystem.
/// Arrange: the typed mint and transfer contexts at their defaults, two
///   distinct canonicals and two distinct roots so the case's KEM scope
///   composes two admitted identities either way, `proof_identity` the
///   first; `count` from `build_vector_count` with `count: Some(7)`, other
///   than the builder's default and other than every other entry's count;
///   the random source `RandomSourceKind::OperatingSystem`; every member of
///   `GENERATE_CONCRETES` crossed with every member of `PAIRING_CONCRETES`,
///   over the `DECLARED_ENCODING_SELECTIONS` and `CHAIN_FORMS_CONCRETES`
///   selections the concrete's declaration admits.
/// Act:     `create_generate`, the single call inside the block's case
///   consumer whose `emit` runs inside the inner consumer, run once per
///   case by `run_create_generate_case`.
/// Assert:  in place inside the inner consumer, the `EmittedFile` whose
///   `kind` is `Some(VectorKind::Possession)` has a `path` whose file
///   `std::fs::read_to_string` reads holds exactly `count.get()` lines by
///   `assert_eq!`; the block asserts `result`'s `output` is `Ok` with an
///   `Ok` `emit` return per case.
/// Boundary: `create_generate`, the selected concrete's `try_new` and
///   `emit`, and the functions `emit` runs, each real, with the pairing,
///   hash-to-scalar, encoding, chain-forms, KEM, key-agreement, and
///   delivery-proof factories, reached through the crate's public surface.
/// Mocked:  nothing; the filesystem under the default root the settings
///   builder names, a directory beneath the operating system's temporary
///   directory, and the operating system's generator runs real.
#[test]
fn a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds() {
    // Arrange
    struct EmitConsumer<'a, F: IChainForms> {
        mint: &'a DeliveryContext<F, MintPurpose>,
        transfer: &'a DeliveryContext<F, TransferPurpose>,
        identities: &'a [AdmittedKemIdentity<'a>],
        count: VectorCount,
    }

    impl<'a, F: IChainForms> IGenerateConsumer<F> for EmitConsumer<'a, F> {
        type Output = EmitReturn;

        fn consume_generate<G: IGenerateAdapter<Forms = F>>(
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
                    EmitPayloadOverrides {
                        proof_identity: Some(self.identities[0]),
                    },
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
                usize::try_from(self.count.get())
                    .expect("a u32 count fits a usize on every supported target"),
                "the file holds the configured count of records"
            );
            result
        }
    }

    struct Consumer {
        params: CreateGenerateParams,
        count: VectorCount,
    }

    impl ICreateGenerateCaseConsumer for Consumer {
        type Output = CreateGenerateReturn<EmitReturn>;

        fn consume_create_generate_case<P, E, F, K, A, D>(
            &self,
            pairing: &P,
            hash_to_scalar: &dyn IHashToScalarAdapter<P::Scalar>,
            encoder: &E,
            kem: &K,
            key_agreement: &A,
            delivery_proof: &D,
            random: &dyn IRandomSourceAdapter,
            identities: Vec<AdmittedKemIdentity<'_>>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            E: IEncoderAdapter,
            F: IChainForms,
            K: ICredentialKemAdapter<Pairing = P>,
            A: IKeyAgreementAdapter<Pairing = P>,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            let mint =
                build_delivery_context::<F, MintPurpose>(DELIVERY_PURPOSE_MINT, Default::default());
            let transfer = build_delivery_context::<F, TransferPurpose>(
                DELIVERY_PURPOSE_TRANSFER,
                Default::default(),
            );
            create_generate(
                &build_create_generate_deps(
                    pairing,
                    hash_to_scalar,
                    encoder,
                    random,
                    kem,
                    key_agreement,
                    delivery_proof,
                    EmitConsumer {
                        mint: &mint,
                        transfer: &transfer,
                        identities: &identities,
                        count: self.count,
                    },
                    Default::default(),
                ),
                self.params.clone(),
                build_create_generate_payload(Default::default()),
            )
        }
    }

    let canonicals: [&[u8]; 2] = [
        b"example-package@1.0.0".as_slice(),
        b"example-package@2.0.0".as_slice(),
    ];
    let first_root = build_root(Default::default());
    let second_root = build_root(RootConstructorParamsOverrides {
        bytes: Some([0x11; 32]),
    });
    let roots = [&first_root, &second_root];

    for kind in GENERATE_CONCRETES {
        for pairing in PAIRING_CONCRETES {
            let count =
                build_vector_count(VectorCountConstructorParamsOverrides { count: Some(7) });
            let entries: Vec<_> =
                build_generate_kind_configurations(*kind, GenerateKindConfigurationsOverrides)
                    .into_iter()
                    .rev()
                    .map(|entry| {
                        if entry.kind == VectorKind::Possession {
                            build_vector_kind_configuration(VectorKindConfigurationOverrides {
                                kind: Some(VectorKind::Possession),
                                quantities: Some(build_vector_kind_quantities(
                                    VectorKindQuantitiesOverrides {
                                        count: Some(Some(count)),
                                        ..Default::default()
                                    },
                                )),
                            })
                        } else {
                            entry
                        }
                    })
                    .collect();
            let consumer = Consumer {
                params: build_create_generate_params(CreateGenerateParamsOverrides {
                    concrete: Some(build_generate_concrete(*kind, GenerateConcreteOverrides)),
                    kinds: Some(entries),
                    ..Default::default()
                }),
                count,
            };
            let case_params =
                build_create_generate_case_params(CreateGenerateCaseParamsOverrides {
                    pairing: Some(build_create_pairing_params(CreatePairingParamsOverrides {
                        concrete: Some(*pairing),
                        ..Default::default()
                    })),
                    encoding: Some(CreateEncodingParams {
                        concrete: DECLARED_ENCODING_SELECTIONS[0].concrete,
                        identifier: DECLARED_ENCODING_SELECTIONS[0].identifier,
                    }),
                    chain_forms: Some(build_create_chain_forms_params(
                        CreateChainFormsParamsOverrides {
                            concrete: Some(CHAIN_FORMS_CONCRETES[0]),
                            ..Default::default()
                        },
                    )),
                    random_source: Some(build_create_random_source_params(
                        CreateRandomSourceParamsOverrides {
                            kind: Some(RandomSourceKind::OperatingSystem),
                        },
                    )),
                    ..Default::default()
                });

            // Act
            let result = run_create_generate_case(
                &build_create_generate_case_deps(consumer, Default::default()),
                case_params,
                build_create_generate_case_payload(&canonicals, &roots, Default::default()),
            );

            // Assert
            let returned = result
                .unwrap()
                .output
                .expect("create_generate admits the case");
            assert!(returned.output.is_ok());
        }
    }
}
// a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds
