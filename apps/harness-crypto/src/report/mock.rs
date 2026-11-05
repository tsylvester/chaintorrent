#![allow(dead_code, clippy::too_many_arguments)]

use core::num::NonZeroU32;
use serde_json::{Map, Value};

use super::interface::{
    ReportAdjustment, ReportDefaults, ReportDeps, ReportDestination, ReportFn, ReportParams,
    ReportPayload, ReportReturn, ReportSuccessReturn, ReportText, ReportTextConstructorParams,
};
use crate::measure::provides::{DeliveryMeasurement, build_delivery_measurement};

#[derive(Default)]
pub struct ReportTextConstructorParamsOverrides {
    pub text: Option<String>,
}

pub fn build_report_text_constructor_params(
    overrides: ReportTextConstructorParamsOverrides,
) -> ReportTextConstructorParams {
    ReportTextConstructorParams {
        text: overrides.text.unwrap_or_else(|| "curve".to_string()),
    }
}

#[derive(Default)]
pub struct ReportTextConstructorParamsCorruptions {
    pub text: Option<Value>,
}

pub fn invalidate_report_text_constructor_params(
    corruptions: ReportTextConstructorParamsCorruptions,
) -> Value {
    let params = build_report_text_constructor_params(Default::default());
    let mut fields = Map::new();
    fields.insert("text".to_string(), Value::from(params.text));
    if let Some(text) = corruptions.text {
        fields.insert("text".to_string(), text);
    }
    Value::Object(fields)
}

pub fn build_report_text(overrides: ReportTextConstructorParamsOverrides) -> ReportText {
    match ReportText::try_new(build_report_text_constructor_params(overrides)) {
        Ok(text) => text,
        Err(_) => build_report_text(Default::default()),
    }
}

#[derive(Default)]
pub struct ReportDefaultsOverrides {
    pub piece_group_size: Option<NonZeroU32>,
    pub primary_curve: Option<ReportText>,
}

pub fn build_report_defaults(overrides: ReportDefaultsOverrides) -> ReportDefaults {
    ReportDefaults {
        piece_group_size: overrides.piece_group_size.unwrap_or(NonZeroU32::MIN),
        primary_curve: overrides
            .primary_curve
            .unwrap_or_else(|| build_report_text(Default::default())),
    }
}

#[derive(Default)]
pub struct ReportAdjustmentOverrides {
    pub piece_group_size: Option<Option<NonZeroU32>>,
    pub primary_curve: Option<Option<ReportText>>,
}

pub fn build_report_adjustment(overrides: ReportAdjustmentOverrides) -> ReportAdjustment {
    ReportAdjustment {
        piece_group_size: overrides.piece_group_size.unwrap_or(None),
        primary_curve: overrides.primary_curve.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct ReportDepsOverrides;

pub fn build_report_deps(_overrides: ReportDepsOverrides) -> ReportDeps {
    ReportDeps
}

#[derive(Default)]
pub struct ReportParamsOverrides {
    pub destinations: Option<Vec<ReportDestination>>,
}

pub fn build_report_params(overrides: ReportParamsOverrides) -> ReportParams {
    ReportParams {
        destinations: overrides.destinations.unwrap_or_default(),
    }
}

#[derive(Default)]
pub struct ReportPayloadOverrides {
    pub measurement: Option<DeliveryMeasurement>,
    pub curve: Option<ReportText>,
    pub conditions: Option<ReportText>,
    pub defaults: Option<ReportDefaults>,
    pub adjustment: Option<Option<ReportAdjustment>>,
}

pub fn build_report_payload(overrides: ReportPayloadOverrides) -> ReportPayload {
    ReportPayload {
        measurement: overrides
            .measurement
            .unwrap_or_else(|| build_delivery_measurement(Default::default())),
        curve: overrides
            .curve
            .unwrap_or_else(|| build_report_text(Default::default())),
        conditions: overrides
            .conditions
            .unwrap_or_else(|| build_report_text(Default::default())),
        defaults: overrides
            .defaults
            .unwrap_or_else(|| build_report_defaults(Default::default())),
        adjustment: overrides.adjustment.unwrap_or(None),
    }
}

#[derive(Default)]
pub struct ReportSuccessReturnOverrides {
    pub document: Option<String>,
}

pub fn build_report_success_return(overrides: ReportSuccessReturnOverrides) -> ReportSuccessReturn {
    ReportSuccessReturn {
        document: overrides.document.unwrap_or_else(|| "{}".to_string()),
    }
}

pub fn mock_report(
    _deps: &ReportDeps,
    _params: ReportParams,
    _payload: ReportPayload,
) -> ReportReturn {
    let _: ReportFn = mock_report;
    Ok(build_report_success_return(Default::default()))
}
