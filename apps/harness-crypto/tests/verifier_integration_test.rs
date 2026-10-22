#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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
use envelope::EnvelopeAlgebra;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    build_create_hash_to_scalar_deps, build_create_hash_to_scalar_params,
    build_create_hash_to_scalar_payload, create_hash_to_scalar,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParamsOverrides, IPairingAdapter,
    IPairingArithmetic, IPairingConsumer, IPairingReference, PAIRING_CONCRETES,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use proof::{
    AlgebraicStatement, ConsumeDeliveryProofParams, ConsumeDeliveryProofPayload,
    CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES, DELIVERY_STATEMENT_VERSION_ONE,
    DeliveryRelation, IDeliveryProofAdapter, IDeliveryProofConsumer, ProveMintParams,
    ProveTransferParams, build_algebraic_mint_statement, build_algebraic_transfer_statement,
    build_create_delivery_proof_deps, build_create_delivery_proof_params,
    build_create_delivery_proof_payload, build_prove_mint_payload, build_prove_transfer_payload,
    build_verify_payload, create_delivery_proof,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    RandomSourceKind, build_create_random_source_params, create_random_source,
};

use harness_crypto::{
    ConsumeVerifierParams, ConsumeVerifierPayload, CreateVerifierErrorReturn,
    CreateVerifierParamsOverrides, CreateVerifierReturn, CreateVerifierSuccessReturn,
    IVerifierAdapter, IVerifierConsumer, MockIVerifierConsumer, VERIFIER_CONCRETES,
    VERIFIER_INTERFACE_VERSION, VerifierConcrete, VerifierConcreteOverrides,
    VerifyOnChainErrorReturn, VerifyOnChainReturn, build_create_verifier_deps,
    build_create_verifier_params, build_create_verifier_payload, build_verifier_concrete,
    build_verify_on_chain_params, create_verifier,
};

/// Contract: own entry
///   `a_statement_version_no_concrete_declares_is_refused_and_the_declared_version_admitted`,
///   as an outside caller observes it: `create_verifier` over a member of
///   `VERIFIER_CONCRETES`, selected by `build_verifier_concrete` at its
///   default settings, with `statement_version` `2` and with
///   `DELIVERY_STATEMENT_VERSION_ONE` →
///   `Err(CreateVerifierErrorReturn::UnsupportedStatementVersion
///   { version: 2 })` for `2` and `Ok(CreateVerifierSuccessReturn
///   { output: () })` for version one. The version is admitted against the
///   real concrete's declaration, read through the factory and never named
///   by the caller.
/// Arrange: every member of `VERIFIER_CONCRETES`, crossed with every member
///   of `PAIRING_CONCRETES`, nested over every selection of
///   `DECLARED_ENCODING_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, the randomness source
///   `create_random_source` returns for `RandomSourceKind::OperatingSystem`,
///   and every member of `DELIVERY_PROOF_CONCRETES` in the verifier form
///   the resolved pairing declares; per case `concrete`
///   `Some(build_verifier_concrete(kind, VerifierConcreteOverrides))` with
///   `statement_version` `Some(2)`, then `Some(DELIVERY_STATEMENT_VERSION_ONE)`,
///   each result relayed with the version its call ran; the innermost
///   consumer `MockIVerifierConsumer` holds nothing and returns `()`.
/// Act:     `create_verifier` once per version.
/// Assert:  the relayed result of the `2` call is
///   `Err(CreateVerifierErrorReturn::UnsupportedStatementVersion
///   { version: 2 })` and of the version-one call
///   `Ok(CreateVerifierSuccessReturn { output: () })`; every other factory
///   result `is_ok()` in place.
/// Boundary: `create_verifier` and the selected concrete's `try_new`,
///   reached through the crate's public surface with the pairing,
///   hash-to-scalar, encoding, chain-forms, random-source, and
///   delivery-proof factories, each real.
/// Mocked:  nothing; the chain is unread.
#[test]
fn a_statement_version_no_concrete_declares_is_refused_and_the_declared_version_admitted() {
    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = Vec<(u16, CreateVerifierReturn<()>)>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            let mut results = Vec::new();
            for version in [2u16, DELIVERY_STATEMENT_VERSION_ONE] {
                results.push((
                    version,
                    create_verifier(
                        &build_create_verifier_deps(
                            self.pairing,
                            &payload.adapter,
                            MockIVerifierConsumer,
                            Default::default(),
                        ),
                        build_create_verifier_params(CreateVerifierParamsOverrides {
                            concrete: Some(self.concrete.clone()),
                            statement_version: Some(version),
                        }),
                        build_create_verifier_payload(Default::default()),
                    ),
                ));
            }
            results
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
        encoder: &'a E,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<(u16, CreateVerifierReturn<()>)>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                concrete: self.concrete,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                    results.extend(success.output);
                }
            }
            results
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<(u16, CreateVerifierReturn<()>)>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            concrete: self.concrete,
                            encoder: &payload.adapter,
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
                    panic!("the chain-forms factory admits the selection")
                };
                results.extend(success.output);
            }
            results
        }
    }

    struct PairingConsumer<'a> {
        concrete: &'a VerifierConcrete,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = Vec<(u16, CreateVerifierReturn<()>)>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                        concrete: self.concrete,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the encoding factory admits the selection")
                };
                results.extend(success.output);
            }
            results
        }
    }

    let mut results = Vec::new();
    for kind in VERIFIER_CONCRETES {
        // Arrange
        let concrete = build_verifier_concrete(*kind, VerifierConcreteOverrides);
        for pairing_concrete in PAIRING_CONCRETES {
            // Act
            let created = create_pairing(
                &build_create_pairing_deps(
                    PairingConsumer {
                        concrete: &concrete,
                    },
                    Default::default(),
                ),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                build_create_pairing_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the pairing factory ran the consumer")
            };
            results.extend(success.output);
        }
    }

    // Assert
    for (version, created) in results {
        match version {
            2 => assert!(matches!(
                created,
                Err(CreateVerifierErrorReturn::UnsupportedStatementVersion { version: 2 })
            )),
            DELIVERY_STATEMENT_VERSION_ONE => assert!(matches!(
                created,
                Ok(CreateVerifierSuccessReturn { output: () })
            )),
            _ => panic!("the case's version is one the arrange named"),
        }
    }
    // a_statement_version_no_concrete_declares_is_refused_and_the_declared_version_admitted
}

