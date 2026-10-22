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

use super::create_verifier;
use super::interface::{
    ConsumeVerifierParams, ConsumeVerifierPayload, CreateVerifierReturn, IVerifierAdapter,
    IVerifierConsumer, MockIVerifierAdapterFailureMode,
    MockIVerifierAdapterVerifyOnChainErrorReturn, VERIFIER_CONCRETES, VerifierConcrete,
    VerifyOnChainErrorReturn, VerifyOnChainReturn,
};
use super::mock::{
    CreateVerifierParamsOverrides, VerifierConcreteOverrides, build_create_verifier_deps,
    build_create_verifier_params, build_create_verifier_payload, build_verifier_concrete,
    build_verify_on_chain_params,
};

/// Contract: own entry
///   `every_concrete_refuses_a_proof_of_the_other_relation_before_any_chain_read`,
///   absorbing the `harness-crypto/verifier/evm` entry
///   `a_real_proof_of_the_other_relation_is_refused_before_any_chain_read`,
///   restated in this node's terms: `verify_on_chain` on the adapter
///   `create_verifier` hands the consumer for a member of
///   `VERIFIER_CONCRETES`, selected by `build_verifier_concrete` at its
///   default settings, and for the mock selection under `NoFailure`, over a
///   real mint statement with the real proof `prove_transfer` returned, and
///   over a real transfer statement with the real proof `prove_mint`
///   returned → `Err(VerifyOnChainErrorReturn::RelationMismatch
///   { statement, proof })` with the relations the case names. The refusal
///   is the family's flavor, decided from the payload by every concrete the
///   factory constructs before any chain read, so the default endpoint, a
///   loopback address nothing listens at, is never reached.
/// Arrange: every member of `PAIRING_CONCRETES`, nested over every
///   selection of `DECLARED_ENCODING_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, the randomness source
///   `create_random_source` returns for `RandomSourceKind::OperatingSystem`,
///   and every member of `DELIVERY_PROOF_CONCRETES` in the verifier form
///   the resolved pairing declares; the real statements
///   `build_algebraic_mint_statement` and `build_algebraic_transfer_statement`
///   build over the received pairing and forms, the real proofs
///   `prove_mint` and `prove_transfer` return over the witnesses
///   `build_prove_mint_payload` and `build_prove_transfer_payload` hold;
///   per case every member of `VERIFIER_CONCRETES` and the mock selection
///   under `NoFailure`, each crossed with both orders, each result relayed
///   with the statement relation its order ran.
/// Act:     `create_verifier` per selection and order; `verify_on_chain`
///   once in the innermost consumer `create_verifier` runs, holding only
///   the case's statement and proof.
/// Assert:  every relayed `CreateVerifierReturn` is `Ok` whose `output` is
///   `Err(VerifyOnChainErrorReturn::RelationMismatch)` with `statement` and
///   `proof` `Mint`/`Transfer` where the case ran the mint statement, and
///   `Transfer`/`Mint` where it ran the transfer statement; every other
///   factory result `is_ok()` in place.
/// Boundary: `create_pairing`, the concrete's `try_new`, `create_encoding`,
///   the selection's `try_new`, `create_chain_forms`, the concrete's
///   `try_new`, `create_hash_to_scalar`, the selection's `try_new`,
///   `create_random_source`, `create_delivery_proof`, the concrete's
///   `try_new`, `prove_mint`, `prove_transfer`, `create_verifier`, the
///   selected concrete's `try_new`, and `verify_on_chain`, each real.
/// Mocked:  nothing; the chain is unreached, the refusal decided from the
///   payload before any read, and the operating system's generator runs
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

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
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
            let mut results = Vec::new();
            for concrete in VERIFIER_CONCRETES
                .iter()
                .map(|kind| build_verifier_concrete(*kind, VerifierConcreteOverrides))
                .chain([VerifierConcrete::Mock(
                    MockIVerifierAdapterFailureMode::NoFailure,
                )])
            {
                for (statement, proof) in [
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
                ] {
                    results.push((
                        statement,
                        create_verifier(
                            &build_create_verifier_deps(
                                self.pairing,
                                &payload.adapter,
                                proof,
                                Default::default(),
                            ),
                            build_create_verifier_params(CreateVerifierParamsOverrides {
                                concrete: Some(concrete.clone()),
                                ..Default::default()
                            }),
                            build_create_verifier_payload(Default::default()),
                        ),
                    ));
                }
            }
            results
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
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

    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
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

    // Arrange
    let mut results = Vec::new();
    for concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(success) = created else {
            panic!("the pairing factory ran the consumer")
        };
        results.extend(success.output);
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
///   selection of `DECLARED_ENCODING_SELECTIONS`, every member of
///   `CHAIN_FORMS_CONCRETES`, every selection of
///   `DECLARED_HASH_TO_SCALAR_SELECTIONS` declaring
///   `HashToScalarIdentifier::Keccak256V1`, the randomness source
///   `create_random_source` returns for `RandomSourceKind::OperatingSystem`,
///   and every member of `DELIVERY_PROOF_CONCRETES` in the verifier form
///   the resolved pairing declares — every case of the other families and
///   no case of `VERIFIER_CONCRETES`; the verifier's selection the mock
///   concrete under `RpcRefused`; the real mint statement
///   `build_algebraic_mint_statement` builds over the received pairing and
///   forms and the real proof `prove_mint` returns over the witnesses
///   `build_prove_mint_payload` holds.
/// Act:     `create_verifier` per case; `verify_on_chain` once in the
///   innermost consumer `create_verifier` runs, holding only the block's
///   statement and proof.
/// Assert:  every relayed `CreateVerifierReturn` is `Ok` whose `output` is
///   `Err(VerifyOnChainErrorReturn::Mock(
///   MockIVerifierAdapterVerifyOnChainErrorReturn::RpcRefused))`; every
///   other factory result `is_ok()` in place.
/// Boundary: `create_pairing`, the concrete's `try_new`, `create_encoding`,
///   the selection's `try_new`, `create_chain_forms`, the concrete's
///   `try_new`, `create_hash_to_scalar`, the selection's `try_new`,
///   `create_random_source`, `create_delivery_proof`, the concrete's
///   `try_new`, `prove_mint`, `create_verifier`, the mock concrete's
///   `try_new`, and `verify_on_chain`, each real but the mock concrete.
/// Mocked:  the verifier family's concrete under its `RpcRefused` mode; the
///   declared real set is never selected, so no endpoint is configured and
///   no chain is reached; the operating system's generator runs real.
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

    struct DeliveryProofConsumer<'a, P: IPairingArithmetic, F: IChainForms> {
        pairing: &'a P,
        forms: PhantomData<F>,
    }

    impl<'a, P, F> IDeliveryProofConsumer<P, F> for DeliveryProofConsumer<'a, P, F>
    where
        P: IPairingArithmetic + IPairingReference,
        F: IChainForms,
    {
        type Output = Vec<CreateVerifierReturn<VerifyOnChainReturn>>;

        fn consume_delivery_proof<D: IDeliveryProofAdapter<Pairing = P, Forms = F>>(
            &self,
            _params: ConsumeDeliveryProofParams,
            payload: ConsumeDeliveryProofPayload<D>,
        ) -> Self::Output {
            // Arrange
            let mint = build_algebraic_mint_statement::<P, F>(self.pairing, Default::default());
            let mint_proof = payload.adapter.prove_mint(
                ProveMintParams,
                build_prove_mint_payload(self.pairing, &mint, Default::default()),
            );
            let Ok(mint_proof) = mint_proof else {
                panic!("the real adapter proves the mint statement it is handed")
            };
            let concrete = VerifierConcrete::Mock(MockIVerifierAdapterFailureMode::RpcRefused);

            // Act
            vec![create_verifier(
                &build_create_verifier_deps(
                    self.pairing,
                    &payload.adapter,
                    VerifierConsumer {
                        statement: AlgebraicStatement::Mint(&mint),
                        proof: &mint_proof.proof,
                    },
                    Default::default(),
                ),
                build_create_verifier_params(CreateVerifierParamsOverrides {
                    concrete: Some(concrete),
                    ..Default::default()
                }),
                build_create_verifier_payload(Default::default()),
            )]
        }
    }

    struct ChainFormsConsumer<'a, P: IPairingArithmetic, E> {
        pairing: &'a P,
        encoder: &'a E,
    }

    impl<'a, P, E> IChainFormsConsumer for ChainFormsConsumer<'a, P, E>
    where
        P: IPairingArithmetic + IPairingReference,
        E: IEncoderAdapter,
    {
        type Output = Vec<CreateVerifierReturn<VerifyOnChainReturn>>;

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
    }

    impl<'a, P> IEncodingConsumer for EncodingConsumer<'a, P>
    where
        P: IPairingArithmetic + IPairingReference,
    {
        type Output = Vec<CreateVerifierReturn<VerifyOnChainReturn>>;

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

    struct PairingConsumer;

    impl IPairingConsumer for PairingConsumer {
        type Output = Vec<CreateVerifierReturn<VerifyOnChainReturn>>;

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

    // Arrange
    let mut results = Vec::new();
    for concrete in PAIRING_CONCRETES {
        // Act
        let created = create_pairing(
            &build_create_pairing_deps(PairingConsumer, Default::default()),
            build_create_pairing_params(CreatePairingParamsOverrides {
                concrete: Some(*concrete),
                ..Default::default()
            }),
            build_create_pairing_payload(Default::default()),
        );
        let Ok(success) = created else {
            panic!("the pairing factory ran the consumer")
        };
        results.extend(success.output);
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
