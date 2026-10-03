#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::interface::PairingBenchmarkErrorReturn;
use super::mock::{PairingBenchmarkConstructorParamsOverrides, build_pairing_benchmark};
use core::num::NonZeroU32;
use core::time::Duration;
use pairing::{
    CreatePairingDeps, CreatePairingParamsOverrides, CreatePairingPayload, PairingConcrete,
    SampleUniformScalarErrorReturn, build_create_pairing_params, create_pairing,
};
use random::{
    CreateRandomSourceDeps, CreateRandomSourceParamsOverrides, CreateRandomSourcePayload,
    FillBytesParams, FillBytesPayload, FillBytesReturn, IRandomSourceAdapter,
    MockIRandomSourceAdapter, RandomSourceDeclaration, RandomSourceKind,
    build_create_random_source_params, build_random_source_declaration, create_random_source,
};

struct WrongLengthRandomSource;

impl IRandomSourceAdapter for WrongLengthRandomSource {
    fn declaration(&self) -> RandomSourceDeclaration {
        build_random_source_declaration(Default::default())
    }

    fn fill_bytes(&self, params: FillBytesParams, _payload: FillBytesPayload) -> FillBytesReturn {
        MockIRandomSourceAdapter.fill_bytes(params, FillBytesPayload { length: 0 })
    }
}

