#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::as_conversions
)]

use super::create_pairing;
use super::interface::{
    ConsumePairingParams, ConsumePairingPayload, CreatePairingDeps, CreatePairingErrorReturn,
    CreatePairingPayload, IPairingAdapter, IPairingConsumer, PairingConcrete, PairingCurve,
    PairingDeclaration, PrecompileEncoding, TargetGroupEncodingIdentifier,
    VerifierGroupArithmetic,
};
use super::mock::{CreatePairingParamsOverrides, build_create_pairing_params};
use core::cell::Cell;

struct DeclarationProbe;

impl IPairingConsumer for DeclarationProbe {
    type Output = PairingDeclaration;

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        payload.declaration
    }
}

struct CallProbe {
    called: Cell<bool>,
}

impl IPairingConsumer for CallProbe {
    type Output = ();

    fn consume_pairing<P: IPairingAdapter>(
        &self,
        _params: ConsumePairingParams,
        _payload: ConsumePairingPayload<P>,
    ) -> Self::Output {
        self.called.set(true);
    }
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed once and
///   handed, with its `DECLARATION`, to the consumer, whose output is returned
///   in `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bn254Arkworks` with the default
///   admitted encodings; a `DeclarationProbe` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `success.output.curve` is `PairingCurve::Bn254` and
///   `success.output.precompile_encoding` is `PrecompileEncoding::Eip196Eip197`.
#[test]
fn create_pairing_hands_the_consumer_the_bn254_arkworks_concrete_and_its_declaration() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: DeclarationProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(matches!(success.output.curve, PairingCurve::Bn254));
    assert!(matches!(
        success.output.precompile_encoding,
        PrecompileEncoding::Eip196Eip197
    ));
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed once and
///   handed, with its `DECLARATION`, to the consumer, whose output is returned
///   in `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bn254Halo2curves` with the default
///   admitted encodings; a `DeclarationProbe` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `success.output.curve` is `PairingCurve::Bn254` and
///   `success.output.precompile_encoding` is `PrecompileEncoding::Eip196Eip197`.
#[test]
fn create_pairing_hands_the_consumer_the_bn254_halo2curves_concrete_and_its_declaration() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: DeclarationProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(matches!(success.output.curve, PairingCurve::Bn254));
    assert!(matches!(
        success.output.precompile_encoding,
        PrecompileEncoding::Eip196Eip197
    ));
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed once and
///   handed, with its `DECLARATION`, to the consumer, whose output is returned
///   in `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bls12381Arkworks` with the default
///   admitted encodings; a `DeclarationProbe` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `success.output.curve` is `PairingCurve::Bls12381`,
///   `success.output.verifier_group_arithmetic` is
///   `VerifierGroupArithmetic::BothGroups`, and
///   `success.output.precompile_encoding` is `PrecompileEncoding::Eip2537`.
#[test]
fn create_pairing_hands_the_consumer_the_bls12_381_arkworks_concrete_and_its_declaration() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: DeclarationProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(matches!(success.output.curve, PairingCurve::Bls12381));
    assert!(matches!(
        success.output.verifier_group_arithmetic,
        VerifierGroupArithmetic::BothGroups
    ));
    assert!(matches!(
        success.output.precompile_encoding,
        PrecompileEncoding::Eip2537
    ));
}

/// Contract: admitted — the named concrete's declared encoding is among
///   `params.supported_encodings`, so the concrete is constructed once and
///   handed, with its `DECLARATION`, to the consumer, whose output is returned
///   in `Ok(CreatePairingSuccessReturn { output })`.
/// Arrange: params naming `PairingConcrete::Bls12381Halo2curves` with the
///   default admitted encodings; a `DeclarationProbe` consumer.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  `success.output.curve` is `PairingCurve::Bls12381`,
///   `success.output.verifier_group_arithmetic` is
///   `VerifierGroupArithmetic::BothGroups`, and
///   `success.output.precompile_encoding` is `PrecompileEncoding::Eip2537`.
#[test]
fn create_pairing_hands_the_consumer_the_bls12_381_halo2curves_concrete_and_its_declaration() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: DeclarationProbe,
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let Ok(success) = create_pairing(&deps, params, CreatePairingPayload) else {
        panic!("an admitted concrete is constructed and consumed");
    };

    // Assert
    assert!(matches!(success.output.curve, PairingCurve::Bls12381));
    assert!(matches!(
        success.output.verifier_group_arithmetic,
        VerifierGroupArithmetic::BothGroups
    ));
    assert!(matches!(
        success.output.precompile_encoding,
        PrecompileEncoding::Eip2537
    ));
}

/// Contract: unsupported encoding — the named concrete's declared encoding is
///   not in `params.supported_encodings`, so the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)` with nothing
///   constructed and the consumer not called.
/// Arrange: params naming `PairingConcrete::Bls12381Arkworks` while the
///   declared encodings admit only EIP-196/EIP-197; a `CallProbe` consumer
///   recording whether it was called.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  the return is `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`
///   and the consumer's `called` is still `false`.
#[test]
fn create_pairing_refuses_a_concrete_whose_encoding_the_chain_does_not_deploy() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: CallProbe {
            called: Cell::new(false),
        },
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        supported_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert!(matches!(
        result,
        Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    ));
    assert!(!deps.consumer.called.get());
}

/// Contract: unsupported target-group encoding — the named concrete's declared
///   `DECLARATION.target_group_encoding` is not equal to
///   `params.target_group_encoding`, so the factory refuses before construction
///   with `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`,
///   nothing constructed and the consumer not called (CR-10).
/// Arrange: params naming `PairingConcrete::Bn254Arkworks` with
///   `target_group_encoding` overridden to
///   `TargetGroupEncodingIdentifier::Bls12381V1`; a `CallProbe` consumer
///   recording whether it was called.
/// Act:     `create_pairing` over the deps and params.
/// Assert:  the return is
///   `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)` and the
///   consumer's `called` is still `false`.
#[test]
fn create_pairing_refuses_a_concrete_whose_target_group_encoding_the_suite_does_not_require() {
    // Arrange
    let deps = CreatePairingDeps {
        consumer: CallProbe {
            called: Cell::new(false),
        },
    };
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert!(matches!(
        result,
        Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)
    ));
    assert!(!deps.consumer.called.get());
}