/// Contract: own entry
///   `an_adapter_handed_to_the_consumer_declares_the_pairings_verifier_form_and_the_params_statement_version`,
///   as an outside caller observes it: `create_verifier` over a member of
///   `VERIFIER_CONCRETES`, selected by `build_verifier_concrete` at its
///   default settings, at `DELIVERY_STATEMENT_VERSION_ONE`, with a consumer
///   that reads `V::DECLARATION` of the adapter it receives →
///   `Ok(CreateVerifierSuccessReturn { output: () })`, the consumer having
///   found that the declaration's `verifier_forms` contains
///   `P::DECLARATION.verifier_group_arithmetic`, its `statement_versions`
///   contains `DELIVERY_STATEMENT_VERSION_ONE`, and its `interface_version`
///   equals `VERIFIER_INTERFACE_VERSION`. The adapter is constructed before
///   the consumer runs and the consumer's output reaches the return
///   unaltered.
/// Arrange: the cases of the version block; `concrete`
///   `Some(build_verifier_concrete(kind, VerifierConcreteOverrides))` at
///   `DELIVERY_STATEMENT_VERSION_ONE`; the innermost consumer a
///   `DeclarationConsumer` holding nothing, asserting the received
///   adapter's declaration in place.
/// Act:     `create_verifier` once per case.
/// Assert:  in place, `V::DECLARATION.verifier_forms` contains
///   `P::DECLARATION.verifier_group_arithmetic`,
///   `V::DECLARATION.statement_versions` contains
///   `DELIVERY_STATEMENT_VERSION_ONE`, and `V::DECLARATION.interface_version`
///   equals `VERIFIER_INTERFACE_VERSION`; every relayed result is
///   `Ok(CreateVerifierSuccessReturn { output: () })`; every other factory
///   result `is_ok()` in place.
/// Boundary: `create_verifier` and the selected concrete's `try_new`,
///   reached through the crate's public surface with the pairing,
///   hash-to-scalar, encoding, chain-forms, random-source, and
///   delivery-proof factories, each real.
/// Mocked:  nothing; the chain is unread.
#[test]
fn an_adapter_handed_to_the_consumer_declares_the_pairings_verifier_form_and_the_params_statement_version()
 {
    struct DeclarationConsumer<P>(PhantomData<P>);

    impl<P, D> IVerifierConsumer<D> for DeclarationConsumer<P>
    where
        P: IPairingAdapter,
        D: IDeliveryProofAdapter<Pairing = P>,
    {
        type Output = ();

        fn consume_verifier<V: IVerifierAdapter<DeliveryProof = D>>(
            &self,
            _params: ConsumeVerifierParams,
            _payload: ConsumeVerifierPayload<V>,
        ) -> Self::Output {
            // Assert
            assert!(
                V::DECLARATION
                    .verifier_forms
                    .contains(&P::DECLARATION.verifier_group_arithmetic)
            );
            assert!(
                V::DECLARATION
                    .statement_versions
                    .contains(&DELIVERY_STATEMENT_VERSION_ONE)
            );
            assert_eq!(V::DECLARATION.interface_version, VERIFIER_INTERFACE_VERSION);
        }
    }

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = Vec<CreateVerifierReturn<()>>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Act
            vec![create_verifier(
                &build_create_verifier_deps(
                    self.pairing,
                    &payload.adapter,
                    DeclarationConsumer::<P>(PhantomData),
                    Default::default(),
                ),
                build_create_verifier_params(CreateVerifierParamsOverrides {
                    concrete: Some(self.concrete.clone()),
                    statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                }),
                build_create_verifier_payload(Default::default()),
            )]
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
        encoder: &'a E,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<CreateVerifierReturn<()>>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                concrete: self.concrete,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                    results.extend(success.output);
                }
            }
            results
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<CreateVerifierReturn<()>>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            concrete: self.concrete,
                            encoder: &payload.adapter,
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
                    panic!("the chain-forms factory admits the selection")
                };
                results.extend(success.output);
            }
            results
        }
    }

    struct PairingConsumer<'a> {
        concrete: &'a VerifierConcrete,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = Vec<CreateVerifierReturn<()>>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                        concrete: self.concrete,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the encoding factory admits the selection")
                };
                results.extend(success.output);
            }
            results
        }
    }

    let mut results = Vec::new();
    for kind in VERIFIER_CONCRETES {
        // Arrange
        let concrete = build_verifier_concrete(*kind, VerifierConcreteOverrides);
        for pairing_concrete in PAIRING_CONCRETES {
            // Act
            let created = create_pairing(
                &build_create_pairing_deps(
                    PairingConsumer {
                        concrete: &concrete,
                    },
                    Default::default(),
                ),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                build_create_pairing_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the pairing factory ran the consumer")
            };
            results.extend(success.output);
        }
    }

    // Assert
    for created in results {
        assert!(matches!(
            created,
            Ok(CreateVerifierSuccessReturn { output: () })
        ));
    }
    // an_adapter_handed_to_the_consumer_declares_the_pairings_verifier_form_and_the_params_statement_version
}

