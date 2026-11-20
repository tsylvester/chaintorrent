#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use chain::{
    CHAIN_FORMS_CONCRETES, ChainMemberIdentifier, CreateChainFormsParamsOverrides,
    CreateChainParamsOverrides, DECLARED_CHAIN_MEMBERS, IChainAdapter, IEntitlementStateAdapter,
    build_chain_selection, build_create_chain_forms_params, build_create_chain_params,
};
use encoding::{
    CreateEncodingParamsOverrides, DECLARED_ENCODING_SELECTIONS, EncodingConcrete,
    EncodingIdentifier, build_create_encoding_params,
};
use envelope::EnvelopeAlgebra;
use hash_to_scalar::{
    CreateHashToScalarParamsOverrides, DECLARED_HASH_TO_SCALAR_SELECTIONS,
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
    CreateRandomSourceParamsOverrides, RANDOM_SOURCE_KINDS, build_create_random_source_params,
};

use super::super::admit_verifier_composition::provides::{
    AdmitVerifierCompositionErrorReturn, ChainMemberIdentifierDisagreement,
    admit_verifier_composition,
};
use super::super::lookup_verifier_declaration::provides::{
    LookupVerifierDeclarationParamsOverrides, build_lookup_verifier_declaration_deps,
    build_lookup_verifier_declaration_params, build_lookup_verifier_declaration_payload,
    lookup_verifier_declaration,
};
use super::create_verifier;
use super::interface::{
    ConsumeCreateVerifierCaseParams, ConsumeCreateVerifierCasePayload, ConsumeVerifierParams,
    ConsumeVerifierPayload, CreateVerifierErrorReturn, CreateVerifierReturn,
    ICreateVerifierCaseConsumer, IVerifierAdapter, IVerifierConsumer,
    MockIVerifierAdapterFailureMode, MockIVerifierAdapterVerifyOnChainErrorReturn,
    VERIFIER_CONCRETES, VerifierConcrete, VerifierConcreteKind, VerifyOnChainErrorReturn,
    VerifyOnChainReturn,
};
use super::mock::{
    CreateVerifierCaseParamsOverrides, CreateVerifierDepsOverrides, CreateVerifierParamsOverrides,
    MockIVerifierConsumer, VerifierConcreteOverrides, build_create_verifier_case_deps,
    build_create_verifier_case_params, build_create_verifier_case_payload,
    build_create_verifier_deps, build_create_verifier_params, build_create_verifier_payload,
    build_verifier_concrete, build_verify_on_chain_params, run_create_verifier_case,
};
use crate::generate::provides::{
    GENERATE_CONCRETES, GenerateConcreteKind, GenerateDeclaration,
    LookupGenerateDeclarationParamsOverrides, build_lookup_generate_declaration_deps,
    build_lookup_generate_declaration_params, build_lookup_generate_declaration_payload,
    lookup_generate_declaration,
};

fn declared_verifier_chain_member(kind: VerifierConcreteKind) -> ChainMemberIdentifier {
    let Ok(looked_up) = lookup_verifier_declaration(
        &build_lookup_verifier_declaration_deps(Default::default()),
        build_lookup_verifier_declaration_params(LookupVerifierDeclarationParamsOverrides {
            identifier: Some(kind),
            ..Default::default()
        }),
        build_lookup_verifier_declaration_payload(Default::default()),
    );
    looked_up.declaration.chain_member
}

fn generate_declaration(kind: GenerateConcreteKind) -> GenerateDeclaration {
    let Ok(looked_up) = lookup_generate_declaration(
        &build_lookup_generate_declaration_deps(Default::default()),
        build_lookup_generate_declaration_params(LookupGenerateDeclarationParamsOverrides {
            identifier: Some(kind),
            ..Default::default()
        }),
        build_lookup_generate_declaration_payload(Default::default()),
    );
    looked_up.declaration
}

