#![allow(dead_code, clippy::too_many_arguments)]

use chain::IChainForms;
use proof::{DeliveryContext, build_delivery_context};

use super::interface::{
    AdmitContextsDeps, AdmitContextsFn, AdmitContextsParams, AdmitContextsPayload,
    AdmitContextsReturn, AdmitContextsSuccessReturn,
};
use crate::config::provides::CanonicalDeliveryContext;

#[derive(Default)]
pub struct AdmitContextsDepsOverrides;

pub fn build_admit_contexts_deps(_overrides: AdmitContextsDepsOverrides) -> AdmitContextsDeps {
    AdmitContextsDeps
}

#[derive(Default)]
pub struct AdmitContextsParamsOverrides;

pub fn build_admit_contexts_params(
    _overrides: AdmitContextsParamsOverrides,
) -> AdmitContextsParams {
    AdmitContextsParams
}

#[derive(Default)]
pub struct AdmitContextsPayloadOverrides;

pub fn build_admit_contexts_payload<'a, Purpose>(
    context: &'a CanonicalDeliveryContext,
    purpose: Purpose,
    _overrides: AdmitContextsPayloadOverrides,
) -> AdmitContextsPayload<'a, Purpose> {
    AdmitContextsPayload { context, purpose }
}

#[derive(Default)]
pub struct AdmitContextsSuccessReturnOverrides;

pub fn build_admit_contexts_success_return<F: IChainForms, Purpose>(
    context: DeliveryContext<F, Purpose>,
    _overrides: AdmitContextsSuccessReturnOverrides,
) -> AdmitContextsSuccessReturn<F, Purpose> {
    AdmitContextsSuccessReturn { context }
}

pub fn mock_admit_contexts<F: IChainForms, Purpose>(
    _deps: &AdmitContextsDeps,
    _params: AdmitContextsParams,
    payload: AdmitContextsPayload<'_, Purpose>,
) -> AdmitContextsReturn<F, Purpose> {
    let _: AdmitContextsFn<F, Purpose> = mock_admit_contexts;
    Ok(build_admit_contexts_success_return(
        build_delivery_context::<F, Purpose>(payload.purpose, Default::default()),
        Default::default(),
    ))
}