/// Contract: own entry
///   `every_concrete_refuses_a_proof_of_the_other_relation_before_any_chain_read`,
///   as an outside caller observes it: `verify_on_chain` on the adapter
///   `create_verifier` hands the consumer for a member of
///   `VERIFIER_CONCRETES`, selected by `build_verifier_concrete` at its
///   default settings, over a real mint statement with the real proof
///   `prove_transfer` returned, and over a real transfer statement with the
///   real proof `prove_mint` returned →
///   `Err(VerifyOnChainErrorReturn::RelationMismatch { statement, proof })`
///   with the relations the case names, decided from the payload before any
///   chain read.
/// Arrange: the cases of the version block; `concrete`
///   `Some(build_verifier_concrete(kind, VerifierConcreteOverrides))` at
///   `DELIVERY_STATEMENT_VERSION_ONE`; the real statements
///   `build_algebraic_mint_statement` and `build_algebraic_transfer_statement`
///   build over the received pairing and forms, proved by `prove_mint` and
///   `prove_transfer` over the witnesses `build_prove_mint_payload` and
///   `build_prove_transfer_payload` hold; a mint statement with the
///   transfer proof, then a transfer statement with the mint proof, each
///   result relayed with the statement relation its order ran.
/// Act:     `create_verifier` per case and order; `verify_on_chain` once in
///   the innermost consumer `create_verifier` runs, holding only the
///   block's statement and proof.
/// Assert:  every relayed `CreateVerifierReturn` is `Ok` whose `output` is
///   `Err(VerifyOnChainErrorReturn::RelationMismatch)` with `statement` and
///   `proof` `Mint`/`Transfer` where the case ran the mint statement, and
///   `Transfer`/`Mint` where it ran the transfer statement; every other
///   factory result `is_ok()` in place.
/// Boundary: `create_verifier`, the selected concrete's `try_new`, and
///   `verify_on_chain`, reached through the crate's public surface with the
///   pairing, hash-to-scalar, encoding, chain-forms, random-source, and
///   delivery-proof factories, `prove_mint`, and `prove_transfer`, each
///   real.
/// Mocked:  nothing; the chain is unread, the refusal decided from the
///   payload before any read, and the operating system's generator runs
///   real.
#[test]
fn every_concrete_refuses_a_proof_of_the_other_relation_before_any_chain_read() {
    struct VerifierConsumer<'a, D: IDeliveryProofAdapter> {
        statement: AlgebraicStatement<
            'a,
            D::Forms,
            <D::Pairing as IPairingAdapter>::G1,
            <D::Pairing as IPairingAdapter>::G2,
        >,
        proof: &'a D::Proof,
    }

    impl<'a, D: IDeliveryProofAdapter> IVerifierConsumer<D> for VerifierConsumer<'a, D> {
        type Output = VerifyOnChainReturn;

        fn consume_verifier<V: IVerifierAdapter<DeliveryProof = D>>(
            &self,
            _params: ConsumeVerifierParams,
            payload: ConsumeVerifierPayload<V>,
        ) -> Self::Output {
            // Act
            payload.adapter.verify_on_chain(
                build_verify_on_chain_params(Default::default()),
                build_verify_payload(
                    match self.statement {
                        AlgebraicStatement::Mint(statement) => AlgebraicStatement::Mint(statement),
                        AlgebraicStatement::Transfer(statement) => {
                            AlgebraicStatement::Transfer(statement)
                        }
                    },
                    self.proof,
                    Default::default(),
                ),
            )
        }
    }

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = Vec<(DeliveryRelation, CreateVerifierReturn<VerifyOnChainReturn>)>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let transfer =
                build_algebraic_transfer_statement::<P, F>(self.pairing, Default::default());
            let mint_proof = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
            );
            let Ok(mint_proof) = mint_proof else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let transfer_proof = payload.adapter.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(self.pairing, &transfer, Default::default()),
            );
            let Ok(transfer_proof) = transfer_proof else {
                panic!("the real adapter proves the transfer statement it is handed")
            };

            // Act
            [
                (
                    DeliveryRelation::Mint,
                    VerifierConsumer {
                        statement: AlgebraicStatement::Mint(&mint),
                        proof: &transfer_proof.proof,
                    },
                ),
                (
                    DeliveryRelation::Transfer,
                    VerifierConsumer {
                        statement: AlgebraicStatement::Transfer(&transfer),
                        proof: &mint_proof.proof,
                    },
                ),
            ]
            .into_iter()
            .map(|(statement, consumer)| {
                (
                    statement,
                    create_verifier(
                        &build_create_verifier_deps(
                            self.pairing,
                            &payload.adapter,
                            consumer,
                            Default::default(),
                        ),
                        build_create_verifier_params(CreateVerifierParamsOverrides {
                            concrete: Some(self.concrete.clone()),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_verifier_payload(Default::default()),
                    ),
                )
            })
            .collect()
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
        encoder: &'a E,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<(DeliveryRelation, CreateVerifierReturn<VerifyOnChainReturn>)>;

        fn consume_chain_forms<F: IChainForms>(
            &self,
            _params: ConsumeChainFormsParams,
            _payload: ConsumeChainFormsPayload<F>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                .iter()
                .filter(|selection| selection.identifier == HashToScalarIdentifier::Keccak256V1)
            {
                let hashed = create_hash_to_scalar::<P::Scalar>(
                    &build_create_hash_to_scalar_deps(Default::default()),
                    build_create_hash_to_scalar_params(CreateHashToScalarParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_hash_to_scalar_payload(Default::default()),
                );
                let Ok(hashed) = hashed else {
                    panic!("the declared hash-to-scalar selection admits the concrete it names")
                };
                let random = create_random_source(
                    &CreateRandomSourceDeps,
                    build_create_random_source_params(CreateRandomSourceParamsOverrides {
                        kind: Some(RandomSourceKind::OperatingSystem),
                    }),
                    CreateRandomSourcePayload,
                );
                let Ok(random) = random;
                for concrete in DELIVERY_PROOF_CONCRETES {
                    let created = create_delivery_proof::<P, E, F, _>(
                        &build_create_delivery_proof_deps(
                            self.pairing,
                            &*hashed.adapter,
                            self.encoder,
                            &*random.adapter,
                            DeliveryProofConsumer {
                                pairing: self.pairing,
                                concrete: self.concrete,
                                forms: PhantomData::<F>,
                            },
                            Default::default(),
                        ),
                        build_create_delivery_proof_params(CreateDeliveryProofParamsOverrides {
                            concrete: Some(*concrete),
                            algebra: Some(EnvelopeAlgebra::PairingElGamal),
                            verifier_form: Some(P::DECLARATION.verifier_group_arithmetic),
                            statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                        }),
                        build_create_delivery_proof_payload(Default::default()),
                    );
                    let Ok(success) = created else {
                        panic!("the delivery-proof factory admits the pairing's declared form")
                    };
                    results.extend(success.output);
                }
            }
            results
        }
    }

    struct EncodingConsumer<'a, P: IPairingArithmetic> {
        pairing: &'a P,
        concrete: &'a VerifierConcrete,
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<(DeliveryRelation, CreateVerifierReturn<VerifyOnChainReturn>)>;

        fn consume_encoding<E: IEncoderAdapter + IDecoderAdapter>(
            &self,
            _params: ConsumeEncodingParams,
            payload: ConsumeEncodingPayload<E>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for concrete in CHAIN_FORMS_CONCRETES {
                let created = create_chain_forms(
                    &build_create_chain_forms_deps(
                        ChainFormsConsumer {
                            pairing: self.pairing,
                            concrete: self.concrete,
                            encoder: &payload.adapter,
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
                    panic!("the chain-forms factory admits the selection")
                };
                results.extend(success.output);
            }
            results
        }
    }

    struct PairingConsumer<'a> {
        concrete: &'a VerifierConcrete,
    }

    impl IPairingConsumer for PairingConsumer<'_> {
        type Output = Vec<(DeliveryRelation, CreateVerifierReturn<VerifyOnChainReturn>)>;

        fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
            &self,
            _params: ConsumePairingParams,
            payload: ConsumePairingPayload<P>,
        ) -> Self::Output {
            let mut results = Vec::new();
            for selection in DECLARED_ENCODING_SELECTIONS {
                let created = create_encoding(
                    &build_create_encoding_deps(EncodingConsumer {
                        pairing: &payload.adapter,
                        concrete: self.concrete,
                    }),
                    build_create_encoding_params(CreateEncodingParamsOverrides {
                        concrete: Some(selection.concrete),
                        identifier: Some(selection.identifier),
                    }),
                    build_create_encoding_payload(Default::default()),
                );
                let Ok(success) = created else {
                    panic!("the encoding factory admits the selection")
                };
                results.extend(success.output);
            }
            results
        }
    }

    let mut results = Vec::new();
    for kind in VERIFIER_CONCRETES {
        // Arrange
        let concrete = build_verifier_concrete(*kind, VerifierConcreteOverrides);
        for pairing_concrete in PAIRING_CONCRETES {
            // Act
            let created = create_pairing(
                &build_create_pairing_deps(
                    PairingConsumer {
                        concrete: &concrete,
                    },
                    Default::default(),
                ),
                build_create_pairing_params(CreatePairingParamsOverrides {
                    concrete: Some(*pairing_concrete),
                    ..Default::default()
                }),
                build_create_pairing_payload(Default::default()),
            );
            let Ok(success) = created else {
                panic!("the pairing factory ran the consumer")
            };
            results.extend(success.output);
        }
    }

    // Assert
    for (relation, created) in results {
        let Ok(success) = created else {
            panic!("create_verifier admits the case's selection, form, and version")
        };
        let result = success.output;
        match relation {
            DeliveryRelation::Mint => assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::RelationMismatch {
                    statement: DeliveryRelation::Mint,
                    proof: DeliveryRelation::Transfer,
                })
            )),
            DeliveryRelation::Transfer => assert!(matches!(
                result,
                Err(VerifyOnChainErrorReturn::RelationMismatch {
                    statement: DeliveryRelation::Transfer,
                    proof: DeliveryRelation::Mint,
                })
            )),
        }
    }
    // every_concrete_refuses_a_proof_of_the_other_relation_before_any_chain_read
}
