#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::{
    PairingBenchmark, PairingBenchmarkConstructorParams, PairingBenchmarkSuccessReturn,
    PairingOperationTimings,
};
use core::num::NonZeroU32;
use core::time::Duration;
use pairing::{MockIPairingAdapterFailureMode, PairingConcrete};
use random::IRandomSourceAdapter;

#[derive(Default)]
pub struct PairingBenchmarkConstructorParamsOverrides {
    pub iterations: Option<NonZeroU32>,
}

pub fn build_pairing_benchmark_constructor_params(
    random: Box<dyn IRandomSourceAdapter>,
    overrides: PairingBenchmarkConstructorParamsOverrides,
) -> PairingBenchmarkConstructorParams {
    PairingBenchmarkConstructorParams {
        random,
        iterations: overrides.iterations.unwrap_or(NonZeroU32::MIN),
    }
}

pub fn build_pairing_benchmark(
    random: Box<dyn IRandomSourceAdapter>,
    overrides: PairingBenchmarkConstructorParamsOverrides,
) -> PairingBenchmark {
    let Ok(benchmark) = PairingBenchmark::try_new(build_pairing_benchmark_constructor_params(
        random, overrides,
    ));
    benchmark
}

#[derive(Default)]
pub struct PairingOperationTimingsOverrides {
    pub mul_g1: Option<Duration>,
    pub mul_g2: Option<Duration>,
    pub msm_g1: Option<Duration>,
    pub msm_g2: Option<Duration>,
    pub pairing_product: Option<Duration>,
}

pub fn build_pairing_operation_timings(
    overrides: PairingOperationTimingsOverrides,
) -> PairingOperationTimings {
    PairingOperationTimings {
        mul_g1: overrides.mul_g1.unwrap_or(Duration::ZERO),
        mul_g2: overrides.mul_g2.unwrap_or(Duration::ZERO),
        msm_g1: overrides.msm_g1.unwrap_or(Duration::ZERO),
        msm_g2: overrides.msm_g2.unwrap_or(Duration::ZERO),
        pairing_product: overrides.pairing_product.unwrap_or(Duration::ZERO),
    }
}

#[derive(Default)]
pub struct PairingBenchmarkSuccessReturnOverrides {
    pub concrete: Option<PairingConcrete>,
    pub iterations: Option<NonZeroU32>,
    pub timings: Option<PairingOperationTimings>,
}

pub fn build_pairing_benchmark_success_return(
    overrides: PairingBenchmarkSuccessReturnOverrides,
) -> PairingBenchmarkSuccessReturn {
    PairingBenchmarkSuccessReturn {
        concrete: overrides.concrete.unwrap_or(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::Succeeds,
        )),
        iterations: overrides.iterations.unwrap_or(NonZeroU32::MIN),
        timings: overrides
            .timings
            .unwrap_or_else(|| build_pairing_operation_timings(Default::default())),
    }
}
