#![allow(dead_code, clippy::too_many_arguments)]

use super::super::layers::provides::{
    DestinationTableOverrides, OutputTable, OutputTableOverrides, ReportTable,
    ReportTableOverrides, build_destination_table, build_output_table, build_report_table,
};
use super::interface::{
    AdmitOutputDeps, AdmitOutputFn, AdmitOutputParams, AdmitOutputPayload, AdmitOutputReturn,
    AdmitOutputSuccessReturn, AdmittedOutput, DESTINATION_KINDS, OutputDestination,
};
use crate::report::provides::ReportDestination;

pub use super::super::admit_selections::provides::declared_name;

#[derive(Default)]
pub struct AdmittedOutputOverrides {
    pub destinations: Option<Vec<OutputDestination>>,
    pub report_destinations: Option<Vec<ReportDestination>>,
}

pub fn build_admitted_output(overrides: AdmittedOutputOverrides) -> AdmittedOutput {
    AdmittedOutput {
        destinations: overrides
            .destinations
            .unwrap_or_else(|| vec![OutputDestination::StandardOutput]),
        report_destinations: overrides
            .report_destinations
            .unwrap_or_else(|| vec![ReportDestination::StandardOutput]),
    }
}

#[derive(Default)]
pub struct AdmitOutputDepsOverrides;

pub fn build_admit_output_deps(_overrides: AdmitOutputDepsOverrides) -> AdmitOutputDeps {
    AdmitOutputDeps
}

#[derive(Default)]
pub struct AdmitOutputParamsOverrides;

pub fn build_admit_output_params(_overrides: AdmitOutputParamsOverrides) -> AdmitOutputParams {
    AdmitOutputParams
}

#[derive(Default)]
pub struct AdmitOutputPayloadOverrides {
    pub output: Option<OutputTable>,
    pub report: Option<ReportTable>,
}

pub fn build_admit_output_payload(overrides: AdmitOutputPayloadOverrides) -> AdmitOutputPayload {
    AdmitOutputPayload {
        output: overrides.output.unwrap_or_else(|| {
            build_output_table(OutputTableOverrides {
                destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(&DESTINATION_KINDS[0])),
                    path: Some(None),
                })]),
            })
        }),
        report: overrides.report.unwrap_or_else(|| {
            build_report_table(ReportTableOverrides {
                destinations: Some(vec![build_destination_table(DestinationTableOverrides {
                    kind: Some(declared_name(&DESTINATION_KINDS[0])),
                    path: Some(None),
                })]),
                ..Default::default()
            })
        }),
    }
}

#[derive(Default)]
pub struct AdmitOutputSuccessReturnOverrides {
    pub admitted: Option<AdmittedOutput>,
}

pub fn build_admit_output_success_return(
    overrides: AdmitOutputSuccessReturnOverrides,
) -> AdmitOutputSuccessReturn {
    AdmitOutputSuccessReturn {
        admitted: overrides
            .admitted
            .unwrap_or_else(|| build_admitted_output(Default::default())),
    }
}

pub fn mock_admit_output(
    _deps: &AdmitOutputDeps,
    _params: AdmitOutputParams,
    _payload: AdmitOutputPayload,
) -> AdmitOutputReturn {
    let _: AdmitOutputFn = mock_admit_output;
    Ok(build_admit_output_success_return(Default::default()))
}