/// Contract: own entry
///   `every_concrete_refuses_a_proof_of_the_other_relation_before_any_chain_read`,
///   absorbing the `harness-crypto/verifier/evm` entry
///   `a_real_proof_of_the_other_relation_is_refused_before_any_chain_read`,
///   restated in this node's terms: `verify_on_chain` on the adapter
///   `create_verifier` hands the consumer for a member of
///   `VERIFIER_CONCRETES`, selected by `build_verifier_concrete` with `kind`
///   `Some(member)`, and for the mock selection under `NoFailure`, over a
///   real mint statement with the real proof `prove_transfer` returned, and
///   over a real transfer statement with the real proof `prove_mint`
///   returned → `Err(VerifyOnChainErrorReturn::RelationMismatch
///   { statement, proof })` with the relations the case names. The refusal
///   is the family's flavor, decided from the payload by every concrete the
///   factory constructs before any chain read, so the chain adapter's
///   endpoint is never connected to.
/// Arrange: every member of `PAIRING_CONCRETES`, nested over every
///   selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every member of `DELIVERY_PROOF_CONCRETES` in
///   the verifier form the resolved pairing declares, every member of
///   `GENERATE_CONCRETES` and `GenerateConcreteKind::Mock`, and every member
///   of `RANDOM_SOURCE_KINDS`; per case every member of `VERIFIER_CONCRETES`
///   and the mock selection under `NoFailure`, each crossed with both
///   orders; the chain the selection `build_chain_selection` returns for
///   the `chain_member` the selected member's declaration requires, read
///   through `lookup_verifier_declaration`, and the encoder declaring the
///   `encoding_identifier` the selected member's declaration requires, the
///   encoding family's declared members for a declared selection and its
///   mock member for the mock selection; the real statements
///   `build_algebraic_mint_statement` and `build_algebraic_transfer_statement`
///   build over the received pairing and forms, the real proofs
///   `prove_mint` and `prove_transfer` return over the witnesses
///   `build_prove_mint_payload` and `build_prove_transfer_payload` hold.
/// Act:     `create_verifier` per selection and order with the real
///   `admit_verifier_composition` as the handle; `verify_on_chain` once in
///   the innermost consumer `create_verifier` runs, holding only the
///   case's statement and proof.
/// Assert:  every relayed `CreateVerifierReturn` is `Ok` whose `output` is
///   `Err(VerifyOnChainErrorReturn::RelationMismatch)` with `statement` and
///   `proof` `Mint`/`Transfer` where the case ran the mint statement, and
///   `Transfer`/`Mint` where it ran the transfer statement; every other
///   factory result `is_ok()` in place.
/// Boundary: `create_pairing`, the concrete's `try_new`, `create_encoding`,
///   the selection's `try_new`, `create_chain_forms`, the concrete's
///   `try_new`, `create_hash_to_scalar`, the selection's `try_new`,
///   `create_random_source`, `create_chain`, the real
///   `admit_chain_composition`, `create_delivery_proof`, the concrete's
///   `try_new`, `prove_mint`, `prove_transfer`, `create_verifier`, the real
///   `admit_verifier_composition`, `lookup_verifier_declaration`,
///   `lookup_generate_declaration`, the selected concrete's `try_new`, and
///   `verify_on_chain`, each real.
/// Mocked:  nothing; the chain is unreached, the refusal decided from the
///   payload before any read, and each declared member's generator runs
///   real; the mock selection under `NoFailure` runs the family's mock
///   concrete, which stands for no real concrete.
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

        fn consume_create_verifier_case<P, C, D>(
            &self,
            _params: ConsumeCreateVerifierCaseParams,
            payload: ConsumeCreateVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Arrange
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let transfer = build_algebraic_transfer_statement::<P, D::Forms>(
                payload.pairing,
                Default::default(),
            );
            let mint_proof = payload.delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(payload.pairing, &mint, Default::default()),
            );
            let Ok(mint_proof) = mint_proof else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let transfer_proof = payload.delivery_proof.prove_transfer(
                ProveTransferParams,
                build_prove_transfer_payload(payload.pairing, &transfer, Default::default()),
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
                            payload.pairing,
                            payload.chain,
                            payload.delivery_proof,
                            payload.encoder,
                            payload.generate,
                            consumer,
                            CreateVerifierDepsOverrides {
                                admit_verifier_composition: Some(admit_verifier_composition),
                            },
                        ),
                        build_create_verifier_params(CreateVerifierParamsOverrides {
                            concrete: Some(self.concrete.clone()),
                            ..Default::default()
                        }),
                        build_create_verifier_payload(Default::default()),
                    ),
                )
            })
            .collect()
        }
    }

    let mut results = Vec::new();
    for (kind, concrete) in VERIFIER_CONCRETES
        .iter()
        .map(|kind| {
            (
                *kind,
                build_verifier_concrete(VerifierConcreteOverrides {
                    kind: Some(*kind),
                    ..Default::default()
                }),
            )
        })
        .chain([(
            VerifierConcreteKind::Mock,
            VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
        )])
    {
        let chain_member = declared_verifier_chain_member(kind);
        let encodings: Vec<(EncodingConcrete, EncodingIdentifier)> = match kind {
            VerifierConcreteKind::Mock => {
                vec![(EncodingConcrete::Mock, EncodingIdentifier::Mock)]
            }
            _ => DECLARED_ENCODING_SELECTIONS
                .iter()
                .map(|selection| (selection.concrete, selection.identifier))
                .collect(),
        };
        for generate_kind in GENERATE_CONCRETES
            .iter()
            .chain(core::iter::once(&GenerateConcreteKind::Mock))
        {
            let generate = generate_declaration(*generate_kind);
            for pairing_concrete in PAIRING_CONCRETES {
                for (encoding_concrete, encoding_identifier) in &encodings {
                    for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
                        for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                            for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                                for random_kind in RANDOM_SOURCE_KINDS {
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
                                                hash_to_scalar: Some(
                                                    build_create_hash_to_scalar_params(
                                                        CreateHashToScalarParamsOverrides {
                                                            concrete: Some(
                                                                hash_to_scalar_selection.concrete,
                                                            ),
                                                            identifier: Some(
                                                                hash_to_scalar_selection.identifier,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                encoding: Some(build_create_encoding_params(
                                                    CreateEncodingParamsOverrides {
                                                        concrete: Some(*encoding_concrete),
                                                        identifier: Some(*encoding_identifier),
                                                    },
                                                )),
                                                chain_forms: Some(build_create_chain_forms_params(
                                                    CreateChainFormsParamsOverrides {
                                                        concrete: Some(*chain_forms_concrete),
                                                        ..Default::default()
                                                    },
                                                )),
                                                chain: Some(build_create_chain_params(
                                                    CreateChainParamsOverrides {
                                                        selection: Some(build_chain_selection(
                                                            chain_member,
                                                            Default::default(),
                                                        )),
                                                    },
                                                )),
                                                delivery_proof: Some(
                                                    build_create_delivery_proof_params(
                                                        CreateDeliveryProofParamsOverrides {
                                                            concrete: Some(
                                                                *delivery_proof_concrete,
                                                            ),
                                                            algebra: Some(
                                                                EnvelopeAlgebra::PairingElGamal,
                                                            ),
                                                            statement_version: Some(
                                                                DELIVERY_STATEMENT_VERSION_ONE,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                random_source: Some(
                                                    build_create_random_source_params(
                                                        CreateRandomSourceParamsOverrides {
                                                            kind: Some(*random_kind),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        build_create_verifier_case_payload(
                                            &generate,
                                            Default::default(),
                                        ),
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

/// Contract: own entry `a_refused_read_returns_the_mock_concretes_own_error`:
///   `verify_on_chain` on the adapter `create_verifier` hands the consumer
///   for `VerifierConcrete::Mock` carrying
///   `MockIVerifierAdapterFailureMode::RpcRefused`, over a real mint
///   statement and the real proof `prove_mint` returned →
///   `Err(VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused))`. The mode
///   is an error arm no input reaches, selected only through configuration,
///   varied against `NoFailure` by the sibling block's mock selection.
/// Arrange: every member of `PAIRING_CONCRETES`, nested over every
///   selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every member of `DELIVERY_PROOF_CONCRETES` in
///   the verifier form the resolved pairing declares, every member of
///   `GENERATE_CONCRETES` and `GenerateConcreteKind::Mock`, and every member
///   of `RANDOM_SOURCE_KINDS` — every case of the other families and no
///   case of `VERIFIER_CONCRETES`; the verifier's selection the mock
///   concrete under `RpcRefused`; the chain and the encoder the members
///   the mock selection's declaration requires; the real mint statement
///   `build_algebraic_mint_statement` builds over the received pairing and
///   forms and the real proof `prove_mint` returns over the witnesses
///   `build_prove_mint_payload` holds.
/// Act:     `create_verifier` per case with the real
///   `admit_verifier_composition` as the handle; `verify_on_chain` once in
///   the innermost consumer `create_verifier` runs, holding only the
///   block's statement and proof.
/// Assert:  every relayed `CreateVerifierReturn` is `Ok` whose `output` is
///   `Err(VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused))`; every
///   other factory result `is_ok()` in place.
/// Boundary: `create_pairing`, the concrete's `try_new`, `create_encoding`,
///   the selection's `try_new`, `create_chain_forms`, the concrete's
///   `try_new`, `create_hash_to_scalar`, the selection's `try_new`,
///   `create_random_source`, `create_chain`, the real
///   `admit_chain_composition`, `create_delivery_proof`, the concrete's
///   `try_new`, `prove_mint`, `create_verifier`, the real
///   `admit_verifier_composition`, `lookup_verifier_declaration`,
///   `lookup_generate_declaration`, the mock concrete's `try_new`, and
///   `verify_on_chain`, each real but the mock concrete.
/// Mocked:  the verifier family's concrete under its `RpcRefused` mode; the
///   declared real set is never selected, so no endpoint is configured and
///   no chain is reached; each declared member's generator runs real.
#[test]
fn a_refused_read_returns_the_mock_concretes_own_error() {
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
        type Output = CreateVerifierReturn<VerifyOnChainReturn>;

        fn consume_create_verifier_case<P, C, D>(
            &self,
            _params: ConsumeCreateVerifierCaseParams,
            payload: ConsumeCreateVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Arrange
            let mint =
                build_algebraic_mint_statement::<P, D::Forms>(payload.pairing, Default::default());
            let mint_proof = payload.delivery_proof.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(payload.pairing, &mint, Default::default()),
            );
            let Ok(mint_proof) = mint_proof else {
                panic!("the real adapter proves the mint statement it is handed")
            };

            // Act
            create_verifier(
                &build_create_verifier_deps(
                    payload.pairing,
                    payload.chain,
                    payload.delivery_proof,
                    payload.encoder,
                    payload.generate,
                    VerifierConsumer {
                        statement: AlgebraicStatement::Mint(&mint),
                        proof: &mint_proof.proof,
                    },
                    CreateVerifierDepsOverrides {
                        admit_verifier_composition: Some(admit_verifier_composition),
                    },
                ),
                build_create_verifier_params(CreateVerifierParamsOverrides {
                    concrete: Some(self.concrete.clone()),
                    ..Default::default()
                }),
                build_create_verifier_payload(Default::default()),
            )
        }
    }

    let concrete = VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused);
    let mut results = Vec::new();
    for generate_kind in GENERATE_CONCRETES
        .iter()
        .chain(core::iter::once(&GenerateConcreteKind::Mock))
    {
        let generate = generate_declaration(*generate_kind);
        for pairing_concrete in PAIRING_CONCRETES {
            for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
                for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                    for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                        for random_kind in RANDOM_SOURCE_KINDS {
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
                                                concrete: Some(EncodingConcrete::Mock),
                                                identifier: Some(EncodingIdentifier::Mock),
                                            },
                                        )),
                                        chain_forms: Some(build_create_chain_forms_params(
                                            CreateChainFormsParamsOverrides {
                                                concrete: Some(*chain_forms_concrete),
                                                ..Default::default()
                                            },
                                        )),
                                        chain: Some(build_create_chain_params(
                                            CreateChainParamsOverrides {
                                                selection: Some(build_chain_selection(
                                                    ChainMemberIdentifier::Mock,
                                                    Default::default(),
                                                )),
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
                                                kind: Some(*random_kind),
                                            },
                                        )),
                                    },
                                ),
                                build_create_verifier_case_payload(&generate, Default::default()),
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
        let Ok(success) = created else {
            panic!("create_verifier admits the case's selection, form, and version")
        };
        let result = success.output;
        assert!(matches!(
            result,
            Err(VerifyOnChainErrorReturn::Mock(
                MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused,
            ))
        ));
    }
    // a_refused_read_returns_the_mock_concretes_own_error
}

/// Contract: own entry
///   `create_verifier_refuses_a_chain_member_declaring_another_member_identifier`:
///   `create_verifier` over a chain member whose declaration's `identifier`
///   is not the chain member the selected member's declaration requires →
///   `Err(CreateVerifierErrorReturn::Composition(
///   AdmitVerifierCompositionErrorReturn::Incompatible { chain_member:
///   Some(ChainMemberIdentifierDisagreement { required, declared }),
///   chain_forms: None, encoding: None, verifier_form: None,
///   contract_suite: None }))`, `required` the identifier the selected
///   member's declaration requires and `declared` the one the received
///   chain declaration carries, with nothing constructed and the consumer
///   never called. The refusal is decided by the family's composition
///   admission from the chain adapter's own declaration and the selected
///   member's own requirement before anything is constructed.
/// Arrange: every member of `PAIRING_CONCRETES`, nested over every
///   selection of `DECLARED_HASH_TO_SCALAR_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every member of `DELIVERY_PROOF_CONCRETES`,
///   every member of `GENERATE_CONCRETES` and `GenerateConcreteKind::Mock`,
///   and every member of `RANDOM_SOURCE_KINDS`; per case every member of
///   `VERIFIER_CONCRETES` with the chain selection `build_chain_selection`
///   returns for `ChainMemberIdentifier::Mock`, and the mock selection with
///   the chain `build_chain_selection` returns for each member of
///   `DECLARED_CHAIN_MEMBERS`; the encoder declaring the
///   `encoding_identifier` each selected member's declaration requires.
/// Act:     `create_verifier` per case with the real
///   `admit_verifier_composition` and `lookup_verifier_declaration` as the
///   handle and the lookup.
/// Assert:  every relayed `CreateVerifierReturn` is `Err(
///   CreateVerifierErrorReturn::Composition(
///   AdmitVerifierCompositionErrorReturn::Incompatible { .. }))` whose
///   `chain_member` is `Some(ChainMemberIdentifierDisagreement
///   { required, declared })` with `required` the `chain_member` the
///   selected member's declaration carries and `declared` the received
///   chain declaration's `identifier`, and whose other outcomes are `None`,
///   the pattern naming every field because `CreateVerifierErrorReturn`
///   omits `PartialEq`; every other factory result `is_ok()` in place.
/// Boundary: `create_pairing`, `create_encoding`, `create_chain_forms`,
///   `create_hash_to_scalar`, `create_random_source`, `create_chain`, the
///   real `admit_chain_composition`, `create_delivery_proof`,
///   `create_verifier`, the real `admit_verifier_composition`,
///   `lookup_verifier_declaration`, and `lookup_generate_declaration`,
///   each real.
/// Mocked:  the chain family's mock member for the declared selections'
///   cases and the verifier family's mock selection for the declared
///   members' cases; no concrete is constructed.
#[test]
fn create_verifier_refuses_a_chain_member_declaring_another_member_identifier() {
    struct CaseConsumer<'a> {
        concrete: &'a VerifierConcrete,
    }

    impl ICreateVerifierCaseConsumer for CaseConsumer<'_> {
        type Output = (ChainMemberIdentifier, CreateVerifierReturn<()>);

        fn consume_create_verifier_case<P, C, D>(
            &self,
            _params: ConsumeCreateVerifierCaseParams,
            payload: ConsumeCreateVerifierCasePayload<'_, P, C, D>,
        ) -> Self::Output
        where
            P: IPairingArithmetic + IPairingReference,
            C: IChainAdapter,
            D: IDeliveryProofAdapter<Pairing = P, Forms = <C as IEntitlementStateAdapter>::Forms>,
        {
            // Act
            (
                C::DECLARATION.identifier,
                create_verifier(
                    &build_create_verifier_deps(
                        payload.pairing,
                        payload.chain,
                        payload.delivery_proof,
                        payload.encoder,
                        payload.generate,
                        MockIVerifierConsumer,
                        CreateVerifierDepsOverrides {
                            admit_verifier_composition: Some(admit_verifier_composition),
                        },
                    ),
                    build_create_verifier_params(CreateVerifierParamsOverrides {
                        concrete: Some(self.concrete.clone()),
                        ..Default::default()
                    }),
                    build_create_verifier_payload(Default::default()),
                ),
            )
        }
    }

    let mut results = Vec::new();
    for (kind, concrete, chain_member) in VERIFIER_CONCRETES
        .iter()
        .map(|kind| {
            (
                *kind,
                build_verifier_concrete(VerifierConcreteOverrides {
                    kind: Some(*kind),
                    ..Default::default()
                }),
                ChainMemberIdentifier::Mock,
            )
        })
        .chain(DECLARED_CHAIN_MEMBERS.iter().map(|member| {
            (
                VerifierConcreteKind::Mock,
                VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::NoFailure),
                *member,
            )
        }))
    {
        let required = declared_verifier_chain_member(kind);
        let (encoding_concrete, encoding_identifier) = match kind {
            VerifierConcreteKind::Mock => (EncodingConcrete::Mock, EncodingIdentifier::Mock),
            _ => (
                DECLARED_ENCODING_SELECTIONS[0].concrete,
                DECLARED_ENCODING_SELECTIONS[0].identifier,
            ),
        };
        for generate_kind in GENERATE_CONCRETES
            .iter()
            .chain(core::iter::once(&GenerateConcreteKind::Mock))
        {
            let generate = generate_declaration(*generate_kind);
            for pairing_concrete in PAIRING_CONCRETES {
                for chain_forms_concrete in CHAIN_FORMS_CONCRETES {
                    for hash_to_scalar_selection in DECLARED_HASH_TO_SCALAR_SELECTIONS {
                        for delivery_proof_concrete in DELIVERY_PROOF_CONCRETES {
                            for random_kind in RANDOM_SOURCE_KINDS {
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
                                            hash_to_scalar: Some(
                                                build_create_hash_to_scalar_params(
                                                    CreateHashToScalarParamsOverrides {
                                                        concrete: Some(
                                                            hash_to_scalar_selection.concrete,
                                                        ),
                                                        identifier: Some(
                                                            hash_to_scalar_selection.identifier,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            encoding: Some(build_create_encoding_params(
                                                CreateEncodingParamsOverrides {
                                                    concrete: Some(encoding_concrete),
                                                    identifier: Some(encoding_identifier),
                                                },
                                            )),
                                            chain_forms: Some(build_create_chain_forms_params(
                                                CreateChainFormsParamsOverrides {
                                                    concrete: Some(*chain_forms_concrete),
                                                    ..Default::default()
                                                },
                                            )),
                                            chain: Some(build_create_chain_params(
                                                CreateChainParamsOverrides {
                                                    selection: Some(build_chain_selection(
                                                        chain_member,
                                                        Default::default(),
                                                    )),
                                                },
                                            )),
                                            delivery_proof: Some(
                                                build_create_delivery_proof_params(
                                                    CreateDeliveryProofParamsOverrides {
                                                        concrete: Some(*delivery_proof_concrete),
                                                        algebra: Some(
                                                            EnvelopeAlgebra::PairingElGamal,
                                                        ),
                                                        statement_version: Some(
                                                            DELIVERY_STATEMENT_VERSION_ONE,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            random_source: Some(build_create_random_source_params(
                                                CreateRandomSourceParamsOverrides {
                                                    kind: Some(*random_kind),
                                                },
                                            )),
                                        },
                                    ),
                                    build_create_verifier_case_payload(
                                        &generate,
                                        Default::default(),
                                    ),
                                );
                                let Ok(success) = created else {
                                    panic!("the case runner admits each case's selections")
                                };
                                results.push((required, success.output));
                            }
                        }
                    }
                }
            }
        }
    }

    // Assert
    for (expected_required, (received_declared, created)) in results {
        let Err(CreateVerifierErrorReturn::Composition(
            AdmitVerifierCompositionErrorReturn::Incompatible {
                chain_forms: None,
                chain_member: Some(ChainMemberIdentifierDisagreement { required, declared }),
                encoding: None,
                verifier_form: None,
                contract_suite: None,
            },
        )) = created
        else {
            panic!("the composition admission refuses the mismatched chain member")
        };
        assert_eq!(required, expected_required);
        assert_eq!(declared, received_declared);
    }
    // create_verifier_refuses_a_chain_member_declaring_another_member_identifier
}
