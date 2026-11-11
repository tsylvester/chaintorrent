use std::path::PathBuf;

use crate::config::provides::OutputDestination;
use crate::run::provides::RunReturn;

#[derive(Clone, Copy, Debug)]
pub struct OutputDeps;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputParams {
    pub destinations: Vec<OutputDestination>,
}

pub struct OutputPayload {
    pub outcome: RunReturn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputSuccessReturn {
    pub text: String,
}

#[derive(Debug)]
pub enum OutputErrorReturn {
    WriteFile {
        path: PathBuf,
        error: std::io::Error,
    },
    WriteStandardOutput(std::io::Error),
}

pub type OutputReturn = Result<OutputSuccessReturn, OutputErrorReturn>;

pub type OutputFn = fn(&OutputDeps, OutputParams, OutputPayload) -> OutputReturn;
