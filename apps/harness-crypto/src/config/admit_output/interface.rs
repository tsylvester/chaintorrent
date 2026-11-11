use std::path::PathBuf;

use super::super::layers::provides::{OutputTable, ReportTable};
use crate::report::provides::ReportDestination;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DestinationKind {
    StandardOutput,
    File,
}

pub const DESTINATION_KINDS: &[DestinationKind] =
    &[DestinationKind::StandardOutput, DestinationKind::File];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputDestination {
    StandardOutput,
    File { path: PathBuf },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedOutput {
    pub destinations: Vec<OutputDestination>,
    pub report_destinations: Vec<ReportDestination>,
}

#[derive(Clone, Copy, Debug)]
pub struct AdmitOutputDeps;

#[derive(Clone, Copy, Debug)]
pub struct AdmitOutputParams;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitOutputPayload {
    pub output: OutputTable,
    pub report: ReportTable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitOutputSuccessReturn {
    pub admitted: AdmittedOutput,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitOutputErrorReturn {
    OutputDestinationsEmpty,
    OutputDestinationKind { index: usize, text: String },
    OutputDestinationPath { index: usize },
    ReportDestinationsEmpty,
    ReportDestinationKind { index: usize, text: String },
    ReportDestinationPath { index: usize },
}

pub type AdmitOutputReturn = Result<AdmitOutputSuccessReturn, AdmitOutputErrorReturn>;

pub type AdmitOutputFn =
    fn(&AdmitOutputDeps, AdmitOutputParams, AdmitOutputPayload) -> AdmitOutputReturn;
