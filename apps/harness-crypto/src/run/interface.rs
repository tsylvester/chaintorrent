use pairing::CreatePairingErrorReturn;
use random::CreateRandomSourceErrorReturn;

use super::pairing_consumer::provides::PairingConsumerErrorReturn;
use crate::benchmark::provides::{PairingBenchmarkErrorReturn, PairingBenchmarkSuccessReturn};
use crate::config::provides::HarnessConfiguration;
use crate::generate::provides::EmittedFile;
use crate::measure::provides::DeliveryMeasurement;
use crate::report::provides::{ReportErrorReturn, ReportSuccessReturn, ReportText};

#[derive(Clone, Copy, Debug)]
pub struct RunDeps;

#[derive(Clone, Copy, Debug)]
pub struct RunParams;

#[derive(Clone)]
pub struct RunPayload {
    pub configuration: HarnessConfiguration,
}

#[derive(Debug)]
pub struct RunCurveOutcome {
    pub label: ReportText,
    pub files: Vec<EmittedFile>,
    pub measurement: DeliveryMeasurement,
    pub report: ReportSuccessReturn,
}

pub struct RunSuccessReturn {
    pub benchmarks: Vec<PairingBenchmarkSuccessReturn>,
    pub curves: Vec<RunCurveOutcome>,
}

#[derive(Debug)]
pub enum RunErrorReturn {
    CreateRandomSource(CreateRandomSourceErrorReturn),
    CreatePairing(CreatePairingErrorReturn),
    Benchmark(PairingBenchmarkErrorReturn),
    PairingConsumer(PairingConsumerErrorReturn),
    Report(ReportErrorReturn),
}

pub type RunReturn = Result<RunSuccessReturn, RunErrorReturn>;

pub type RunFn = fn(&RunDeps, RunParams, RunPayload) -> RunReturn;
