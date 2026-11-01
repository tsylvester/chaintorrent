#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_arguments
)]

use super::interface::{
    IPairingBenchmarkCaseConsumer, PairingBenchmark, PairingBenchmarkCaseDeps,
    PairingBenchmarkCaseErrorReturn, PairingBenchmarkCaseParams, PairingBenchmarkCasePayload,
    PairingBenchmarkCaseReturn, PairingBenchmarkCaseSuccessReturn,
    PairingBenchmarkConstructorParams, PairingBenchmarkSuccessReturn, PairingOperationTimings,
};
use core::num::NonZeroU32;
use core::time::Duration;
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingParams, IPairingArithmetic,
    IPairingConsumer, IPairingReference, MockIPairingAdapterFailureMode, PairingConcrete,
    build_create_pairing_deps, build_create_pairing_params, build_create_pairing_payload,
    create_pairing,
};
use random::{
    CreateRandomSourceParams, CreateRandomSourceParamsOverrides, IRandomSourceAdapter,
    build_create_random_source_deps, build_create_random_source_params,
    build_create_random_source_payload, create_random_source,
};

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

#[derive(Default)]
pub struct PairingBenchmarkCaseParamsOverrides {
    pub pairing: Option<CreatePairingParams>,
    pub random_source: Option<CreateRandomSourceParams>,
}

pub fn build_pairing_benchmark_case_params(
    overrides: PairingBenchmarkCaseParamsOverrides,
) -> PairingBenchmarkCaseParams {
    PairingBenchmarkCaseParams {
        pairing: overrides
            .pairing
            .unwrap_or_else(|| build_create_pairing_params(Default::default())),
        random_source: overrides
            .random_source
            .unwrap_or_else(|| build_create_random_source_params(Default::default())),
    }
}

#[derive(Default)]
pub struct PairingBenchmarkCaseDepsOverrides;

pub fn build_pairing_benchmark_case_deps<C: IPairingBenchmarkCaseConsumer>(
    consumer: C,
    _overrides: PairingBenchmarkCaseDepsOverrides,
) -> PairingBenchmarkCaseDeps<C> {
    PairingBenchmarkCaseDeps { consumer }
}

#[derive(Default)]
pub struct PairingBenchmarkCasePayloadOverrides;

pub fn build_pairing_benchmark_case_payload(
    _overrides: PairingBenchmarkCasePayloadOverrides,
) -> PairingBenchmarkCasePayload {
    PairingBenchmarkCasePayload
}

#[derive(Default)]
pub struct PairingBenchmarkCaseSuccessReturnOverrides;

pub fn build_pairing_benchmark_case_success_return<O>(
    output: O,
    _overrides: PairingBenchmarkCaseSuccessReturnOverrides,
) -> PairingBenchmarkCaseSuccessReturn<O> {
    PairingBenchmarkCaseSuccessReturn { output }
}

struct CaseAtPairing<'a, C: IPairingBenchmarkCaseConsumer> {
    consumer: &'a C,
    random_source: CreateRandomSourceParams,
}

impl<C: IPairingBenchmarkCaseConsumer> IPairingConsumer for CaseAtPairing<'_, C> {
    type Output = PairingBenchmarkCaseReturn<C::Output>;

    fn consume_pairing<P: IPairingArithmetic + IPairingReference>(
        &self,
        params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let created = match create_random_source(
            &build_create_random_source_deps(Default::default()),
            build_create_random_source_params(CreateRandomSourceParamsOverrides {
                kind: Some(self.random_source.kind),
            }),
            build_create_random_source_payload(Default::default()),
        ) {
            Ok(created) => created,
            Err(error) => {
                return Err(PairingBenchmarkCaseErrorReturn::CreateRandomSource(error));
            }
        };
        let output = self
            .consumer
            .consume_pairing_benchmark_case(created.adapter, params, payload);
        Ok(PairingBenchmarkCaseSuccessReturn { output })
    }
}

pub fn run_pairing_benchmark_case<C: IPairingBenchmarkCaseConsumer>(
    deps: &PairingBenchmarkCaseDeps<C>,
    params: PairingBenchmarkCaseParams,
    _payload: PairingBenchmarkCasePayload,
) -> PairingBenchmarkCaseReturn<C::Output> {
    match create_pairing(
        &build_create_pairing_deps(
            CaseAtPairing {
                consumer: &deps.consumer,
                random_source: params.random_source,
            },
            Default::default(),
        ),
        params.pairing,
        build_create_pairing_payload(Default::default()),
    ) {
        Ok(success) => success.output,
        Err(error) => Err(PairingBenchmarkCaseErrorReturn::CreatePairing(error)),
    }
}
