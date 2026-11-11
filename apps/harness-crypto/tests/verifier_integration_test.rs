#![cfg(feature = "mocks")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::marker::PhantomData;

use chain::{
    CHAIN_FORMS_CONCRETES, CreateChainFormsParamsOverrides, IChainForms,
    build_create_chain_forms_params,
};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, build_create_encoding_params,
};
use envelope::EnvelopeAlgebra;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS, HashToScalarIdentifier,
    build_create_hash_to_scalar_params,
};
use pairing::{
    CreatePairingParamsOverrides, IPairingAdapter, IPairingArithmetic, IPairingReference,
    PAIRING_CONCRETES, build_create_pairing_params,
};
use proof::{
    AlgebraicStatement, CreateDeliveryProofParamsOverrides, DELIVERY_PROOF_CONCRETES,
    DELIVERY_STATEMENT_VERSION_ONE, DeliveryRelation, IDeliveryProofAdapter, ProveMintParams,
    ProveTransferParams, build_algebraic_mint_statement, build_algebraic_transfer_statement,
    build_create_delivery_proof_params, build_prove_mint_payload, build_prove_transfer_payload,
    build_verify_payload,
};
use random::{
    CreateRandomSourceParamsOverrides, RandomSourceKind, build_create_random_source_params,
};

