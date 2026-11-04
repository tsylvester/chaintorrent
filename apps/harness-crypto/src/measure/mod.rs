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

use core::time::Duration;
use domain::{Secret, SecretConstructorParams};
use envelope::{
    EnvelopeCoins, EnvelopeComponentsParams, EnvelopeComponentsPayload, IKeyAgreementAdapter,
    KeyPairComponents,
};
use interface::{
    DeliveryCosts, DeliveryMeasurement, DeliveryOperationTimings, DeliverySizes,
    MeasureErrorReturn, MeasureFn, MeasureParams, MeasurePayload, MeasureReturn,
    MeasureSuccessReturn, RejectedDelivery,
};
use kem::{
    CapsuleComponents, CapsuleComponentsParams, CapsuleComponentsPayload, DecapsulateParams,
    DecapsulatePayload, EncapsulateParams, EncapsulatePayload, ICredentialKemAdapter,
    MasterScalarComponents,
};
use pairing::{
    EncodeG1Params, EncodeG1Payload, EncodeG2Params, EncodeG2Payload, EncodeScalarParams,
    EncodeScalarPayload, IPairingAdapter, ISampleUniformScalar,
};
use proof::{
    AlgebraicStatement, DeliveryProofComponents, DeliveryRelation, IDeliveryProofAdapter,
    ProofComponentsParams, ProofComponentsPayload, ProveMintParams, ProveMintPayload,
    ProveTransferParams, ProveTransferPayload, VerifyParams, VerifyPayload,
};
use random::{FillBytesParams, FillBytesPayload};
use std::time::Instant;

use crate::delivery::provides::{
    MintDeps, MintParams, MintPayload, TransferDeps, TransferParams, TransferPayload, mint,
    transfer,
};
use crate::verifier::provides::{
    IVerifierAdapter, VerifyOnChainParams, VerifyOnChainSuccessReturn,
};

use interface::MeasureDeps;

