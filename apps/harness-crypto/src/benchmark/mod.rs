mod interface;
#[cfg(any(test, feature = "mocks"))]
mod mock;
pub mod provides;
#[cfg(test)]
mod test;

use interface::{
    PairingBenchmark, PairingBenchmarkConstructorParams, PairingBenchmarkErrorReturn,
    PairingBenchmarkReturn, PairingBenchmarkSuccessReturn, PairingBenchmarkTryNewReturn,
    PairingOperationTimings,
};
use pairing::{
    ConsumePairingParams, ConsumePairingPayload, G1GeneratorParams, G1GeneratorPayload,
    G2GeneratorParams, G2GeneratorPayload, IPairingAdapter, IPairingConsumer, ISampleUniformScalar,
    MsmG1Params, MsmG1Payload, MsmG1Term, MsmG2Params, MsmG2Payload, MsmG2Term, MulG1Params,
    MulG1Payload, MulG2Params, MulG2Payload, PairingProductIsOneParams, PairingProductIsOnePayload,
    PairingProductTerm, SampleUniformScalarParams, SampleUniformScalarPayload,
};
use random::{FillBytesParams, FillBytesPayload};
use std::time::Instant;

impl PairingBenchmark {
    pub fn try_new(params: PairingBenchmarkConstructorParams) -> PairingBenchmarkTryNewReturn {
        Ok(PairingBenchmark {
            random: params.random,
            iterations: params.iterations,
        })
    }
}

impl IPairingConsumer for PairingBenchmark {
    type Output = PairingBenchmarkReturn;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        let adapter = &payload.adapter;

        let draw_a = match self.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: P::Scalar::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(draw) => draw,
            Err(error) => return Err(PairingBenchmarkErrorReturn::FillBytes(error)),
        };
        let draw_b = match self.random.fill_bytes(
            FillBytesParams,
            FillBytesPayload {
                length: P::Scalar::UNIFORM_BYTES_LENGTH,
            },
        ) {
            Ok(draw) => draw,
            Err(error) => return Err(PairingBenchmarkErrorReturn::FillBytes(error)),
        };

        let a = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: draw_a.bytes,
            },
        ) {
            Ok(sample) => sample.scalar,
            Err(error) => return Err(PairingBenchmarkErrorReturn::SampleScalar(error)),
        };
        let b = match P::Scalar::sample_from_uniform_bytes(
            SampleUniformScalarParams,
            SampleUniformScalarPayload {
                uniform: draw_b.bytes,
            },
        ) {
            Ok(sample) => sample.scalar,
            Err(error) => return Err(PairingBenchmarkErrorReturn::SampleScalar(error)),
        };

        let Ok(g1) = adapter.g1_generator(G1GeneratorParams, G1GeneratorPayload);
        let g1 = g1.point;
        let Ok(g2) = adapter.g2_generator(G2GeneratorParams, G2GeneratorPayload);
        let g2 = g2.point;
        let Ok(p1) = adapter.mul_g1(
            MulG1Params,
            MulG1Payload {
                point: g1.clone(),
                scalar: a.expose().clone(),
            },
        );
        let p1 = p1.product;
        let Ok(p2) = adapter.mul_g2(
            MulG2Params,
            MulG2Payload {
                point: g2.clone(),
                scalar: b.expose().clone(),
            },
        );
        let p2 = p2.product;

        let iterations = self.iterations.get();

        let start = Instant::now();
        for _ in 0..iterations {
            let Ok(_) = adapter.mul_g1(
                MulG1Params,
                MulG1Payload {
                    point: p1.clone(),
                    scalar: a.expose().clone(),
                },
            );
        }
        let mul_g1 = start.elapsed() / iterations;

        let start = Instant::now();
        for _ in 0..iterations {
            let Ok(_) = adapter.mul_g2(
                MulG2Params,
                MulG2Payload {
                    point: p2.clone(),
                    scalar: b.expose().clone(),
                },
            );
        }
        let mul_g2 = start.elapsed() / iterations;

        let start = Instant::now();
        for _ in 0..iterations {
            let Ok(_) = adapter.msm_g1(
                MsmG1Params,
                MsmG1Payload {
                    terms: vec![
                        MsmG1Term {
                            base: p1.clone(),
                            scalar: a.expose().clone(),
                        },
                        MsmG1Term {
                            base: g1.clone(),
                            scalar: b.expose().clone(),
                        },
                    ],
                },
            );
        }
        let msm_g1 = start.elapsed() / iterations;

        let start = Instant::now();
        for _ in 0..iterations {
            let Ok(_) = adapter.msm_g2(
                MsmG2Params,
                MsmG2Payload {
                    terms: vec![
                        MsmG2Term {
                            base: p2.clone(),
                            scalar: b.expose().clone(),
                        },
                        MsmG2Term {
                            base: g2.clone(),
                            scalar: a.expose().clone(),
                        },
                    ],
                },
            );
        }
        let msm_g2 = start.elapsed() / iterations;

        let start = Instant::now();
        for _ in 0..iterations {
            let Ok(_) = adapter.pairing_product_is_one(
                PairingProductIsOneParams,
                PairingProductIsOnePayload {
                    terms: vec![
                        PairingProductTerm {
                            g1: p1.clone(),
                            g2: g2.clone(),
                        },
                        PairingProductTerm {
                            g1: g1.clone(),
                            g2: p2.clone(),
                        },
                    ],
                },
            );
        }
        let pairing_product = start.elapsed() / iterations;

        Ok(PairingBenchmarkSuccessReturn {
            concrete: P::CONCRETE,
            iterations: self.iterations,
            timings: PairingOperationTimings {
                mul_g1,
                mul_g2,
                msm_g1,
                msm_g2,
                pairing_product,
            },
        })
    }
}