use harness_crypto::{
    ConsumeVerifierParams, ConsumeVerifierPayload, CreateVerifierCaseParamsOverrides,
    CreateVerifierErrorReturn, CreateVerifierParamsOverrides, CreateVerifierReturn,
    CreateVerifierSuccessReturn, ICreateVerifierCaseConsumer, IVerifierAdapter, IVerifierConsumer,
    MockIVerifierConsumer, VERIFIER_CONCRETES, VERIFIER_INTERFACE_VERSION, VerifierConcrete,
    VerifierConcreteOverrides, VerifyOnChainErrorReturn, VerifyOnChainReturn,
    build_create_verifier_case_deps, build_create_verifier_case_params,
    build_create_verifier_case_payload, build_create_verifier_deps, build_create_verifier_params,
    build_create_verifier_payload, build_verifier_concrete, build_verify_on_chain_params,
    create_verifier, run_create_verifier_case,
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
    struct CaseConsumer<'a> {
        concrete: &'a VerifierConcrete,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = Vec<(u16, CreateVerifierReturn<()>)>;

        fn consume_create_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Act
            let mut results = Vec::new();
            for version in [2u16, DELIVERY_STATEMENT_VERSION_ONE] {
                results.push((
                    version,
                    create_verifier(
                        &build_create_verifier_deps(
                            pairing,
                            delivery_proof,
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

    let mut results = Vec::new();
    for kind in VERIFIER_CONCRETES {
        // Arrange
        let concrete = build_verifier_concrete(*kind, VerifierConcreteOverrides);
        for pairing_concrete in PAIRING_CONCRETES {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
                    for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                        .iter()
                        .filter(|selection| {
                            selection.identifier == HashToScalarIdentifier::Keccak256V1
                        })
                    {
                        for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                            // Act
                            let created = run_create_verifier_case(
                                &build_create_verifier_case_deps(
                                    CaseConsumer {
                                        concrete: &concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_verifier_case_params(
                                    CreateVerifierCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(hash_to_scalar_selection.concrete),
                                                identifier: Some(
                                                    hash_to_scalar_selection.identifier,
                                                ),
                                            },
                                        )),
                                        encoding: Some(build_create_encoding_params(
                                            CreateEncodingParamsOverrides {
                                                concrete: Some(encoding_selection.concrete),
                                                identifier: Some(encoding_selection.identifier),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*chain_forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*delivery_proof_concrete),
                                                algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                statement_version: Some(
                                                    DELIVERY_STATEMENT_VERSION_ONE,
                                                ),
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    },
                                ),
                                build_create_verifier_case_payload(Default::default()),
                            );
                            let Ok(success) = created else {
                                panic!("the case runner admits each case's selections")
                            };
                            results.extend(success.output);
                        }
                    }
                }
            }
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

    struct CaseConsumer<'a> {
        concrete: &'a VerifierConcrete,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = CreateVerifierReturn<()>;

        fn consume_create_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Act
            create_verifier(
                &build_create_verifier_deps(
                    pairing,
                    delivery_proof,
                    DeclarationConsumer::<P>(PhantomData),
                    Default::default(),
                ),
                build_create_verifier_params(CreateVerifierParamsOverrides {
                    concrete: Some(self.concrete.clone()),
                    statement_version: Some(DELIVERY_STATEMENT_VERSION_ONE),
                }),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let mut results = Vec::new();
    for kind in VERIFIER_CONCRETES {
        // Arrange
        let concrete = build_verifier_concrete(*kind, VerifierConcreteOverrides);
        for pairing_concrete in PAIRING_CONCRETES {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
                    for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                        .iter()
                        .filter(|selection| {
                            selection.identifier == HashToScalarIdentifier::Keccak256V1
                        })
                    {
                        for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                            // Act
                            let created = run_create_verifier_case(
                                &build_create_verifier_case_deps(
                                    CaseConsumer {
                                        concrete: &concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_verifier_case_params(
                                    CreateVerifierCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(hash_to_scalar_selection.concrete),
                                                identifier: Some(
                                                    hash_to_scalar_selection.identifier,
                                                ),
                                            },
                                        )),
                                        encoding: Some(build_create_encoding_params(
                                            CreateEncodingParamsOverrides {
                                                concrete: Some(encoding_selection.concrete),
                                                identifier: Some(encoding_selection.identifier),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*chain_forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*delivery_proof_concrete),
                                                algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                statement_version: Some(
                                                    DELIVERY_STATEMENT_VERSION_ONE,
                                                ),
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    },
                                ),
                                build_create_verifier_case_payload(Default::default()),
                            );
                            let Ok(success) = created else {
                                panic!("the case runner admits each case's selections")
                            };
                            results.push(success.output);
                        }
                    }
                }
            }
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

    struct CaseConsumer<'a> {
        concrete: &'a VerifierConcrete,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = Vec<(DeliveryRelation, CreateVerifierReturn<VerifyOnChainReturn>)>;

        fn consume_create_verifier_case<P, F, D>(
            &self,
            pairing: &P,
            _forms: &F,
            delivery_proof: &D,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            F: IChainForms,
            D: IDeliveryProofAdapter<Pairing = P, Forms = F>,
        {
            // Arrange
            let mint = build_algebraic_mint_statement::<P, F>(pairing, Default::default());
            let transfer = build_algebraic_transfer_statement::<P, F>(pairing, Default::default());
            let mint_proof = delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(pairing, &mint, Default::default()),
            );
            let Ok(mint_proof) = mint_proof else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let transfer_proof = delivery_proof.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(pairing, &transfer, Default::default()),
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
            .map(|(relation, consumer)| {
                (
                    relation,
                    create_verifier(
                        &build_create_verifier_deps(
                            pairing,
                            delivery_proof,
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

    let mut results = Vec::new();
    for kind in VERIFIER_CONCRETES {
        // Arrange
        let concrete = build_verifier_concrete(*kind, VerifierConcreteOverrides);
        for pairing_concrete in PAIRING_CONCRETES {
            for encoding_selection in DECLARED_ENCODING_SELECTIONS {
                for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
                    for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS
                        .iter()
                        .filter(|selection| {
                            selection.identifier == HashToScalarIdentifier::Keccak256V1
                        })
                    {
                        for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                            // Act
                            let created = run_create_verifier_case(
                                &build_create_verifier_case_deps(
                                    CaseConsumer {
                                        concrete: &concrete,
                                    },
                                    Default::default(),
                                ),
                                build_create_verifier_case_params(
                                    CreateVerifierCaseParamsOverrides {
                                        pairing: Some(build_create_pairing_params(
                                            CreatePairingParamsOverrides {
                                                concrete: Some(*pairing_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        hash_to_scalar: Some(build_create_hash_to_scalar_params(
                                            CreateHashToScalarParamsOverrides {
                                                concrete: Some(hash_to_scalar_selection.concrete),
                                                identifier: Some(
                                                    hash_to_scalar_selection.identifier,
                                                ),
                                            },
                                        )),
                                        encoding: Some(build_create_encoding_params(
                                            CreateEncodingParamsOverrides {
                                                concrete: Some(encoding_selection.concrete),
                                                identifier: Some(encoding_selection.identifier),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*chain_forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        delivery_proof: Some(build_create_delivery_proof_params(
                                            CreateDeliveryProofParamsOverrides {
                                                concrete: Some(*delivery_proof_concrete),
                                                algebra: Some(EnvelopeAlgebra::PairingElGamal),
                                                statement_version: Some(
                                                    DELIVERY_STATEMENT_VERSION_ONE,
                                                ),
                                            },
                                        )),
                                        random_source: Some(build_create_random_source_params(
                                            CreateRandomSourceParamsOverrides {
                                                kind: Some(RandomSourceKind::OperatingSystem),
                                            },
                                        )),
                                    },
                                ),
                                build_create_verifier_case_payload(Default::default()),
                            );
                            let Ok(success) = created else {
                                panic!("the case runner admits each case's selections")
                            };
                            results.extend(success.output);
                        }
                    }
                }
            }
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
