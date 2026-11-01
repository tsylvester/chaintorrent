use core::convert::Infallible;
use core::num::NonZeroU32;
use core::time::Duration;
#[cfg(any(test, feature = "mocks"))]
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingErrorReturn, CreatePairingParams,
    IPairingArithmetic, IPairingReference,
};
use pairing::{PairingConcrete, SampleUniformScalarErrorReturn};
#[cfg(any(test, feature = "mocks"))]
use random::{CreateRandomSourceErrorReturn, CreateRandomSourceParams};
use random::{FillBytesErrorReturn, IRandomSourceAdapter};

pub struct PairingBenchmark {
    pub(super) random: Box<dyn IRandomSourceAdapter>,
    pub(super) iterations: NonZeroU32,
}

pub struct PairingBenchmarkConstructorParams {
    pub random: Box<dyn IRandomSourceAdapter>,
    pub iterations: NonZeroU32,
}

pub type PairingBenchmarkTryNewReturn = Result<PairingBenchmark, Infallible>;

pub struct PairingOperationTimings {
    pub mul_g1: Duration,
    pub mul_g2: Duration,
    pub msm_g1: Duration,
    pub msm_g2: Duration,
    pub pairing_product: Duration,
}

pub struct PairingBenchmarkSuccessReturn {
    pub concrete: PairingConcrete,
    pub iterations: NonZeroU32,
    pub timings: PairingOperationTimings,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PairingBenchmarkErrorReturn {
    FillBytes(FillBytesErrorReturn),
    SampleScalar(SampleUniformScalarErrorReturn),
}

pub type PairingBenchmarkReturn =
    Result<PairingBenchmarkSuccessReturn, PairingBenchmarkErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub trait IPairingBenchmarkCaseConsumer {
    type Output;

    fn consume_pairing_benchmark_case<P: IPairingArithmetic + IPairingReference>(
        &self,
        random: Box<dyn IRandomSourceAdapter>,
        params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output;
}

#[cfg(any(test, feature = "mocks"))]
pub struct PairingBenchmarkCaseDeps<C: IPairingBenchmarkCaseConsumer> {
    pub consumer: C,
}

#[cfg(any(test, feature = "mocks"))]
pub struct PairingBenchmarkCaseParams {
    pub pairing: CreatePairingParams,
    pub random_source: CreateRandomSourceParams,
}

#[cfg(any(test, feature = "mocks"))]
pub struct PairingBenchmarkCasePayload;

#[cfg(any(test, feature = "mocks"))]
pub struct PairingBenchmarkCaseSuccessReturn<O> {
    pub output: O,
}

#[cfg(any(test, feature = "mocks"))]
#[derive(Debug, PartialEq, Eq)]
pub enum PairingBenchmarkCaseErrorReturn {
    CreatePairing(CreatePairingErrorReturn),
    CreateRandomSource(CreateRandomSourceErrorReturn),
}

#[cfg(any(test, feature = "mocks"))]
pub type PairingBenchmarkCaseReturn<O> =
    Result<PairingBenchmarkCaseSuccessReturn<O>, PairingBenchmarkCaseErrorReturn>;

#[cfg(any(test, feature = "mocks"))]
pub type RunPairingBenchmarkCaseFn<C: IPairingBenchmarkCaseConsumer> =
    fn(
        &PairingBenchmarkCaseDeps<C>,
        PairingBenchmarkCaseParams,
        PairingBenchmarkCasePayload,
    ) -> PairingBenchmarkCaseReturn<<C as IPairingBenchmarkCaseConsumer>::Output>;
