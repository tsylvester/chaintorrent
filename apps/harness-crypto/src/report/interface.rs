use core::num::NonZeroU32;
use std::path::PathBuf;

use crate::measure::provides::DeliveryMeasurement;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportText {
    pub(super) text: String,
}

pub struct ReportTextConstructorParams {
    pub text: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReportTextTryNewErrorReturn {
    Empty,
}

pub type ReportTextTryNewReturn = Result<ReportText, ReportTextTryNewErrorReturn>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReportDestination {
    StandardOutput,
    File { path: PathBuf },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportDefaults {
    pub piece_group_size: NonZeroU32,
    pub primary_curve: ReportText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportAdjustment {
    pub piece_group_size: Option<NonZeroU32>,
    pub primary_curve: Option<ReportText>,
}

#[derive(Clone, Copy, Debug)]
pub struct ReportDeps;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportParams {
    pub destinations: Vec<ReportDestination>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportPayload {
    pub measurement: DeliveryMeasurement,
    pub curve: ReportText,
    pub conditions: ReportText,
    pub defaults: ReportDefaults,
    pub adjustment: Option<ReportAdjustment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportSuccessReturn {
    pub document: String,
}

#[derive(Debug)]
pub enum ReportErrorReturn {
    WriteFile {
        path: PathBuf,
        error: std::io::Error,
    },
    WriteStandardOutput(std::io::Error),
}

pub type ReportReturn = Result<ReportSuccessReturn, ReportErrorReturn>;

pub type ReportFn = fn(&ReportDeps, ReportParams, ReportPayload) -> ReportReturn;
