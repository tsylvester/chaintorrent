use std::path::PathBuf;

use super::super::layers::provides::OutputDocument;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputDestination {
    StandardOutput,
    File { path: PathBuf },
}

pub struct AdmitOutputDeps;

pub struct AdmitOutputParams;

pub struct AdmitOutputPayload<'a> {
    pub output: &'a OutputDocument,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmitOutputSuccessReturn {
    pub destinations: Vec<OutputDestination>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdmitOutputErrorReturn {
    Empty,
    UnknownKind { index: usize, text: String },
    MissingPath { index: usize },
    UnexpectedPath { index: usize },
}

pub type AdmitOutputReturn = Result<AdmitOutputSuccessReturn, AdmitOutputErrorReturn>;

pub type AdmitOutputFn =
    fn(&AdmitOutputDeps, AdmitOutputParams, AdmitOutputPayload<'_>) -> AdmitOutputReturn;
