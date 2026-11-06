use chain::IChainForms;
use encoding::FromFieldErrorReturn;
use proof::DeliveryContext;

use super::delivery_context_field::provides::DeliveryContextField;
use crate::config::provides::CanonicalDeliveryContext;

#[derive(Clone, Copy, Debug)]
pub struct AdmitContextsDeps;

#[derive(Clone, Copy, Debug)]
pub struct AdmitContextsParams;

pub struct AdmitContextsPayload<'a, Purpose> {
    pub context: &'a CanonicalDeliveryContext,
    pub purpose: Purpose,
}

pub struct AdmitContextsSuccessReturn<F: IChainForms, Purpose> {
    pub context: DeliveryContext<F, Purpose>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitContextsErrorReturn {
    ChainIdentifier(FromFieldErrorReturn),
    Field {
        field: DeliveryContextField,
        error: FromFieldErrorReturn,
    },
}

pub type AdmitContextsReturn<F, Purpose> =
    Result<AdmitContextsSuccessReturn<F, Purpose>, AdmitContextsErrorReturn>;

pub type AdmitContextsFn<F, Purpose> = for<'a> fn(
    &AdmitContextsDeps,
    AdmitContextsParams,
    AdmitContextsPayload<'a, Purpose>,
) -> AdmitContextsReturn<F, Purpose>;
