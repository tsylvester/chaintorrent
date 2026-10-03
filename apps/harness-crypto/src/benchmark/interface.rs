use core::convert::Infallible;
use core::num::NonZeroU32;
use core::time::Duration;
use pairing::{PairingConcrete, SampleUniformScalarErrorReturn};
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

pub enum PairingBenchmarkErrorReturn {
    FillBytes(FillBytesErrorReturn),
    SampleScalar(SampleUniformScalarErrorReturn),
}

pub type PairingBenchmarkReturn =
    Result<PairingBenchmarkSuccessReturn, PairingBenchmarkErrorReturn>;