/// Contract: handed a concrete by the factory, the benchmark returns a nonzero
///   mean time for each of the five operations together with the selection and
///   iteration count.
/// Arrange: the operating-system random source and four iterations; the
///   BN254-arkworks concrete.
/// Act:     create_pairing with the benchmark as consumer, then its output.
/// Assert:  output.concrete is PairingConcrete::Bn254Arkworks; output.iterations
///   is the arranged count; timings.mul_g1, timings.mul_g2, timings.msm_g1,
///   timings.msm_g2, and timings.pairing_product are each greater than
///   Duration::ZERO.
#[test]
fn consume_pairing_times_every_operation_on_the_bn254_arkworks_concrete() {
    // Arrange
    let Ok(source) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let Some(iterations) = NonZeroU32::new(4) else {
        panic!("four is nonzero")
    };
    let benchmark = build_pairing_benchmark(PairingBenchmarkConstructorParamsOverrides {
        random: Some(source.adapter),
        iterations: Some(iterations),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps {
            consumer: benchmark,
        },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the admitted concrete constructs")
    };
    let Ok(output) = success.output else {
        panic!("the benchmark returns its timings")
    };

    // Assert
    assert!(output.concrete == PairingConcrete::Bn254Arkworks);
    assert!(output.iterations == iterations);
    assert!(output.timings.mul_g1 > Duration::ZERO);
    assert!(output.timings.mul_g2 > Duration::ZERO);
    assert!(output.timings.msm_g1 > Duration::ZERO);
    assert!(output.timings.msm_g2 > Duration::ZERO);
    assert!(output.timings.pairing_product > Duration::ZERO);
}

/// Contract: handed a concrete by the factory, the benchmark returns a nonzero
///   mean time for each of the five operations together with the selection and
///   iteration count.
/// Arrange: the operating-system random source and four iterations; the
///   BN254-halo2curves concrete.
/// Act:     create_pairing with the benchmark as consumer, then its output.
/// Assert:  output.concrete is PairingConcrete::Bn254Halo2curves;
///   output.iterations is the arranged count; timings.mul_g1, timings.mul_g2,
///   timings.msm_g1, timings.msm_g2, and timings.pairing_product are each
///   greater than Duration::ZERO.
#[test]
fn consume_pairing_times_every_operation_on_the_bn254_halo2curves_concrete() {
    // Arrange
    let Ok(source) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let Some(iterations) = NonZeroU32::new(4) else {
        panic!("four is nonzero")
    };
    let benchmark = build_pairing_benchmark(PairingBenchmarkConstructorParamsOverrides {
        random: Some(source.adapter),
        iterations: Some(iterations),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps {
            consumer: benchmark,
        },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the admitted concrete constructs")
    };
    let Ok(output) = success.output else {
        panic!("the benchmark returns its timings")
    };

    // Assert
    assert!(output.concrete == PairingConcrete::Bn254Halo2curves);
    assert!(output.iterations == iterations);
    assert!(output.timings.mul_g1 > Duration::ZERO);
    assert!(output.timings.mul_g2 > Duration::ZERO);
    assert!(output.timings.msm_g1 > Duration::ZERO);
    assert!(output.timings.msm_g2 > Duration::ZERO);
    assert!(output.timings.pairing_product > Duration::ZERO);
}

/// Contract: handed a concrete by the factory, the benchmark returns a nonzero
///   mean time for each of the five operations together with the selection and
///   iteration count.
/// Arrange: the operating-system random source and four iterations; the
///   BLS12-381-arkworks concrete.
/// Act:     create_pairing with the benchmark as consumer, then its output.
/// Assert:  output.concrete is PairingConcrete::Bls12381Arkworks;
///   output.iterations is the arranged count; timings.mul_g1, timings.mul_g2,
///   timings.msm_g1, timings.msm_g2, and timings.pairing_product are each
///   greater than Duration::ZERO.
#[test]
fn consume_pairing_times_every_operation_on_the_bls12_381_arkworks_concrete() {
    // Arrange
    let Ok(source) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let Some(iterations) = NonZeroU32::new(4) else {
        panic!("four is nonzero")
    };
    let benchmark = build_pairing_benchmark(PairingBenchmarkConstructorParamsOverrides {
        random: Some(source.adapter),
        iterations: Some(iterations),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps {
            consumer: benchmark,
        },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the admitted concrete constructs")
    };
    let Ok(output) = success.output else {
        panic!("the benchmark returns its timings")
    };

    // Assert
    assert!(output.concrete == PairingConcrete::Bls12381Arkworks);
    assert!(output.iterations == iterations);
    assert!(output.timings.mul_g1 > Duration::ZERO);
    assert!(output.timings.mul_g2 > Duration::ZERO);
    assert!(output.timings.msm_g1 > Duration::ZERO);
    assert!(output.timings.msm_g2 > Duration::ZERO);
    assert!(output.timings.pairing_product > Duration::ZERO);
}

/// Contract: handed a concrete by the factory, the benchmark returns a nonzero
///   mean time for each of the five operations together with the selection and
///   iteration count.
/// Arrange: the operating-system random source and four iterations; the
///   BLS12-381-halo2curves concrete.
/// Act:     create_pairing with the benchmark as consumer, then its output.
/// Assert:  output.concrete is PairingConcrete::Bls12381Halo2curves;
///   output.iterations is the arranged count; timings.mul_g1, timings.mul_g2,
///   timings.msm_g1, timings.msm_g2, and timings.pairing_product are each
///   greater than Duration::ZERO.
#[test]
fn consume_pairing_times_every_operation_on_the_bls12_381_halo2curves_concrete() {
    // Arrange
    let Ok(source) = create_random_source(
        &CreateRandomSourceDeps,
        build_create_random_source_params(CreateRandomSourceParamsOverrides {
            kind: Some(RandomSourceKind::OperatingSystem),
        }),
        CreateRandomSourcePayload,
    );
    let Some(iterations) = NonZeroU32::new(4) else {
        panic!("four is nonzero")
    };
    let benchmark = build_pairing_benchmark(PairingBenchmarkConstructorParamsOverrides {
        random: Some(source.adapter),
        iterations: Some(iterations),
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps {
            consumer: benchmark,
        },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the admitted concrete constructs")
    };
    let Ok(output) = success.output else {
        panic!("the benchmark returns its timings")
    };

    // Assert
    assert!(output.concrete == PairingConcrete::Bls12381Halo2curves);
    assert!(output.iterations == iterations);
    assert!(output.timings.mul_g1 > Duration::ZERO);
    assert!(output.timings.mul_g2 > Duration::ZERO);
    assert!(output.timings.msm_g1 > Duration::ZERO);
    assert!(output.timings.msm_g2 > Duration::ZERO);
    assert!(output.timings.pairing_product > Duration::ZERO);
}

/// Contract: a deliberately malformed draw the sampling bound rejects is
///   returned in the error arm before anything is timed.
/// Arrange: the test-local WrongLengthRandomSource, which reports the default
///   declaration but returns an empty draw for the nonzero request; the
///   BN254-arkworks concrete.
/// Act:     create_pairing with the benchmark as consumer, then its output.
/// Assert:  the output is Err(PairingBenchmarkErrorReturn::SampleScalar(
///   SampleUniformScalarErrorReturn::WrongLength { expected: 64, actual: 0 })).
#[test]
fn consume_pairing_returns_the_sampling_error_for_a_draw_of_the_wrong_length() {
    // Arrange
    let benchmark = build_pairing_benchmark(PairingBenchmarkConstructorParamsOverrides {
        random: Some(Box::new(WrongLengthRandomSource)),
        ..Default::default()
    });
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(
        &CreatePairingDeps {
            consumer: benchmark,
        },
        params,
        CreatePairingPayload,
    ) else {
        panic!("the admitted concrete constructs")
    };

    // Assert
    assert!(matches!(
        success.output,
        Err(PairingBenchmarkErrorReturn::SampleScalar(
            SampleUniformScalarErrorReturn::WrongLength {
                expected: 64,
                actual: 0
            }
        ))
    ));
}
