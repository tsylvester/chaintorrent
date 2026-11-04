#[cfg(test)]
mod integration_test;
mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test_basic;
#[cfg(test)]
mod test_error;

use crate::DeliveryContextField;
use domain::{
    ParameterSetIdentifier, ParameterSetIdentifierConstructorParams, SuiteIdentifier,
    SuiteIdentifierConstructorParams,
};
use encoding::{CanonicalFieldValue, FromFieldParams, ICanonicalField, ToFieldParams};
use envelope::{EnvelopeComponents, IKeyAgreementAdapter, PublicKeysComponents};
use hashing::{Root, RootConstructorParams};
use kem::ICredentialKemAdapter;
use pairing::{
    AddG1Params, AddG1Payload, AddG2Params, AddG2Payload, AddScalarParams, AddScalarPayload,
    G1GeneratorParams, G1GeneratorPayload, G2GeneratorParams, G2GeneratorPayload,
    IPairingArithmetic, ISampleUniformScalar, MulG1Params, MulG1Payload, MulG2Params, MulG2Payload,
    SampleUniformScalarParams, SampleUniformScalarPayload,
};
use proof::{
    AlgebraicMintStatement, AlgebraicStatement, AlgebraicTransferStatement,
    DeliveryProofComponents, DeliveryRelation, IDeliveryProofAdapter, MintPurpose, MintResponses,
    ProofComponentsParams, ProofComponentsPayload, ProofFromComponentsParams,
    ProofFromComponentsPayload, TransferPurpose, TransferResponses, VerifyParams, VerifyPayload,
};
use random::{FillBytesParams, FillBytesPayload};

use crate::delivery::provides::{
    MintDeps, MintParams, MintPayload, TransferDeps, TransferParams, TransferPayload, mint,
    transfer,
};
use interface::{
    MINT_MUTATION_FIELDS, MintMutation, MintMutationField, StatementMutationsDeps,
    StatementMutationsErrorReturn, StatementMutationsParams, StatementMutationsPayload,
    StatementMutationsReturn, StatementMutationsSuccessReturn, StatementMutationsVector,
    TRANSFER_MUTATION_FIELDS, TransferMutation, TransferMutationField,
};

