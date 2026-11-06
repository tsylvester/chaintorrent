pub mod delivery_context_field;
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

use chain::IChainForms;
use encoding::{FromFieldParams, ICanonicalField};
use proof::DeliveryContext;

use delivery_context_field::provides::DeliveryContextField;
use interface::{
    AdmitContextsDeps, AdmitContextsErrorReturn, AdmitContextsFn, AdmitContextsParams,
    AdmitContextsPayload, AdmitContextsReturn, AdmitContextsSuccessReturn,
};

pub fn admit_contexts<F: IChainForms, Purpose>(
    _deps: &AdmitContextsDeps,
    _params: AdmitContextsParams,
    payload: AdmitContextsPayload<'_, Purpose>,
) -> AdmitContextsReturn<F, Purpose> {
    let _: AdmitContextsFn<F, Purpose> = admit_contexts;

    let chain = match F::ChainIdentifier::from_field(FromFieldParams, payload.context.chain.clone())
    {
        Ok(built) => built.value,
        Err(error) => return Err(AdmitContextsErrorReturn::ChainIdentifier(error)),
    };
    let entitlement_contract = match F::Identity::from_field(
        FromFieldParams,
        payload.context.entitlement_contract.clone(),
    ) {
        Ok(built) => built.value,
        Err(error) => {
            return Err(AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::EntitlementContract,
                error,
            });
        }
    };
    let source_entitlement = match F::Entitlement::from_field(
        FromFieldParams,
        payload.context.source_entitlement.clone(),
    ) {
        Ok(built) => built.value,
        Err(error) => {
            return Err(AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::SourceEntitlement,
                error,
            });
        }
    };
    let target_entitlement = match F::Entitlement::from_field(
        FromFieldParams,
        payload.context.target_entitlement.clone(),
    ) {
        Ok(built) => built.value,
        Err(error) => {
            return Err(AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::TargetEntitlement,
                error,
            });
        }
    };
    let old_interval =
        match F::Interval::from_field(FromFieldParams, payload.context.old_interval.clone()) {
            Ok(built) => built.value,
            Err(error) => {
                return Err(AdmitContextsErrorReturn::Field {
                    field: DeliveryContextField::OldInterval,
                    error,
                });
            }
        };
    let new_interval =
        match F::Interval::from_field(FromFieldParams, payload.context.new_interval.clone()) {
            Ok(built) => built.value,
            Err(error) => {
                return Err(AdmitContextsErrorReturn::Field {
                    field: DeliveryContextField::NewInterval,
                    error,
                });
            }
        };
    let seller = match F::Identity::from_field(FromFieldParams, payload.context.seller.clone()) {
        Ok(built) => built.value,
        Err(error) => {
            return Err(AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::Seller,
                error,
            });
        }
    };
    let buyer = match F::Identity::from_field(FromFieldParams, payload.context.buyer.clone()) {
        Ok(built) => built.value,
        Err(error) => {
            return Err(AdmitContextsErrorReturn::Field {
                field: DeliveryContextField::Buyer,
                error,
            });
        }
    };

    Ok(AdmitContextsSuccessReturn {
        context: DeliveryContext {
            suite_identifier: payload.context.suite_identifier.clone(),
            chain,
            entitlement_contract,
            asset_root: payload.context.asset_root,
            parameter_set_identifier: payload.context.parameter_set_identifier.clone(),
            source_entitlement,
            target_entitlement,
            old_interval,
            new_interval,
            purpose: payload.purpose,
            seller,
            buyer,
            expiry: payload.context.expiry,
        },
    })
}
