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

use crate::report::provides::ReportDestination;
use interface::{
    AdmitOutputDeps, AdmitOutputErrorReturn, AdmitOutputFn, AdmitOutputParams, AdmitOutputPayload,
    AdmitOutputReturn, AdmitOutputSuccessReturn, AdmittedOutput, DESTINATION_KINDS,
    DestinationKind, OutputDestination,
};

pub fn admit_output(
    _deps: &AdmitOutputDeps,
    _params: AdmitOutputParams,
    payload: AdmitOutputPayload,
) -> AdmitOutputReturn {
    let _: AdmitOutputFn = admit_output;

    if payload.output.destinations.is_empty() {
        return Err(AdmitOutputErrorReturn::OutputDestinationsEmpty);
    }
    let mut destinations = Vec::with_capacity(payload.output.destinations.len());
    for (index, entry) in payload.output.destinations.iter().enumerate() {
        let kind = match kind_named(&entry.kind) {
            Some(kind) => kind,
            None => {
                return Err(AdmitOutputErrorReturn::OutputDestinationKind {
                    index,
                    text: entry.kind.clone(),
                });
            }
        };
        let destination = match kind {
            DestinationKind::StandardOutput => OutputDestination::StandardOutput,
            DestinationKind::File => match &entry.path {
                Some(path) => OutputDestination::File { path: path.clone() },
                None => {
                    return Err(AdmitOutputErrorReturn::OutputDestinationPath { index });
                }
            },
        };
        destinations.push(destination);
    }

    if payload.report.destinations.is_empty() {
        return Err(AdmitOutputErrorReturn::ReportDestinationsEmpty);
    }
    let mut report_destinations = Vec::with_capacity(payload.report.destinations.len());
    for (index, entry) in payload.report.destinations.iter().enumerate() {
        let kind = match kind_named(&entry.kind) {
            Some(kind) => kind,
            None => {
                return Err(AdmitOutputErrorReturn::ReportDestinationKind {
                    index,
                    text: entry.kind.clone(),
                });
            }
        };
        let destination = match kind {
            DestinationKind::StandardOutput => ReportDestination::StandardOutput,
            DestinationKind::File => match &entry.path {
                Some(path) => ReportDestination::File { path: path.clone() },
                None => {
                    return Err(AdmitOutputErrorReturn::ReportDestinationPath { index });
                }
            },
        };
        report_destinations.push(destination);
    }

    Ok(AdmitOutputSuccessReturn {
        admitted: AdmittedOutput {
            destinations,
            report_destinations,
        },
    })
}

fn kind_named(text: &str) -> Option<DestinationKind> {
    DESTINATION_KINDS
        .iter()
        .find(|member| format!("{:?}", member) == text)
        .copied()
}