pub fn statement_mutations<'a, P, K, A, D>(
    deps: &StatementMutationsDeps<'a, P, K, A, D>,
    params: StatementMutationsParams,
    payload: StatementMutationsPayload<'_, D::Forms>,
) -> StatementMutationsReturn<P, D>
where
    P: IPairingArithmetic,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    let mut vectors: Vec<StatementMutationsVector<P, D>> = Vec::new();
    for _ in 0..params.count.get() {
        let minted = match mint(
            &MintDeps {
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                random: deps.random,
            },
            MintParams,
            MintPayload {
                context: payload.mint,
                identity: payload.identity,
            },
        ) {
            Ok(success) => success,
            Err(error) => return Err(StatementMutationsErrorReturn::Mint(error)),
        };
        let transferred = match transfer(
            &TransferDeps {
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                random: deps.random,
            },
            TransferParams,
            TransferPayload {
                context: payload.transfer,
                parameter_set: &minted.delivery.parameter_set,
                identity_element: &minted.delivery.identity_element,
                holder: &minted.delivery.recipient,
            },
        ) {
            Ok(success) => success,
            Err(error) => return Err(StatementMutationsErrorReturn::Transfer(error)),
        };
        let mint_statement = &minted.delivery.statement;
        let mint_proof = &minted.delivery.proof;
        let transfer_statement = &transferred.delivery.statement;
        let transfer_proof = &transferred.delivery.proof;
        let mint_is_valid = match deps.delivery_proof.verify(
            VerifyParams,
            VerifyPayload {
                statement: AlgebraicStatement::Mint(mint_statement),
                proof: mint_proof,
            },
        ) {
            Ok(success) => success.is_valid,
            Err(error) => {
                return Err(StatementMutationsErrorReturn::Verify {
                    relation: DeliveryRelation::Mint,
                    error,
                });
            }
        };
        let transfer_is_valid = match deps.delivery_proof.verify(
            VerifyParams,
            VerifyPayload {
                statement: AlgebraicStatement::Transfer(transfer_statement),
                proof: transfer_proof,
            },
        ) {
            Ok(success) => success.is_valid,
            Err(error) => {
                return Err(StatementMutationsErrorReturn::Verify {
                    relation: DeliveryRelation::Transfer,
                    error,
                });
            }
        };
        let drawn = match deps.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(success) => success,
            Err(error) => return Err(StatementMutationsErrorReturn::FillBytes(error)),
        };
        let mut draws = minted.delivery.draws.clone();
        draws.extend_from_slice(&transferred.delivery.draws);
        draws.extend_from_slice(drawn.bytes.expose());
        let delta = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: drawn.bytes,
            },
        ) {
            Ok(success) => success.scalar.expose().clone(),
            Err(error) => return Err(StatementMutationsErrorReturn::OffsetSampling(error)),
        };
        let Ok(g1_generator) = deps
            .pairing
            .g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let Ok(g2_generator) = deps
            .pairing
            .g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let Ok(g1_offset) = deps.pairing.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1_generator.point,
                scalar: delta.clone(),
            },
        );
        let Ok(g2_offset) = deps.pairing.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2_generator.point,
                scalar: delta.clone(),
            },
        );
        let add_g1 = |point: &P::G1| -> P::G1 {
            let Ok(sum) = deps.pairing.add_g1(
                AddG1Params,
                AddG1Payload {
                    left: point.clone(),
                    right: g1_offset.product.clone(),
                },
            );
            sum.sum
        };
        let add_g2 = |point: &P::G2| -> P::G2 {
            let Ok(sum) = deps.pairing.add_g2(
                AddG2Params,
                AddG2Payload {
                    left: point.clone(),
                    right: g2_offset.product.clone(),
                },
            );
            sum.sum
        };
        let add_scalar = |scalar: &P::Scalar| -> P::Scalar {
            let Ok(sum) = deps.pairing.add_scalar(
                AddScalarParams,
                AddScalarPayload {
                    left: scalar.clone(),
                    right: delta.clone(),
                },
            );
            sum.sum
        };
        let mutate_canonical = |value: CanonicalFieldValue| -> CanonicalFieldValue {
            match value {
                CanonicalFieldValue::FixedBytes32(mut bytes) => {
                    bytes[31] ^= 0x01;
                    CanonicalFieldValue::FixedBytes32(bytes)
                }
                CanonicalFieldValue::FixedBytes20(mut bytes) => {
                    bytes[19] ^= 0x01;
                    CanonicalFieldValue::FixedBytes20(bytes)
                }
                CanonicalFieldValue::Unsigned256(mut bytes) => {
                    bytes[31] ^= 0x01;
                    CanonicalFieldValue::Unsigned256(bytes)
                }
                CanonicalFieldValue::Unsigned16(value) => {
                    CanonicalFieldValue::Unsigned16(value ^ 1)
                }
                CanonicalFieldValue::Unsigned32(value) => {
                    CanonicalFieldValue::Unsigned32(value ^ 1)
                }
                CanonicalFieldValue::Unsigned64(value) => {
                    CanonicalFieldValue::Unsigned64(value ^ 1)
                }
                CanonicalFieldValue::Bytes(mut bytes) => {
                    bytes.push(0x00);
                    CanonicalFieldValue::Bytes(bytes)
                }
                CanonicalFieldValue::Text(mut text) => {
                    text.push('0');
                    CanonicalFieldValue::Text(text)
                }
            }
        };
        let clone_mint = |statement: &proof::AlgebraicMintStatement<D::Forms, P::G1, P::G2>| {
            AlgebraicMintStatement {
                context: statement.context.clone(),
                hpub: statement.hpub.clone(),
                identity_element: statement.identity_element.clone(),
                buyer_keys: PublicKeysComponents {
                    pk1: statement.buyer_keys.pk1.clone(),
                    pk2: statement.buyer_keys.pk2.clone(),
                },
                envelope: EnvelopeComponents {
                    c1: statement.envelope.c1.clone(),
                    c2: statement.envelope.c2.clone(),
                    d1: statement.envelope.d1.clone(),
                    d2: statement.envelope.d2.clone(),
                },
            }
        };
        let clone_transfer =
            |statement: &proof::AlgebraicTransferStatement<D::Forms, P::G1, P::G2>| {
                AlgebraicTransferStatement {
                    context: statement.context.clone(),
                    identity_element: statement.identity_element.clone(),
                    seller_keys: PublicKeysComponents {
                        pk1: statement.seller_keys.pk1.clone(),
                        pk2: statement.seller_keys.pk2.clone(),
                    },
                    buyer_keys: PublicKeysComponents {
                        pk1: statement.buyer_keys.pk1.clone(),
                        pk2: statement.buyer_keys.pk2.clone(),
                    },
                    old_envelope: EnvelopeComponents {
                        c1: statement.old_envelope.c1.clone(),
                        c2: statement.old_envelope.c2.clone(),
                        d1: statement.old_envelope.d1.clone(),
                        d2: statement.old_envelope.d2.clone(),
                    },
                    new_envelope: EnvelopeComponents {
                        c1: statement.new_envelope.c1.clone(),
                        c2: statement.new_envelope.c2.clone(),
                        d1: statement.new_envelope.d1.clone(),
                        d2: statement.new_envelope.d2.clone(),
                    },
                }
            };
        let mut mint_mutations: Vec<MintMutation<P, D>> = Vec::new();
        for field in MINT_MUTATION_FIELDS.iter().take(21) {
            let mut statement = clone_mint(mint_statement);
            match field {
                MintMutationField::SuiteIdentifier => {
                    let mut identifier = *statement.context.suite_identifier.identifier();
                    identifier[31] ^= 0x01;
                    statement.context.suite_identifier =
                        match SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                            identifier,
                            version: statement.context.suite_identifier.version(),
                        }) {
                            Ok(value) => value,
                            Err(error) => {
                                return Err(
                                    StatementMutationsErrorReturn::MutatedSuiteIdentifier {
                                        relation: DeliveryRelation::Mint,
                                        error,
                                    },
                                );
                            }
                        };
                }
                MintMutationField::Chain => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::ChainIdentifier::to_field(
                        ToFieldParams,
                        &statement.context.chain,
                    );
                    statement.context.chain =
                        match <D::Forms as chain::IChainForms>::ChainIdentifier::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(
                                    StatementMutationsErrorReturn::MutatedChainIdentifier {
                                        relation: DeliveryRelation::Mint,
                                        error,
                                    },
                                );
                            }
                        };
                }
                MintMutationField::EntitlementContract => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Identity::to_field(
                        ToFieldParams,
                        &statement.context.entitlement_contract,
                    );
                    statement.context.entitlement_contract =
                        match <D::Forms as chain::IChainForms>::Identity::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedIdentity {
                                    relation: DeliveryRelation::Mint,
                                    field: DeliveryContextField::EntitlementContract,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::AssetRoot => {
                    let mut bytes = *statement.context.asset_root.as_bytes();
                    bytes[31] ^= 0x01;
                    statement.context.asset_root =
                        match Root::try_new(RootConstructorParams { bytes }) {
                            Ok(value) => value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedAssetRoot {
                                    relation: DeliveryRelation::Mint,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::ParameterSetIdentifier => {
                    let mut bytes = *statement.context.parameter_set_identifier.as_bytes();
                    bytes[31] ^= 0x01;
                    statement.context.parameter_set_identifier =
                        match ParameterSetIdentifier::try_new(
                            ParameterSetIdentifierConstructorParams { bytes },
                        ) {
                            Ok(value) => value,
                            Err(error) => {
                                return Err(
                                    StatementMutationsErrorReturn::MutatedParameterSetIdentifier {
                                        relation: DeliveryRelation::Mint,
                                        error,
                                    },
                                );
                            }
                        };
                }
                MintMutationField::SourceEntitlement => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Entitlement::to_field(
                        ToFieldParams,
                        &statement.context.source_entitlement,
                    );
                    statement.context.source_entitlement =
                        match <D::Forms as chain::IChainForms>::Entitlement::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedEntitlement {
                                    relation: DeliveryRelation::Mint,
                                    field: DeliveryContextField::SourceEntitlement,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::TargetEntitlement => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Entitlement::to_field(
                        ToFieldParams,
                        &statement.context.target_entitlement,
                    );
                    statement.context.target_entitlement =
                        match <D::Forms as chain::IChainForms>::Entitlement::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedEntitlement {
                                    relation: DeliveryRelation::Mint,
                                    field: DeliveryContextField::TargetEntitlement,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::OldInterval => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Interval::to_field(
                        ToFieldParams,
                        &statement.context.old_interval,
                    );
                    statement.context.old_interval =
                        match <D::Forms as chain::IChainForms>::Interval::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedInterval {
                                    relation: DeliveryRelation::Mint,
                                    field: DeliveryContextField::OldInterval,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::NewInterval => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Interval::to_field(
                        ToFieldParams,
                        &statement.context.new_interval,
                    );
                    statement.context.new_interval =
                        match <D::Forms as chain::IChainForms>::Interval::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedInterval {
                                    relation: DeliveryRelation::Mint,
                                    field: DeliveryContextField::NewInterval,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::Purpose => {
                    statement.context.purpose = match statement.context.purpose {
                        MintPurpose::Mint => MintPurpose::Replacement,
                        MintPurpose::Replacement => MintPurpose::Mint,
                    };
                }
                MintMutationField::Seller => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Identity::to_field(
                        ToFieldParams,
                        &statement.context.seller,
                    );
                    statement.context.seller =
                        match <D::Forms as chain::IChainForms>::Identity::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedIdentity {
                                    relation: DeliveryRelation::Mint,
                                    field: DeliveryContextField::Seller,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::Buyer => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Identity::to_field(
                        ToFieldParams,
                        &statement.context.buyer,
                    );
                    statement.context.buyer =
                        match <D::Forms as chain::IChainForms>::Identity::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedIdentity {
                                    relation: DeliveryRelation::Mint,
                                    field: DeliveryContextField::Buyer,
                                    error,
                                });
                            }
                        };
                }
                MintMutationField::Expiry => {
                    statement.context.expiry ^= 1;
                }
                MintMutationField::Hpub => {
                    statement.hpub = add_g2(&statement.hpub);
                }
                MintMutationField::IdentityElement => {
                    statement.identity_element = add_g1(&statement.identity_element);
                }
                MintMutationField::BuyerPk1 => {
                    statement.buyer_keys.pk1 = add_g1(&statement.buyer_keys.pk1);
                }
                MintMutationField::BuyerPk2 => {
                    statement.buyer_keys.pk2 = add_g2(&statement.buyer_keys.pk2);
                }
                MintMutationField::EnvelopeC1 => {
                    statement.envelope.c1 = add_g1(&statement.envelope.c1);
                }
                MintMutationField::EnvelopeC2 => {
                    statement.envelope.c2 = add_g1(&statement.envelope.c2);
                }
                MintMutationField::EnvelopeD1 => {
                    statement.envelope.d1 = add_g2(&statement.envelope.d1);
                }
                MintMutationField::EnvelopeD2 => {
                    statement.envelope.d2 = add_g2(&statement.envelope.d2);
                }
                _ => {}
            }
            let Ok(baseline_components) = deps.delivery_proof.proof_components(
                ProofComponentsParams,
                ProofComponentsPayload { proof: mint_proof },
            );
            let proof = match deps.delivery_proof.proof_from_components(
                ProofFromComponentsParams,
                ProofFromComponentsPayload {
                    components: baseline_components.components,
                },
            ) {
                Ok(success) => success.proof,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::ProofFromComponents {
                        relation: DeliveryRelation::Mint,
                        error,
                    });
                }
            };
            let is_valid = match deps.delivery_proof.verify(
                VerifyParams,
                VerifyPayload {
                    statement: AlgebraicStatement::Mint(&statement),
                    proof: &proof,
                },
            ) {
                Ok(success) => success.is_valid,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::Verify {
                        relation: DeliveryRelation::Mint,
                        error,
                    });
                }
            };
            mint_mutations.push(MintMutation {
                field: *field,
                statement,
                proof,
                is_valid,
            });
        }
        for field in MINT_MUTATION_FIELDS.iter().skip(21) {
            let Ok(baseline) = deps.delivery_proof.proof_components(
                ProofComponentsParams,
                ProofComponentsPayload { proof: mint_proof },
            );
            let components = match baseline.components {
                DeliveryProofComponents::MintBothGroups {
                    challenge,
                    responses,
                } => DeliveryProofComponents::MintBothGroups {
                    challenge: if *field == MintMutationField::Challenge {
                        add_scalar(&challenge)
                    } else {
                        challenge
                    },
                    responses: MintResponses {
                        alpha: if *field == MintMutationField::Alpha {
                            add_scalar(&responses.alpha)
                        } else {
                            responses.alpha
                        },
                        r: if *field == MintMutationField::R {
                            add_scalar(&responses.r)
                        } else {
                            responses.r
                        },
                        rho: if *field == MintMutationField::Rho {
                            add_scalar(&responses.rho)
                        } else {
                            responses.rho
                        },
                        sigma: if *field == MintMutationField::Sigma {
                            add_scalar(&responses.sigma)
                        } else {
                            responses.sigma
                        },
                    },
                },
                DeliveryProofComponents::MintFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => DeliveryProofComponents::MintFirstGroupOnly {
                    challenge: if *field == MintMutationField::Challenge {
                        add_scalar(&challenge)
                    } else {
                        challenge
                    },
                    responses: MintResponses {
                        alpha: if *field == MintMutationField::Alpha {
                            add_scalar(&responses.alpha)
                        } else {
                            responses.alpha
                        },
                        r: if *field == MintMutationField::R {
                            add_scalar(&responses.r)
                        } else {
                            responses.r
                        },
                        rho: if *field == MintMutationField::Rho {
                            add_scalar(&responses.rho)
                        } else {
                            responses.rho
                        },
                        sigma: if *field == MintMutationField::Sigma {
                            add_scalar(&responses.sigma)
                        } else {
                            responses.sigma
                        },
                    },
                    first_messages,
                },
                _ => {
                    return Err(StatementMutationsErrorReturn::ProofComponents {
                        relation: DeliveryRelation::Mint,
                    });
                }
            };
            let proof = match deps.delivery_proof.proof_from_components(
                ProofFromComponentsParams,
                ProofFromComponentsPayload { components },
            ) {
                Ok(success) => success.proof,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::ProofFromComponents {
                        relation: DeliveryRelation::Mint,
                        error,
                    });
                }
            };
            let statement = clone_mint(mint_statement);
            let is_valid = match deps.delivery_proof.verify(
                VerifyParams,
                VerifyPayload {
                    statement: AlgebraicStatement::Mint(&statement),
                    proof: &proof,
                },
            ) {
                Ok(success) => success.is_valid,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::Verify {
                        relation: DeliveryRelation::Mint,
                        error,
                    });
                }
            };
            mint_mutations.push(MintMutation {
                field: *field,
                statement,
                proof,
                is_valid,
            });
        }
        let mut transfer_mutations: Vec<TransferMutation<P, D>> = Vec::new();
        for field in TRANSFER_MUTATION_FIELDS.iter().take(26) {
            let mut statement = clone_transfer(transfer_statement);
            match field {
                TransferMutationField::SuiteIdentifier => {
                    let mut identifier = *statement.context.suite_identifier.identifier();
                    identifier[31] ^= 0x01;
                    statement.context.suite_identifier =
                        match SuiteIdentifier::try_new(SuiteIdentifierConstructorParams {
                            identifier,
                            version: statement.context.suite_identifier.version(),
                        }) {
                            Ok(value) => value,
                            Err(error) => {
                                return Err(
                                    StatementMutationsErrorReturn::MutatedSuiteIdentifier {
                                        relation: DeliveryRelation::Transfer,
                                        error,
                                    },
                                );
                            }
                        };
                }
                TransferMutationField::Chain => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::ChainIdentifier::to_field(
                        ToFieldParams,
                        &statement.context.chain,
                    );
                    statement.context.chain =
                        match <D::Forms as chain::IChainForms>::ChainIdentifier::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(
                                    StatementMutationsErrorReturn::MutatedChainIdentifier {
                                        relation: DeliveryRelation::Transfer,
                                        error,
                                    },
                                );
                            }
                        };
                }
                TransferMutationField::EntitlementContract => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Identity::to_field(
                        ToFieldParams,
                        &statement.context.entitlement_contract,
                    );
                    statement.context.entitlement_contract =
                        match <D::Forms as chain::IChainForms>::Identity::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedIdentity {
                                    relation: DeliveryRelation::Transfer,
                                    field: DeliveryContextField::EntitlementContract,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::AssetRoot => {
                    let mut bytes = *statement.context.asset_root.as_bytes();
                    bytes[31] ^= 0x01;
                    statement.context.asset_root =
                        match Root::try_new(RootConstructorParams { bytes }) {
                            Ok(value) => value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedAssetRoot {
                                    relation: DeliveryRelation::Transfer,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::ParameterSetIdentifier => {
                    let mut bytes = *statement.context.parameter_set_identifier.as_bytes();
                    bytes[31] ^= 0x01;
                    statement.context.parameter_set_identifier =
                        match ParameterSetIdentifier::try_new(
                            ParameterSetIdentifierConstructorParams { bytes },
                        ) {
                            Ok(value) => value,
                            Err(error) => {
                                return Err(
                                    StatementMutationsErrorReturn::MutatedParameterSetIdentifier {
                                        relation: DeliveryRelation::Transfer,
                                        error,
                                    },
                                );
                            }
                        };
                }
                TransferMutationField::SourceEntitlement => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Entitlement::to_field(
                        ToFieldParams,
                        &statement.context.source_entitlement,
                    );
                    statement.context.source_entitlement =
                        match <D::Forms as chain::IChainForms>::Entitlement::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedEntitlement {
                                    relation: DeliveryRelation::Transfer,
                                    field: DeliveryContextField::SourceEntitlement,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::TargetEntitlement => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Entitlement::to_field(
                        ToFieldParams,
                        &statement.context.target_entitlement,
                    );
                    statement.context.target_entitlement =
                        match <D::Forms as chain::IChainForms>::Entitlement::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedEntitlement {
                                    relation: DeliveryRelation::Transfer,
                                    field: DeliveryContextField::TargetEntitlement,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::OldInterval => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Interval::to_field(
                        ToFieldParams,
                        &statement.context.old_interval,
                    );
                    statement.context.old_interval =
                        match <D::Forms as chain::IChainForms>::Interval::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedInterval {
                                    relation: DeliveryRelation::Transfer,
                                    field: DeliveryContextField::OldInterval,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::NewInterval => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Interval::to_field(
                        ToFieldParams,
                        &statement.context.new_interval,
                    );
                    statement.context.new_interval =
                        match <D::Forms as chain::IChainForms>::Interval::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedInterval {
                                    relation: DeliveryRelation::Transfer,
                                    field: DeliveryContextField::NewInterval,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::Purpose => {
                    statement.context.purpose = match statement.context.purpose {
                        TransferPurpose::Grant => TransferPurpose::Transfer,
                        TransferPurpose::Transfer => TransferPurpose::Grant,
                    };
                }
                TransferMutationField::Seller => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Identity::to_field(
                        ToFieldParams,
                        &statement.context.seller,
                    );
                    statement.context.seller =
                        match <D::Forms as chain::IChainForms>::Identity::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedIdentity {
                                    relation: DeliveryRelation::Transfer,
                                    field: DeliveryContextField::Seller,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::Buyer => {
                    let Ok(canonical) = <D::Forms as chain::IChainForms>::Identity::to_field(
                        ToFieldParams,
                        &statement.context.buyer,
                    );
                    statement.context.buyer =
                        match <D::Forms as chain::IChainForms>::Identity::from_field(
                            FromFieldParams,
                            mutate_canonical(canonical.field),
                        ) {
                            Ok(success) => success.value,
                            Err(error) => {
                                return Err(StatementMutationsErrorReturn::MutatedIdentity {
                                    relation: DeliveryRelation::Transfer,
                                    field: DeliveryContextField::Buyer,
                                    error,
                                });
                            }
                        };
                }
                TransferMutationField::Expiry => {
                    statement.context.expiry ^= 1;
                }
                TransferMutationField::IdentityElement => {
                    statement.identity_element = add_g1(&statement.identity_element);
                }
                TransferMutationField::SellerPk1 => {
                    statement.seller_keys.pk1 = add_g1(&statement.seller_keys.pk1);
                }
                TransferMutationField::SellerPk2 => {
                    statement.seller_keys.pk2 = add_g2(&statement.seller_keys.pk2);
                }
                TransferMutationField::BuyerPk1 => {
                    statement.buyer_keys.pk1 = add_g1(&statement.buyer_keys.pk1);
                }
                TransferMutationField::BuyerPk2 => {
                    statement.buyer_keys.pk2 = add_g2(&statement.buyer_keys.pk2);
                }
                TransferMutationField::OldEnvelopeC1 => {
                    statement.old_envelope.c1 = add_g1(&statement.old_envelope.c1);
                }
                TransferMutationField::OldEnvelopeC2 => {
                    statement.old_envelope.c2 = add_g1(&statement.old_envelope.c2);
                }
                TransferMutationField::OldEnvelopeD1 => {
                    statement.old_envelope.d1 = add_g2(&statement.old_envelope.d1);
                }
                TransferMutationField::OldEnvelopeD2 => {
                    statement.old_envelope.d2 = add_g2(&statement.old_envelope.d2);
                }
                TransferMutationField::NewEnvelopeC1 => {
                    statement.new_envelope.c1 = add_g1(&statement.new_envelope.c1);
                }
                TransferMutationField::NewEnvelopeC2 => {
                    statement.new_envelope.c2 = add_g1(&statement.new_envelope.c2);
                }
                TransferMutationField::NewEnvelopeD1 => {
                    statement.new_envelope.d1 = add_g2(&statement.new_envelope.d1);
                }
                TransferMutationField::NewEnvelopeD2 => {
                    statement.new_envelope.d2 = add_g2(&statement.new_envelope.d2);
                }
                _ => {}
            }
            let Ok(baseline_components) = deps.delivery_proof.proof_components(
                ProofComponentsParams,
                ProofComponentsPayload {
                    proof: transfer_proof,
                },
            );
            let proof = match deps.delivery_proof.proof_from_components(
                ProofFromComponentsParams,
                ProofFromComponentsPayload {
                    components: baseline_components.components,
                },
            ) {
                Ok(success) => success.proof,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::ProofFromComponents {
                        relation: DeliveryRelation::Transfer,
                        error,
                    });
                }
            };
            let is_valid = match deps.delivery_proof.verify(
                VerifyParams,
                VerifyPayload {
                    statement: AlgebraicStatement::Transfer(&statement),
                    proof: &proof,
                },
            ) {
                Ok(success) => success.is_valid,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::Verify {
                        relation: DeliveryRelation::Transfer,
                        error,
                    });
                }
            };
            transfer_mutations.push(TransferMutation {
                field: *field,
                statement,
                proof,
                is_valid,
            });
        }
        for field in TRANSFER_MUTATION_FIELDS.iter().skip(26) {
            let Ok(baseline) = deps.delivery_proof.proof_components(
                ProofComponentsParams,
                ProofComponentsPayload {
                    proof: transfer_proof,
                },
            );
            let components = match baseline.components {
                DeliveryProofComponents::TransferBothGroups {
                    challenge,
                    responses,
                } => DeliveryProofComponents::TransferBothGroups {
                    challenge: if *field == TransferMutationField::Challenge {
                        add_scalar(&challenge)
                    } else {
                        challenge
                    },
                    responses: TransferResponses {
                        x: if *field == TransferMutationField::X {
                            add_scalar(&responses.x)
                        } else {
                            responses.x
                        },
                        y: if *field == TransferMutationField::Y {
                            add_scalar(&responses.y)
                        } else {
                            responses.y
                        },
                        s: if *field == TransferMutationField::S {
                            add_scalar(&responses.s)
                        } else {
                            responses.s
                        },
                        rho: if *field == TransferMutationField::Rho {
                            add_scalar(&responses.rho)
                        } else {
                            responses.rho
                        },
                        sigma: if *field == TransferMutationField::Sigma {
                            add_scalar(&responses.sigma)
                        } else {
                            responses.sigma
                        },
                    },
                },
                DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge,
                    responses,
                    first_messages,
                } => DeliveryProofComponents::TransferFirstGroupOnly {
                    challenge: if *field == TransferMutationField::Challenge {
                        add_scalar(&challenge)
                    } else {
                        challenge
                    },
                    responses: TransferResponses {
                        x: if *field == TransferMutationField::X {
                            add_scalar(&responses.x)
                        } else {
                            responses.x
                        },
                        y: if *field == TransferMutationField::Y {
                            add_scalar(&responses.y)
                        } else {
                            responses.y
                        },
                        s: if *field == TransferMutationField::S {
                            add_scalar(&responses.s)
                        } else {
                            responses.s
                        },
                        rho: if *field == TransferMutationField::Rho {
                            add_scalar(&responses.rho)
                        } else {
                            responses.rho
                        },
                        sigma: if *field == TransferMutationField::Sigma {
                            add_scalar(&responses.sigma)
                        } else {
                            responses.sigma
                        },
                    },
                    first_messages,
                },
                _ => {
                    return Err(StatementMutationsErrorReturn::ProofComponents {
                        relation: DeliveryRelation::Transfer,
                    });
                }
            };
            let proof = match deps.delivery_proof.proof_from_components(
                ProofFromComponentsParams,
                ProofFromComponentsPayload { components },
            ) {
                Ok(success) => success.proof,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::ProofFromComponents {
                        relation: DeliveryRelation::Transfer,
                        error,
                    });
                }
            };
            let statement = clone_transfer(transfer_statement);
            let is_valid = match deps.delivery_proof.verify(
                VerifyParams,
                VerifyPayload {
                    statement: AlgebraicStatement::Transfer(&statement),
                    proof: &proof,
                },
            ) {
                Ok(success) => success.is_valid,
                Err(error) => {
                    return Err(StatementMutationsErrorReturn::Verify {
                        relation: DeliveryRelation::Transfer,
                        error,
                    });
                }
            };
            transfer_mutations.push(TransferMutation {
                field: *field,
                statement,
                proof,
                is_valid,
            });
        }
        vectors.push(StatementMutationsVector {
            draws,
            mint_statement: minted.delivery.statement,
            mint_proof: minted.delivery.proof,
            mint_is_valid,
            mint_mutations,
            transfer_statement: transferred.delivery.statement,
            transfer_proof: transferred.delivery.proof,
            transfer_is_valid,
            transfer_mutations,
        });
    }
    Ok(StatementMutationsSuccessReturn { vectors })
}
