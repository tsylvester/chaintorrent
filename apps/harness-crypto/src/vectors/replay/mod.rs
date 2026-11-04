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

use encoding::{CanonicalFieldValue, ICanonicalField, ToFieldParams};
use envelope::IKeyAgreementAdapter;
use kem::ICredentialKemAdapter;
use pairing::IPairingAdapter;
use proof::{
    AlgebraicStatement, AlgebraicTransferStatement, IDeliveryProofAdapter, VerifyParams,
    VerifyPayload,
};

use crate::delivery::provides::{
    MintDeps, MintParams, MintPayload, TransferDeps, TransferParams, TransferPayload, mint,
    transfer,
};
use interface::{
    NextSettlementField, ReplayDeps, ReplayErrorReturn, ReplayParams, ReplayPayload, ReplayReturn,
    ReplaySettlement, ReplaySuccessReturn, ReplayVector, ReplayVerdict,
};

pub fn replay<'a, P, K, A, D>(
    deps: &ReplayDeps<'a, P, K, A, D>,
    params: ReplayParams,
    payload: ReplayPayload<'_, D::Forms>,
) -> ReplayReturn<P, D>
where
    P: IPairingAdapter,
    K: ICredentialKemAdapter<Pairing = P>,
    A: IKeyAgreementAdapter<Pairing = P>,
    D: IDeliveryProofAdapter<Pairing = P>,
{
    fn field<X: ICanonicalField>(form: &X) -> CanonicalFieldValue {
        let Ok(canonical) = X::to_field(ToFieldParams, form);
        canonical.field
    }
    if field(&payload.next.chain) != field(&payload.transfer.chain) {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::Chain,
        });
    }
    if field(&payload.next.entitlement_contract) != field(&payload.transfer.entitlement_contract) {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::EntitlementContract,
        });
    }
    if payload.next.asset_root.as_bytes() != payload.transfer.asset_root.as_bytes() {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::AssetRoot,
        });
    }
    if payload.next.parameter_set_identifier.as_bytes()
        != payload.transfer.parameter_set_identifier.as_bytes()
    {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::ParameterSetIdentifier,
        });
    }
    if field(&payload.next.source_entitlement) != field(&payload.transfer.source_entitlement) {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::SourceEntitlement,
        });
    }
    if field(&payload.next.target_entitlement) != field(&payload.transfer.target_entitlement) {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::TargetEntitlement,
        });
    }
    if field(&payload.next.old_interval) != field(&payload.transfer.new_interval) {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::OldInterval,
        });
    }
    if field(&payload.next.seller) != field(&payload.transfer.buyer) {
        return Err(ReplayErrorReturn::NotTheNextSettlement {
            field: NextSettlementField::Seller,
        });
    }
    let verdict = |statement: &AlgebraicTransferStatement<D::Forms, P::G1, P::G2>,
                   proof: &D::Proof,
                   verdict: ReplayVerdict|
     -> Result<bool, ReplayErrorReturn> {
        match deps.delivery_proof.verify(
            VerifyParams,
            VerifyPayload {
                statement: AlgebraicStatement::Transfer(statement),
                proof,
            },
        ) {
            Ok(success) => Ok(success.is_valid),
            Err(error) => Err(ReplayErrorReturn::Verify { verdict, error }),
        }
    };
    let mut vectors: Vec<ReplayVector<P, D>> = Vec::new();
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
            Err(error) => return Err(ReplayErrorReturn::Mint(error)),
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
            Err(error) => {
                return Err(ReplayErrorReturn::Transfer {
                    settlement: ReplaySettlement::Transfer,
                    error,
                });
            }
        };
        let next = match transfer(
            &TransferDeps {
                kem: deps.kem,
                key_agreement: deps.key_agreement,
                delivery_proof: deps.delivery_proof,
                random: deps.random,
            },
            TransferParams,
            TransferPayload {
                context: payload.next,
                parameter_set: &minted.delivery.parameter_set,
                identity_element: &minted.delivery.identity_element,
                holder: &transferred.delivery.buyer,
            },
        ) {
            Ok(success) => success,
            Err(error) => {
                return Err(ReplayErrorReturn::Transfer {
                    settlement: ReplaySettlement::Next,
                    error,
                });
            }
        };
        let transfer_is_valid = verdict(
            &transferred.delivery.statement,
            &transferred.delivery.proof,
            ReplayVerdict::Transfer,
        )?;
        let next_is_valid = verdict(
            &next.delivery.statement,
            &next.delivery.proof,
            ReplayVerdict::Next,
        )?;
        let replay_is_valid = verdict(
            &next.delivery.statement,
            &transferred.delivery.proof,
            ReplayVerdict::Replay,
        )?;
        let mut draws = minted.delivery.draws.clone();
        draws.extend_from_slice(&transferred.delivery.draws);
        draws.extend_from_slice(&next.delivery.draws);
        vectors.push(ReplayVector {
            draws,
            transfer_statement: transferred.delivery.statement,
            transfer_proof: transferred.delivery.proof,
            transfer_is_valid,
            next_statement: next.delivery.statement,
            next_proof: next.delivery.proof,
            next_is_valid,
            replay_is_valid,
        });
    }
    Ok(ReplaySuccessReturn { vectors })
}