pub fn measure<'a, P, K, A, D, V>(
    deps: &MeasureDeps<'a, P, K, A, D, V>,
    params: MeasureParams,
    payload: MeasurePayload<'_, D::Forms>,
) -> MeasureReturn
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
    V: IVerifierAdapter<DeliveryProof = D>,
{
    let _: MeasureFn<'a, P, K, A, D, V> = measure;
    let mut prove_mint_total = Duration::ZERO;
    let mut prove_transfer_total = Duration::ZERO;
    let mut verify_mint_total = Duration::ZERO;
    let mut verify_transfer_total = Duration::ZERO;
    let mut decapsulate_total = Duration::ZERO;
    for _ in 0..params.iterations.get() {
        let minted = match mint::<P, K, A, D>(
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
            Ok(minted) => minted,
            Err(error) => return Err(MeasureErrorReturn::Mint(error)),
        };
        let transferred = match transfer::<P, K, A, D>(
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
            Ok(transferred) => transferred,
            Err(error) => return Err(MeasureErrorReturn::Transfer(error)),
        };
        let draw = match deps.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(draw) => draw,
            Err(error) => return Err(MeasureErrorReturn::FillBytes(error)),
        };
        let encapsulated = match deps.kem.encapsulate(
            EncapsulateParams,
            EncapsulatePayload {
                parameter_set: &minted.delivery.parameter_set,
                uniform: draw.bytes,
            },
        ) {
            Ok(encapsulated) => encapsulated,
            Err(error) => return Err(MeasureErrorReturn::Encapsulate(error)),
        };
        let Ok(master_scalar) = Secret::try_new(SecretConstructorParams {
            value: minted
                .delivery
                .witnesses
                .master_scalar
                .value
                .expose()
                .clone(),
        });
        let Ok(credential_randomness) = Secret::try_new(SecretConstructorParams {
            value: minted
                .delivery
                .witnesses
                .credential_randomness
                .expose()
                .clone(),
        });
        let Ok(mint_rho) = Secret::try_new(SecretConstructorParams {
            value: minted.delivery.witnesses.coins.rho.expose().clone(),
        });
        let Ok(mint_sigma) = Secret::try_new(SecretConstructorParams {
            value: minted.delivery.witnesses.coins.sigma.expose().clone(),
        });
        let prove_mint_payload = ProveMintPayload {
            master_scalar: MasterScalarComponents {
                value: master_scalar,
            },
            credential_randomness,
            coins: EnvelopeCoins {
                rho: mint_rho,
                sigma: mint_sigma,
            },
            statement: &minted.delivery.statement,
        };
        let prove_mint_start = Instant::now();
        let proved_mint = deps
            .delivery_proof
            .prove_mint(ProveMintParams, prove_mint_payload);
        prove_mint_total = prove_mint_total.saturating_add(prove_mint_start.elapsed());
        match proved_mint {
            Ok(_proof) => {}
            Err(error) => return Err(MeasureErrorReturn::ProveMint(error)),
        }
        let Ok(seller_x) = Secret::try_new(SecretConstructorParams {
            value: transferred
                .delivery
                .witnesses
                .seller_secrets
                .x
                .expose()
                .clone(),
        });
        let Ok(seller_y) = Secret::try_new(SecretConstructorParams {
            value: transferred
                .delivery
                .witnesses
                .seller_secrets
                .y
                .expose()
                .clone(),
        });
        let Ok(offset) = Secret::try_new(SecretConstructorParams {
            value: transferred.delivery.witnesses.offset.expose().clone(),
        });
        let Ok(transfer_rho) = Secret::try_new(SecretConstructorParams {
            value: transferred.delivery.witnesses.coins.rho.expose().clone(),
        });
        let Ok(transfer_sigma) = Secret::try_new(SecretConstructorParams {
            value: transferred.delivery.witnesses.coins.sigma.expose().clone(),
        });
        let prove_transfer_payload = ProveTransferPayload {
            seller_secrets: KeyPairComponents {
                x: seller_x,
                y: seller_y,
            },
            offset,
            coins: EnvelopeCoins {
                rho: transfer_rho,
                sigma: transfer_sigma,
            },
            statement: &transferred.delivery.statement,
        };
        let prove_transfer_start = Instant::now();
        let proved_transfer = deps
            .delivery_proof
            .prove_transfer(ProveTransferParams, prove_transfer_payload);
        prove_transfer_total = prove_transfer_total.saturating_add(prove_transfer_start.elapsed());
        match proved_transfer {
            Ok(_proof) => {}
            Err(error) => return Err(MeasureErrorReturn::ProveTransfer(error)),
        }
        let verify_mint_payload = VerifyPayload {
            statement: AlgebraicStatement::Mint(&minted.delivery.statement),
            proof: &minted.delivery.proof,
        };
        let verify_mint_start = Instant::now();
        let verified_mint = deps
            .delivery_proof
            .verify(VerifyParams, verify_mint_payload);
        verify_mint_total = verify_mint_total.saturating_add(verify_mint_start.elapsed());
        match verified_mint {
            Ok(_verdict) => {}
            Err(error) => {
                return Err(MeasureErrorReturn::Verify {
                    relation: DeliveryRelation::Mint,
                    error,
                });
            }
        }
        let verify_transfer_payload = VerifyPayload {
            statement: AlgebraicStatement::Transfer(&transferred.delivery.statement),
            proof: &transferred.delivery.proof,
        };
        let verify_transfer_start = Instant::now();
        let verified_transfer = deps
            .delivery_proof
            .verify(VerifyParams, verify_transfer_payload);
        verify_transfer_total =
            verify_transfer_total.saturating_add(verify_transfer_start.elapsed());
        match verified_transfer {
            Ok(_verdict) => {}
            Err(error) => {
                return Err(MeasureErrorReturn::Verify {
                    relation: DeliveryRelation::Transfer,
                    error,
                });
            }
        }
        let decapsulate_payload = DecapsulatePayload {
            identity_element: &minted.delivery.identity_element,
            credential: &minted.delivery.credential,
            capsule: &encapsulated.capsule,
        };
        let decapsulate_start = Instant::now();
        let opened = deps.kem.decapsulate(DecapsulateParams, decapsulate_payload);
        decapsulate_total = decapsulate_total.saturating_add(decapsulate_start.elapsed());
        let Ok(_opened) = opened;
    }
    let timings = DeliveryOperationTimings {
        prove_mint: prove_mint_total / params.iterations.get(),
        prove_transfer: prove_transfer_total / params.iterations.get(),
        verify_mint: verify_mint_total / params.iterations.get(),
        verify_transfer: verify_transfer_total / params.iterations.get(),
        decapsulate: decapsulate_total / params.iterations.get(),
    };
    let minted = match mint::<P, K, A, D>(
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
        Ok(minted) => minted,
        Err(error) => return Err(MeasureErrorReturn::Mint(error)),
    };
    let transferred = match transfer::<P, K, A, D>(
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
        Ok(transferred) => transferred,
        Err(error) => return Err(MeasureErrorReturn::Transfer(error)),
    };
    let draw = match deps.random.fill_bytes(
        FillBytesParams,
        FillBytesPayload {
            length: <P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH,
        },
    ) {
        Ok(draw) => draw,
        Err(error) => return Err(MeasureErrorReturn::FillBytes(error)),
    };
    let encapsulated = match deps.kem.encapsulate(
        EncapsulateParams,
        EncapsulatePayload {
            parameter_set: &minted.delivery.parameter_set,
            uniform: draw.bytes,
        },
    ) {
        Ok(encapsulated) => encapsulated,
        Err(error) => return Err(MeasureErrorReturn::Encapsulate(error)),
    };
    let onchain_mint_payload = VerifyPayload {
        statement: AlgebraicStatement::Mint(&minted.delivery.statement),
        proof: &minted.delivery.proof,
    };
    let mint_cost = match deps
        .verifier
        .verify_on_chain(VerifyOnChainParams, onchain_mint_payload)
    {
        Ok(VerifyOnChainSuccessReturn::Accepted { cost }) => cost,
        Ok(VerifyOnChainSuccessReturn::Rejected) => {
            return Err(MeasureErrorReturn::NotAccepted {
                relation: DeliveryRelation::Mint,
                verdict: RejectedDelivery::Rejected,
            });
        }
        Ok(VerifyOnChainSuccessReturn::Reverted { data }) => {
            return Err(MeasureErrorReturn::NotAccepted {
                relation: DeliveryRelation::Mint,
                verdict: RejectedDelivery::Reverted { data },
            });
        }
        Err(error) => {
            return Err(MeasureErrorReturn::OnChain {
                relation: DeliveryRelation::Mint,
                error,
            });
        }
    };
    let onchain_transfer_payload = VerifyPayload {
        statement: AlgebraicStatement::Transfer(&transferred.delivery.statement),
        proof: &transferred.delivery.proof,
    };
    let transfer_cost = match deps
        .verifier
        .verify_on_chain(VerifyOnChainParams, onchain_transfer_payload)
    {
        Ok(VerifyOnChainSuccessReturn::Accepted { cost }) => cost,
        Ok(VerifyOnChainSuccessReturn::Rejected) => {
            return Err(MeasureErrorReturn::NotAccepted {
                relation: DeliveryRelation::Transfer,
                verdict: RejectedDelivery::Rejected,
            });
        }
        Ok(VerifyOnChainSuccessReturn::Reverted { data }) => {
            return Err(MeasureErrorReturn::NotAccepted {
                relation: DeliveryRelation::Transfer,
                verdict: RejectedDelivery::Reverted { data },
            });
        }
        Err(error) => {
            return Err(MeasureErrorReturn::OnChain {
                relation: DeliveryRelation::Transfer,
                error,
            });
        }
    };
    let encode_g1_len = |point: P::G1| -> usize {
        let Ok(encoded) = deps
            .pairing
            .encode_g1(EncodeG1Params, EncodeG1Payload { point });
        encoded.bytes.as_ref().len()
    };
    let encode_g2_len = |point: P::G2| -> usize {
        let Ok(encoded) = deps
            .pairing
            .encode_g2(EncodeG2Params, EncodeG2Payload { point });
        encoded.bytes.as_ref().len()
    };
    let encode_scalar_len = |scalar: P::Scalar| -> usize {
        let Ok(encoded) = deps
            .pairing
            .encode_scalar(EncodeScalarParams, EncodeScalarPayload { scalar });
        encoded.bytes.expose().as_ref().len()
    };
    let Ok(capsule_components) = deps.kem.capsule_components(
        CapsuleComponentsParams,
        CapsuleComponentsPayload {
            capsule: &encapsulated.capsule,
        },
    );
    let capsule = match capsule_components.components {
        CapsuleComponents::Entitlement { u, v, w } => {
            encode_g2_len(u) + encode_g1_len(v) + encode_g1_len(w)
        }
        CapsuleComponents::Asset { u, v } => encode_g2_len(u) + encode_g1_len(v),
    };
    let Ok(envelope_components) = deps.key_agreement.envelope_components(
        EnvelopeComponentsParams,
        EnvelopeComponentsPayload {
            envelope: &minted.delivery.recipient.envelope,
        },
    );
    let envelope = encode_g1_len(envelope_components.components.c1)
        + encode_g1_len(envelope_components.components.c2)
        + encode_g2_len(envelope_components.components.d1)
        + encode_g2_len(envelope_components.components.d2);
    let Ok(mint_components) = deps.delivery_proof.proof_components(
        ProofComponentsParams,
        ProofComponentsPayload {
            proof: &minted.delivery.proof,
        },
    );
    let mint_proof = match mint_components.components {
        DeliveryProofComponents::MintBothGroups {
            challenge,
            responses,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.alpha)
                + encode_scalar_len(responses.r)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
        }
        DeliveryProofComponents::MintFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.alpha)
                + encode_scalar_len(responses.r)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
                + encode_g2_len(first_messages.hpub)
                + encode_g2_len(first_messages.d1)
                + encode_g2_len(first_messages.d2)
        }
        DeliveryProofComponents::TransferBothGroups {
            challenge,
            responses,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.x)
                + encode_scalar_len(responses.y)
                + encode_scalar_len(responses.s)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
        }
        DeliveryProofComponents::TransferFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.x)
                + encode_scalar_len(responses.y)
                + encode_scalar_len(responses.s)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
                + encode_g2_len(first_messages.pk2)
                + encode_g2_len(first_messages.d1)
                + encode_g2_len(first_messages.d2)
        }
    };
    let Ok(transfer_components) = deps.delivery_proof.proof_components(
        ProofComponentsParams,
        ProofComponentsPayload {
            proof: &transferred.delivery.proof,
        },
    );
    let transfer_proof = match transfer_components.components {
        DeliveryProofComponents::MintBothGroups {
            challenge,
            responses,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.alpha)
                + encode_scalar_len(responses.r)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
        }
        DeliveryProofComponents::MintFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.alpha)
                + encode_scalar_len(responses.r)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
                + encode_g2_len(first_messages.hpub)
                + encode_g2_len(first_messages.d1)
                + encode_g2_len(first_messages.d2)
        }
        DeliveryProofComponents::TransferBothGroups {
            challenge,
            responses,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.x)
                + encode_scalar_len(responses.y)
                + encode_scalar_len(responses.s)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
        }
        DeliveryProofComponents::TransferFirstGroupOnly {
            challenge,
            responses,
            first_messages,
        } => {
            encode_scalar_len(challenge)
                + encode_scalar_len(responses.x)
                + encode_scalar_len(responses.y)
                + encode_scalar_len(responses.s)
                + encode_scalar_len(responses.rho)
                + encode_scalar_len(responses.sigma)
                + encode_g2_len(first_messages.pk2)
                + encode_g2_len(first_messages.d1)
                + encode_g2_len(first_messages.d2)
        }
    };
    Ok(MeasureSuccessReturn {
        measurement: DeliveryMeasurement {
            iterations: params.iterations,
            timings,
            sizes: DeliverySizes {
                capsule,
                envelope,
                mint_proof,
                transfer_proof,
            },
            costs: DeliveryCosts {
                mint: mint_cost,
                transfer: transfer_cost,
            },
        },
    })
}
